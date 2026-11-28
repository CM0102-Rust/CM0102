use super::*;

/// Club k of the 8 selected (+0xee: group A slots 0..3, B 4..7).
pub(crate) unsafe fn selected_club(this: League, k: usize) -> usize {
    read_usize(read_usize(this.addr() + oceania_club_champ::DRAWN_CLUBS) + k * 4) // cqt10: o escolhido k
}

/// Rule: the Oceania Club Championship is drawn on the start day (`.next_draw_day()`/`.next_draw_year_offset()`) and
/// only on the call without `full`: the 8 clubs, the predicted positions, group A in the cup's own table, the fixtures
/// and group B as a stage; before that day the cup does not even run the league day; an error while building returns 1
/// without the league day.
///
/// `today.update_for_date(full)` (virtual +0x14; SEH frame only for the local dates, no destructor): the Oceania Club
/// Championship's day. The draw date is that of 0x549ef0(`.next_draw_day()`, `.next_draw_year_offset()` + `.year()`)
/// (lea ecx, local; push year; push day -> thiscall, ret 8, pure). Before it (today's year lower, unsigned; or the same
/// year and today's day lower, signed) the routine returns 0 without the league day. On the draw day, if the draw is
/// not done (`.active()` = 0) and WITHOUT `full`: the 8 clubs (`exe::oceaniaclubchamp_select_qualified_clubs`: mov ecx
/// -> thiscall, ret 0), the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`, thiscall, ret 0), the group A
/// table (`exe::oceaniaclubchamp_create_group_a_table`, thiscall, ret 0; 0: the error box at line 0x21d), the fixtures
/// (`exe::league_create_fixtures` League::create_fixtures, thiscall, ret 0; 0: the error box at line 0x224), the sorted
/// table (`exe::league_sort_table` push 0; mov ecx -> thiscall, ret 4; 0: the error box at line 0x22b) (an error
/// returns 1, without the league day), the group B stage (`exe::oceaniaclubchamp_create_group_b_stage`, thiscall, ret
/// 0), the list `oceania_club_champ::DRAWN_CLUBS` freed (`exe::crt_free`: push; add esp, 4 -> cdecl) and zeroed,
/// `oceania_club_champ::DRAWN_CLUB_COUNT` = 0 and `.set_active(1)`. Then the league day (`exe::league_update_for_date`:
/// push full; push today; mov ecx, this -> thiscall, ret 8), whose return value it passes on.
/// Original: `OceaniaClubChamp::update_for_date` (0x797b40, oceania_club_champ.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let day = this.next_draw_day() as u16;
    let year = (this.next_draw_year_offset() as u16).wrapping_add(this.year() as u16);
    let day_of_today = read_u16(today);
    let year_of_today = read_u16(today + 2);
    // simplification: in the loops before the draw and on errors the original returns only al (0 or 1; the rest of eax is leftover from the
    // earlier calls); the caller reads the byte
    if year_of_today < year || (year_of_today == year && (day_of_today as i16) < day as i16) {
        return 0;
    }
    if year_of_today == year && day_of_today == day && this.active() == 0 && full == 0 {
        covered!("cqt_10 oceania draw");
        exe::oceaniaclubchamp_select_qualified_clubs(this);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this.addr());
        if exe::oceaniaclubchamp_create_group_a_table(this) == 0 {
            show_error_at(0xa6a068, 0x21d);
            return 1;
        }
        if exe::league_create_fixtures(this) == 0 {
            show_error_at(0xa6a068, 0x224);
            return 1;
        }
        if exe::league_sort_table(this, 0) == 0 {
            show_error_at(0xa6a068, 0x22b);
            return 1;
        }
        exe::oceaniaclubchamp_create_group_b_stage(this);
        exe::crt_free(read_usize(this.addr() + oceania_club_champ::DRAWN_CLUBS));
        write_usize(this.addr() + oceania_club_champ::DRAWN_CLUBS, 0);
        write_usize(this.addr() + oceania_club_champ::DRAWN_CLUB_COUNT, 0);
        this.set_active(1);
    }
    exe::league_update_for_date(this, today, full) as u32 
}

// simplification: the clubs, the predictions, the tables, the matches and the league day do not run twice. Original only; the plan
// on the draw day (the copy’s condition): +0x45 = 1, the list +0xee and +0xf2 zeroed, the group A table and the stage of
// group B created and the current stage (+0x30) = 0.

