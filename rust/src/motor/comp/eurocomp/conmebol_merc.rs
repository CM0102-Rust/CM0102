use super::*;

/// Club k of the drawn list (+0xee pointers).
pub(crate) unsafe fn conmebol_merc_drawn_club(this: League, k: usize) -> usize {
    read_usize(read_usize(this.addr() + conmebol_merc::DRAWN_CLUBS) + k * 4) // cqr11: o sorteado k
}

/// `update_for_date(today, full)` (virtual +0x14): the Copa Mercosur's day. Rule: the Mercosur has 20 invited clubs
/// (the clubs the South American rules invite, shuffled) in 5 groups of 4 [inference: six rounds on Tuesdays, from 29
/// July to 14 October]; the season turns on 1 June; on the draw day group 1 stays in the cup's own table (drawn clubs 0
/// to 3) and groups 2 to 5 are stages (4g + 4 to 4g + 7); the 5 winners and the 3 best runners-up go to the
/// quarter-finals.
///
/// First the turn of the season: on 1 June of today's year (the date from `exe::date_date_549f70`(1, 5, year, −1): lea
/// ecx, local; push −1; push year; push 5; push 1 -> thiscall, ret 0x10, pure: the day of the year from the month
/// tables, the month counted from 0), if the cup's year (`.year()`, signed) is not today's, virtual
/// `comp_vtable::START_NEW_SEASON` (the new season: mov ecx, this -> thiscall, ret 0).
///
/// When the draw has not been done (`.active()` = 0) and with `full`, on the draw day (the date from
/// 0x549ef0(`.next_draw_day()`, `.next_draw_year_offset()` + `.year()`): the day of the year and the relative year;
/// compared with today's, +0 the day of the year and +2 the year, unsigned): the clubs
/// (`exe::conmebolmerc_select_invited_clubs`, the 20 invited clubs shuffled: mov ecx, this -> thiscall, ret 0), the
/// group 1 table (`exe::conmebolmerc_create_first_group_table`, thiscall, ret 0; 0: the box of line 0x396), the
/// fixtures (`exe::league_create_fixtures` League::create_fixtures, thiscall, ret 0; 0: the box of line 0x39b), the
/// sorted table (`exe::league_sort_table` push 0; mov ecx -> thiscall, ret 4; 0: the box of line 0x3a1), the stages of
/// groups 0..3 (`exe::conmebolmerc_create_group_stage`: push g; mov ecx -> thiscall, ret 4), the predicted positions
/// (virtual `comp_vtable::SET_PREDICTED_POSITIONS`, thiscall, ret 0), the list `conmebol_merc::DRAWN_CLUBS` freed
/// (`exe::crt_free`: push; add esp, 4 -> cdecl) and zeroed, and `.set_active(1)`. An error gives the box of its line
/// and goes on. Then the league day (`exe::league_update_for_date`: push full; push today; mov ecx, this -> thiscall,
/// ret 8), whose return value it passes on.
/// Original: `ConmebolMerc::update_for_date` (0x4c5c90, conmebol_merc.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let year_of_today = core::ptr::read_unaligned((today + 2) as *const u16);
    let mut junho = [0u8; 8];
    exe::date_date_549f70((junho.as_mut_ptr()) as *mut u8 as *mut [u16; 4], 1, 5, year_of_today as u32, (-1) as i32 as u32);
    if core::ptr::read_unaligned(junho.as_ptr().add(2) as *const u16) == year_of_today
        && core::ptr::read_unaligned(junho.as_ptr() as *const u16) == core::ptr::read_unaligned(today as *const u16)
        && this.year() as i32 != year_of_today as i32
    {
        let turned: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::START_NEW_SEASON));
        turned(this.addr());
    }
    if this.active() == 0 && full != 0 {
        let day = this.next_draw_day() as u16; // cqr11: the draw day
        let year = (this.next_draw_year_offset() as u16).wrapping_add(this.year() as u16);
        if year == core::ptr::read_unaligned((today + 2) as *const u16) && day == core::ptr::read_unaligned(today as *const u16) {
            covered!("cqr_11 mercosul draw");
            exe::conmebolmerc_select_invited_clubs(this);
            if exe::conmebolmerc_create_first_group_table(this) == 0 {
                show_error_at(0x9a7e5c, 0x396);
            }
            if exe::league_create_fixtures(this) == 0 {
                show_error_at(0x9a7e5c, 0x39b);
            }
            if exe::league_sort_table(this, 0) == 0 {
                show_error_at(0x9a7e5c, 0x3a1);
            }
            for g in 0..4i32 {
                exe::conmebolmerc_create_group_stage(this, g);
            }
            let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
            predicted(this.addr());
            exe::crt_free(read_usize(this.addr() + conmebol_merc::DRAWN_CLUBS));
            write_usize(this.addr() + conmebol_merc::DRAWN_CLUBS, 0);
            this.set_active(1);
        }
    }
    exe::league_update_for_date(this, today, full) as u32 
}

