// ---------- rx10_4: the leagues divided into sections (shared) and the German Regionalliga constructor, rules and table (ger_regional.cpp) ----------

/// What changes between the two leagues whose main table holds one section and whose other sections are League_Stage
/// objects in the stage slots (the Regionalliga and the Portuguese Segunda B).
pub(crate) struct DividedLeague {
    /// The vtable of the derived class.
    pub(crate) vtable: usize,
    /// The discipline rule (`league::DISCIPLINE_RULE`).
    pub(crate) discipline: u8,
    /// How many section stages (`league::STAGE_COUNT`; the slot vector is 4 bytes each).
    pub(crate) sections: u8,
    /// The class's set_up_rules, create_table, section stage (thiscall(k), ret 4) and set_predicted_positions.
    pub(crate) set_up_rules: usize,
    pub(crate) create_table: usize,
    pub(crate) section_stage: usize,
    pub(crate) predicted: usize,
    /// The source file and the constructor's box lines (rules, table, order, statistics, fixtures).
    pub(crate) file: usize,
    pub(crate) lines: [u32; 5],
}

/// The constructor of a league divided into sections (the same body in GerRegional and PorSecondB, checked
/// instruction by instruction).
///
/// League::League (`exe::new_league`), the competition record, the vtable, the year (word) and the discipline rule;
/// the saved state (`exe::league_load_from_temp_file`(this, 1)) ends it. Otherwise the main stage index,
/// `league::CURRENT_STAGE` = −1, `league::STAGE_COUNT` = the sections and the slot vector (`exe::crt_malloc` of 4 × the
/// sections, unchecked); then the rules, the table, the order (`exe::league_sort_table`(this, 0)), the statistics
/// (`exe::crt_operator_new`(0x5ce), `exe::new_comp_stats`) and the fixtures (`exe::league_create_fixtures`), each with
/// its box when it fails (and the constructor returns); then each section stage k (from 0) and the predicted
/// positions, both called directly. The original pushes the section index with the high bytes of a stack slot (the
/// statistics pointer); the section routine reads its byte.
pub(crate) unsafe fn construct_divided_league(this: usize, year: u32, record: usize, p: &DividedLeague) -> usize {
    let l = League(this);
    exe::new_league(l);
    write_usize(this + league::COMPETITION, record);
    write_usize(this, p.vtable);
    l.set_year(year as u16 as i16);
    l.set_discipline_rule(p.discipline);
    if exe::league_load_from_temp_file(l, 1) != 0 {
        return this;
    }
    l.set_stage_index(stage_index::MAIN);
    l.set_current_stage(0xffff_ffff);
    l.set_stage_count(p.sections as usize);
    l.set_stages(exe::crt_malloc(4 * p.sections as u32));
    let call = |f: usize| -> u32 {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(f);
        f(this)
    };
    if call(p.set_up_rules) == 0 {
        show_error_at(p.file, p.lines[0]);
        return this;
    }
    if call(p.create_table) == 0 {
        show_error_at(p.file, p.lines[1]);
        return this;
    }
    if exe::league_sort_table(l, 0) == 0 {
        show_error_at(p.file, p.lines[2]);
        return this;
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, l) } else { 0 };
    l.set_comp_stats(s);
    if s == 0 {
        show_error_at(p.file, p.lines[3]);
        return this;
    }
    if exe::league_create_fixtures(l) == 0 {
        show_error_at(p.file, p.lines[4]);
        return this;
    }
    let section: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(p.section_stage);
    for k in 0..p.sections {
        section(this, k as u32);
    }
    call(p.predicted);
    this
}

/// The constructor comparer of a divided league: only the original (it allocates and loads), then the plan: the
/// vtable, the year, the record and the discipline rule; when it was built (not loaded), the section count, the slot
/// vector, the statistics and every section stage made.
pub(crate) unsafe fn compare_divided_league_ctor(name: &str, this: usize, year: u32, record: usize, p: &DividedLeague, original: impl FnOnce() -> usize) -> usize {
    let l = League(this);
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this && read_usize(this) == p.vtable && built_year_ok(l.year() as u16, year as u16) && l.competition().addr() == record
        && l.discipline_rule() == p.discipline;
    if !loaded && l.comp_stats() != 0 {
        ok = ok && l.stage_count() == p.sections as usize && l.stages() != 0 && (0..p.sections as usize).all(|k| read_usize(l.stages() + 4 * k) != 0);
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): returned {:#x}, vtable {:#x}, year {}, discipline {}, sections {}, stages {:#x}, statistics {:#x}", name, this, r, read_usize(this),
        l.year(), l.discipline_rule(), l.stage_count(), l.stages(), l.comp_stats()));
    r
}