/// Rule: group A of the Oceania Club Championship has the first 4 chosen clubs, each in the row of its order; human
/// managers get the group draw news.
///
/// `OceaniaClubChamp::create_group_a_table()`: 4 teams (`.set_club_count()`), the table (`.set_table()`;
/// `exe::crt_malloc`(0xec): push 0xec; add esp, 4 -> cdecl; 4 rows of 0x3b bytes; out of memory, the error box at line
/// 0x182 and returns 0), each row i with chosen club i (`selected_club`) (`exe::league_init_table_row`
/// League::init_table_row: push 0; push 0; push club; push i; mov ecx, this -> thiscall, ret 0x10), the group draw news
/// (`exe::league_send_group_draw_news`: mov ecx, this -> thiscall, ret 0) and returns 1. The same body as
/// `init_group_a_table` of the African Cup (only the file and the error box line change).
/// Original: `OceaniaClubChamp::create_group_a_table` (0x7978c0, oceania_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_group_a_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    this.set_table(t);
    if t == 0 {
        show_error_at(0xa6a068, 0x182);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < this.club_count() {
        covered!("cqt_10 group a row");
        // simplification: the index goes in a dword whose 3 high bytes are stack leftovers; 0x687430 reads only the byte (char)
        exe::league_init_table_row(this, i as u8 as u32, selected_club(this, i as usize), 0, 0); // cqt10: row i of group A
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

// simplification: the table is allocated (and each row allocates the history) and the news goes to the managers: does not run twice. Only the
// original; the plan: returns 1, 4 teams and in row i (0x3b bytes) the chosen i with position i (+4).

/// Rule: group B of the Oceania Club Championship is a league stage with chosen clubs 4 to 7, the cup's rules and year
/// and the cup's round table for stage 0 (10, 12 and 14 January); human managers get the news.
///
/// `OceaniaClubChamp::create_group_b_stage()` (SEH frame only for the stage's new: no destructor on the normal path):
/// the teams in a vector of 4 pointers (`exe::crt_malloc`(0x10): push; add esp, 4 -> cdecl; out of memory, the error
/// box at line 0x296 and 0): chosen clubs 4 to 7 (`selected_club`); the round table through
/// `comp_vtable::CREATE_SCHEDULE` (push 0; push &id; push &rounds; push 0; mov ecx, this -> thiscall, ret 0x10); the
/// stage (`exe::crt_operator_new`(0xee) push; add esp, 4 -> cdecl; the League_Stage constructor
/// `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58, 22 arguments: the cup, 4, the vector, 1, `.competition()`, the
/// round table, the rounds (word), `.win_points()`, `.draw_points()`, `.match_category()`, &`league::TIEBREAKS`,
/// &`league::PROMOTED_COUNT`, the year `.year()` (word), 0, the id (word), `.importance()`, 2, 0, `.league_flags()`
/// (word), −1, 0, 2; the original pushes the words and bytes with garbage in the high bytes, which the constructor does
/// not read: the copy passes clean values) into `.stages()`[0]; the vector and the round table freed (`exe::crt_free`).
/// No stage: the error box at line 0x2ad and 0. With it: `.set_current_stage(0)`, the group news
/// (`exe::league_send_group_draw_news` on the stage) and 1.
/// Original: `OceaniaClubChamp::create_group_b_stage` (0x797e80, oceania_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_group_b_stage(this: usize) -> u32 {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let teams = exe::crt_malloc(0x10);
    if teams == 0 {
        show_error_at(0xa6a068, 0x296);
        return 0;
    }
    for n in 0..4usize {
        covered!("cqt_10 group b team");
        write_usize(teams + n * 4, selected_club(this, 4 + n)); // cqt10: the team in the group B slot
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this.addr(), 0, &mut rodadas, &mut id, 0);
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
            0,
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
    write_usize(this.stages(), stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    let f = read_usize(this.stages());
    if f == 0 {
        show_error_at(0xa6a068, 0x2ad);
        return 0;
    }
    this.set_current_stage(0);
    exe::league_send_group_draw_news(f);
    1
}

// simplification: the stage is allocated and the news goes to the managers: does not run twice. Original only; the plan: returns 1 with the
// stage in [+0xc][0], +0x30 = 0, the stage with 4 teams and the cup year and, in its table (+0xb1, rows of 0x3b bytes), the
// chosen 4 to 7 (in any order: the constructor sorts the table).

/// The clubs of one source for the Oceania competition: up to 20, in the order of the club table.
pub(crate) struct SourceClubList {
    pub(crate) clubs: [usize; 20],
    pub(crate) n: i16,
}

impl SourceClubList {
    pub(crate) fn new() -> Self {
        SourceClubList { clubs: [0; 20], n: 0 }
    }

    /// Adds the club if it fits (the count is a signed word, < 20).
    pub(crate) fn cabe(&self) -> bool {
        self.n < 0x14
    }

    pub(crate) fn por(&mut self, c: usize) {
        self.clubs[self.n as usize] = c;
        self.n += 1;
    }

    /// Sorts by reputation and shuffles the top band.
    ///
    /// Uses `comp_util_sort_and_shuffle_top_clubs` comp_util_sort_and_shuffle_top_clubs (push how many; push the count; push list; add esp -> cdecl;
    /// draws at random).
    pub(crate) unsafe fn ordenar(&mut self, quantos: u32) {
        let f: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(FN_COMP_UTIL_SORT_AND_SHUFFLE_TOP_CLUBS);
        f(self.clubs.as_mut_ptr() as usize, self.n as u16 as u32, quantos);
    }

    /// The first club of the list and the first one after it from another nation (+0x53), as in the original.
    ///
    /// It walks the `n` slots; an empty slot does not count as the first.
    pub(crate) unsafe fn two_from_different_nations(&self) -> (usize, usize) {
        let (mut a, mut b) = (0usize, 0usize);
        for &c in self.clubs.iter().take(self.n.max(0) as usize) {
            if a == 0 {
                a = c;
            } else if b == 0 && read_usize(c + club::NATION) != read_usize(a + club::NATION) {
                b = c;
            }
        }
        (a, b)
    }
}

/// Places a pot pair into groups A and B by the draw: first to slot k and second to k+4, or reversed.
pub(crate) unsafe fn place_pot_pair(this: usize, k: usize, first: usize, second: usize, sorteio: i32) {
    let l = read_usize(this + oceania_club_champ::DRAWN_CLUBS);
    if sorteio != 0 {
        write_usize(l + k * 4, first);
        write_usize(read_usize(this + oceania_club_champ::DRAWN_CLUBS) + (k + 4) * 4, second);
    } else {
        write_usize(l + (k + 4) * 4, first);
        write_usize(read_usize(this + oceania_club_champ::DRAWN_CLUBS) + k * 4, second);
    }
}

/// `OceaniaClubChamp::select_qualified_clubs()` (`0x798220`, thiscall, ret 0, `oceania_club_champ.cpp`): builds the
/// 8-club list of the Oceania Club Championship from the Australian champion and the clubs of the Oceania nations, by
/// drawn pairs.
///
/// Sets `+0xf2` = 8 and allocates list `+0xee` (`0x944e46(0x20)`: push; add esp, 4 -> cdecl; out of memory, the box of
/// line `0x307` and returns 0). The champion of the last record of the Australian league (`0x4afd80`
/// comp_util_get_latest_placed_club: push 0; push competition `0xae23d0` + 0x6b × [`0x9cf89c`]; add esp, 8 -> cdecl).
///
/// The clubs ([`0xae23b4`], 0x245 bytes, [`0xae235c`] − 2 × [`0xae2350`] of them) from an Oceania nation ([`0x9cfa18`]:
/// `+0x71` of the nation, its `+0`) go, by nation id, to the lists of New Zealand ([`0x9cf3f4`]), Australia
/// ([`0x9cf224`]), Tahiti ([`0x9cf4b4`]) and Fiji ([`0x9cf2f8`]), up to 20 in each (the count is a signed word); the
/// others (or when the list is full) go to the Melanesian list (`0x53d900` database_is_melanesian_nation: push nation;
/// add esp -> cdecl; when full, the club is left out) or, otherwise, the Polynesian list (`0x53d940` with the nation
/// again), up to 20. Without a champion, the best Australian (the list with a drawn range of 1) takes its place.
///
/// The pairs, each drawn between groups A (slots 0..3) and B (4..7) by `0x90cfc0(2)`: the champion and the best New
/// Zealander (slots 0/4); the best from Fiji and from Tahiti (1/5); the first two Melanesians from different nations
/// (2/6, the range of 2); the two Polynesians (3/7). Returns 1.
///
/// The original routine keeps the lists on the stack without zeroing them; with fewer clubs than it reads, it reads
/// leftovers; the copy reads zero [inference: in this career the lists have enough clubs; the comparer checks the
/// list].
pub(crate) unsafe fn select_qualified_clubs(this: usize) -> u32 {
    write_usize(this + oceania_club_champ::DRAWN_CLUB_COUNT, 8);
    let list = exe::crt_malloc(0x20);
    write_usize(this + oceania_club_champ::DRAWN_CLUBS, list);
    if list == 0 {
        show_error_at(0xa6a068, 0x307);
        return 0;
    }
    let mut champion = exe::comp_util_get_latest_placed_club(read_usize(G_CLUB_COMP_TABLE) + read_usize(KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_LEAGUE) * 0x6b, 0);
    let (mut nz, mut aus, mut tai, mut fij, mut mel, mut pol) =
        (SourceClubList::new(), SourceClubList::new(), SourceClubList::new(), SourceClubList::new(), SourceClubList::new(), SourceClubList::new());
    let mut i = 0i32;
    let mut c = read_usize(G_CLUB_TABLE);
    while i < (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2)) {
        let nation = Nation(read_usize(c + club::NATION));
        if !nation.is_null() && nation.continent() != 0 && read_usize(nation.continent()) == read_usize(KEY_CONTINENT_OCEANIA) {
            let id = read_usize(nation.addr());
            if id == read_usize(KEY_NATION_NEW_ZEALAND) && nz.cabe() {
                nz.por(c);
            } else if id == read_usize(KEY_NATION_AUSTRALIA) && aus.cabe() {
                aus.por(c);
            } else if id == read_usize(KEY_NATION_TAHITI) && tai.cabe() {
                tai.por(c);
            } else if id == read_usize(KEY_NATION_FIJI) && fij.cabe() {
                fij.por(c);
            } else if exe::database_is_melanesian_nation(nation) != 0 {
                if mel.cabe() {
                    mel.por(c);
                }
            } else if exe::database_is_polynesian_nation(read_usize(c + club::NATION)) != 0 && pol.cabe() {
                pol.por(c);
            }
        }
        i += 1;
        c += club::SIZE;
    }
    covered!("cqt_11 oceania clubs");
    if champion == 0 {
        aus.ordenar(1);
        champion = aus.clubs[0];
    }
    nz.ordenar(1);
    place_pot_pair(this, 0, champion, nz.clubs[0], exe::utils_random(2)); // cqt11: the Australian and New Zealand champions
    fij.ordenar(1);
    tai.ordenar(1);
    place_pot_pair(this, 1, fij.clubs[0], tai.clubs[0], exe::utils_random(2)); // cqt11: Fiji e Taiti
    mel.ordenar(2);
    let (a, b) = mel.two_from_different_nations();
    place_pot_pair(this, 2, a, b, exe::utils_random(2)); // cqt11: os dois melanésios
    pol.ordenar(2);
    let (a, b) = pol.two_from_different_nations();
    place_pot_pair(this, 3, a, b, exe::utils_random(2));
    1
}

/// rule: the Oceania Club Championship has 8 clubs in 4 pairs, each pair split between groups A and B by a draw: the
/// Australian league champion (without one, the best Australian club) and the best New Zealand club; the best of Fiji
/// and of Tahiti; two Melanesian clubs of different nations; two Polynesian clubs of different nations (each list
/// sorted by reputation, with the top band shuffled)
/// Original: `OceaniaClubChamp::select_qualified_clubs` (0x798220, oceania_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_select_qualified_clubs(this: usize) -> u32 {
    select_qualified_clubs(this)
}