// simplification: the clubs (shuffled), the tables, the matches and the league day do not run twice. Original only; the plan
// on draw day (the copy’s condition): +0x45 = 1, the list +0xee zeroed, the first group’s table and the 4 stages
// created and the current stage (+0x30) the last of them.

/// Is today June 1 with the cup from another year (rollover day, draw not checked)?
pub(crate) unsafe fn is_rollover_day(this: League, today: usize) -> bool {
    let year_of_today = core::ptr::read_unaligned((today + 2) as *const u16);
    let mut junho = [0u8; 8];
    exe::date_date_549f70((junho.as_mut_ptr()) as *mut u8 as *mut [u16; 4], 1, 5, year_of_today as u32, (-1) as i32 as u32);
    core::ptr::read_unaligned(junho.as_ptr() as *const u16) == core::ptr::read_unaligned(today as *const u16) && this.year() as i32 != year_of_today as i32
}

/// Builds the table of group 1 of the Copa Mercosur. Rule: group 1 has 4 clubs, the first 4 drawn, each in the row of
/// its order; human managers get the group draw news.
///
/// 4 teams (`.set_club_count()`), the table `.set_table()` (`exe::crt_malloc`(0xec): push 0xec; add esp, 4 -> cdecl; 4
/// rows of 0x3b bytes; without memory, the box of line 0x155 and returns 0), each row i with drawn club i
/// (`conmebol_merc::DRAWN_CLUBS`) (`exe::league_init_table_row` League::init_table_row: push 0; push 0; push club; push
/// i; mov ecx, this -> thiscall, ret 0x10), the group draw news (`exe::league_send_group_draw_news`: mov ecx, this ->
/// thiscall, ret 0), and returns 1. The same body as `init_group_a_table` of the African Cup (only the file and the box
/// line change).
/// Original: `ConmebolMerc::create_first_group_table` (0x4c4e60, conmebol_merc.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_create_first_group_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    this.set_table(t);
    if t == 0 {
        show_error_at(0x9a7e5c, 0x155);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < this.club_count() {
        covered!("cqr_11 first group row");
        // simplification: the index goes in a dword whose 3 high bytes are stack leftovers; 0x687430 reads only the byte (char)
        exe::league_init_table_row(this, i as u8 as u32, conmebol_merc_drawn_club(this, i as usize), 0, 0);
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

// simplification: the table is allocated (and each row allocates the history) and the news goes to the managers: does not run twice. Only the
// original; the plan: returns 1, 4 teams and in row i (0x3b bytes) the drawn team i with position i (+4).

/// Creates the stage of group g + 1 of the Copa Mercosur (g from 0 to 3, on the draw day). Rule: each of the other
/// groups is a league stage with the next 4 drawn clubs (4g + 4 to 4g + 7), the cup's rules and year and the cup's
/// round table for the group; human managers get the news.
///
/// SEH frame only for the stage's new: no destructor on the normal path. The teams go into a vector of `.club_count()`
/// pointers (`exe::crt_malloc`(`.club_count()` << 2): push; add esp, 4 -> cdecl; without memory, the box of line 0x1fb
/// and 0): drawn clubs 4g + 4 to 4g + 7 (`conmebol_merc::DRAWN_CLUBS`); the round table through virtual
/// `comp_vtable::CREATE_SCHEDULE` (push 0; push &id; push &rounds; push g; mov ecx, this -> thiscall, ret 0x10); the
/// stage (`exe::crt_operator_new`(0xee) push; add esp, 4 -> cdecl; the League_Stage constructor
/// `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58, 22 arguments: the cup, the teams (`.club_count()`, word), the
/// vector, `.legs()` (word), `.competition()`, the round table, the rounds (word), `.win_points()`, `.draw_points()`,
/// `.match_category()`, &`league::TIEBREAKS`, &`league::PROMOTED_COUNT`, the year `.year()` (word), g, the id (word),
/// `.importance()`, 2, 0, `.league_flags()` (word), −1, 0, 2; the original pushes the words and bytes with garbage in
/// the high bytes, which the constructor does not read: the port passes clean values) in `.stages()`[g]; the vector and
/// the round table freed (`exe::crt_free`). Without a stage: the box of line 0x211 and 0. With it: the group news
/// (`exe::league_send_group_draw_news` on the stage), `.set_current_stage(g)` and 1.
/// Original: `ConmebolMerc::create_group_stage` (0x4c51d0, conmebol_merc.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_create_group_stage(this: usize, g: i32) -> u32 {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let teams = exe::crt_malloc(((this.club_count() as i32) << 2) as u32);
    if teams == 0 {
        show_error_at(0x9a7e5c, 0x1fb);
        return 0;
    }
    let mut k = g * 4 + 4;
    while k < g * 4 + 8 {
        let n = (k - (g * 4 + 4)) as usize;
        covered!("cqr_11 group team");
        write_usize(teams + n * 4, conmebol_merc_drawn_club(this, k as usize)); // cqr11: the team in the group slot
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
        show_error_at(0x9a7e5c, 0x211);
        return 0;
    }
    exe::league_send_group_draw_news(f);
    this.set_current_stage(g as usize);
    1
}

// simplification: the stage is allocated and the news goes to the managers: does not run twice. Original only; the plan: returns 1 with the
// stage in [+0xc][g], +0x30 = g, the stage with the teams and the cup year and, in its table (+0xb1, rows of 0x3b bytes), the
// drawn 4g + 4 to 4g + 7 (in any order: the constructor sorts the table).

/// Writes one group round (a 0x41-byte record, like the league rounds).
///
/// The date goes through `league_round_set_date` (day, month with 0 = January, weekday, code), with no years to add and value 0; and
/// match 0 is left empty (`store_round_match` with −1, −1, −1 and value 0).
pub(crate) type ConmebolMercGroupRound = (u32, u32, u32, u32);

/// Shared body of virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`, thiscall, ret 0x10; the stage read
/// as a signed byte) of the South American cups: group rounds below `limite`, knockout rounds at it, nothing above.
///
/// Below `limite` (−1 included), the groups: é_liga (if requested) = 1, rodadas = the number of group rounds, extra =
/// stage + 0x3e9 (16 bits); the table of 0x41 bytes per round (`exe::crt_malloc`; when out of memory, the error box `linha_g`
/// and returns 0) with the dates of `grupos`.
///
/// At stage `limite`, the knockout: é_liga = 0, rodadas = the size of `mata`, extra = 0; the table of 0x68 bytes per
/// round (when out of memory, the error box `linha_m`), the rounds written by `eng_cc_cup_write_cup_round` with the
/// cup's year +0x40 (signed) and the words of `a_mais` (round, offset, value).
///
/// Above `limite`: returns 0 without writing anything.
pub(crate) unsafe fn south_american_cup_schedule(this: League, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32, limit: i8, groups: &[ConmebolMercGroupRound],
    knockout: &[EngCcCupCupRound], more: &[(usize, usize, u16)], file: usize, row_g: u32, row_m: u32, name_g: &'static str, name: &'static str) -> usize {
    let f = stage as u8 as i8;
    if f < limit {
        if !league.is_null() {
            *league = 1;
        }
        *rounds = groups.len() as i16;
        *extra = (f as i16).wrapping_add(0x3e9);
        let t = exe::crt_malloc(groups.len() as u32 * 0x41);
        if t == 0 {
            show_error_at(file, row_g);
            return 0;
        }
        covered!(name_g);
        // the year goes with garbage in the high bytes in the original (the previous register with the word +0x40 over it); the
        // 0x68a160 only uses the low 16 bits: the copy passes the clean year
        let year = (this.year() as u16) as u32;
        for (r, &(dia, mes, semana, codigo)) in groups.iter().enumerate() {
            exe::league_round_set_date(t, r as i32, dia as i32, mes as i32, 0, semana as i32, codigo as i32, year, 0);
            exe::league_round_set_match_slot(t, r as i32, 0, -1, -1, -1, 0);
        }
        return t;
    }
    if f != limit {
        return 0;
    }
    if !league.is_null() {
        *league = 0;
    }
    *rounds = knockout.len() as i16;
    *extra = 0;
    let t = exe::crt_malloc(knockout.len() as u32 * 0x68);
    if t == 0 {
        show_error_at(file, row_m);
        return 0;
    }
    covered!(name);
    let year = this.year() as i32 as u32;
    for (i, r) in knockout.iter().enumerate() {
        eng_cc_cup_write_cup_round(t, i as u32, r, year);
    }
    for &(r, o, v) in more {
        write_u16(t + r * 0x68 + o, v);
    }
    t
}

/// Side-by-side comparer for South American cup schedules.
///
/// The original runs with the caller's outputs, the copy with its own outputs that start with the same values, from the
/// same [0xb67a34]; the outputs and the bytes written in the two tables (groups or knockout, the latter including the
/// words of `a_mais`) must be equal; the copy's table is freed (`exe::crt_free`). Above the knockout stage, only the original
/// runs.
pub(crate) unsafe fn compare_south_american_cup_schedule(name: &str, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32, limit: i8, more: &[(usize, usize, u16)],
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let f = stage as u8 as i8;
    if f > limit {
        // simplification: above the knockout neither of the two writes anything; the outputs may not even exist
        let a = o();
        SCOREBOARD.record(a == 0, || format!("{}(stage {}) returned {:#x}", name, f, a));
        return a;
    }
    let (mut r2, mut e2) = (*rounds, *extra);
    let mut l2 = if league.is_null() { 0 } else { *league };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, o, || c(&mut r2, &mut e2, if league.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let outs = *rounds == r2 && *extra == e2 && (league.is_null() || *league == l2);
    let mais = |t: usize| more.iter().flat_map(|&(r, o, _)| core::slice::from_raw_parts((t + r * 0x68 + o) as *const u8, 2).to_vec()).collect::<Vec<u8>>();
    let table = if a != 0 && b != 0 {
        if f < limit {
            written_group_schedule_bytes(a, *rounds) == written_group_schedule_bytes(b, r2)
        } else {
            eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, r2) && mais(a) == mais(b)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(igual && outs && table, || format!("{}(stage {}): outs {} ({} x {}) table {}", name, f, outs, *rounds, r2, table));
    a
}

/// A Libertadores, grupos: seis rodadas às terças [inferência: dia da semana 2], de 25 de fevereiro a 1º de abril.
pub(crate) const LIBERTADORES_GROUP_ROUNDS: [ConmebolMercGroupRound; 6] = [(0x19, 1, 2, 2), (4, 2, 2, 2), (0xb, 2, 2, 2), (0x12, 2, 2, 2), (0x19, 2, 2, 2), (1, 3, 2, 2)]; // cqo13: the Libertadores groups

/// The Copa Libertadores knockout: four two-legged rounds from April to June.
///
/// The second leg is 14 days after the first in the round of 16 and 7 in the other rounds.
pub(crate) const LIBERTADORES_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 4] = [
    cql10r((2, 3, 0, 3), (0xf, 3, 0, 2, 2, 0), 0x14, (0, 4, 1), 7, (0x10, 8, 0x10, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqo13: the Libertadores round of 16
    cql10r((0x1e, 3, 0, 3), (0x14, 4, 0, 2, 2, 0), 0x78, (0, 4, 1), 8, (8, 4, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0x1c, 4, 0, 3), (3, 5, 0, 2, 2, 0), 0x82, (0, 4, 1), 8, (4, 2, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0xb, 5, 0, 3), (0x11, 5, 0, 2, 2, 0), 0x96, (0, 4, 1), 8, (2, 1, 0, 0), (0, 2, 7), (0, 0x1e8480, 0xc3500)),
];

/// A Mercosul, grupos: seis rodadas às terças, de 29 de julho a 14 de outubro.
pub(crate) const MERCOSUR_GROUP_ROUNDS: [ConmebolMercGroupRound; 6] = [(0x1d, 6, 2, 2), (0x13, 7, 2, 2), (2, 8, 2, 2), (0x10, 8, 2, 2), (0x1e, 8, 2, 2), (0xe, 9, 2, 2)]; // cqo13: the Mercosur groups

/// A Mercosul, mata-mata: quartas, semifinal e final em ida e volta, de outubro a dezembro (a volta 3 dias depois nas
/// quartas e na semifinal, 7 na final).
pub(crate) const MERCOSUR_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((0xf, 9, 0, 3), (0x1c, 9, 0, 2, 2, 0), 0x78, (0, 4, 1), 8, (8, 4, 8, 0), (0, 2, 3), (0, 0xffff_ffff, 0)),
    cql10r((5, 0xa, 0, 3), (0xb, 0xa, 0, 2, 2, 0), 0x82, (0, 4, 1), 8, (4, 2, 0, 0), (0, 2, 3), (0, 0xffff_ffff, 0)),
    cql10r((0x13, 0xa, 0, 3), (2, 0xb, 0, 2, 2, 0), 0x96, (0, 0x2000, 0x2000), 8, (2, 1, 0, 0), (0, 2, 7), (0, 0x1e8480, 0)),
];

/// A final da Mercosul grava ainda a palavra +0x11 = 3 [inferência: o desempate da final, pênaltis].
pub(crate) const MERCOSUR_FINAL_EXTRA_WRITES: [(usize, usize, u16); 1] = [(2, 0x11, 3)];

/// rule: the Copa Mercosur starts with six-round groups on Tuesdays, from 29/7 to 14/10 (id 0x3e9 + stage, up to stage
/// 3: 5 groups [inference]); the knockout (stage 4) over two legs: quarter-finals drawn on 15/10 and played on 28/10 (8
/// teams, return 3 days later), semi-final 5/11 and 11/11 (return 3 days later), final draw 19/11 and match on 2/12
/// (return 7 days later, tie-break 0x2000 and the word +0x11 = 3 [inference: penalties]); category 8; prize of 2
/// million to the winner
/// Original: `ConmebolMerc::create_schedule` (0x4c4930, conmebol_merc.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    south_american_cup_schedule(League(this), stage, rounds, extra, league, 4, &MERCOSUR_GROUP_ROUNDS, &MERCOSUR_KNOCKOUT_ROUNDS, &MERCOSUR_FINAL_EXTRA_WRITES, 0x9a7e5c, 0x97, 0xbd,
        "cqo_13 grupos da mercosul", "cqo_13 mata-mata da mercosul")
}

/// `ConmebolMerc::select_invited_clubs()` (`0x4c5850`, thiscall, ret 0, `conmebol_merc.cpp`): builds the shuffled
/// 20-slot list of clubs invited by the seeding rules.
///
/// Sets `+0xf2` = 20 and allocates the pointer list (`0x944e46` of 0x50 into `+0xee`; out of memory, the box of line
/// `0x2c6` and 0). Each club of the table ([`0xae23b4`], [`0xae235c`] − 2 × [`0xae2350`]) that the rules ([`0xae1000`],
/// `0x4c6890`: thiscall with the club) invite enters the list, without checking the space; the list is shuffled
/// (`0x4afb40` with `+0xf2`). Returns 1.
pub(crate) unsafe fn mercosur_select_invited_clubs(this: usize) -> (i32, i32) {
    write_u32(this + conmebol_merc::DRAWN_CLUB_COUNT, 0x14);
    let l = exe::crt_malloc(0x50);
    write_u32(this + conmebol_merc::DRAWN_CLUBS, l as u32);
    if l == 0 {
        show_error_at(0x9a7e5c, 0x2c6);
        return (0, 0);
    }
    let clubs = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut k = 0i16;
    let mut i = 0i32;
    while i < clubs() {
        let c = club_at(i as isize).addr();
        if exe::conmebolseeding_is_mercosur_invited_club(read_usize(G_CONMEBOL_SEEDING), c) != 0 {
            write_u32(read_usize(this + conmebol_merc::DRAWN_CLUBS) + k as usize * 4, c as u32);
            k = k.wrapping_add(1);
        }
        i += 1;
    }
    covered!("cqo_13 mercosul guests");
    // simplification: the original pushes the word +0xf2 with garbage in the high 16 bits (the previous eax); 0x4afb40 reads the word
    exe::comp_util_shuffle_pointer_array(read_usize(this + conmebol_merc::DRAWN_CLUBS), read_u16(this + conmebol_merc::DRAWN_CLUB_COUNT) as u32); // cqo13: embaralha os convidados
    (1, k as i32)
}

/// rule: the Copa Mercosur is invitation-only: the 20 clubs of the South American rules' fixed list (`is_mercosur_invited_club`), in club
/// table order, shuffled for the group draw
/// Original: `ConmebolMerc::select_invited_clubs` (0x4c5850, conmebol_merc.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_select_invited_clubs(this: usize) -> i32 {
    mercosur_select_invited_clubs(this).0
}

