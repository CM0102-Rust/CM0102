// ---------- batch %%LEVA%%: the Belgian First Division: the constructor, the rules and the table (bel_first.cpp) ----------

/// What changes from one division to another in the constructors of the Belgian, Danish, German and Dutch leagues.
pub(crate) struct NationDivision {
    /// The vtable of the derived class.
    pub(crate) vtable: usize,
    /// The bench, when the constructor writes it: substitutes named (`league::SUBSTITUTES_NAMED`) and substitutions
    /// allowed (`league::SUBSTITUTIONS_ALLOWED`); the Belgian divisions write it here, the others in set_up_rules.
    pub(crate) bench: Option<(u8, u8)>,
    /// The discipline rule (`+0x50`), the nation's line of the Discipline table.
    pub(crate) discipline: u8,
    /// How many stages besides the league (`+0x2c`); with stages, the vector [`+0xc`] is a malloc of 4 per stage.
    pub(crate) stages: u32,
    /// The division's set_up_rules and create_table (thiscall, ret 0; 0 on failure).
    pub(crate) set_up_rules: usize,
    pub(crate) create_table: usize,
    /// The source file and the lines of the five error boxes (rules, table, fixtures, order, statistics).
    pub(crate) source_file: usize,
    pub(crate) box_lines: [u32; 5],
}