/// rule: the Oceania Club Championship moves on one stage when the current one ends; after the groups (on reaching 1)
/// it builds the semi-finals; going past the last stage is an error
/// Original: `OceaniaClubChamp::advance_stage` (0x797df0, oceania_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_advance_stage(this: usize) {
    let this = League(this);
    let f = this.current_stage() as i32;
    if f >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0xa6a068, 0x274);
    }
    let f = f + 1;
    this.set_current_stage(f as usize);
    if f == 1 {
        covered!("cqt_11 oceania semifinals");
        exe::oceaniaclubchamp_create_knockout_stage(this);
    }
}

// simplification: the semi-finals allocate and draw: they do not run twice. Original only; the plan: +0x30 goes up one (outside the
// last stage, which shows the box and changes nothing).

/// The 4 semifinalists: 1st A, 2nd B, 2nd A, 1st B.
pub(crate) unsafe fn semifinalists(this: League) -> [usize; 4] {
    let a = |l: usize| read_usize(this.table() + l * league_table_row::SIZE);
    let b = |l: usize| read_usize(read_usize(read_usize(this.stages()) + league::TABLE) + l * league_table_row::SIZE);
    [a(0), b(1), a(1), b(0)] // cqt11: 1º A × 2º B, 2º A × 1º B
}