/// rule: the Mercosur moves on one stage when the current one ends; after the 4 group stages (on reaching 4) it builds
/// the quarter-finals; going past the last stage is an error
/// Original: `ConmebolMerc::advance_stage` (0x4c5140, conmebol_merc.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_advance_stage(this: usize) {
    con_merc_cup_advance_stage_cqs11_next_stage(League(this), 0x9a7e5c, 0x1d9, 4, 0x4c53d0, "cqs_11 quartas da mercosul");
}

/// Rule: the Mercosur records: in the groups (stages below 4) the qualified club gets the record of round 0x78 (weight
/// 0x28); in the knockout (stage 4) elimination and qualification weigh by round (50 to 100 over the 2 rounds) and the
/// winner and runner-up get the final records.
///
/// `ConmebolMerc::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44). Stage
/// (signed byte) below 4: outcome 6, the qualification (`european_cup_add_qualified_record`: 0, 0x78, 0x28), returns 0;
/// otherwise −1. Stage 4, weight = `scale_by_round`(0x32, 0x64, the fixture's `round_number()`, 2): 0xff, the
/// elimination (`european_cup_add_knocked_out_record`: `round_prefix()`, `round_name()`, the weight); 0, the
/// qualification with `round_prefix()` and the name word +0x6f of the fixture's round in the round table of stage 4
/// ([`cup::STAGES`][4] + `cup::ROUND_TABLE`; the high 16 bits come from a small product of the round: zero); 1, the
/// winner (0x86a800); 2, the runner-up (0x86b0b0); returns 0. Above 4: 0.
/// Original: `ConmebolMerc::update_staff_records_for_status` (0x4c5960, conmebol_merc.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let g = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < 4 {
        if d != 6 {
            return u32::MAX;
        }
        covered!("cqx_12 mercosur group qualified");
        european_cup_add_qualified_record(club, comp, 0, 0x78, 0x28);
    } else if f == 4 {
        match d {
            0xff => {
                let weight = scale_by_round(0x32, 0x64, g.round_number(), 2);
                european_cup_add_knocked_out_record(club, comp, g.round_prefix() as u16 as u32, g.round_name() as u16 as u32, weight);
            }
            0 => {
                let weight = scale_by_round(0x32, 0x64, g.round_number(), 2);
                let knockout = read_usize(read_usize(this + cup::STAGES) + 0x10);
                let name = read_u16(read_usize(knockout + cup::ROUND_TABLE).wrapping_add((g.round_number() as isize * cup_round::SIZE as isize) as usize) + 0x6f) as u32;
                european_cup_add_qualified_record(club, comp, g.round_prefix() as u16 as u32, name, weight);
            }
            1 => cup_record_final_result(0x86a800, club, g, final_),
            2 => cup_record_final_result(0x86b0b0, club, g, final_),
            _ => {}
        }
    }
    0
}