/// Fills the main table of a divided league with one section's clubs (the same body in GerRegional and PorSecondB).
///
/// `league::CLUB_COUNT` is already written. The table (`exe::crt_malloc` of `league_table_row::SIZE` × the count; none:
/// the box at `lines[0]` and 0); the clubs of the database before the national teams (index below the club count
/// minus twice the nation count) whose division (`club::DIVISION`) is the league of `division` and whose reserve
/// division (`club::RESERVE_DIVISION`, the section) is that of `section`, compared by the record's id, each in the
/// next row (`exe::league_init_table_row`(this, row, club, 0, 0)); a club past the count: the box at `lines[1]`, its
/// division zeroed and the scan stops. With `mark_b_teams`, a row whose club is a B team (`exe::club_get_linked_b_team`
/// (club, &flag, 1) not 0) with flag 0 gets `league_table_row::STATUS` = 0xfc (it cannot go up). Returns 1 when the
/// table is full.
pub(crate) unsafe fn fill_section_table(this: League, division: usize, section: usize, mark_b_teams: bool, file: usize, lines: [u32; 2]) -> u32 {
    let n = this.club_count();
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(league_table_row::SIZE as u32));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, lines[0]);
        return 0;
    }
    let mut row: i8 = 0;
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 - 2 * read_usize(G_NATION_COUNT) as i32 {
        let club = Club(read_usize(G_CLUB_TABLE) + i as usize * club::SIZE);
        let d = club.division().addr();
        let s = club.reserve_division().addr();
        if d != 0 && s != 0 && read_usize(s) == read_usize(section) && read_usize(d) == read_usize(division) {
            if row as i16 >= this.club_count() {
                show_error_at(file, lines[1]);
                write_usize(club.addr() + club::DIVISION, 0); // the original drops the club that does not fit
                break;
            }
            exe::league_init_table_row(this, row as u8 as u32, club, 0, 0);
            if mark_b_teams {
                let mut flag = 0u32;
                if exe::club_get_linked_b_team(club, &mut flag, 1) != 0 && flag == 0 {
                    write_u8(this.table() + (row as isize as usize).wrapping_mul(league_table_row::SIZE) + league_table_row::STATUS, 0xfc);
                }
            }
            row = row.wrapping_add(1);
        }
        i += 1;
    }
    (row as i16 == this.club_count()) as u32
}

/// The table comparer of a divided league: only the original (it allocates and may move clubs), then the plan: the
/// result is 1 exactly when the count is reached, and every row holds a club of the league's division and the section.
pub(crate) unsafe fn compare_section_table(name: &str, this: usize, division: usize, section: usize, original: impl FnOnce() -> u32) -> u32 {
    let l = League(this);
    let r = original();
    let t = l.table();
    let n = l.club_count().max(0) as usize;
    let rows: Vec<usize> = if t != 0 && r == 1 { (0..n).map(|k| read_usize(t + k * league_table_row::SIZE)).collect() } else { Vec::new() };
    let ok = rows.iter().all(|&c| {
        let club = Club(c);
        c != 0 && club.division().addr() != 0 && club.reserve_division().addr() != 0 && read_usize(club.division().addr()) == read_usize(division)
            && read_usize(club.reserve_division().addr()) == read_usize(section)
    });
    SCOREBOARD.record(ok && (t != 0 || r == 0), || format!("{}({:#x}): returned {}, table {:#x}, clubs {:x?}", name, this, r, t, rows));
    r
}

/// The section stage of a divided league (GerRegional's South, PorSecondB's two extra zones).
pub(crate) struct SectionStage {
    /// The section's club count (the League_Stage's, and the bound of the list).
    pub(crate) count: i16,
    /// The division test: `Some(g)`, the club's division id equals the id of global g; `None`, the club's division
    /// record is this league's own record (`league::COMPETITION`, compared as a pointer).
    pub(crate) division: Option<usize>,
    /// The places of the stage (promoted, play-off, relegation play-off, relegated): a fixed set, or `None` for this
    /// league's (`league::PROMOTED_COUNT`).
    pub(crate) places: Option<[u8; 4]>,
    /// Mark the B teams (`league_table_row::STATUS` = 0xfc) in the new stage's table.
    pub(crate) mark_b_teams: bool,
    /// The source file and the box lines (no list, a club too many, too few clubs, no stage).
    pub(crate) file: usize,
    pub(crate) lines: [u32; 4],
}