/// The body of the league constructors of Belgium, Denmark, Germany and Holland (thiscall(this, year, record), ret 8,
/// SEH frame only for the operator new); returns this.
///
/// The base (League::League `exe::new_league`: mov ecx -> thiscall, ret 0), the competition record (`+4`), the vtable,
/// the year (`+0x40`), the bench when the division writes it (`+0x49`, `+0x4a`) and the discipline rule (`+0x50`);
/// `before_load` (the Danish Superliga moves the Kvalifikationsraekken clubs first); then it tries to load the saved
/// state (League::load_from_temp_file `exe::league_load_from_temp_file`: push 1; mov ecx -> thiscall, ret 4). Loaded:
/// done.
///
/// Otherwise the new league: stage 0xff (`+0x44`), current stage -1 (`+0x30`), the stage count (`+0x2c`) and the stage
/// vector [`+0xc`] (`exe::crt_malloc` of 4 per stage: push; add esp, 4 -> cdecl; 0 without stages, unchecked); the
/// rules, the table, the fixtures (League::create_fixtures `exe::league_create_fixtures`), the order
/// (League::sort_table `exe::league_sort_table`: push 0 -> thiscall, ret 4), the statistics (operator new of 0x5ce:
/// push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats `exe::new_comp_stats`: push league; mov ecx, new -> thiscall, ret
/// 4) into `+8` and the predicted positions (`exe::league_set_predicted_positions`: mov ecx -> thiscall, ret 0); each
/// step that fails shows the error box of its line and ends.
pub(crate) unsafe fn construct_nation_division(this: League, year: u32, record: usize, p: &NationDivision, before_load: impl FnOnce()) -> usize {
    let set_up_rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.set_up_rules);
    let create_table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.create_table);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), p.vtable);
    this.set_year((year as u16) as i16);
    if let Some((named, allowed)) = p.bench {
        this.set_substitutes_named(named);
        this.set_substitutions_allowed(allowed);
    }
    this.set_discipline_rule(p.discipline);
    before_load();
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("mwc_1 new division");
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(p.stages as usize);
    if p.stages != 0 {
        let v = exe::crt_malloc(p.stages * 4);
        this.set_stages(v);
    } else {
        this.set_stages(0);
    }
    if set_up_rules(this.addr()) == 0 {
        show_error_at(p.source_file, p.box_lines[0]);
        return this.addr();
    }
    if create_table(this.addr()) == 0 {
        show_error_at(p.source_file, p.box_lines[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(p.source_file, p.box_lines[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, (0) as u32 as usize) == 0 {
        show_error_at(p.source_file, p.box_lines[3]);
        return this.addr();
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(p.source_file, p.box_lines[4]);
        return this.addr();
    }
    exe::league_set_predicted_positions(this);
    this.addr()
}

/// The comparer of these constructors: the original builds the league (it allocates and loads files), so only it runs.
/// Checks the return value, vtable, year, record, discipline rule and, for a new league, the bench and the stages.
pub(crate) unsafe fn compare_nation_division_ctor(name: &str, this: League, year: u32, record: usize, p: &NationDivision, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == p.vtable && (this.year() as u16) == year as u16
        && this.competition().addr() == record && this.discipline_rule() == p.discipline;
    if this.stage_index() == stage_index::MAIN && this.comp_stats() != 0 && !loaded {
        ok = ok && this.stage_count() == p.stages as usize && (this.stages() != 0) == (p.stages != 0);
        if let Some((named, allowed)) = p.bench {
            ok = ok && this.substitutes_named() == named && this.substitutions_allowed() == allowed;
        }
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year {} arg {}, record {:#x} arg {:#x}, discipline {}, bench {}/{}, stage index {}]: returned {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", name, this.addr(), this.year(), year as u16, this.competition().addr(), record, this.discipline_rule(), this.substitutes_named(), this.substitutions_allowed(), this.stage_index() as i32, r, read_usize(this.addr()), this.stage_count(), this.stages(), this.comp_stats()));
    r
}

/// The comparer of the set_up_rules of these leagues, as a dry run: the original builds the round table (it allocates;
/// it does not run twice).
///
/// The original runs in full; the range from `+0x1c` to the end of the last field written is restored to what it was
/// and the copy writes the rules without virtual +0x3c (`FRA_FIRST_RULES_DRY_RUN`); the fields the copy writes must
/// come out equal to the original's, and the return value too; at the end the original's range is kept (with its round
/// table at +0xba, +0xa9 and +0x3a). The range ends at the class's last field written (up to +0x104 in the Belgian
/// Third Division), never past it: the original allocates in between.
pub(crate) unsafe fn compare_nation_league_rules(name: &str, this: League, written: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    let end = written.iter().map(|e| e.0 + e.1 as usize).max().unwrap_or(0).max(league::ROUND_DATES + 4);
    let band = [(this.addr() + league::UPPER_DIVISION_ID, end - league::UPPER_DIVISION_ID)];
    let before = crate::comparison::MemorySnapshot::read(&band);
    let r = original();
    let mo = crate::comparison::MemorySnapshot::read(&band);
    before.write_back();
    FRA_FIRST_RULES_DRY_RUN.store(true, Relaxed);
    let rr = crate::rng::pause_trace(copy);
    FRA_FIRST_RULES_DRY_RUN.store(false, Relaxed);
    let mr = crate::comparison::MemorySnapshot::read(&band);
    mo.write_back();
    let field = |m: &crate::comparison::MemorySnapshot, d: usize, t: u8| -> u32 {
        let b = &m.0[0].1[d - league::UPPER_DIVISION_ID..];
        match t {
            1 => b[0] as u32,
            2 => u16::from_le_bytes([b[0], b[1]]) as u32,
            _ => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        }
    };
    let difs: Vec<(usize, u32, u32)> = written.iter().filter(|e| field(&mo, e.0, e.1) != field(&mr, e.0, e.1)).map(|e| (e.0, field(&mo, e.0, e.1), field(&mr, e.0, e.1))).collect();
    SCOREBOARD.record(difs.is_empty() && r == rr, || format!("{}({:#x}) original={} rust={} (offset, original, rust)={:x?}", name, this.addr(), r, rr, difs));
    r
}

/// The comparer of the create_table of these leagues: the original allocates the table and pays the clubs (it cannot
/// run twice), so only it runs, checked against a plan made beforehand.
///
/// The plan: the division's clubs (competition id of global `g`) in database order, up to `n`. Afterwards: `+0x3e` =
/// `n`, the return value = 1 if the plan filled up, the rows with the plan's clubs in order and, when full and the
/// division has a prize fund (`quota`), the prize mode 1 and the fund of that share.
pub(crate) unsafe fn compare_nation_league_table(name: &str, this: League, n: i16, g: usize, quota: Option<i32>, original: impl FnOnce() -> u32) -> u32 {
    let mut plan: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && (plan.len() as i16) < n {
        let club = Club(read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE);
        let comp = club.division().addr();
        if comp != 0 && read_usize(comp) == read_usize(g) {
            plan.push(club.addr());
        }
        i += 1;
    }
    let r = original();
    let t = this.table();
    let mut same = r == (plan.len() as i16 == n) as u32 && this.club_count() == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if same {
        found = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        same = found == plan;
        if let (1, Some(c)) = (r, quota) {
            same = same && this.prize_mode() == 1 && this.prize_fund() == exe::league_calc_prize_fund(this, c);
        }
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} plan={:x?} table={:x?}", name, this.addr(), r, plan, found));
    r
}

pub(crate) const BEL_FIRST_DIVISION: NationDivision = NationDivision {
    vtable: 0x967940,
    bench: Some((7, 3)),
    discipline: discipline_rule::BELGIUM_LEAGUE, // mwc1: the Belgian First Division discipline rule
    stages: 0,
    set_up_rules: 0x41e9e0,
    create_table: 0x41e8b0,
    source_file: 0x989b18,
    box_lines: [0x4b, 0x52, 0x59, 0x62, 0x69],
};

/// The Belgian First Division: discipline rule 4 (`BELGIUM_LEAGUE`), the bench (7 substitutes named, 3 substitutions),
/// only the league; with no saved state, the rules, the table, the fixtures, the order, the statistics and the
/// predicted positions (`construct_nation_division`).
/// Original: `BelFirst::BelFirst` (0x41dd40, bel_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bel_first(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &BEL_FIRST_DIVISION, || {})
}

/// Comparer for `construct_bel_first`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_bel_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_nation_division_ctor("construct_bel_first", League(this), year, record, &BEL_FIRST_DIVISION, || o(this, year, record))
}

/// The rule writes of the Belgian First Division (offset, size, value).
pub(crate) unsafe fn bel_first_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0xc5, 1, 3), (0xc6, 1, 1), (0xc7, 1, 2), (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 2), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x20, 4, read_usize(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION) as u32), (0x42, 1, 1), (0xd9, 2, 0x43)];
    v.push((0xc2, 1, 3)); // mwc1: the Belgian First Division points for a win
    v
}