pub(crate) unsafe extern "thiscall" fn compare_conmebol_merc_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let plan = if f < 4 && d != 6 { u32::MAX } else { 0 };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_CONMEBOL_MERC_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == plan, || format!("conmebol_merc_update_staff_records_for_status({:#x}, {}, {}) returned {:#x}, plan {:#x}", club, d, f, r, plan));
    r
}

/// The Mercosur Cup quarter-finals, at the end of the five groups (SEH frame only for the stage `new`s, no destructor
/// on the normal path). A league stage of the runners-up (`exe::crt_operator_new`(0xee) and the League_Stage
/// constructor `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58, 22 arguments: the cup, 5, no teams, 0, the
/// competition record (`league::COMPETITION`), no table, 0, `league::WIN_POINTS`, `league::DRAW_POINTS`, 0,
/// &`league::TIEBREAKS`, &`league::PROMOTED_COUNT`, the year, -2, 0, 0, 1, 0, 2, -1, 0, 2; words and bytes clean), with
/// row 1 (the runner-up) of each group copied (`league_table_row::SIZE` bytes) into its row and the dword
/// `league_table_row::POSITION_HISTORY` zeroed (the points stay [inference]), and the table sorted
/// (`exe::league_sort_table`(stage, 0)). An array of 8 clubs (`exe::crt_malloc`(0x20); out of memory, the box for line
/// 0x248 and it returns without destroying the runners-up stage): the first `league::PROMOTED_COUNT` of group A
/// (`league::TABLE`) and of each of the groups B..E (stages 0..3, `exe::league_get_table_row`), and the first three of
/// the runners-up table. Each of those three gets one more place in its group (`league::PROMOTED_COUNT` of the group +
/// 1), its row marked (`league_table_row::STATUS` = 6) and the status (the virtual `comp_vtable::ANNOUNCE_TEAM_STATUS`(club,
/// 6, group, 0, 0): 5 pushes; mov ecx -> thiscall). Runners-up 4 and 5 stay out (the virtual
/// `comp_vtable::SET_CLUB_PROGRESS`(club, -1, 3, 3, 3)). Whoever played the groups and is not among the 8 gets the
/// failed-to-qualify record (`exe::staff_records_add_failed_to_qualify_record`(`G_STAFF_RECORDS`, club, competition
/// record, 0x41c, 0x28): 4 pushes; mov ecx -> thiscall, ret 0x10). The runners-up stage is destroyed (the virtual
/// `comp_vtable::SCALAR_DELETING_DESTRUCTOR` with 1). The quarter-finals: the round table of stage 4 (the virtual
/// `comp_vtable::CREATE_SCHEDULE`), the stage (`exe::crt_operator_new`(0xb2) and the Cup_Stage constructor
/// `FN_CUP_STAGE_CUP_STAGE`, ret 0x3c: the cup, 8, the array, the rounds, the competition record, the table, the year,
/// 4, 2, the extra, 0x14, 0, 0, 0, 0) in `league::STAGES`[4]; the array and the table freed; without a stage, the box
/// for line 0x2ab; with it, the draw of the first round (`exe::cup_draw_round`(stage, 0): push 0; mov ecx -> thiscall,
/// ret 4).
/// Original: `ConmebolMerc::create_quarter_final_stage` (0x4c53d0, conmebol_merc.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_create_quarter_final_stage(this: usize) {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let league_stage: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, i32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    #[allow(clippy::type_complexity)]
    let cup_stage: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let b = |d: usize| read_u8(this.addr() + d) as u32;
    let p = exe::crt_operator_new(0xee);
    let runners_up = League(if p != 0 {
        league_stage(p, this.addr(), 5, 0, 0, this.competition().addr(), 0, 0, b(league::WIN_POINTS), b(league::DRAW_POINTS), 0, this.addr() + league::TIEBREAKS, this.addr() + league::PROMOTED_COUNT, this.year() as u16 as u32, -2, 0, 0, 1, 0, 2, -1, 0, 2)
    } else {
        0
    });
    for g in 0..5usize {
        let source = read_usize(conmebol_merc_group(this.addr(), g as i8 - 1) + league::TABLE) + league_table_row::SIZE; // cqv11: the row of the group runner-up
        let target = runners_up.table() + g * league_table_row::SIZE;
        core::ptr::copy(source as *const u8, target as *mut u8, league_table_row::SIZE);
        write_u32(runners_up.table() + g * league_table_row::SIZE + league_table_row::POSITION_HISTORY, 0);
    }
    exe::league_sort_table(runners_up, 0);
    let v = exe::crt_malloc(0x20);
    if v == 0 {
        return show_error_at(0x9a7e5c, 0x248);
    }
    let mut k = 0usize;
    for i in 0..(this.promoted_count() as i8).max(0) as usize {
        write_usize(v + k * 4, read_usize(this.table() + i * league_table_row::SIZE));
        k += 1;
    }
    for g in 0..4usize {
        let stage = read_usize(this.stages() + g * 4);
        for i in 0..(this.promoted_count() as i8).max(0) as i32 {
            write_usize(v + k * 4, read_usize(exe::league_get_table_row(stage, i)));
            k += 1;
        }
    }
    for s in 0..3i32 {
        covered!("cqv_11 best runners-up");
        write_usize(v + (k + s as usize) * 4, read_usize(exe::league_get_table_row(runners_up, s))); // cqv11: runner-up s into the quarter-finals
        for g in -1i8..4 {
            let group = conmebol_merc_group(this.addr(), g);
            let club = read_usize(exe::league_get_table_row(runners_up, s));
            if read_usize(exe::league_get_table_row(group, 1)) == club {
                let places = group + league::PROMOTED_COUNT;
                write_u8(places, read_u8(places).wrapping_add(1));
                write_u8(exe::league_get_table_row(group, 1) + league_table_row::STATUS, 6);
                let status: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::ANNOUNCE_TEAM_STATUS));
                // simplification: the original pushes the dword whose low byte is the group (the high ones are those of the count of 8, < 256: 0)
                status(this.addr(), read_usize(exe::league_get_table_row(runners_up, s)), 6, g as u8 as u32, 0, 0);
                break;
            }
        }
    }
    for s in 3..5i32 {
        let out: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_CLUB_PROGRESS));
        out(this.addr(), read_usize(exe::league_get_table_row(runners_up, s)), -1, 3, 3, 3);
    }
    for g in -1i8..4 {
        let group = conmebol_merc_group(this.addr(), g);
        for r in 0..read_i16(group + league::CLUB_COUNT).max(0) as usize {
            let club = read_usize(read_usize(group + league::TABLE) + r * league_table_row::SIZE);
            if !(0..8).any(|j| read_usize(v + j * 4) == club) {
                exe::staff_records_add_failed_to_qualify_record(read_usize(G_STAFF_RECORDS), club, this.competition().addr(), 0x41c, 0x28);
            }
        }
    }
    if !runners_up.is_null() {
        let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(runners_up.addr()) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
        destroy(runners_up.addr(), 1);
    }
    let round_table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = round_table(this.addr(), 4, &mut rounds, &mut extra, 0);
    let eight = v; // cqv11: the 8 quarter-finalists in order
    let p = exe::crt_operator_new(0xb2);
    let quarter_finals = if p != 0 {
        cup_stage(p, this.addr(), 8, eight, rounds as u16 as u32, this.competition().addr(), table, this.year() as u16 as u32, 4, 2, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages() + 0x10, quarter_finals);
    exe::crt_free(v);
    exe::crt_free(table);
    let f = read_usize(this.stages() + 0x10);
    if f == 0 {
        return show_error_at(0x9a7e5c, 0x2ab);
    }
    exe::cup_draw_round(f, 0);
}