/// Rule: the Oceania Club Championship semi-finals cross the groups: A winner × B runner-up and A runner-up × B winner,
/// in a cup stage (stage 1) with the first round set up at once.
///
/// `OceaniaClubChamp::create_knockout_stage()` (SEH frame only for the stage's new: no destructor on the normal path),
/// the semi-finals: the list of the 4 (`exe::crt_malloc`(0x10): push; add esp, 4 -> cdecl; out of memory, the error box
/// at line 0x2cb and return), the semi-finalist pairings (`semifinalists`); the round table through
/// `comp_vtable::CREATE_SCHEDULE` (push 0; push &id; push &rounds; push 1; mov ecx, this -> thiscall, ret 0x10); the
/// stage (`exe::crt_operator_new`(0xb2) push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`,
/// thiscall, ret 0x3c: the cup, 4, the list, the rounds, `.competition()`, the table, the year `.year()`, 1, 2, the id,
/// 0x14, 0, 0, 0 and no seed; the original pushes the rounds, the id and the year with garbage in the high 16 bits,
/// which the constructor does not read) into `.stages()`[1]; the table and the list freed (`exe::crt_free`). No stage:
/// the error box at line 0x2e3 and return. With it: the round set up (`exe::cup_set_up_round` Cup::set_up_round: push
/// 0; mov ecx, stage -> thiscall, ret 4).
/// Original: `OceaniaClubChamp::create_knockout_stage` (0x798050, oceania_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_create_knockout_stage(this: usize) {
    let this = League(this);
    let list = exe::crt_malloc(0x10);
    if list == 0 {
        return show_error_at(0xa6a068, 0x2cb);
    }
    covered!("cqt_11 oceania pairings");
    for (k, &c) in semifinalists(this).iter().enumerate() {
        write_usize(list + k * 4, c);
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this.addr(), 1, &mut rodadas, &mut id, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 4, list, rodadas as u16 as u32, this.competition().addr(), table, this.year() as u16 as u32, 1, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(this.stages() + 4, stage);
    exe::crt_free(table);
    exe::crt_free(list);
    let f = read_usize(this.stages() + 4);
    if f == 0 {
        return show_error_at(0xa6a068, 0x2e3);
    }
    exe::cup_set_up_round(f, 0);
}

// simplification: allocates the stage and builds the round: does not run twice. Original only; the plan: the stage at [+0xc][1] with 4 teams,
// the cup’s +4 and the 4 crossed ones (read earlier) in its list (+0xa7, 6 bytes; as a set: the round may reorder).

/// One operation of building the round table, in the original routine's order.
///
/// `D` the date of a group round (`league_round_set_date`: round, day, month with 0 = January, years to add, weekday, code, value;
/// records of 0x41 bytes), `L` a match of the group round (`store_round_match`: round, match, a, b, c, value), `S` the draw of a
/// cup round (`cup_set_round_draw_date`: round, day, month, years, weekday; records of 0x68 bytes), `J` the match of the cup round
/// (`cup_set_round_match_date`: round, day, month, years, weekday, time, home side), `E` a day of the tie (`cup_set_round_match_day`: round, slot,
/// weekday, time, code, home side) and `B`/`W`/`X` writing a byte/word/dword at the given offset of the table.
#[derive(Clone, Copy)]
pub(crate) enum OceaniaClubChampScheduleOp {
    D(i32, i32, i32, i32, i32, i32, i32),
    L(i32, i32, i32, i32, i32, u32),
    S(u32, u32, u32, u32, u32),
    J(u32, u32, u32, u32, u32, u32, u32),
    E(i32, i32, i32, i32, i32, u32),
    B(usize, u8),
    W(usize, u16),
    X(usize, u32),
}
pub(crate) use OceaniaClubChampScheduleOp::{B as Cqq12B, D as Cqq12D, E as Cqq12E, J as Cqq12J, L as Cqq12L, S as Cqq12S, W as Cqq12W, X as Cqq12X};

/// Describes one stage of virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`).
///
/// It holds the é_liga written (if requested), the rounds, the extra (only if the pointer is not null, when
/// `extra_checa`), the table size, the error-box line when out of memory, the common operations and the stage's own
/// operations (the matches of each group) and the counter name. `None`: the stage returns 0 without writing anything.
#[derive(Clone, Copy)]
pub(crate) struct OceaniaStage {
    pub(crate) league: i32,
    pub(crate) rounds: i16,
    pub(crate) extra: i16,
    pub(crate) extra_check: bool,
    pub(crate) size: u32,
    pub(crate) row: u32,
    pub(crate) ops: &'static [OceaniaClubChampScheduleOp],
    pub(crate) ops2: &'static [OceaniaClubChampScheduleOp],
    pub(crate) name: &'static str,
}

/// Runs stage `f` (or returns 0 without it): the outputs, the table (`exe::crt_malloc`; out of memory, the box `linha` of source
/// file `arquivo` and 0) and the date/match operations in order.
///
/// The cup's year (+0x40) goes as clean 16 bits to `league_round_set_date` (the original pushes the register with garbage in the high
/// 16 bits; `league_round_set_date` only uses the word) and signed to the cup ones.
pub(crate) unsafe fn build_stage_table(this: League, f: Option<OceaniaStage>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, file: usize) -> usize {
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
            Cqq12D(r, dia, mes, anos, sem, cod, v) => {
                exe::league_round_set_date(t, r, dia, mes, anos, sem, cod, ano16, v);
            }
            Cqq12L(r, j, a, b, c, v) => exe::league_round_set_match_slot(t, r, j, a, b, c, v),
            Cqq12S(r, dia, mes, anos, sem) => {
                exe::cup_set_round_draw_date(t, r, dia, mes, anos, sem, ano);
            }
            Cqq12J(r, dia, mes, anos, sem, hora, mando) => {
                exe::cup_set_round_match_date(t, r, dia, mes, anos, sem, hora, ano, mando);
            }
            Cqq12E(r, vaga, sem, hora, cod, mando) => {
                exe::cup_set_round_match_day(t, r, vaga, sem, hora, cod, mando);
            }
            Cqq12B(o, v) => *((t + o) as *mut u8) = v,
            Cqq12W(o, v) => write_u16(t + o, v),
            Cqq12X(o, v) => write_u32(t + o, v),
        }
    }
    t
}

/// Bytes the stage operations write into table t, in operation order.
pub(crate) unsafe fn oceania_club_champ_written_table_bytes(ops: &[OceaniaClubChampScheduleOp], ops2: &[OceaniaClubChampScheduleOp], t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut pega = |a: usize, n: usize| v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, n));
    for op in ops.iter().chain(ops2.iter()) {
        match *op {
            Cqq12D(r, ..) => {
                pega(r as usize * 0x41, 5);
                pega(r as usize * league_round_date::SIZE + league_round_date::VENUE_VALUE, 4);
            }
            Cqq12L(r, j, ..) => pega(r as usize * 0x41 + 5 + j as usize * 7, 7),
            Cqq12S(r, ..) => pega(r as usize * 0x68, 3),
            Cqq12J(r, ..) => {
                pega(r as usize * 0x68 + 3, 4);
                pega(r as usize * cup_round::SIZE + cup_round::VENUE_VALUE, 4);
            }
            Cqq12E(r, vaga, ..) => pega(r as usize * cup_round::SIZE + cup_round::SLOT_VENUE + vaga as usize * 7, 7),
            Cqq12B(o, _) => pega(o, 1),
            Cqq12W(o, _) => pega(o, 2),
            Cqq12X(o, _) => pega(o, 4),
        }
    }
    v
}