/// Builds section stage k of a divided league (the same body in GerRegional::create_south_stage and
/// PorSecondB::create_zone_stage, checked instruction by instruction).
///
/// The round table (virtual `comp_vtable::CREATE_SCHEDULE`(k, &rounds, &extra, 0)); the list (`exe::crt_malloc` of 4 ×
/// the count; none: the box at `lines[0]` and nothing else); the clubs before the national teams with a division and a
/// reserve division, the reserve division's id that of `section` and the division as in `SectionStage::division`, in
/// database order; a club past the count: the box at `lines[1]` and its division zeroed (the scan goes on). Fewer than
/// the count: the box at `lines[2]` and nothing else. Otherwise a League_Stage (`exe::crt_operator_new`(0xee);
/// League_Stage::League_Stage: this league, the count, the list, 2 legs, the competition, the round table and its
/// rounds, the win, draw and category bytes, the tie-breaks, the places, the year, stage k, the extra as its label, the
/// importance, 1, 0, the rule bits `league::LEAGUE_FLAGS`, −1, 0 and 2) into slot k; none: the box at `lines[3]`;
/// otherwise the list and the round table freed, the B teams marked (`SectionStage::mark_b_teams`: the rows whose club
/// is a B team, `exe::club_get_linked_b_team`(club, &flag, 1) not 0 with flag 0, get `league_table_row::STATUS` =
/// 0xfc) and `league::CURRENT_STAGE` = k. On the error paths the round table (and the list) are not freed (leaks of the
/// original).
pub(crate) unsafe fn create_section_stage(this: usize, k: u8, section: usize, p: &SectionStage) {
    let l = League(this);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = schedule(this, k as u32, &mut rounds, &mut extra, 0);
    let list = exe::crt_malloc(4 * p.count as u32);
    if list == 0 {
        return show_error_at(p.file, p.lines[0]);
    }
    let mut n: i16 = 0;
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 - 2 * read_usize(G_NATION_COUNT) as i32 {
        let club = Club(read_usize(G_CLUB_TABLE) + i as usize * club::SIZE);
        let d = club.division().addr();
        let s = club.reserve_division().addr();
        let same_division = match p.division {
            Some(g) => d != 0 && read_usize(d) == read_usize(g),
            None => d == l.competition().addr(),
        };
        if d != 0 && s != 0 && read_usize(s) == read_usize(section) && same_division {
            if n >= p.count {
                show_error_at(p.file, p.lines[1]);
                write_usize(club.addr() + club::DIVISION, 0); // the original drops the club that does not fit
            } else {
                write_usize(list + 4 * n as usize, club.addr()); // rx10_4: the next club of the section
                n += 1;
            }
        }
        i += 1;
    }
    if n < p.count {
        return show_error_at(p.file, p.lines[2]);
    }
    let places_local: [u8; 4] = p.places.unwrap_or([0; 4]);
    let places = if p.places.is_some() { places_local.as_ptr() as usize } else { this + league::PROMOTED_COUNT };
    let e = exe::crt_operator_new(0xee);
    let s = if e != 0 {
        let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        ctor(e, this, p.count as u16 as u32, list, 2, l.competition().addr(), table, rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32,
            l.match_category() as u32, this + league::TIEBREAKS, places, l.year() as u16 as u32, k as u32, extra as u16 as u32, l.importance() as u32, 1, 0,
            l.league_flags() as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    write_usize(l.stages() + 4 * k as usize, s);
    if s == 0 {
        return show_error_at(p.file, p.lines[3]);
    }
    exe::crt_free(list);
    exe::crt_free(table);
    if p.mark_b_teams {
        let stage = League(s);
        for row in 0..p.count as usize {
            let club = read_usize(stage.table() + row * league_table_row::SIZE);
            let mut flag = 0u32;
            if exe::club_get_linked_b_team(club, &mut flag, 1) != 0 && flag == 0 {
                write_u8(stage.table() + row * league_table_row::SIZE + league_table_row::STATUS, 0xfc);
            }
        }
    }
    l.set_current_stage(k as usize);
}

/// The section stage comparer: only the original (it builds the stage), then the plan: slot k holds a stage of the
/// count's clubs, all of the section, and it is the current stage.
pub(crate) unsafe fn compare_section_stage(name: &str, this: usize, k: u8, section: usize, p: &SectionStage, original: impl FnOnce()) {
    let l = League(this);
    original();
    let s = read_usize(l.stages() + 4 * k as usize);
    let ok = s != 0 && League(s).club_count() == p.count && l.current_stage() as i32 == k as i32
        && (0..p.count as usize).all(|r| {
            let c = Club(read_usize(League(s).table() + r * league_table_row::SIZE));
            c.reserve_division().addr() != 0 && read_usize(c.reserve_division().addr()) == read_usize(section)
        });
    SCOREBOARD.record(ok, || format!("{}({:#x}, {}): stage {:#x}, current {}", name, this, k, s, l.current_stage() as i32));
}

/// The season turnover of a divided league (the same body in GerRegional and PorSecondB, checked instruction by
/// instruction).
///
/// `league::SEASON_CLOSED` = 0; the table freed (`exe::league_free_table`(this, 0)); `league::ROUND_DATES` freed and
/// zeroed; with statistics, `exe::comp_stats_archive_season_and_reset`(stats, 1); with a current stage ≥ 0, the stages
/// 0..=current destroyed (virtual 0 with 1; the slot vector is kept); the year up by one and `league::CURRENT_STAGE` =
/// −1. Then the rules (virtual `comp_vtable::SET_UP_RULES`), the table, the order (`exe::league_sort_table`(this, 0))
/// and the fixtures (`exe::league_create_fixtures`), each with its box (`lines`) when it returns 0 (the copy returns 0;
/// the original the box's eax, which the caller does not read); then each section stage k, the predicted positions
/// (virtual `comp_vtable::SET_PREDICTED_POSITIONS`) and the referee renewal (`exe::officialsmanager_renew_officials`
/// ([`G_OFFICIALS_MANAGER`], the competition)), whose al it returns. Unlike the English and Brazilian turnovers, no board
/// expectations.
pub(crate) unsafe fn divided_league_turnover(this: usize, p: &DividedLeague, lines: [u32; 4]) -> u32 {
    let l = League(this);
    l.set_season_closed(0);
    exe::league_free_table(l, 0);
    let dates = l.round_dates();
    if dates != 0 {
        exe::crt_free(dates);
        l.set_round_dates(0);
    }
    let stats = l.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    if (l.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(l.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
                destroy(f, 1);
            }
            i += 1;
            if i > l.current_stage() as i32 {
                break;
            }
        }
    }
    l.set_year(l.year().wrapping_add(1));
    l.set_current_stage(0xffff_ffff);
    if bra_nat_first_call_virtual_no_args(this, comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(p.file, lines[0]);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.create_table);
    if table(this) == 0 {
        show_error_at(p.file, lines[1]);
        return 0;
    }
    if exe::league_sort_table(l, 0) == 0 {
        show_error_at(p.file, lines[2]);
        return 0;
    }
    if exe::league_create_fixtures(l) == 0 {
        show_error_at(p.file, lines[3]);
        return 0;
    }
    let section: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(p.section_stage);
    for k in 0..p.sections {
        section(this, k as u32);
    }
    bra_nat_first_call_virtual_no_args(this, comp_vtable::SET_PREDICTED_POSITIONS);
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), l.competition().addr()) as u32
}