pub(crate) unsafe extern "thiscall" fn compare_conmebol_merc_create_quarter_final_stage(this: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CONMEBOL_MERC_CREATE_QUARTER_FINAL_STAGE);
    let q = this.promoted_count() as i8;
    let stages = this.stages();
    if !(1..=2).contains(&q) || stages == 0 || (0..4).any(|g| read_usize(stages + g * 4) == 0) {
        return o(this.addr());
    }
    let mut first: Vec<usize> = Vec::new();
    let mut second: Vec<usize> = Vec::new();
    for g in -1i8..4 {
        let group = conmebol_merc_group(this.addr(), g);
        for i in 0..q as i32 {
            first.push(read_usize(exe::league_get_table_row(group, i)));
        }
        second.push(read_usize(exe::league_get_table_row(group, 1)));
    }
    o(this.addr());
    let f = read_usize(this.stages() + 0x10);
    if f == 0 {
        return;
    }
    let t = read_usize(f + cup::TEAM_TABLE);
    let mut read: Vec<usize> = if t != 0 { (0..8).map(|i| read_usize(t + i * 6)).collect() } else { Vec::new() };
    let got_in: Vec<usize> = read.iter().copied().filter(|c| !first.contains(c)).collect();
    let ok = read.len() == 8
        && first.iter().all(|c| read.contains(c))
        && got_in.len() + first.len() == 8
        && got_in.iter().all(|c| second.contains(c))
        && {
            read.sort_unstable();
            read.windows(2).all(|w| w[0] != w[1])
        };
    SCOREBOARD.record(ok, || format!("conmebol_merc_create_quarter_final_stage({:#x}): quarter-finals {:x?} group winners {:x?} runners-up {:x?}", this.addr(), read, first, second));
}