/// Side-by-side comparer of the stage table builder.
///
/// The original routine runs with the caller's outputs, the copy with its own outputs that start with the same values,
/// both from the same [0xb67a34]; the outputs and the bytes written to the two tables must be equal; the copy's table
/// is freed (`exe::crt_free`).
///
/// In the stage without a table only the original runs (neither writes anything; the outputs may not even exist).
pub(crate) unsafe fn compare_stage_table(name: &str, stage: u32, f: Option<OceaniaStage>, rounds: *mut i16, extra: *mut i16, league: *mut i32,
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
    let table = if a != 0 && b != 0 { oceania_club_champ_written_table_bytes(f.ops, f.ops2, a) == oceania_club_champ_written_table_bytes(f.ops, f.ops2, b) } else { a == 0 && b == 0 };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(igual && outs && table, || format!("{}(stage {}): outs {} ({} x {}) table {}", name, stage as u8 as i8, outs, *rounds, r2, table));
    a
}

/// O Campeonato de Clubes da Oceania, as três rodadas de cada grupo (10, 12 e 14 de janeiro do ano seguinte, valor 1) com o jogo 2 vazio.
pub(crate) const OCEANIA_CLUB_GROUP_OPS: [OceaniaClubChampScheduleOp; 6] = [
    Cqq12D(0, 0xa, 0, 1, 4, 2, 1), Cqq12L(0, 2, -1, -1, -1, 0), Cqq12D(1, 0xc, 0, 1, 6, 1, 1),
    Cqq12L(1, 2, -1, -1, -1, 0), Cqq12D(2, 0xe, 0, 1, 1, 2, 1), Cqq12L(2, 2, -1, -1, -1, 0),
];
/// Os jogos do grupo da fase −1.
pub(crate) const OCEANIA_GROUP_A_MATCHES: [OceaniaClubChampScheduleOp; 6] = [
    Cqq12L(0, 0, 4, 1, 3, 4), Cqq12L(0, 1, 4, 2, 3, 0x40), Cqq12L(1, 0, 6, 1, 3, 4), Cqq12L(1, 1, 6, 2, 3, 0x40),
    Cqq12L(2, 0, 1, 1, 3, 4), Cqq12L(2, 1, 1, 1, 3, 0x40),
];
/// Os jogos do grupo da fase 0.
pub(crate) const OCEANIA_GROUP_B_OPS: [OceaniaClubChampScheduleOp; 6] = [
    Cqq12L(0, 0, 5, 1, 3, 0x40), Cqq12L(0, 1, 5, 2, 3, 4), Cqq12L(1, 0, 0, 1, 3, 0x40), Cqq12L(1, 1, 0, 2, 3, 4),
    Cqq12L(2, 0, 2, 1, 3, 4), Cqq12L(2, 1, 2, 1, 3, 0x40),
];
/// O mata-mata em jogo único: semifinal em 17/1, final e terceiro lugar em 19/1 do ano seguinte.
pub(crate) const OCEANIA_CLUB_KNOCKOUT_OPS: [OceaniaClubChampScheduleOp; 54] = [
    Cqq12S(0, 0x10, 0, 1, 3), Cqq12J(0, 0x11, 0, 1, 4, 2, 0x40), Cqq12W(0x7, 0x82), Cqq12W(0x9, 0), Cqq12W(0xb, 0),
    Cqq12W(0xd, 0x283), Cqq12B(0x17, 8), Cqq12W(0x18, 4), Cqq12W(0x1a, 2), Cqq12W(0x1c, 4), Cqq12W(0xf, 0),
    Cqq12W(0x1e, 0), Cqq12B(0x20, 0), Cqq12B(0x21, 1), Cqq12B(0x22, 0), Cqq12X(0x5c, 0), Cqq12X(0x60, 0),
    Cqq12X(0x64, 0), Cqq12S(1, 0x12, 0, 1, 5), Cqq12J(1, 0x13, 0, 1, 6, 1, 4), Cqq12W(0x6f, 0x96), Cqq12W(0x71, 1),
    Cqq12W(0x73, 0), Cqq12W(0x75, 0x83), Cqq12B(0x7f, 8), Cqq12W(0x80, 2), Cqq12W(0x82, 1), Cqq12W(0x84, 0),
    Cqq12W(0x77, 0), Cqq12W(0x86, 0), Cqq12B(0x88, 0), Cqq12B(0x89, 1), Cqq12B(0x8a, 0), Cqq12X(0xc4, 0),
    Cqq12X(0xc8, 0), Cqq12X(0xcc, 0), Cqq12S(2, 0x12, 0, 1, 5), Cqq12J(2, 0x13, 0, 1, 6, 1, 0x40), Cqq12W(0xd7, 0x8c),
    Cqq12W(0xd9, 2), Cqq12W(0xdb, 0), Cqq12W(0xdd, 0x83), Cqq12B(0xe7, 8), Cqq12W(0xe8, 2), Cqq12W(0xea, 1),
    Cqq12W(0xec, 0), Cqq12W(0xdf, 0), Cqq12W(0xee, 0), Cqq12B(0xf0, 0), Cqq12B(0xf1, 1), Cqq12B(0xf2, 0),
    Cqq12X(0x12c, 0), Cqq12X(0x130, 0), Cqq12X(0x134, 0),
];

