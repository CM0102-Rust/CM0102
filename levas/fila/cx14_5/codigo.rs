// ---------- cx14_5: the Turkish category B constructor, table and season honours (tur_second_b.cpp) ----------

/// The clubs of the first category B group in database order: among the first [`G_CLUB_COUNT`] − 2 ×
/// [`G_NATION_COUNT`] clubs, those with a division (`club::DIVISION`) and a reserve division (`club::RESERVE_DIVISION`)
/// whose ids are category B's ([`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B`]) and group 1's
/// ([`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G1`]).
pub(crate) unsafe fn category_b_first_group_clubs() -> Vec<usize> {
    let mut v = Vec::new();
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2)) {
        let division = read_usize(club + club::DIVISION);
        let reserve = read_usize(club + club::RESERVE_DIVISION);
        if division != 0 && reserve != 0 && read_usize(reserve) == read_usize(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G1)
            && read_usize(division) == read_usize(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B) {
            v.push(club);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    v
}

/// Creates the table of the Turkish second division category B (its first group): `club_count` = 10, the table
/// (`exe::crt_malloc` of 0x24e; out of memory: the box at line 0x1c4 and 0) and a row (League::init_table_row
/// `exe::league_init_table_row`(this, k, club, 0, 0)) for each club of `category_b_first_group_clubs`. An eleventh club
/// shows the box at line 0x1d5, loses its division (`club::DIVISION` = 0) and ends the scan. Returns 1 when the table
/// is full, else 0.
/// Original: `TurSecondB::create_table` (0x9002c0, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_create_table(this: usize) -> u32 {
    let l = League(this);
    let mut k: u8 = 0;
    l.set_club_count(10);
    let t = exe::crt_malloc(0x24e);
    l.set_table(t);
    if t == 0 {
        show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x1c4);
        return 0;
    }
    covered!("cx14_5 category b table");
    for club in category_b_first_group_clubs() {
        if (k as i8 as i16) >= l.club_count() {
            show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x1d5);
            write_usize(club + club::DIVISION, 0);
            break;
        }
        exe::league_init_table_row(l, k as u32, club, 0, 0); // cx14_5: the row of the k-th club
        k = k.wrapping_add(1);
    }
    ((k as i8 as i16) == l.club_count()) as u32
}

/// Comparer for `tur_second_b_create_table`: only the original runs (it allocates the table); the club count, the
/// result and the clubs of the rows against `category_b_first_group_clubs`, planned beforehand.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_create_table(this: usize) -> u32 {
    let plan: Vec<usize> = category_b_first_group_clubs().into_iter().take(10).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let l = League(this);
    let found: Vec<usize> = if l.table() != 0 { (0..plan.len()).map(|k| aus_nsl_table_row_club(l, k)).collect() } else { Vec::new() };
    let same = r == (plan.len() == 10) as u32 && l.club_count() == 10 && found == plan;
    SCOREBOARD.record(same, || format!("tur_second_b_create_table({:#x}) original={} plan={:x?} table={:x?}", this, r, plan, found));
    r
}

/// The season honours of the Turkish second division category B are those of its promotion group: the call goes on to
/// stage 4 (`league::STAGES`[4]), virtual `comp_vtable::RECORD_SEASON_HONOURS`, whose result it returns (a jump in the
/// original: the caller's arguments pass through).
/// Original: `TurSecondB::record_season_honours` (0x901c60, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_record_season_honours(this: usize) -> usize {
    covered!("cx14_5 category b honours");
    let group = read_usize(League(this).stages() + 4 * 4);
    let honours: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(read_usize(read_usize(group) + comp_vtable::RECORD_SEASON_HONOURS));
    honours(group)
}

/// Comparer for `tur_second_b_record_season_honours`: only the original runs (it writes the history).
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_record_season_honours(this: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%)(this)
}