/// The Mercosur Cup predicted positions: by the rounds of stage 4 (the quarter-finals on) and in blocks of 5 for those
/// who stay in the groups (box at line 0x338); see `conmebol_set_predicted_positions`.
/// Original: `ConmebolMerc::set_predicted_positions` (0x4c5a90, conmebol_merc.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_set_predicted_positions(this: usize) {
    conmebol_set_predicted_positions(this, 4, 5, 0x9a7e5c, 0x338)
}

pub(crate) unsafe extern "thiscall" fn compare_conmebol_merc_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CONMEBOL_MERC_SET_PREDICTED_POSITIONS);
    conmebol_compare_predicted_positions("conmebol_merc_set_predicted_positions", this, 4, 5, || o(this))
}

/// The Mercosur Cup season turnover: the first half (`conmebol_free_season`), the next year, `league::ACTIVE` = 0 and
/// the rules (the box for line 0x1c6).
/// Original: `ConmebolMerc::start_new_season` (0x4c5040, conmebol_merc.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let league = this;
    conmebol_free_season(league);
    conmebol_next_year(league);
    this.set_active((0) as usize);
    conmebol_set_up_rules_again(league, 0x9a7e5c, 0x1c6)
}

pub(crate) unsafe extern "thiscall" fn compare_conmebol_merc_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CONMEBOL_MERC_START_NEW_SEASON);
    conmebol_compare_new_season("conmebol_merc_start_new_season", this, true, || o(this))
}

