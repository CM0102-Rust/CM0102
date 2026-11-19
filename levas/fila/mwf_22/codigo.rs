// ---------- batch %%LEVA%%: the constructors of the K League and the Korean League Cup (kor_league.cpp, kor_league_cup.cpp) ----------

pub(crate) const KOR_LEAGUE_DIVISION: NationDivision = NationDivision {
    vtable: 0x96cdd0,
    bench: None,
    discipline: discipline_rule::KOREA, // mwf22: the K League discipline rule
    stages: 0,
    set_up_rules: 0x66bcc0,
    create_table: 0x66bba0,
    source_file: 0x9eb120,
    box_lines: [0x47, 0x4e, 0x55, 0x5e, 0x65],
};

/// Rule: the K League at career creation: discipline rule 42, only the league (no stages); with no saved state, the
/// rules, the table with its ten clubs, the fixtures, the order, the statistics and the predicted positions
/// (`construct_nation_division`, the GerFirst body checked instruction by instruction: only the discipline rule, the
/// vtable, the rules and table routines and the file change).
/// Original: `KorLeague::KorLeague` (0x66ae70, kor_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_kor_league(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &KOR_LEAGUE_DIVISION, || {})
}

/// Comparer for `construct_kor_league`: only the original runs (`compare_nation_division_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_kor_league(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_KOR_LEAGUE);
    compare_nation_division_ctor("construct_kor_league", League(this), year, record, &KOR_LEAGUE_DIVISION, || o(this, year, record))
}

/// The source file of the Korean League Cup's error boxes.
pub(crate) const KOR_LEAGUE_CUP_FILE: usize = 0x9eb15c;

/// The fixed fields the Korean League Cup constructor writes on a new cup (offset, width, value).
const KOREAN_LEAGUE_CUP_FIELDS: [(usize, u8, u32); 7] = [(league::STAGE_COUNT, 4, 2), (league::STAGE_INDEX, 1, 0xff), (league::SUBSTITUTES_NAMED, 1, 5), (league::SUBSTITUTIONS_ALLOWED, 1, 3),
    (league::DISCIPLINE_RULE, 1, discipline_rule::KOREA as u32), (league::IMPORTANCE, 1, 0xc), (league::LEAGUE_FLAGS, 2, 8)];

/// Rule: the Korean League Cup at career creation: the year and the database competition, 5 substitutes on the bench
/// and 3 substitutions, discipline rule 42, importance 12; with nothing saved, flags 8, two stages (group B and the
/// knockout), the ten clubs, the statistics, the predicted positions, the rules, group A's table (the cup is itself the
/// league of group A), its fixtures and order, and group B's stage.
///
/// The ItaCCup/IreLgeCup body (`construct_irish_league_cup`), checked instruction by instruction: over League::League
/// (`exe::new_league`, thiscall, ret 0) the record, the vtable 0x96ce84, the year, the bench, the discipline rule and
/// importance 0xc; the saved state (`exe::league_load_from_temp_file`(this, 1); nonzero: returns this). Otherwise
/// `league::LEAGUE_FLAGS` = 8 (0x28 in the Irish and Italian cups), the main stage, current stage −1, two stages and
/// their array (`exe::crt_malloc` of 8, unchecked); KorLeagueCup::select_teams (0x66cf20, unchecked); the statistics
/// (`ita_cups_create_comp_stats`; the box at line 0x53); KorLeagueCup::set_predicted_positions (0x66d3a0, unchecked);
/// KorLeagueCup::set_up_rules (0x66c870; 0: line 0x5d); KorLeagueCup::create_group_a_table (0x66c770; 0: line 0x64);
/// the fixtures (`exe::league_create_fixtures`; 0: line 0x6a); the order (`exe::league_sort_table`(this, 0); 0: line
/// 0x71); KorLeagueCup::create_group_b_stage(0) (0x66cb60: push 0; mov ecx, this -> thiscall, ret 4). Every routine
/// called is thiscall, ret 0 unless said. Each box returns this. Returns this.
/// Original: `KorLeagueCup::KorLeagueCup` (0x66bf00, kor_league_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_kor_league_cup(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), 0x96ce84);
    this.set_year(year as u16 as i16);
    this.set_substitutions_allowed(3);
    this.set_substitutes_named(5);
    this.set_discipline_rule(discipline_rule::KOREA);
    this.set_importance(0xc);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("mwf_22 korean league cup new");
    this.set_league_flags(8); // mwf22: the Korean League Cup flags
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    this.set_stage_count(2);
    this.set_stages(exe::crt_malloc(8));
    let call = |f: usize| {
        let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(f);
        f(this.addr())
    };
    call(0x66cf20);
    if !ita_cups_create_comp_stats(this) {
        show_error_at(KOR_LEAGUE_CUP_FILE, 0x53);
        return this.addr();
    }
    call(0x66d3a0);
    if call(0x66c870) == 0 {
        show_error_at(KOR_LEAGUE_CUP_FILE, 0x5d);
        return this.addr();
    }
    if call(0x66c770) == 0 {
        show_error_at(KOR_LEAGUE_CUP_FILE, 0x64);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(KOR_LEAGUE_CUP_FILE, 0x6a);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(KOR_LEAGUE_CUP_FILE, 0x71);
        return this.addr();
    }
    let group_b: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x66cb60usize);
    group_b(this.addr(), 0);
    this.addr()
}

/// Comparer for `construct_kor_league_cup`: only the original runs (it allocates, draws and reads the save file); the
/// plan is checked (`ita_cups_check_constructor_plan`: this, the vtable, the record, the year and, on a new cup, the
/// fixed fields and the statistics).
pub(crate) unsafe extern "thiscall" fn compare_construct_kor_league_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_KOR_LEAGUE_CUP)(this, year, record);
    let error = ita_cups_check_constructor_plan(this, r, year, record, 0x96ce84, &KOREAN_LEAGUE_CUP_FIELDS, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_kor_league_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}