/// Sets up the rules of the Belgian First Division: 2 rounds, 3 points for a win and 1 for a draw; 0 go up, 0 to the
/// promotion play-off, 0 to the relegation play-off and 2 go down; above: none; below:
/// `KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION`; tie-breaker codes 3, 1, 2, 0 (`+0xc5`..); rule bits `+0xd9` = 0x43; then
/// builds the main-stage round table through virtual +0x3c into `league::ROUND_DATES` (`fra_first_apply_league_rules`).
/// Returns 1.
/// Original: `BelFirst::set_up_rules` (0x41e9e0, bel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_first_set_up_rules(this: usize) -> u32 {
    covered!("mwc_1 bel first rules");
    fra_first_apply_league_rules(League(this), &bel_first_rules(this))
}

/// Comparer for `bel_first_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_bel_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_nation_league_rules("bel_first_set_up_rules", League(this), bel_first_rules(this), || o(this), || bel_first_set_up_rules(this))
}

/// The Belgian First Division has 18 clubs; prize fund by the share of 20,000 per place; TV pays 50,000 to each club.
///
/// `BelFirst::create_table()`: `+0x3e` = 18, the table (`exe::crt_malloc` of 18 x 0x3b; out of memory, the error box of
/// line 0x13d and 0) with the clubs of `KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the fund `+0x28` (League::calc_prize_fund, share 0x4e20), the prize
/// mode `+0xe1` = 1, the TV money (League::pay_tv_rights, 0xc350 to every club) and 1.
/// Original: `BelFirst::create_table` (0x41e8b0, bel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_first_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x12, 0x989b18, 0x13d, KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION, 0x4e20, || {
        eng_prm_pay_tv_rights(this, 0xc350, 0); // mwc1: the TV money of each club
    })
}

/// Comparer for `bel_first_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
unsafe extern "thiscall" fn compare_bel_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_nation_league_table("bel_first_create_table", League(this), 0x12, KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION, Some(0x4e20), || o(this))
}