/// Group `g` of the Mercosur Cup: -1 is the cup itself (group A), the others the stages `league::STAGES`[g].
pub(crate) unsafe fn conmebol_merc_group(this: usize, g: i8) -> usize {
    let this = League(this);
    if g == -1 {
        this.addr()
    } else {
        read_usize(this.stages() + g as isize as usize * 4)
    }
}

/// The Copa Mercosur at career creation: as the Libertadores, with five stages and no seeding step
/// (`build_south_american_cup`).
/// Original: `ConmebolMerc::ConmebolMerc` (0x4c46c0, conmebol_merc.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_conmebol_merc(this: usize, year: u32, record: usize) -> usize {
    build_south_american_cup(League(this), year, record, &COPA_MERCOSUR, "cx03_19 mercosur new")
}

/// rule: the Copa Mercosur groups are home and away, 3 points a win and 1 a draw, tiebreak by codes 1, 2 and 3, one
/// club going through directly from each group, clubs from several nations, category 7, 7 substitutes named and 3
/// changes; the season starts on day 15 of month 5 of the competition year [inference: 15 June]; prize fund of 2,000,000
/// (`apply_cup_rules` with `MERCOSUR_RULES`: the fields, the start day word, its year offset 0, the prize and the
/// round table from virtual +0x3c into `league::ROUND_DATES`). Returns 1.
/// Original: `ConmebolMerc::set_up_rules` (0x4c4f50, conmebol_merc.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_set_up_rules(this: usize) -> u32 {
    covered!("cx03_19 mercosur rules");
    apply_cup_rules(League(this), &MERCOSUR_RULES, MERCOSUR_RULES.day, 3)
}