/// The Turkish second division category B in the shape the constructors' comparer (`compare_league_with_stages_ctor`)
/// checks: vtable 0x970694, the Turkish discipline rule 0x25, eleven stages (the four other first phase groups, the
/// promotion group, the five classification groups and the play-off). Its order of steps differs from
/// `construct_league_with_stages` (see `construct_tur_second_b`).
pub(crate) const TURKISH_CATEGORY_B: LeagueWithStages = LeagueWithStages {
    division: LeagueDivision { vtabela: 0x970694, discipline: 0x25, rules: 0x900460, table: 0x9002c0, file: TURKISH_CATEGORY_B_SOURCE, rows: [0x4a, 0x51, 0x69, 0x58, 0x60], low: false },
    stages: 0xb,
    finish: None,
};

/// Builds the Turkish second division category B at game creation (SEH frame only for the operator new of the
/// statistics): League::League (`exe::new_league`), the record (`league::COMPETITION`), the vtable 0x970694, the year
/// (word) and the discipline rule 0x25; the saved state (`exe::league_load_from_temp_file`, push 1) ends it. Otherwise
/// stage index 0xff, current stage −1, eleven stages and their vector (`exe::crt_malloc` of 0x2c, not checked); then,
/// each failure showing the box of its line and returning: the rules (TurSecondB::set_up_rules 0x900460, 0x4a), the
/// table (TurSecondB::create_table 0x9002c0, 0x51), League::sort_table (`exe::league_sort_table`(0), 0x58), the
/// Comp_Stats (operator new of 0x5ce, `exe::new_comp_stats`, into `league::COMP_STATS`, 0x60) and League::create_fixtures
/// (0x69). Last, the four other first phase groups (TurSecondB::create_group_stage(i), 0x900780, i = 0..3; the original
/// pushes a dword whose upper bytes are stack leftovers, the callee reads the byte) and the predicted positions
/// (TurSecondB::set_predicted_positions 0x901950). Returns this.
/// Original: `TurSecondB::TurSecondB` (0x8ff4c0, tur_second_b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_tur_second_b(this: usize, year: u32, record: usize) -> usize {
    let l = League(this);
    exe::new_league(l);
    write_usize(this + league::COMPETITION, record);
    write_usize(this, 0x970694);
    l.set_year(year as u16 as i16);
    l.set_discipline_rule(0x25); // cx14_5: the Turkish discipline rule
    if exe::league_load_from_temp_file(l, 1) != 0 {
        return this;
    }
    covered!("cx14_5 new category b");
    l.set_stage_index(stage_index::MAIN);
    l.set_current_stage(0xffff_ffff);
    l.set_stage_count(0xb);
    l.set_stages(exe::crt_malloc(0x2c));
    let step = |f: usize| {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(f);
        f(this)
    };
    if step(0x900460) == 0 {
        show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x4a);
        return this;
    }
    if step(0x9002c0) == 0 {
        show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x51);
        return this;
    }
    if exe::league_sort_table(l, 0) == 0 {
        show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x58);
        return this;
    }
    let e = exe::crt_operator_new(0x5ce);
    let stats = if e != 0 { exe::new_comp_stats(e, l) } else { 0 };
    l.set_comp_stats(stats);
    if stats == 0 {
        show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x60);
        return this;
    }
    if exe::league_create_fixtures(l) == 0 {
        show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x69);
        return this;
    }
    for i in 0..4 {
        exe::tursecondb_create_group_stage(l, i);
    }
    call_on_league(0x901950, l);
    this
}

/// Comparer for `construct_tur_second_b` (`compare_league_with_stages_ctor` with `TURKISH_CATEGORY_B`): only the
/// original runs (it allocates the league, the stages and the statistics).
pub(crate) unsafe extern "thiscall" fn compare_construct_tur_second_b(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_league_with_stages_ctor("construct_tur_second_b", League(this), year, record, &TURKISH_CATEGORY_B, || o(this, year, record))
}