/// The club array of a divided league (the same body in GerRegional and PorSecondB).
///
/// `*count` = `total` before the array (`exe::crt_malloc` of 4 × total; none: the box at `line`, `*count` = 0 and
/// nothing else); the main table's clubs first, then each of the `sections` stages' from index `league::CLUB_COUNT` (this
/// league's) + row. The rows of a section run to the stage's own count (`bound_by_stage`) or to this league's count.
/// With more than one section every stage starts at the same index (see PorSecondB::get_club_array: a defect of the
/// original).
pub(crate) unsafe fn divided_league_club_array(this: usize, out: *mut usize, count: *mut i16, total: i16, sections: usize, bound_by_stage: bool, file: usize, line: u32) {
    let l = League(this);
    *count = total;
    let a = exe::crt_malloc(4 * total as i32 as u32);
    *out = a;
    if a == 0 {
        show_error_at(file, line);
        *count = 0;
        return;
    }
    let mut i = 0i16;
    while i < l.club_count() {
        write_usize(a + 4 * i as usize, read_usize(l.table() + i as usize * league_table_row::SIZE));
        i += 1;
    }
    for k in 0..sections {
        let stage = League(read_usize(l.stages() + 4 * k));
        let mut r = 0i16;
        while r < if bound_by_stage { stage.club_count() } else { l.club_count() } {
            write_usize(a + 4 * (l.club_count() as i32 + r as i32) as usize, read_usize(stage.table() + r as usize * league_table_row::SIZE));
            r += 1;
        }
    }
}