/// Comparer for `construct_conmebol_merc` (as above).
pub(crate) unsafe extern "thiscall" fn compare_construct_conmebol_merc(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_CONMEBOL_MERC)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, COPA_MERCOSUR.vtable, &south_american_cup_ctor_fields(5), false);
    SCOREBOARD.record(error.is_none(), || format!("construct_conmebol_merc({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// Comparer for `conmebol_merc_set_up_rules`: only the original runs (virtual +0x3c allocates the round table); the plan
/// checks the rules (`check_cup_rules`).
pub(crate) unsafe extern "thiscall" fn compare_conmebol_merc_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CONMEBOL_MERC_SET_UP_RULES);
    check_cup_rules("conmebol_merc_set_up_rules", League(this), &MERCOSUR_RULES, || o(this))
}

/// The Copa Mercosur: 5 stages, the boxes of lines 0x4a and 0x58.
pub(crate) const COPA_MERCOSUR: SouthAmericanCup = SouthAmericanCup {
    vtable: 0x968ca8, stages: 5, seeding: false, rules: 0x4c4f50, file: 0x9a7e5c, rows: [0x4a, 0x58],
};

/// The Copa Mercosur rules: the Merconorte's fields with 7 substitutes named and 3 substitutions allowed, one qualifier
/// per group, the start on day 15 of month 5 [inference: 15 June] and the prize fund of 2,000,000.
pub(crate) const MERCOSUR_RULES: ContinentalGroupRules = ContinentalGroupRules {
    fields: &[(0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0x3c, 2, 2), (0xc6, 1, 2), (0x42, 1, 2), (0xc3, 1, 1), (0xc4, 1, 7), (0xc5, 1, 1),
        (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x1c, 4, 0xffff_ffff), (0x20, 4, 0xffff_ffff), (0x49, 1, 7)],
    day: 0xf, month: 5, week: 0xffff_ffff, prize: Some(0x1e8480),
};