/// Stage spec of the Oceania Club Championship: groups below 1, knockout at 1, None above.
pub(crate) fn oceania_stage_spec(stage: u32) -> Option<OceaniaStage> {
    let f = stage as u8 as i8;
    if f < 1 {
        let games: &'static [OceaniaClubChampScheduleOp] = match f {
            -1 => &OCEANIA_GROUP_A_MATCHES,
            0 => &OCEANIA_GROUP_B_OPS,
            _ => &[],
        };
        Some(OceaniaStage { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x420), extra_check: false, size: 0xc3, row: 0x93,
            ops: &OCEANIA_CLUB_GROUP_OPS, ops2: games, name: "cqq_12 grupos da oceania" })
    } else if f == 1 {
        Some(OceaniaStage { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0xc5,
            ops: &OCEANIA_CLUB_KNOCKOUT_OPS, ops2: &[], name: "cqq_12 mata-mata da oceania" })
    } else {
        None
    }
}

/// rule: the Oceania Club Championship has two groups (stages −1 and 0, ids 0x41f and 0x420) of three rounds on 10, 12
/// and 14 January of the year after the cup year, with each group's fixed matches; the knockout (stage 1) in single
/// matches: semi-final on 17/1, third place and final on 19/1 (venue 4 in the final); category 8; tie-break 0x283 in
/// the semi-final and 0x83 after; no prizes
/// Original: `OceaniaClubChamp::create_schedule` (0x797230, oceania_club_champ.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_stage_table(League(this), oceania_stage_spec(stage), rounds, extra, league, 0xa6a068)
}

/// The 16-team table of the Asian cups (AsiaClubChamp::build_team_table `asia_club_champ_build_team_table` and AsiaCupWinner::create_team_entries
/// `create_team_entries`, the same body).
///
/// +0x3e = 16 and the table (`exe::crt_malloc` of 0x60 at +0xa7; out of memory, the box `linha` of `arquivo` and −1); slot i
/// gets club i of the qualified list (+0xb2, pointers; `ordem` gives the index) with class and status 0. Returns 1.
pub(crate) unsafe fn build_team_table(this: Cup, file: usize, row: u32, name: &'static str, order: impl Fn(i16) -> i16) -> i32 {
    this.set_team_count(0x10);
    let l = exe::crt_malloc(0x60);
    this.set_team_table((l as u32) as usize);
    if l == 0 {
        show_error_at(file, row);
        return -1;
    }
    covered!(name);
    let mut i: i16 = 0;
    while i < this.team_count() {
        let c = read_usize(read_usize(this.addr() + asia_club_champ_fields::QUALIFIED_CLUBS) + order(i) as usize * 4);
        let e = this.team_table() + i as usize * 6;
        write_u32(e, c as u32);
        write_u8(e + 4, 0);
        write_u8(e + 5, 0);
        i = i.wrapping_add(1);
    }
    1
}

/// Side-by-side comparer of the team table: +0x3e, return and the 16 entries must match; frees the copy's table.
pub(crate) unsafe fn compare_team_table(name: &str, this: Cup, o: impl FnOnce() -> i32, c: impl FnOnce() -> i32) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this.addr() + cup::TEAM_COUNT, 2usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o();
        lo.set(this.team_table());
        r
    }, || {
        let r = c();
        lr.set(this.team_table());
        this.set_team_table((lo.get() as u32) as usize);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { 0x60 } else { 0 };
    let list = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("{}({:#x}) returned {} x {} memory {} list {}", name, this.addr(), a, b, igual, list));
    a
}

/// rule: the Oceania Club Championship groups are single round robin, 3/1 points, tiebreak by codes 1, 2 and 4, two
/// qualifiers per group (the top ones go on, the others are out: rule bits 0x28), category 8 (the deciding stages one),
/// 3 substitutes on the bench and 3 changes; starts on day 20 of month 7 moved to weekday 5 [inference]; no prize fund
/// Original: `OceaniaClubChamp::set_up_rules` (0x7979b0, oceania_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_set_up_rules(this: usize) -> u32 {
    covered!("cqj_11 oceania rules");
    let day = 0x14; // cqj11: the start day in Oceania
    apply_cup_rules(League(this), &OCEANIA_RULES, day, 3)
}

/// Rule: the Oceania Club Championship is every 2 years: when it ends, the table and stages are undone, the season goes
/// to the statistics history, the year goes up 2, the host alternates between Australia and New Zealand and the rules
/// are set up again.
///
/// `OceaniaClubChamp::start_new_season()` (virtual +8): `.set_season_closed(0)`, the table freed
/// (`exe::league_free_table` League::free_table: push 0; mov ecx, this -> thiscall, ret 4), the round dates
/// `.round_dates()` freed (`exe::crt_free`) and zeroed, the season archived in the statistics
/// (`exe::comp_stats_archive_season_and_reset`: push 1; mov ecx, `.comp_stats()` -> thiscall, ret 4), the stages
/// 0..=`.current_stage()` (signed, read again on each pass) of `.stages()` destroyed (virtual 0 with 1: the destructor
/// that frees), the year `.year()` up 2, `.set_current_stage()` = −1, `.set_active(0)`, the host alternated
/// (`alternate_oceania_club_champ_host` on [`G_HOST_COUNTRY`], thiscall, ret 0) and the rules again (virtual
/// `comp_vtable::SET_UP_RULES`, thiscall, ret 0, without a box), whose eax it returns.
/// Original: `OceaniaClubChamp::start_new_season` (0x797d60, oceania_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_start_new_season(this: usize) -> u32 {
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
    let mut i = 0i32;
    while (this.current_stage() as i32) >= 0 && i <= this.current_stage() as i32 {
        let f = read_usize(this.stages() + i as usize * 4);
        if f != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
            destroy(f, 1);
        }
        i += 1;
    }
    covered!("mwg_11 oceania club championship rollover");
    this.set_year(this.year().wrapping_add(2)); // mwg11: the Oceania championship every 2 years
    this.set_current_stage(usize::MAX);
    this.set_active(0);
    let alternate: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_HOSTCOUNTRY_ALTERNATE_OCEANIA_CLUB_CHAMP_HOST);
    alternate(read_usize(G_HOST_COUNTRY));
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES));
    rules(this.addr())
}

/// Comparer for `oceania_club_champ_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_OCEANIA_CLUB_CHAMP_START_NEW_SEASON);
    let year = this.year();
    let r = o(this.addr());
    let ok = this.year() == year.wrapping_add(2) && this.season_closed() == 0;
    SCOREBOARD.record(ok, || format!("oceania_club_champ_start_new_season({:#x}): year {} -> {}, +0x4c {}", this.addr(), year, this.year(), this.season_closed()));
    r
}