/// The club array comparer: side by side (the copy into its own array, freed after): the count and every index the
/// routine writes must be equal.
pub(crate) unsafe fn compare_divided_league_club_array(name: &str, this: usize, out: *mut usize, count: *mut i16, written: usize, original: impl FnOnce(), copy: impl FnOnce(*mut usize, *mut i16)) {
    original();
    let (mut a, mut n) = (0usize, 0i16);
    copy(&mut a, &mut n);
    let ok = n == *count && (a == 0) == (*out == 0)
        && (a == 0 || (0..written.min(n.max(0) as usize)).all(|k| read_usize(a + 4 * k) == read_usize(*out + 4 * k)));
    if a != 0 {
        exe::crt_free(a);
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): count original {} copy {}", name, this, *count, n));
}

/// The predicted positions of a divided league (the same body in GerRegional and PorSecondB).
///
/// No statistics (`league::COMP_STATS`): the box at `lines[0]`. The list (`exe::crt_malloc`(0x50), room for 20; none:
/// the box at `lines[1]`); for the main table and then each section stage: its clubs into the list, sorted by
/// reputation (`exe::crt_qsort` with 0x4b07d0), and row r's predicted position (Comp_Stats::set_club_final_position
/// `exe::comp_stats_set_club_final_position`(stats, club, place)) is r × `step` + 1 (a byte). Every section gets the
/// same places (1, 1 + step, ...), not interleaved ones (a defect of the original). The list is freed.
pub(crate) unsafe fn section_predicted_positions(this: usize, sections: i8, step: u8, file: usize, lines: [u32; 2]) {
    let l = League(this);
    if l.comp_stats() == 0 {
        return show_error_at(file, lines[0]);
    }
    let list = exe::crt_malloc(0x50);
    if list == 0 {
        return show_error_at(file, lines[1]);
    }
    let mut k: i8 = -1;
    while k < sections {
        let s = if k == -1 { l } else { League(read_usize(l.stages() + 4 * k as usize)) };
        let mut r = 0i16;
        while r < s.club_count() {
            write_usize(list + 4 * r as usize, read_usize(s.table() + r as usize * league_table_row::SIZE));
            r += 1;
        }
        exe::crt_qsort(list, s.club_count() as i32 as u32, 4, 0x4b07d0usize);
        let mut r = 0i16;
        while r < s.club_count() {
            // simplification: the original pushes the place with stale high bytes; Comp_Stats reads the byte
            exe::comp_stats_set_club_final_position(l.comp_stats(), read_usize(list + 4 * r as usize), (r as u8).wrapping_mul(step).wrapping_add(1) as u32);
            r += 1;
        }
        k += 1;
    }
    exe::crt_free(list);
}

/// The predicted positions comparer: the original, then the copy over the same statistics; each club's place must
/// come out the same.
pub(crate) unsafe fn compare_section_predicted(name: &str, this: usize, sections: i8, original: impl FnOnce(), copy: impl FnOnce()) {
    let l = League(this);
    if l.comp_stats() == 0 {
        return original();
    }
    let mut clubs: Vec<usize> = Vec::new();
    for k in -1..sections {
        let s = if k == -1 { l } else { League(read_usize(l.stages() + 4 * k as usize)) };
        clubs.extend((0..s.club_count().max(0) as usize).map(|r| read_usize(s.table() + r * league_table_row::SIZE)));
    }
    original();
    let a = state_league_positions(l.comp_stats(), &clubs);
    copy();
    let b = state_league_positions(l.comp_stats(), &clubs);
    SCOREBOARD.record(a == b, || format!("{}({:#x}): original {:?} copy {:?}", name, this, a, b));
}

/// The German Regionalliga: vtable 0x96b250, the German league discipline rules, one section stage (the South), its
/// set_up_rules 0x5dd9f0, create_table 0x5dd7f0, create_south_stage 0x5ddc50 and set_predicted_positions 0x5de130,
/// the boxes of ger_regional.cpp at lines 0x4a, 0x51, 0x58, 0x60 and 0x67.
pub(crate) const GER_REGIONAL: DividedLeague = DividedLeague { vtable: 0x96b250, discipline: discipline_rule::GERMANY_LEAGUE, sections: 1, set_up_rules: 0x5dd9f0,
    create_table: 0x5dd7f0, section_stage: 0x5ddc50, predicted: 0x5de130, file: 0x9cd488, lines: [0x4a, 0x51, 0x58, 0x60, 0x67] };