/// Rule: the Oceania Club Championship records: in the groups (stage below 1) the qualified club gets the record of
/// round 0x82 and the club going out the failed record (weight 0x19 both); in the final (stage 1) the winner and
/// runner-up get the final records.
///
/// `OceaniaClubChamp::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44);
/// returns 0. Stage (signed byte) below 1: outcome 6, the qualification (`european_cup_add_qualified_record`: 0, 0x82,
/// 0x19); 0xfe, the failure (`european_cup_add_failed_to_qualify_record`: 0x41c, 0x19). Stage 1: outcome 1 the winner
/// (Staff_Records 0x86a800), 2 the runner-up (0x86b0b0). Anything else: nothing.
/// Original: `OceaniaClubChamp::update_staff_records_for_status` (0x797a90, oceania_club_champ.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < 1 {
        if d == 6 {
            covered!("cqx_11 oceania group qualified");
            european_cup_add_qualified_record(club, comp, 0, 0x82, 0x19);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x19);
        }
    } else if f == 1 {
        match d {
            1 => cup_record_final_result(0x86a800, club, Fixture(game), final_),
            2 => cup_record_final_result(0x86b0b0, club, Fixture(game), final_),
            _ => {}
        }
    }
    0
}

pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_OCEANIA_CLUB_CHAMP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("oceania_club_champ_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}

/// The predicted positions of the Oceania Club Championship and its group draw: without statistics
/// (`league::COMP_STATS`), the box for line 0x3a0; otherwise the 8 chosen clubs (+0xee, +0xf2 clubs) sorted by
/// reputation (`predicted_by_reputation_sort`), the positions 1, 2, 3, 3, 5, 5, 7, 7 and the list shuffled
/// (`exe::comp_util_shuffle_pointer_array`(list, n): push n (word); push list; add esp, 8 -> cdecl: 3n drawn swaps).
/// That shuffle is what makes the groups: the first 4 go to group A, the others to B.
/// Original: `OceaniaClubChamp::set_predicted_positions` (0x798640, oceania_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_set_predicted_positions(this: usize) {
    let this = League(this);
    if this.comp_stats() == 0 {
        return show_error_at(0xa6a068, 0x3a0);
    }
    let list = read_usize(this.addr() + 0xee);
    predicted_by_reputation_sort(list, read_usize(this.addr() + 0xf2) as u32); // cqv13: the 8 by reputation
    predicted_by_reputation_write(this.addr(), read_usize(this.addr() + 0xee), &OCEANIA_CLUB_CHAMP_POSITIONS);
    covered!("cqv_13 oceania group draw");
    let n = read_i16(this.addr() + 0xf2) as u16 as u32;
    exe::comp_util_shuffle_pointer_array(read_usize(this.addr() + 0xee), n); // cqv13: the 8 shuffled
}

pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_OCEANIA_CLUB_CHAMP_SET_PREDICTED_POSITIONS);
    let list = read_usize(this + 0xee);
    let n = read_i16(this + 0xf2);
    if list == 0 || !(0..=64).contains(&n) {
        return o(this);
    }
    let mut before: Vec<usize> = (0..n as usize).map(|k| read_usize(list + k * 4)).collect();
    o(this);
    let mut after: Vec<usize> = (0..n as usize).map(|k| read_usize(read_usize(this + 0xee) + k * 4)).collect();
    predicted_by_reputation_check("oceania_club_champ_set_predicted_positions", this, &before, &OCEANIA_CLUB_CHAMP_POSITIONS);
    before.sort_unstable();
    after.sort_unstable();
    SCOREBOARD.record(before == after, || format!("oceania_club_champ_set_predicted_positions({:#x}): the shuffled list is not the same", this));
}

/// The positions in the order of the array sorted by reputation (`exe::comp_stats_set_club_final_position`: push
/// position; push club; mov ecx, stats (`league::COMP_STATS`) -> thiscall, ret 8).
pub(crate) unsafe fn predicted_by_reputation_write(this: usize, v: usize, positions: &[u32]) {
    let this = League(this);
    for (k, &p) in positions.iter().enumerate() {
        exe::comp_stats_set_club_final_position(this.comp_stats(), read_usize(v + k * 4), p);
    }
}

/// `exe::crt_qsort`(v, n, 4, 0x4b07d0) (4 pushes; add esp -> cdecl): the reputation (`club::REPUTATION`) descending,
/// nulls at the end.
pub(crate) unsafe fn predicted_by_reputation_sort(v: usize, n: u32) {
    exe::crt_qsort(v, n, 4, 0x4b07d0);
}

/// The plan of the predicted positions: the clubs `clubs` (taken before the original), each with the position written
/// (`comp_stats_club::FINAL_POSITION` of its entry, `exe::comp_stats_find_club_entry`) after it; by reputation
/// (`club::REPUTATION`) descending the positions make `positions` (reputation ties may swap with each other: the qsort
/// is not stable). Only with as many clubs as positions and none null.
pub(crate) unsafe fn predicted_by_reputation_check(name: &str, this: usize, clubs: &[usize], positions: &[u32]) {
    let this = League(this);
    let stats = this.comp_stats();
    if stats == 0 || clubs.len() != positions.len() || clubs.iter().any(|&c| c == 0) {
        return;
    }
    let mut v: Vec<(i16, u8)> = clubs
        .iter()
        .map(|&c| {
            let e = exe::comp_stats_find_club_entry(stats, c);
            (Club(c).reputation(), if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 })
        })
        .collect();
    v.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    let read: Vec<u32> = v.iter().map(|x| x.1 as u32).collect();
    SCOREBOARD.record(read == positions, || format!("{}({:#x}): positions by reputation {:?}", name, this.addr(), read));
}

/// HostCountry::alternate_oceania_club_champ_host (thiscall, ret 0), called by the Oceania turnover on the host table.
pub(crate) const FN_HOSTCOUNTRY_ALTERNATE_OCEANIA_CLUB_CHAMP_HOST: usize = 0x5face0;

/// The predicted positions of the 8 Oceania Club Championship entrants, by reputation.
pub(crate) const OCEANIA_CLUB_CHAMP_POSITIONS: [u32; 8] = [1, 2, 3, 3, 5, 5, 7, 7];