/// Rule: the German Regionalliga is a league of the game's year under the German league discipline rules, in two
/// sections (North in its own table, South as a stage); with nothing saved, its rules, North table, order,
/// statistics, fixtures, South stage and predicted positions are made.
///
/// `construct_divided_league` with `GER_REGIONAL`.
/// Original: `GerRegional::GerRegional` (0x5dcbd0, ger_regional.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ger_regional(this: usize, year: u32, record: usize) -> usize {
    covered!("rx10_4 new regionalliga");
    construct_divided_league(this, year, record, &GER_REGIONAL)
}

/// Comparer for `construct_ger_regional` (`compare_divided_league_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_ger_regional(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_divided_league_ctor("construct_ger_regional", this, year, record, &GER_REGIONAL, || o(this, year, record))
}

/// The rule writes of the German Regionalliga (offset, size, value) for the year.
///
/// Two legs, 3 points a win and 1 a draw, category 2, national clubs, tie-breaks 1, 2 and 0, 2 promoted to the 2.
/// Bundesliga (`KEY_CLUB_COMP_GERMAN_SECOND_DIVISION` in `league::UPPER_DIVISION_ID`), no play-off, 4 relegated (5 in
/// 2000) with no league below (−1) and the rule bits `league::LEAGUE_FLAGS` = 0x42.
pub(crate) unsafe fn ger_regional_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 0), (0xbe, 1, 2), (0xbf, 1, 0), (0xc0, 1, 0)];
    v.push((0xc1, 1, if League(this).year() == 0x7d0 { 5 } else { 4 })); // rx10_4: four go down from each section (five in 2000)
    v.extend_from_slice(&[(0x1c, 4, read_usize(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION) as u32), (0x20, 4, 0xffff_ffff), (0xd9, 2, 0x42)]);
    v
}

/// Rule: the Regionalliga is played home and away, 3 points a win and 1 a draw; two go up from each section to the 2.
/// Bundesliga and four go down (five in 2000), with no league below.
///
/// The fields of `ger_regional_rules`, then the round table (virtual `comp_vtable::CREATE_SCHEDULE`(−1,
/// &`league::ROUND_COUNT`, &`league::STAGE_ID`, 0)) in `league::ROUND_DATES` (`eng_prm_apply_league_rules`). Returns 1.
/// Original: `GerRegional::set_up_rules` (0x5dd9f0, ger_regional.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_regional_set_up_rules(this: usize) -> u32 {
    covered!("rx10_4 regionalliga rules");
    eng_prm_apply_league_rules(League(this), &ger_regional_rules(this))
}

/// Comparer for `ger_regional_set_up_rules`, as a dry run (`eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_ger_regional_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_rules("ger_regional_set_up_rules", League(this), ger_regional_rules(this), || o(this), || ger_regional_set_up_rules(this))
}

/// Rule: the Regionalliga North has 18 clubs (19 in 2000): the Regionalliga clubs of the North section, in database
/// order; a B team whose senior club is above cannot go up.
///
/// `league::CLUB_COUNT` = 18 (19 in 2000), `league::GAMES_PER_CLUB` = −1, then `fill_section_table` with the
/// Regionalliga (`KEY_CLUB_COMP_GERMAN_REGIONAL`) and the North section (0x9cf55c) in the reserve division, the B teams
/// marked, the boxes at lines 0x13b and 0x14c. No prize fund. Returns 1 when full.
/// Original: `GerRegional::create_table` (0x5dd7f0, ger_regional.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_regional_create_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(if l.year() == 0x7d0 { 0x13 } else { 0x12 }); // rx10_4: 18 in the North (19 in 2000)
    l.set_games_per_club(-1);
    covered!("rx10_4 regionalliga north table");
    fill_section_table(l, KEY_CLUB_COMP_GERMAN_REGIONAL, 0x9cf55c, true, 0x9cd488, [0x13b, 0x14c])
}

/// Comparer for `ger_regional_create_table`: only the original, with the plan (`compare_section_table`).
pub(crate) unsafe extern "thiscall" fn compare_ger_regional_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_section_table("ger_regional_create_table", this, KEY_CLUB_COMP_GERMAN_REGIONAL, 0x9cf55c, || o(this))
}