/// Writes the news text of a club's fate in the Oceania Club Championship (`group_championship_fate_text`): through
/// from its group, "{club} qualify for semi final" (0xa6a0ac); at stage 1 the base text; in the final stage the winner
/// (0x98732c) and the runner-up (0x9872c8).
/// Original: `OceaniaClubChamp::build_fate_news_text` (0x7976b0, oceania_club_champ.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    group_championship_fate_text(this, club_id, outcome, stage, a4, fixture, a6, a8, news, text, 1, Some(1), 0xa6a0ac, &OCEANIA_FINALS);
}

/// Stores a club's progress in the Oceania Club Championship (`group_championship_progress`): in the group stage
/// (stage below 1) current 1→1, 2→3, 3→5, 4→7; best 1, 2→1, 3→5, 4→7; worst 1, 2→3, 3→5, 4→7.
/// Original: `OceaniaClubChamp::set_club_progress` (0x7987a0, oceania_club_champ.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx04_5 oceania club progress");
    group_championship_progress(this, club, stage, current, best, worst, 1, [[1, 3, 5, 7], [1, 1, 5, 7], [3, 3, 5, 7]]);
}

/// Comparer for `oceania_club_champ_build_fate_news_text`: the text-only paths side by side (`compare_text_writer`);
/// the group and tie news only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(IDX_OCEANIA_CLUB_CHAMP_BUILD_FATE_NEWS_TEXT);
    if !group_championship_text_only(outcome, stage, news, text, 1) {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text),
        |t| oceania_club_champ_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, t),
        || format!("oceania_club_champ_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
}

pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_OCEANIA_CLUB_CHAMP_SET_CLUB_PROGRESS);
    compare_group_championship_progress("oceania_club_champ_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || oceania_club_champ_set_club_progress(this, club, stage, current, best, worst));
}

/// The fate news text of the two club championships run in groups and a final stage (Oceania, Club World): in the group
/// stages (signed stage below `groups`) outcome 6 (through) with `news` the group news
/// (`exe::comp_util_write_group_qualify_news`(club_id, a4, competition, 6, news, text)), without it `qualify`; outcome
/// 0xfe (out) the group news with −2; at stage `middle` (Oceania's 1) the base text
/// (`exe::comp_build_fate_news_text` with a7 = 0); at the final stage (`groups`), with `news` the cup tie result news
/// (`exe::comp_util_write_cup_tie_result_news`(club_id, fixture, a8, outcome, a6, text)), without it
/// `final_stage_fate_text`. Other stages and outcomes: nothing.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn group_championship_fate_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a8: u32, news: u32, text: usize,
        groups: i8, middle: Option<i8>, qualify: usize, finals: &[(u8, usize)]) {
    let (s, d) = (stage as u8 as i8, outcome as u8);
    let comp = read_usize(this + 4);
    if s < groups {
        if d == 6 {
            if news != 0 {
                exe::comp_util_write_group_qualify_news(club_id, a4, comp, 6, news, text);
            } else {
                group_qualify_text(club_id, qualify, text);
            }
        } else if d == 0xfe {
            exe::comp_util_write_group_qualify_news(club_id, a4, comp, -2, news, text);
        }
    } else if Some(s) == middle {
        exe::comp_build_fate_news_text(this, club_id, outcome, s as i32, a4, fixture, a6, 0, a8, news, text);
    } else if s == groups {
        if news != 0 {
            exe::comp_util_write_cup_tie_result_news(club_id, fixture as u32, a8, outcome, a6, text);
        } else {
            covered!("cx04_5 club championship final text");
            final_stage_fate_text(this, club_id, d, finals, text);
        }
    }
}

/// Whether `group_championship_fate_text` only writes the text on this call (what its comparers compare side by side).
pub(crate) fn group_championship_text_only(outcome: u32, stage: u32, news: u32, text: usize, groups: i8) -> bool {
    let (s, d) = (stage as u8 as i8, outcome as u8);
    text != 0 && news == 0 && ((s < groups && d == 6) || s == groups)
}

/// The club championships' progress codes: each of the three codes (current, best, worst) passes through its table when
/// it is 1 to 4 and the stage (signed) is below `groups`; the bytes go to `comp_stats_club::BEST_POSITION`,
/// `CURRENT_POSITION` and `WORST_POSITION` of the club's entry (`exe::comp_stats_find_club_entry` on the statistics,
/// not checked for null), in that order.
pub(crate) unsafe fn group_championship_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32, groups: i8, tables: [[u8; 4]; 3]) {
    let e = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let code = |p: u32, t: &[u8; 4]| {
        let p = p as u8;
        if (stage as u8 as i8) < groups && (1..=4).contains(&p) { t[p as usize - 1] } else { p }
    };
    write_u8(e + comp_stats_club::BEST_POSITION, code(best, &tables[1]));
    write_u8(e + comp_stats_club::CURRENT_POSITION, code(current, &tables[0]));
    write_u8(e + comp_stats_club::WORST_POSITION, code(worst, &tables[2]));
}

/// The final-stage fate text shared by the Oceania Club Championship and the Club World Championship: the template of
/// `finals` for the outcome, if any, with the club's and the competition's short names (`european_cup_fate_text`, the
/// competition record at +4); with no template for the outcome the original still hands the Text the shared buffer
/// 0xde1f64 as it is (stale text, a defect of the original) and adds the club's (0x7d5) and the competition's (0x7d0)
/// links.
pub(crate) unsafe fn final_stage_fate_text(this: usize, club_id: u32, outcome: u8, finals: &[(u8, usize)], text: usize) {
    let comp = read_usize(this + 4);
    if let Some(&(_, format)) = finals.iter().find(|f| f.0 == outcome) {
        return european_cup_fate_text(text, club_id, format, read_u8(comp + competition::SHORT_GENDER_NAME), comp + competition::SHORT_NAME, comp);
    }
    let club = club_at(club_id as isize);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// The Oceania Club Championship's fate texts: groups before stage 1, the base text at stage 1, the final at stage 2.
pub(crate) const OCEANIA_FINALS: [(u8, usize); 2] = [(1, 0x98732c), (2, 0x9872c8)]; // "win" / "finish runners-up in"
