// ---------- batch 299: the Belgian First Division: the constructor, the rules and the table (bel_first.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BEL_FIRST);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BEL_FIRST_SET_UP_RULES);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BEL_FIRST_CREATE_TABLE);
    compare_nation_league_table("bel_first_create_table", League(this), 0x12, KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION, Some(0x4e20), || o(this))
}

// ---------- batch 299: the schedules of the Belgian First Division, the 2. Bundesliga and the Eredivisie (bel_first.cpp, ger_second.cpp, hol_prm.cpp) ----------

/// The Belgian First Division: 34 rounds (date, TV slots).
pub(crate) const BEL_FIRST_SCHEDULE_ROUNDS: [FraThirdLeagueRound; 34] = [
    (12, 7, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 0: 12 Aug
    (19, 7, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 1: 19 Aug
    (26, 7, 0, 5, 2, &[(-1, -1, -1)]), // 2: 26 Aug
    (6, 8, 0, 2, 2, &[(-1, -1, -1)]), // 3: 6 Sep
    (9, 8, 0, 5, 2, &[(-1, -1, -1)]), // 4: 9 Sep
    (16, 8, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 5: 16 Sep (mwc2 bel_first)
    (23, 8, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 6: 23 Sep
    (30, 8, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 7: 30 Sep
    (14, 9, 0, 5, 2, &[(-1, -1, -1)]), // 8: 14 Oct
    (21, 9, 0, 5, 2, &[(-1, -1, -1)]), // 9: 21 Oct
    (28, 9, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 10: 28 Oct
    (11, 10, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 11: 11 Nov
    (18, 10, 0, 5, 2, &[(-1, -1, -1)]), // 12: 18 Nov
    (25, 10, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 13: 25 Nov
    (2, 11, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 14: 2 Dec
    (9, 11, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 15: 9 Dec
    (16, 11, 0, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 16: 16 Dec
    (20, 11, 0, 2, 2, &[(-1, -1, -1)]), // 17: 20 Dec
    (20, 0, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 18: 20 Jan (+1 year)
    (27, 0, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 19: 27 Jan (+1 year)
    (3, 1, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 20: 3 Feb (+1 year)
    (10, 1, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 21: 10 Feb (+1 year)
    (17, 1, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 22: 17 Feb (+1 year)
    (3, 2, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 23: 3 Mar (+1 year)
    (10, 2, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 24: 10 Mar (+1 year)
    (17, 2, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 25: 17 Mar (+1 year)
    (31, 2, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 26: 31 Mar (+1 year)
    (7, 3, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 27: 7 Apr (+1 year)
    (14, 3, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 28: 14 Apr (+1 year)
    (21, 3, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 29: 21 Apr (+1 year)
    (28, 3, 1, 5, 2, &[(4, 2, 1), (-1, -1, -1)]), // 30: 28 Apr (+1 year)
    (5, 4, 1, 5, 2, &[(-1, -1, -1)]), // 31: 5 May (+1 year)
    (13, 4, 1, 6, 2, &[(-1, -1, -1)]), // 32: 13 May (+1 year)
    (20, 4, 1, 6, 2, &[(-1, -1, -1)]), // 33: 20 May (+1 year)
];
pub(crate) const BEL_FIRST_SCHEDULE: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 34, file: 0x989b18, row: 0xa5, table: &BEL_FIRST_SCHEDULE_ROUNDS, branch: "mwc_2 bel first schedule" };

/// The Belgian First Division schedule: 34 rounds, from 12 Aug to 20 May; 24 with matches in TV slots.
///
/// `BelFirst::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage (-1;
/// the others return 0 without writing); is_league = 1, rounds = 34, extra = 0 and the table of 34 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa5 and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `BelFirst::create_schedule` (0x41e010, bel_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bel_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_league_schedule(League(this), stage, rounds, extra, league, &BEL_FIRST_SCHEDULE)
}

/// Comparer for `bel_first_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
unsafe extern "thiscall" fn compare_bel_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BEL_FIRST_CREATE_SCHEDULE);
    compare_cfa_schedule("bel_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| bel_first_create_schedule(this, stage, r, e, l))
}

/// The 2. Bundesliga: 34 rounds (date, TV slots).
pub(crate) const GER_SECOND_SCHEDULE_ROUNDS: [FraThirdLeagueRound; 34] = [
    (30, 6, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 0: 30 Jul
    (6, 7, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 1: 6 Aug
    (13, 7, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 2: 13 Aug
    (20, 7, 0, 6, 1, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 3: 20 Aug
    (6, 8, 0, 2, 2, &[(-1, -1, -1)]), // 4: 6 Sep
    (10, 8, 0, 6, 1, &[(0, 2, 1), (-1, -1, -1)]), // 5: 10 Sep (mwc2 ger_second)
    (17, 8, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 6: 17 Sep
    (24, 8, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 7: 24 Sep
    (1, 9, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 8: 1 Oct
    (15, 9, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 9: 15 Oct
    (22, 9, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 10: 22 Oct
    (29, 9, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 11: 29 Oct
    (5, 10, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 12: 5 Nov
    (12, 10, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 13: 12 Nov
    (19, 10, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 14: 19 Nov
    (26, 10, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 15: 26 Nov
    (3, 11, 0, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 16: 3 Dec
    (10, 11, 0, 6, 1, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 17: 10 Dec
    (13, 11, 0, 2, 2, &[(-1, -1, -1)]), // 18: 13 Dec
    (17, 11, 0, 6, 1, &[(-1, -1, -1)]), // 19: 17 Dec
    (28, 0, 1, 6, 1, &[(-1, -1, -1)]), // 20: 28 Jan (+1 year)
    (4, 1, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 21: 4 Feb (+1 year)
    (11, 1, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 22: 11 Feb (+1 year)
    (18, 1, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 23: 18 Feb (+1 year)
    (25, 1, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 24: 25 Feb (+1 year)
    (4, 2, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 25: 4 Mar (+1 year)
    (11, 2, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 26: 11 Mar (+1 year)
    (18, 2, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 27: 18 Mar (+1 year)
    (1, 3, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 28: 1 Apr (+1 year)
    (8, 3, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 29: 8 Apr (+1 year)
    (15, 3, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 30: 15 Apr (+1 year)
    (22, 3, 1, 6, 1, &[(0, 2, 1), (4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 31: 22 Apr (+1 year)
    (29, 3, 1, 6, 1, &[(-1, -1, -1)]), // 32: 29 Apr (+1 year)
    (6, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 6 May (+1 year)
];
pub(crate) const GER_SECOND_SCHEDULE: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 34, file: 0x9cd4c4, row: 0xa7, table: &GER_SECOND_SCHEDULE_ROUNDS, branch: "mwc_2 ger second schedule" };

/// The 2. Bundesliga schedule: 34 rounds, from 30 Jul to 6 May; 28 with matches in TV slots.
///
/// `GerSecond::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage
/// (-1; the others return 0 without writing); is_league = 1, rounds = 34, extra = 0 and the table of 34 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa7 and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `GerSecond::create_schedule` (0x5de5d0, ger_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ger_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_league_schedule(League(this), stage, rounds, extra, league, &GER_SECOND_SCHEDULE)
}

/// Comparer for `ger_second_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
unsafe extern "thiscall" fn compare_ger_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GER_SECOND_CREATE_SCHEDULE);
    compare_cfa_schedule("ger_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| ger_second_create_schedule(this, stage, r, e, l))
}

/// The Eredivisie: 34 rounds (date, TV slots).
pub(crate) const HOL_PRM_SCHEDULE_ROUNDS: [FraThirdLeagueRound; 34] = [
    (14, 7, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 0: 14 Aug
    (21, 7, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 1: 21 Aug
    (28, 7, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 2: 28 Aug
    (11, 8, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 3: 11 Sep
    (15, 8, 0, 2, 2, &[(1, 2, 1), (-1, -1, -1)]), // 4: 15 Sep
    (18, 8, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 5: 18 Sep (mwc2 hol_prm)
    (25, 8, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 6: 25 Sep
    (29, 8, 0, 2, 2, &[(1, 2, 1), (-1, -1, -1)]), // 7: 29 Sep
    (2, 9, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 8: 2 Oct
    (13, 9, 0, 2, 2, &[(1, 2, 1), (-1, -1, -1)]), // 9: 13 Oct
    (16, 9, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 10: 16 Oct
    (23, 9, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 11: 23 Oct
    (30, 10, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 12: 30 Nov
    (6, 10, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 13: 6 Nov
    (20, 10, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 14: 20 Nov
    (27, 10, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 15: 27 Nov
    (4, 11, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 16: 4 Dec
    (11, 11, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 17: 11 Dec
    (18, 11, 0, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 18: 18 Dec
    (5, 1, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 19: 5 Feb (+1 year)
    (12, 1, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 20: 12 Feb (+1 year)
    (19, 1, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 21: 19 Feb (+1 year)
    (26, 1, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 22: 26 Feb (+1 year)
    (4, 2, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 23: 4 Mar (+1 year)
    (11, 2, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 24: 11 Mar (+1 year)
    (18, 2, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 25: 18 Mar (+1 year)
    (25, 2, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 26: 25 Mar (+1 year)
    (1, 3, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 27: 1 Apr (+1 year)
    (8, 3, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 28: 8 Apr (+1 year)
    (15, 3, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 29: 15 Apr (+1 year)
    (22, 3, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 30: 22 Apr (+1 year)
    (29, 3, 1, 5, 2, &[(4, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 31: 29 Apr (+1 year)
    (7, 4, 1, 6, 2, &[(-1, -1, -1)]), // 32: 7 May (+1 year)
    (14, 4, 1, 6, 2, &[(-1, -1, -1)]), // 33: 14 May (+1 year)
];
pub(crate) const HOL_PRM_SCHEDULE: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 34, file: 0x9cefa4, row: 0x9d, table: &HOL_PRM_SCHEDULE_ROUNDS, branch: "mwc_2 hol prm schedule" };

/// The Dutch Eredivisie schedule: 34 rounds, from 14 Aug to 14 May; 32 with matches in TV slots.
///
/// `HolPrm::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage (-1;
/// the others return 0 without writing); is_league = 1, rounds = 34, extra = 0 and the table of 34 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0x9d and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `HolPrm::create_schedule` (0x5f78d0, hol_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn hol_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_league_schedule(League(this), stage, rounds, extra, league, &HOL_PRM_SCHEDULE)
}

/// Comparer for `hol_prm_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
unsafe extern "thiscall" fn compare_hol_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_HOL_PRM_CREATE_SCHEDULE);
    compare_cfa_schedule("hol_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| hol_prm_create_schedule(this, stage, r, e, l))
}

// ---------- batch 299: the schedules of the three Danish divisions (odd and even years) (den_first.cpp, den_second.cpp, den_prm.cpp) ----------

/// The Danish First Division in a season starting in an odd year (2001/02): the spring from round 15 differs.
pub(crate) const DEN_FIRST_ODD_YEAR_ROUNDS: [FraThirdLeagueRound; 30] = [
    (3, 7, 0, 6, 1, &[(-1, -1, -1)]), // 0: 3 Aug
    (10, 7, 0, 6, 1, &[(-1, -1, -1)]), // 1: 10 Aug
    (17, 7, 0, 6, 1, &[(-1, -1, -1)]), // 2: 17 Aug
    (24, 7, 0, 6, 1, &[(-1, -1, -1)]), // 3: 24 Aug
    (31, 7, 0, 6, 1, &[(-1, -1, -1)]), // 4: 31 Aug
    (7, 8, 0, 6, 1, &[(-1, -1, -1)]), // 5: 7 Sep (mwc3 den_first)
    (14, 8, 0, 6, 1, &[(-1, -1, -1)]), // 6: 14 Sep
    (21, 8, 0, 6, 1, &[(-1, -1, -1)]), // 7: 21 Sep
    (28, 8, 0, 6, 1, &[(-1, -1, -1)]), // 8: 28 Sep
    (5, 9, 0, 6, 1, &[(-1, -1, -1)]), // 9: 5 Oct
    (12, 9, 0, 6, 1, &[(-1, -1, -1)]), // 10: 12 Oct
    (19, 9, 0, 6, 1, &[(-1, -1, -1)]), // 11: 19 Oct
    (26, 9, 0, 6, 1, &[(-1, -1, -1)]), // 12: 26 Oct
    (2, 10, 0, 6, 1, &[(-1, -1, -1)]), // 13: 2 Nov
    (9, 10, 0, 6, 1, &[(-1, -1, -1)]), // 14: 9 Nov
    (4, 2, 1, 2, 2, &[(-1, -1, -1)]), // 15: 4 Mar (+1 year)
    (8, 2, 1, 6, 1, &[(-1, -1, -1)]), // 16: 8 Mar (+1 year)
    (15, 2, 1, 6, 1, &[(-1, -1, -1)]), // 17: 15 Mar (+1 year)
    (22, 2, 1, 6, 1, &[(-1, -1, -1)]), // 18: 22 Mar (+1 year)
    (25, 2, 1, 2, 2, &[(-1, -1, -1)]), // 19: 25 Mar (+1 year)
    (29, 2, 1, 6, 1, &[(-1, -1, -1)]), // 20: 29 Mar (+1 year)
    (5, 3, 1, 6, 1, &[(-1, -1, -1)]), // 21: 5 Apr (+1 year)
    (8, 3, 1, 2, 2, &[(-1, -1, -1)]), // 22: 8 Apr (+1 year)
    (12, 3, 1, 6, 1, &[(-1, -1, -1)]), // 23: 12 Apr (+1 year)
    (19, 3, 1, 6, 1, &[(-1, -1, -1)]), // 24: 19 Apr (+1 year)
    (26, 3, 1, 6, 1, &[(-1, -1, -1)]), // 25: 26 Apr (+1 year)
    (3, 4, 1, 6, 1, &[(-1, -1, -1)]), // 26: 3 May (+1 year)
    (10, 4, 1, 6, 1, &[(-1, -1, -1)]), // 27: 10 May (+1 year)
    (17, 4, 1, 6, 1, &[(-1, -1, -1)]), // 28: 17 May (+1 year)
    (24, 4, 1, 6, 1, &[(-1, -1, -1)]), // 29: 24 May (+1 year)
];
pub(crate) const DEN_FIRST_ODD_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 30, file: 0x9b8e30, row: 0x9c, table: &DEN_FIRST_ODD_YEAR_ROUNDS, branch: "mwc_3 den first schedule" };

/// The Danish First Division in a season starting in an even year.
pub(crate) const DEN_FIRST_EVEN_YEAR_ROUNDS: [FraThirdLeagueRound; 30] = [
    (3, 7, 0, 6, 1, &[(-1, -1, -1)]), // 0: 3 Aug
    (10, 7, 0, 6, 1, &[(-1, -1, -1)]), // 1: 10 Aug
    (17, 7, 0, 6, 1, &[(-1, -1, -1)]), // 2: 17 Aug
    (24, 7, 0, 6, 1, &[(-1, -1, -1)]), // 3: 24 Aug
    (31, 7, 0, 6, 1, &[(-1, -1, -1)]), // 4: 31 Aug
    (7, 8, 0, 6, 1, &[(-1, -1, -1)]), // 5: 7 Sep
    (14, 8, 0, 6, 1, &[(-1, -1, -1)]), // 6: 14 Sep
    (21, 8, 0, 6, 1, &[(-1, -1, -1)]), // 7: 21 Sep
    (28, 8, 0, 6, 1, &[(-1, -1, -1)]), // 8: 28 Sep
    (5, 9, 0, 6, 1, &[(-1, -1, -1)]), // 9: 5 Oct
    (12, 9, 0, 6, 1, &[(-1, -1, -1)]), // 10: 12 Oct
    (19, 9, 0, 6, 1, &[(-1, -1, -1)]), // 11: 19 Oct
    (26, 9, 0, 6, 1, &[(-1, -1, -1)]), // 12: 26 Oct
    (2, 10, 0, 6, 1, &[(-1, -1, -1)]), // 13: 2 Nov
    (9, 10, 0, 6, 1, &[(-1, -1, -1)]), // 14: 9 Nov
    (15, 2, 1, 6, 1, &[(-1, -1, -1)]), // 15: 15 Mar (+1 year)
    (22, 2, 1, 6, 1, &[(-1, -1, -1)]), // 16: 22 Mar (+1 year)
    (25, 2, 1, 2, 2, &[(-1, -1, -1)]), // 17: 25 Mar (+1 year)
    (29, 2, 1, 6, 1, &[(-1, -1, -1)]), // 18: 29 Mar (+1 year)
    (5, 3, 1, 6, 1, &[(-1, -1, -1)]), // 19: 5 Apr (+1 year)
    (8, 3, 1, 2, 2, &[(-1, -1, -1)]), // 20: 8 Apr (+1 year)
    (12, 3, 1, 6, 1, &[(-1, -1, -1)]), // 21: 12 Apr (+1 year)
    (19, 3, 1, 6, 1, &[(-1, -1, -1)]), // 22: 19 Apr (+1 year)
    (26, 3, 1, 6, 1, &[(-1, -1, -1)]), // 23: 26 Apr (+1 year)
    (3, 4, 1, 6, 1, &[(-1, -1, -1)]), // 24: 3 May (+1 year)
    (10, 4, 1, 6, 1, &[(-1, -1, -1)]), // 25: 10 May (+1 year)
    (17, 4, 1, 6, 1, &[(-1, -1, -1)]), // 26: 17 May (+1 year)
    (24, 4, 1, 6, 1, &[(-1, -1, -1)]), // 27: 24 May (+1 year)
    (31, 4, 1, 6, 1, &[(-1, -1, -1)]), // 28: 31 May (+1 year)
    (7, 5, 1, 6, 1, &[(-1, -1, -1)]), // 29: 7 Jun (+1 year)
];
pub(crate) const DEN_FIRST_EVEN_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 30, file: 0x9b8e30, row: 0x9c, table: &DEN_FIRST_EVEN_YEAR_ROUNDS, branch: "mwc_3 den first schedule" };

/// The Danish First Division schedule: 30 rounds, from 3 Aug to 24 May; 0 with matches in TV slots; the spring changes
/// with the parity of the season year (from round 15: odd, as 2001/02, and even).
///
/// `DenFirst::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage (-1;
/// the others return 0 without writing); is_league = 1, rounds = 30, extra = 0 and the table of 30 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0x9c and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `DenFirst::create_schedule` (0x5519b0, den_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn den_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let year = League(this).year();
    let p = if (year as i32) % 2 != 0 { &DEN_FIRST_ODD_YEAR } else { &DEN_FIRST_EVEN_YEAR };
    create_league_schedule(League(this), stage, rounds, extra, league, p)
}

/// Comparer for `den_first_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
unsafe extern "thiscall" fn compare_den_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_DEN_FIRST_CREATE_SCHEDULE);
    compare_cfa_schedule("den_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| den_first_create_schedule(this, stage, r, e, l))
}

/// The Danish Second Division in a season starting in an odd year (2001/02): the spring from round 15 differs.
pub(crate) const DEN_SECOND_ODD_YEAR_ROUNDS: [FraThirdLeagueRound; 30] = [
    (2, 7, 0, 6, 1, &[(-1, -1, -1)]), // 0: 2 Aug
    (9, 7, 0, 6, 1, &[(-1, -1, -1)]), // 1: 9 Aug
    (16, 7, 0, 6, 1, &[(-1, -1, -1)]), // 2: 16 Aug
    (23, 7, 0, 6, 1, &[(-1, -1, -1)]), // 3: 23 Aug
    (30, 7, 0, 6, 1, &[(-1, -1, -1)]), // 4: 30 Aug
    (6, 8, 0, 6, 1, &[(-1, -1, -1)]), // 5: 6 Sep (mwc3 den_second)
    (13, 8, 0, 6, 1, &[(-1, -1, -1)]), // 6: 13 Sep
    (20, 8, 0, 6, 1, &[(-1, -1, -1)]), // 7: 20 Sep
    (27, 8, 0, 6, 1, &[(-1, -1, -1)]), // 8: 27 Sep
    (4, 9, 0, 6, 1, &[(-1, -1, -1)]), // 9: 4 Oct
    (11, 9, 0, 6, 1, &[(-1, -1, -1)]), // 10: 11 Oct
    (18, 9, 0, 6, 1, &[(-1, -1, -1)]), // 11: 18 Oct
    (25, 9, 0, 6, 1, &[(-1, -1, -1)]), // 12: 25 Oct
    (1, 10, 0, 6, 1, &[(-1, -1, -1)]), // 13: 1 Nov
    (8, 10, 0, 6, 1, &[(-1, -1, -1)]), // 14: 8 Nov
    (15, 2, 1, 6, 1, &[(-1, -1, -1)]), // 15: 15 Mar (+1 year)
    (22, 2, 1, 6, 1, &[(-1, -1, -1)]), // 16: 22 Mar (+1 year)
    (29, 2, 1, 6, 1, &[(-1, -1, -1)]), // 17: 29 Mar (+1 year)
    (1, 3, 1, 2, 2, &[(-1, -1, -1)]), // 18: 1 Apr (+1 year)
    (5, 3, 1, 6, 1, &[(-1, -1, -1)]), // 19: 5 Apr (+1 year)
    (12, 3, 1, 6, 1, &[(-1, -1, -1)]), // 20: 12 Apr (+1 year)
    (15, 3, 1, 2, 2, &[(-1, -1, -1)]), // 21: 15 Apr (+1 year)
    (19, 3, 1, 6, 1, &[(-1, -1, -1)]), // 22: 19 Apr (+1 year)
    (26, 3, 1, 6, 1, &[(-1, -1, -1)]), // 23: 26 Apr (+1 year)
    (3, 4, 1, 6, 1, &[(-1, -1, -1)]), // 24: 3 May (+1 year)
    (10, 4, 1, 6, 1, &[(-1, -1, -1)]), // 25: 10 May (+1 year)
    (13, 4, 1, 2, 2, &[(-1, -1, -1)]), // 26: 13 May (+1 year)
    (17, 4, 1, 6, 1, &[(-1, -1, -1)]), // 27: 17 May (+1 year)
    (24, 4, 1, 6, 1, &[(-1, -1, -1)]), // 28: 24 May (+1 year)
    (31, 4, 1, 6, 1, &[(-1, -1, -1)]), // 29: 31 May (+1 year)
];
pub(crate) const DEN_SECOND_ODD_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 30, file: 0x9b8ea4, row: 0xa2, table: &DEN_SECOND_ODD_YEAR_ROUNDS, branch: "mwc_3 den second schedule" };

/// The Danish Second Division in a season starting in an even year.
pub(crate) const DEN_SECOND_EVEN_YEAR_ROUNDS: [FraThirdLeagueRound; 30] = [
    (2, 7, 0, 6, 1, &[(-1, -1, -1)]), // 0: 2 Aug
    (9, 7, 0, 6, 1, &[(-1, -1, -1)]), // 1: 9 Aug
    (16, 7, 0, 6, 1, &[(-1, -1, -1)]), // 2: 16 Aug
    (23, 7, 0, 6, 1, &[(-1, -1, -1)]), // 3: 23 Aug
    (30, 7, 0, 6, 1, &[(-1, -1, -1)]), // 4: 30 Aug
    (6, 8, 0, 6, 1, &[(-1, -1, -1)]), // 5: 6 Sep
    (13, 8, 0, 6, 1, &[(-1, -1, -1)]), // 6: 13 Sep
    (20, 8, 0, 6, 1, &[(-1, -1, -1)]), // 7: 20 Sep
    (27, 8, 0, 6, 1, &[(-1, -1, -1)]), // 8: 27 Sep
    (4, 9, 0, 6, 1, &[(-1, -1, -1)]), // 9: 4 Oct
    (11, 9, 0, 6, 1, &[(-1, -1, -1)]), // 10: 11 Oct
    (18, 9, 0, 6, 1, &[(-1, -1, -1)]), // 11: 18 Oct
    (25, 9, 0, 6, 1, &[(-1, -1, -1)]), // 12: 25 Oct
    (1, 10, 0, 6, 1, &[(-1, -1, -1)]), // 13: 1 Nov
    (8, 10, 0, 6, 1, &[(-1, -1, -1)]), // 14: 8 Nov
    (29, 2, 1, 6, 1, &[(-1, -1, -1)]), // 15: 29 Mar (+1 year)
    (1, 3, 1, 2, 2, &[(-1, -1, -1)]), // 16: 1 Apr (+1 year)
    (5, 3, 1, 6, 1, &[(-1, -1, -1)]), // 17: 5 Apr (+1 year)
    (12, 3, 1, 6, 1, &[(-1, -1, -1)]), // 18: 12 Apr (+1 year)
    (15, 3, 1, 2, 2, &[(-1, -1, -1)]), // 19: 15 Apr (+1 year)
    (19, 3, 1, 6, 1, &[(-1, -1, -1)]), // 20: 19 Apr (+1 year)
    (26, 3, 1, 6, 1, &[(-1, -1, -1)]), // 21: 26 Apr (+1 year)
    (3, 4, 1, 6, 1, &[(-1, -1, -1)]), // 22: 3 May (+1 year)
    (10, 4, 1, 6, 1, &[(-1, -1, -1)]), // 23: 10 May (+1 year)
    (13, 4, 1, 2, 2, &[(-1, -1, -1)]), // 24: 13 May (+1 year)
    (17, 4, 1, 6, 1, &[(-1, -1, -1)]), // 25: 17 May (+1 year)
    (24, 4, 1, 6, 1, &[(-1, -1, -1)]), // 26: 24 May (+1 year)
    (31, 4, 1, 6, 1, &[(-1, -1, -1)]), // 27: 31 May (+1 year)
    (7, 5, 1, 6, 1, &[(-1, -1, -1)]), // 28: 7 Jun (+1 year)
    (14, 5, 1, 6, 1, &[(-1, -1, -1)]), // 29: 14 Jun (+1 year)
];
pub(crate) const DEN_SECOND_EVEN_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 30, file: 0x9b8ea4, row: 0xa2, table: &DEN_SECOND_EVEN_YEAR_ROUNDS, branch: "mwc_3 den second schedule" };

/// The Danish Second Division schedule: 30 rounds, from 2 Aug to 31 May; 0 with matches in TV slots; the spring changes
/// with the parity of the season year (from round 15: odd, as 2001/02, and even).
///
/// `DenSecond::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage
/// (-1; the others return 0 without writing); is_league = 1, rounds = 30, extra = 0 and the table of 30 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa2 and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `DenSecond::create_schedule` (0x554170, den_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn den_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let year = League(this).year();
    let p = if (year as i32) % 2 != 0 { &DEN_SECOND_ODD_YEAR } else { &DEN_SECOND_EVEN_YEAR };
    create_league_schedule(League(this), stage, rounds, extra, league, p)
}

/// Comparer for `den_second_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
unsafe extern "thiscall" fn compare_den_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_DEN_SECOND_CREATE_SCHEDULE);
    compare_cfa_schedule("den_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| den_second_create_schedule(this, stage, r, e, l))
}

/// The Danish Superliga in a season starting in an odd year (2001/02): the spring from round 18 differs.
pub(crate) const DEN_PRM_ODD_YEAR_ROUNDS: [FraThirdLeagueRound; 33] = [
    (27, 6, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 0: 27 Jul
    (3, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 1: 3 Aug
    (10, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 2: 10 Aug
    (17, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 3: 17 Aug
    (24, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 4: 24 Aug
    (31, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 5: 31 Aug (mwc3 den_prm)
    (7, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 6: 7 Sep
    (14, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 7: 14 Sep
    (21, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 8: 21 Sep
    (28, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 9: 28 Sep
    (5, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 10: 5 Oct
    (12, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 11: 12 Oct
    (19, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 12: 19 Oct
    (26, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 13: 26 Oct
    (2, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 14: 2 Nov
    (9, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 15: 9 Nov
    (16, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 16: 16 Nov
    (23, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 17: 23 Nov
    (1, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 18: 1 Mar (+1 year)
    (4, 2, 1, 2, 2, &[(-1, -1, -1)]), // 19: 4 Mar (+1 year)
    (8, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 20: 8 Mar (+1 year)
    (11, 3, 1, 2, 2, &[(-1, -1, -1)]), // 21: 11 Apr (+1 year)
    (15, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 22: 15 Mar (+1 year)
    (22, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 23: 22 Mar (+1 year)
    (29, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 24: 29 Mar (+1 year)
    (5, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 25: 5 Apr (+1 year)
    (12, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 26: 12 Apr (+1 year)
    (19, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 27: 19 Apr (+1 year)
    (26, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 28: 26 Apr (+1 year)
    (3, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 29: 3 May (+1 year)
    (10, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 30: 10 May (+1 year)
    (17, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 31: 17 May (+1 year)
    (24, 4, 1, 6, 1, &[(-1, -1, -1)]), // 32: 24 May (+1 year)
];
pub(crate) const DEN_PRM_ODD_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 33, file: 0x9b8e6c, row: 0xa6, table: &DEN_PRM_ODD_YEAR_ROUNDS, branch: "mwc_3 den prm schedule" };

/// The Danish Superliga in a season starting in an even year.
pub(crate) const DEN_PRM_EVEN_YEAR_ROUNDS: [FraThirdLeagueRound; 33] = [
    (27, 6, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 0: 27 Jul
    (3, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 1: 3 Aug
    (10, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 2: 10 Aug
    (17, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 3: 17 Aug
    (24, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 4: 24 Aug
    (31, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 5: 31 Aug
    (7, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 6: 7 Sep
    (14, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 7: 14 Sep
    (21, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 8: 21 Sep
    (28, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 9: 28 Sep
    (5, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 10: 5 Oct
    (12, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 11: 12 Oct
    (19, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 12: 19 Oct
    (26, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 13: 26 Oct
    (2, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 14: 2 Nov
    (9, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 15: 9 Nov
    (16, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 16: 16 Nov
    (23, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 17: 23 Nov
    (15, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 18: 15 Mar (+1 year)
    (22, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 19: 22 Mar (+1 year)
    (29, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 20: 29 Mar (+1 year)
    (5, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 21: 5 Apr (+1 year)
    (12, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 22: 12 Apr (+1 year)
    (19, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 23: 19 Apr (+1 year)
    (26, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 24: 26 Apr (+1 year)
    (3, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 25: 3 May (+1 year)
    (10, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 26: 10 May (+1 year)
    (17, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 27: 17 May (+1 year)
    (24, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 28: 24 May (+1 year)
    (28, 4, 1, 2, 2, &[(-1, -1, -1)]), // 29: 28 May (+1 year)
    (31, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 30: 31 May (+1 year)
    (7, 5, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 31: 7 Jun (+1 year)
    (14, 5, 1, 6, 1, &[(-1, -1, -1)]), // 32: 14 Jun (+1 year)
];
pub(crate) const DEN_PRM_EVEN_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 33, file: 0x9b8e6c, row: 0xa6, table: &DEN_PRM_EVEN_YEAR_ROUNDS, branch: "mwc_3 den prm schedule" };

/// The Danish Superliga schedule: 33 rounds, from 27 Jul to 24 May; 30 with matches in TV slots; the spring changes
/// with the parity of the season year (from round 18: odd, as 2001/02, and even).
///
/// `DenPrm::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage (-1;
/// the others return 0 without writing); is_league = 1, rounds = 33, extra = 0 and the table of 33 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa6 and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `DenPrm::create_schedule` (0x552900, den_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn den_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let year = League(this).year();
    let p = if (year as i32) % 2 != 0 { &DEN_PRM_ODD_YEAR } else { &DEN_PRM_EVEN_YEAR };
    create_league_schedule(League(this), stage, rounds, extra, league, p)
}

/// Comparer for `den_prm_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
unsafe extern "thiscall" fn compare_den_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_DEN_PRM_CREATE_SCHEDULE);
    compare_cfa_schedule("den_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| den_prm_create_schedule(this, stage, r, e, l))
}

// ---------- batch 299: the Belgian Second Division: the constructor, the rules and the table (bel_second.cpp) ----------

pub(crate) const BEL_SECOND_DIVISION: NationDivision = NationDivision {
    vtable: 0x9679f4,
    bench: Some((4, 3)),
    discipline: discipline_rule::BELGIUM_LEAGUE, // mwc4: the Belgian Second Division discipline rule
    stages: 4,
    set_up_rules: 0x41fea0,
    create_table: 0x41fd70,
    source_file: 0x989b54,
    box_lines: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// The Belgian Second Division: discipline rule 4 (`BELGIUM_LEAGUE`), the bench (4 substitutes named, 3 substitutions),
/// 4 stages besides the league (the stage vector, malloc of 16); with no saved state, the rules, the table, the
/// fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `BelSecond::BelSecond` (0x41f030, bel_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bel_second(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &BEL_SECOND_DIVISION, || {})
}

/// Comparer for `construct_bel_second`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_bel_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BEL_SECOND);
    compare_nation_division_ctor("construct_bel_second", League(this), year, record, &BEL_SECOND_DIVISION, || o(this, year, record))
}

/// The rule writes of the Belgian Second Division (offset, size, value).
pub(crate) unsafe fn bel_second_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0xc5, 1, 3), (0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc6, 1, 1), (0xc7, 1, 2), (0xc8, 1, 0), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 1), (0xc1, 1, 2), (0x1c, 4, read_usize(KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION) as u32), (0xf8, 1, 0), (0xf7, 1, 0), (0xf6, 1, 0), (0x52, 1, 2), (0x20, 4, read_usize(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION) as u32), (0xd9, 2, 0x43)];
    v.push((0xc2, 1, 3)); // mwc4: the Belgian Second Division points for a win
    v
}

/// Sets up the rules of the Belgian Second Division: 2 rounds, 3 points for a win and 1 for a draw; 1 go up, 0 to the
/// promotion play-off, 1 to the relegation play-off and 2 go down; above: `KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION`;
/// below: `KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION`; tie-breaker codes 3, 1, 2, 0 (`+0xc5`..); zeroes the class fields
/// `+0xf6`, `+0xf7`, `+0xf8` [inference: the period winners]; rule bits `+0xd9` = 0x43; then builds the main-stage
/// round table through virtual +0x3c into `league::ROUND_DATES` (`fra_first_apply_league_rules`). Returns 1.
/// Original: `BelSecond::set_up_rules` (0x41fea0, bel_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_second_set_up_rules(this: usize) -> u32 {
    covered!("mwc_4 bel second rules");
    fra_first_apply_league_rules(League(this), &bel_second_rules(this))
}

/// Comparer for `bel_second_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_bel_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BEL_SECOND_SET_UP_RULES);
    compare_nation_league_rules("bel_second_set_up_rules", League(this), bel_second_rules(this), || o(this), || bel_second_set_up_rules(this))
}

/// The Belgian Second Division has 18 clubs; prize fund by the share of 10,000 per place; TV pays 40,000 to each club
/// (before the fund).
///
/// `BelSecond::create_table()`: `+0x3e` = 18, the table (`exe::crt_malloc` of 18 x 0x3b; out of memory, the error box
/// of line 0x1a4 and 0) with the clubs of `KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the TV money (League::pay_tv_rights, 0x9c40 to every club), the fund
/// `+0x28` (League::calc_prize_fund, share 0x2710), the prize mode `+0xe1` = 1 and 1.
/// Original: `BelSecond::create_table` (0x41fd70, bel_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_second_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x12);
    if fra_first_fill_league_table(this, 0x989b54, 0x1a4, KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION) != Some(true) {
        return 0;
    }
    covered!("mwc_4 full table");
    eng_prm_pay_tv_rights(this.addr(), 0x9c40, 0); // mwc4: the TV money of each club
    let fund = exe::league_calc_prize_fund(this, 0x2710);
    this.set_prize_fund(fund);
    this.set_prize_mode(1);
    1
}

/// Comparer for `bel_second_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
unsafe extern "thiscall" fn compare_bel_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BEL_SECOND_CREATE_TABLE);
    compare_nation_league_table("bel_second_create_table", League(this), 0x12, KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION, Some(0x2710), || o(this))
}

// ---------- batch 299: the Danish Superliga: the constructor, the rules and the table (den_prm.cpp) ----------

pub(crate) const DEN_PRM_DIVISION: NationDivision = NationDivision {
    vtable: 0x9696e4,
    bench: None,
    discipline: discipline_rule::DENMARK, // mwc5: the Danish Superliga discipline rule
    stages: 0,
    set_up_rules: 0x553a00,
    create_table: 0x5538d0,
    source_file: 0x9b8e6c,
    box_lines: [0x50, 0x57, 0x5e, 0x67, 0x6e],
};

/// The Danish Superliga: discipline rule 8 (`DENMARK`), only the league; before the load, the Kvalifikationsraekken
/// clubs move to the generic lower division (`KEY_CLUB_COMP_A_LOWER_DIVISION`); with no saved state, the rules, the
/// table, the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `DenPrm::DenPrm` (0x5525e0, den_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_den_prm(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &DEN_PRM_DIVISION, || {
        // mwc5: the Kvalifikationsraekken clubs go to the lower division before the load
        let kval = read_usize(KEY_CLUB_COMP_DANISH_KVALIFIKATIONSRAEKKEN);
        let mut i = 0i32;
        while i < read_usize(G_CLUB_COUNT) as i32 {
            let club = Club(read_usize(G_CLUB_TABLE) + i as usize * club::SIZE);
            let comp = read_usize(club.addr() + club::DIVISION);
            if comp != 0 && read_usize(comp) == kval {
                covered!("mwc_5 kvalifikationsraekken club moved");
                write_usize(club.addr() + club::DIVISION, comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION));
            }
            i += 1;
        }
    })
}

/// Comparer for `construct_den_prm`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_den_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_DEN_PRM);
    compare_nation_division_ctor("construct_den_prm", League(this), year, record, &DEN_PRM_DIVISION, || o(this, year, record))
}

/// The rule writes of the Danish Superliga (offset, size, value).
pub(crate) unsafe fn den_prm_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 2), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x4a, 1, 3), (0x20, 4, read_usize(KEY_CLUB_COMP_DANISH_FIRST_DIVISION) as u32), (0x49, 1, 4)];
    v.push((0xc2, 1, 3)); // mwc5: the Danish Superliga points for a win
    v
}

/// Sets up the rules of the Danish Superliga: 3 rounds, 3 points for a win and 1 for a draw; 0 go up, 0 to the
/// promotion play-off, 0 to the relegation play-off and 2 go down; above: none; below:
/// `KEY_CLUB_COMP_DANISH_FIRST_DIVISION`; tie-breaker codes 1, 2, 0 (`+0xc5`..); 4 substitutes named and 3
/// substitutions; then builds the main-stage round table through virtual +0x3c into `league::ROUND_DATES`
/// (`fra_first_apply_league_rules`). Returns 1.
/// Original: `DenPrm::set_up_rules` (0x553a00, den_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwc_5 den prm rules");
    fra_first_apply_league_rules(League(this), &den_prm_rules(this))
}

/// Comparer for `den_prm_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_den_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_PRM_SET_UP_RULES);
    compare_nation_league_rules("den_prm_set_up_rules", League(this), den_prm_rules(this), || o(this), || den_prm_set_up_rules(this))
}

/// The Danish Superliga has 12 clubs; prize fund by the share of 50,000 per place; TV pays 100,000 to each club.
///
/// `DenPrm::create_table()`: `+0x3e` = 12, the table (`exe::crt_malloc` of 12 x 0x3b; out of memory, the error box of
/// line 0x1ab and 0) with the clubs of `KEY_CLUB_COMP_DANISH_PREMIER_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the fund `+0x28` (League::calc_prize_fund, share 0xc350), the prize
/// mode `+0xe1` = 1, the TV money (League::pay_tv_rights, 0x186a0 to every club) and 1.
/// Original: `DenPrm::create_table` (0x5538d0, den_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_prm_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0xc, 0x9b8e6c, 0x1ab, KEY_CLUB_COMP_DANISH_PREMIER_DIVISION, 0xc350, || {
        eng_prm_pay_tv_rights(this, 0x186a0, 0); // mwc5: the TV money of each club
    })
}

/// Comparer for `den_prm_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
unsafe extern "thiscall" fn compare_den_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_PRM_CREATE_TABLE);
    compare_nation_league_table("den_prm_create_table", League(this), 0xc, KEY_CLUB_COMP_DANISH_PREMIER_DIVISION, Some(0xc350), || o(this))
}

// ---------- batch 299: the Danish First Division: the constructor, the rules and the table (den_first.cpp) ----------

pub(crate) const DEN_FIRST_DIVISION: NationDivision = NationDivision {
    vtable: 0x969630,
    bench: None,
    discipline: discipline_rule::DENMARK, // mwc6: the Danish First Division discipline rule
    stages: 0,
    set_up_rules: 0x5523b0,
    create_table: 0x552290,
    source_file: 0x9b8e30,
    box_lines: [0x42, 0x49, 0x50, 0x59, 0x60],
};

/// The Danish First Division: discipline rule 8 (`DENMARK`), only the league; with no saved state, the rules, the
/// table, the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `DenFirst::DenFirst` (0x5516f0, den_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_den_first(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &DEN_FIRST_DIVISION, || {})
}

/// Comparer for `construct_den_first`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_den_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_DEN_FIRST);
    compare_nation_division_ctor("construct_den_first", League(this), year, record, &DEN_FIRST_DIVISION, || o(this, year, record))
}

/// The rule writes of the Danish First Division (offset, size, value).
pub(crate) unsafe fn den_first_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 0), (0xbe, 1, 2), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_DANISH_PREMIER_DIVISION) as u32), (0x20, 4, read_usize(KEY_CLUB_COMP_DANISH_SECOND_DIVISION) as u32), (0x4a, 1, 3), (0x49, 1, 4)];
    v.push((0xc2, 1, 3)); // mwc6: the Danish First Division points for a win
    v
}

/// Sets up the rules of the Danish First Division: 2 rounds, 3 points for a win and 1 for a draw; 2 go up, 0 to the
/// promotion play-off, 0 to the relegation play-off and 3 go down; above: `KEY_CLUB_COMP_DANISH_PREMIER_DIVISION`;
/// below: `KEY_CLUB_COMP_DANISH_SECOND_DIVISION`; tie-breaker codes 1, 2, 0 (`+0xc5`..); 4 substitutes named and 3
/// substitutions; then builds the main-stage round table through virtual +0x3c into `league::ROUND_DATES`
/// (`fra_first_apply_league_rules`). Returns 1.
/// Original: `DenFirst::set_up_rules` (0x5523b0, den_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_first_set_up_rules(this: usize) -> u32 {
    covered!("mwc_6 den first rules");
    fra_first_apply_league_rules(League(this), &den_first_rules(this))
}

/// Comparer for `den_first_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_den_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_FIRST_SET_UP_RULES);
    compare_nation_league_rules("den_first_set_up_rules", League(this), den_first_rules(this), || o(this), || den_first_set_up_rules(this))
}

/// The Danish First Division has 16 clubs; prize fund by the share of 10,000 per place; no TV money.
///
/// `DenFirst::create_table()`: `+0x3e` = 16, the table (`exe::crt_malloc` of 16 x 0x3b; out of memory, the error box of
/// line 0x142 and 0) with the clubs of `KEY_CLUB_COMP_DANISH_FIRST_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the fund `+0x28` (League::calc_prize_fund, share 0x2710), the prize
/// mode `+0xe1` = 1 and 1.
/// Original: `DenFirst::create_table` (0x552290, den_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_first_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0x9b8e30, 0x142, KEY_CLUB_COMP_DANISH_FIRST_DIVISION, 0x2710, || {
        // no TV money
    })
}

/// Comparer for `den_first_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
unsafe extern "thiscall" fn compare_den_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_FIRST_CREATE_TABLE);
    compare_nation_league_table("den_first_create_table", League(this), 0x10, KEY_CLUB_COMP_DANISH_FIRST_DIVISION, Some(0x2710), || o(this))
}

// ---------- batch 299: the Danish Second Division: the constructor, the rules and the table (den_second.cpp) ----------

pub(crate) const DEN_SECOND_DIVISION: NationDivision = NationDivision {
    vtable: 0x969798,
    bench: None,
    discipline: discipline_rule::DENMARK, // mwc7: the Danish Second Division discipline rule
    stages: 0,
    set_up_rules: 0x554b60,
    create_table: 0x554a40,
    source_file: 0x9b8ea4,
    box_lines: [0x43, 0x4a, 0x51, 0x5a, 0x61],
};

/// The Danish Second Division: discipline rule 8 (`DENMARK`), only the league; with no saved state, the rules, the
/// table, the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `DenSecond::DenSecond` (0x553e80, den_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_den_second(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &DEN_SECOND_DIVISION, || {})
}

/// Comparer for `construct_den_second`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_den_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_DEN_SECOND);
    compare_nation_division_ctor("construct_den_second", League(this), year, record, &DEN_SECOND_DIVISION, || o(this, year, record))
}

/// The rule writes of the Danish Second Division (offset, size, value).
pub(crate) unsafe fn den_second_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc4, 1, 2), (0xc6, 1, 2), (0xc3, 1, 1), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc7, 1, 0), (0xbe, 1, 3), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 4), (0x4a, 1, 3), (0x49, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_DANISH_FIRST_DIVISION) as u32), (0x20, 4, 0xffff_ffff)];
    v.push((0xc2, 1, 3)); // mwc7: the Danish Second Division points for a win
    v
}

/// Sets up the rules of the Danish Second Division: 2 rounds, 3 points for a win and 1 for a draw; 3 go up, 0 to the
/// promotion play-off, 0 to the relegation play-off and 4 go down; above: `KEY_CLUB_COMP_DANISH_FIRST_DIVISION`; below:
/// none; tie-breaker codes 1, 2, 0 (`+0xc5`..); 4 substitutes named and 3 substitutions; then builds the main-stage
/// round table through virtual +0x3c into `league::ROUND_DATES` (`fra_first_apply_league_rules`). Returns 1.
/// Original: `DenSecond::set_up_rules` (0x554b60, den_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_second_set_up_rules(this: usize) -> u32 {
    covered!("mwc_7 den second rules");
    fra_first_apply_league_rules(League(this), &den_second_rules(this))
}

/// Comparer for `den_second_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_den_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_SECOND_SET_UP_RULES);
    compare_nation_league_rules("den_second_set_up_rules", League(this), den_second_rules(this), || o(this), || den_second_set_up_rules(this))
}

/// The Danish Second Division has 16 clubs; prize fund by the share of 5,000 per place; no TV money.
///
/// `DenSecond::create_table()`: `+0x3e` = 16, the table (`exe::crt_malloc` of 16 x 0x3b; out of memory, the error box
/// of line 0x149 and 0) with the clubs of `KEY_CLUB_COMP_DANISH_SECOND_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the fund `+0x28` (League::calc_prize_fund, share 0x1388), the prize
/// mode `+0xe1` = 1 and 1.
/// Original: `DenSecond::create_table` (0x554a40, den_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_second_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0x9b8ea4, 0x149, KEY_CLUB_COMP_DANISH_SECOND_DIVISION, 0x1388, || {
        // no TV money
    })
}

/// Comparer for `den_second_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
unsafe extern "thiscall" fn compare_den_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_SECOND_CREATE_TABLE);
    compare_nation_league_table("den_second_create_table", League(this), 0x10, KEY_CLUB_COMP_DANISH_SECOND_DIVISION, Some(0x1388), || o(this))
}

// ---------- batch 299: the 2. Bundesliga: the constructor, the rules and the table (ger_second.cpp) ----------

pub(crate) const GER_SECOND_DIVISION: NationDivision = NationDivision {
    vtable: 0x96b304,
    bench: None,
    discipline: discipline_rule::GERMANY_LEAGUE, // mwc8: the 2. Bundesliga discipline rule
    stages: 0,
    set_up_rules: 0x5df3e0,
    create_table: 0x5df2b0,
    source_file: 0x9cd4c4,
    box_lines: [0x43, 0x4a, 0x51, 0x5a, 0x61],
};

/// The 2. Bundesliga: discipline rule 13 (`GERMANY_LEAGUE`), only the league; with no saved state, the rules, the
/// table, the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `GerSecond::GerSecond` (0x5de2e0, ger_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ger_second(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &GER_SECOND_DIVISION, || {})
}

/// Comparer for `construct_ger_second`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_ger_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_GER_SECOND);
    compare_nation_division_ctor("construct_ger_second", League(this), year, record, &GER_SECOND_DIVISION, || o(this, year, record))
}

/// The rule writes of the 2. Bundesliga (offset, size, value).
pub(crate) unsafe fn ger_second_rules(this: usize) -> Vec<(usize, u8, u32)> {
    // the Regionalliga below only when the nation has its lower division playable (bit 4 of `nation::LEAGUE_SELECTED`)
    let nation = Nation(read_usize(read_usize(this + league::COMPETITION) + competition::NATION));
    let below = if nation.league_selected() & 4 != 0 { read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL) as u32 } else { 0xffff_ffff };
    let mut v = vec![(0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3), (0xbe, 1, 3), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION) as u32), (0x20, 4, below), (0x52, 1, 2), (0x4a, 1, 3), (0x49, 1, 6)];
    v.push((0xc2, 1, 3)); // mwc8: the 2. Bundesliga points for a win
    v
}

/// Sets up the rules of the 2. Bundesliga: 2 rounds, 3 points for a win and 1 for a draw; 3 go up, 0 to the promotion
/// play-off, 0 to the relegation play-off and 4 go down; above: `KEY_CLUB_COMP_GERMAN_FIRST_DIVISION`; below:
/// `KEY_CLUB_COMP_GERMAN_REGIONAL` when the German lower division is playable (bit 4 of the nation's
/// `LEAGUE_SELECTED`), else none; tie-breaker codes 1, 2, 3 (`+0xc5`..); 6 substitutes named and 3 substitutions; then
/// builds the main-stage round table through virtual +0x3c into `league::ROUND_DATES` (`fra_first_apply_league_rules`).
/// Returns 1.
/// Original: `GerSecond::set_up_rules` (0x5df3e0, ger_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_second_set_up_rules(this: usize) -> u32 {
    covered!("mwc_8 ger second rules");
    fra_first_apply_league_rules(League(this), &ger_second_rules(this))
}

/// Comparer for `ger_second_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_ger_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GER_SECOND_SET_UP_RULES);
    compare_nation_league_rules("ger_second_set_up_rules", League(this), ger_second_rules(this), || o(this), || ger_second_set_up_rules(this))
}

/// The 2. Bundesliga has 18 clubs; prize fund by the share of 50,000 per place; TV pays 1,300,000 to each club.
///
/// `GerSecond::create_table()`: `+0x3e` = 18, the table (`exe::crt_malloc` of 18 x 0x3b; out of memory, the error box
/// of line 0x19f and 0) with the clubs of `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the fund `+0x28` (League::calc_prize_fund, share 0xc350), the prize
/// mode `+0xe1` = 1, the TV money (League::pay_tv_rights, 0x13d620 to every club) and 1.
/// Original: `GerSecond::create_table` (0x5df2b0, ger_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_second_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x12, 0x9cd4c4, 0x19f, KEY_CLUB_COMP_GERMAN_SECOND_DIVISION, 0xc350, || {
        eng_prm_pay_tv_rights(this, 0x13d620, 0); // mwc8: the TV money of each club
    })
}

/// Comparer for `ger_second_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
unsafe extern "thiscall" fn compare_ger_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GER_SECOND_CREATE_TABLE);
    compare_nation_league_table("ger_second_create_table", League(this), 0x12, KEY_CLUB_COMP_GERMAN_SECOND_DIVISION, Some(0xc350), || o(this))
}

// ---------- batch 299: the Eredivisie: the constructor, the rules and the table (hol_prm.cpp) ----------

pub(crate) const HOL_PRM_DIVISION: NationDivision = NationDivision {
    vtable: 0x96b920,
    bench: None,
    discipline: discipline_rule::HOLLAND, // mwc9: the Dutch Eredivisie discipline rule
    stages: 0,
    set_up_rules: 0x5f8560,
    create_table: 0x5f8430,
    source_file: 0x9cefa4,
    box_lines: [0x43, 0x4a, 0x51, 0x5a, 0x61],
};

/// The Dutch Eredivisie: discipline rule 15 (`HOLLAND`), only the league; with no saved state, the rules, the table,
/// the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `HolPrm::HolPrm` (0x5f7610, hol_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_hol_prm(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &HOL_PRM_DIVISION, || {})
}

/// Comparer for `construct_hol_prm`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_hol_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_HOL_PRM);
    compare_nation_division_ctor("construct_hol_prm", League(this), year, record, &HOL_PRM_DIVISION, || o(this, year, record))
}

/// The rule writes of the Dutch Eredivisie (offset, size, value).
pub(crate) unsafe fn hol_prm_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 0), (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 2), (0xc1, 1, 1), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x4a, 1, 3), (0x20, 4, read_usize(KEY_CLUB_COMP_DUTCH_TOTO_DIVISION) as u32), (0x49, 1, 7)];
    v.push((0xc2, 1, 3)); // mwc9: the Dutch Eredivisie points for a win
    v
}

/// Sets up the rules of the Dutch Eredivisie: 2 rounds, 3 points for a win and 1 for a draw; 0 go up, 0 to the
/// promotion play-off, 2 to the relegation play-off and 1 go down; above: none; below:
/// `KEY_CLUB_COMP_DUTCH_TOTO_DIVISION`; tie-breaker codes 1, 2, 0, 0 (`+0xc5`..); 7 substitutes named and 3
/// substitutions; then builds the main-stage round table through virtual +0x3c into `league::ROUND_DATES`
/// (`fra_first_apply_league_rules`). Returns 1.
/// Original: `HolPrm::set_up_rules` (0x5f8560, hol_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwc_9 hol prm rules");
    fra_first_apply_league_rules(League(this), &hol_prm_rules(this))
}

/// Comparer for `hol_prm_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_hol_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HOL_PRM_SET_UP_RULES);
    compare_nation_league_rules("hol_prm_set_up_rules", League(this), hol_prm_rules(this), || o(this), || hol_prm_set_up_rules(this))
}

/// The Dutch Eredivisie has 18 clubs; prize fund by the share of 50,000 per place; TV pays 1,000,000 to each club.
///
/// `HolPrm::create_table()`: `+0x3e` = 18, the table (`exe::crt_malloc` of 18 x 0x3b; out of memory, the error box of
/// line 0x182 and 0) with the clubs of `KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION` in database order
/// (League::init_table_row); not full: 0. Full: the fund `+0x28` (League::calc_prize_fund, share 0xc350), the prize
/// mode `+0xe1` = 1, the TV money (League::pay_tv_rights, 0xf4240 to every club) and 1.
/// Original: `HolPrm::create_table` (0x5f8430, hol_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_prm_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x12, 0x9cefa4, 0x182, KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION, 0xc350, || {
        eng_prm_pay_tv_rights(this, 0xf4240, 0); // mwc9: the TV money of each club
    })
}

/// Comparer for `hol_prm_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
unsafe extern "thiscall" fn compare_hol_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HOL_PRM_CREATE_TABLE);
    compare_nation_league_table("hol_prm_create_table", League(this), 0x12, KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION, Some(0xc350), || o(this))
}

// ---------- batch 299: the Eerste Divisie: the constructor, the rules and the table (hol_first.cpp) ----------

pub(crate) const HOL_FIRST_DIVISION: NationDivision = NationDivision {
    vtable: 0x96b86c,
    bench: None,
    discipline: discipline_rule::HOLLAND, // mwc10: the Dutch Eerste Divisie discipline rule
    stages: 6,
    set_up_rules: 0x5f6340,
    create_table: 0x5f6240,
    source_file: 0x9cee08,
    box_lines: [0x49, 0x50, 0x57, 0x60, 0x67],
};

/// The Dutch Eerste Divisie: discipline rule 15 (`HOLLAND`), 6 stages besides the league (the stage vector, malloc of
/// 24); with no saved state, the rules, the table, the fixtures, the order, the statistics and the predicted positions
/// (`construct_nation_division`).
/// Original: `HolFirst::HolFirst` (0x5f5490, hol_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_hol_first(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &HOL_FIRST_DIVISION, || {})
}

/// Comparer for `construct_hol_first`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_hol_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_HOL_FIRST);
    compare_nation_division_ctor("construct_hol_first", League(this), year, record, &HOL_FIRST_DIVISION, || o(this, year, record))
}

/// The rule writes of the Dutch Eerste Divisie (offset, size, value).
pub(crate) unsafe fn hol_first_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc4, 1, 2), (0xc6, 1, 2), (0xc3, 1, 1), (0x42, 1, 1), (0xc5, 1, 1), (0xc7, 1, 0), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0xf1, 1, 0), (0xf0, 1, 0), (0xef, 1, 0), (0xee, 1, 0), (0x4a, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION) as u32), (0x20, 4, 0xffff_ffff), (0x49, 1, 7)];
    v.push((0xc2, 1, 3)); // mwc10: the Dutch Eerste Divisie points for a win
    v
}

/// Sets up the rules of the Dutch Eerste Divisie: 2 rounds, 3 points for a win and 1 for a draw; 1 go up, 0 to the
/// promotion play-off, 0 to the relegation play-off and 0 go down; above: `KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION`;
/// below: none; tie-breaker codes 1, 2, 0 (`+0xc5`..); 7 substitutes named and 3 substitutions; zeroes the class fields
/// `+0xee`, `+0xef`, `+0xf0`, `+0xf1` [inference: the period winners]; then builds the main-stage round table through
/// virtual +0x3c into `league::ROUND_DATES` (`fra_first_apply_league_rules`). Returns 1.
/// Original: `HolFirst::set_up_rules` (0x5f6340, hol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_first_set_up_rules(this: usize) -> u32 {
    covered!("mwc_10 hol first rules");
    fra_first_apply_league_rules(League(this), &hol_first_rules(this))
}

/// Comparer for `hol_first_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_hol_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HOL_FIRST_SET_UP_RULES);
    compare_nation_league_rules("hol_first_set_up_rules", League(this), hol_first_rules(this), || o(this), || hol_first_set_up_rules(this))
}

/// The Dutch Eerste Divisie has 18 clubs, with no prize fund and no TV money.
///
/// `HolFirst::create_table()`: `+0x3e` = 18, the table (`exe::crt_malloc` of 18 x 0x3b; out of memory, the error box of
/// line 0x19d and 0) with the clubs of `KEY_CLUB_COMP_DUTCH_TOTO_DIVISION` in database order (League::init_table_row);
/// not full: 0. Full: 1 (nothing else is written).
/// Original: `HolFirst::create_table` (0x5f6240, hol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_first_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x12); // mwc10: 18 clubs
    (fra_first_fill_league_table(this, 0x9cee08, 0x19d, KEY_CLUB_COMP_DUTCH_TOTO_DIVISION) == Some(true)) as u32
}

/// Comparer for `hol_first_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
unsafe extern "thiscall" fn compare_hol_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HOL_FIRST_CREATE_TABLE);
    compare_nation_league_table("hol_first_create_table", League(this), 0x12, KEY_CLUB_COMP_DUTCH_TOTO_DIVISION, None, || o(this))
}

// ---------- batch 299: the Bundesliga: the constructor, the rules and the schedule (ger_first.cpp) ----------

pub(crate) const GER_FIRST_DIVISION: NationDivision = NationDivision {
    vtable: 0x96b0e4,
    bench: None,
    discipline: discipline_rule::GERMANY_LEAGUE, // mwc11: the Bundesliga discipline rule
    stages: 0,
    set_up_rules: 0x5db400,
    create_table: 0x5db230,
    source_file: 0x9cd414,
    box_lines: [0x47, 0x4e, 0x55, 0x5e, 0x65],
};

/// The Bundesliga: discipline rule 13 (`GERMANY_LEAGUE`), only the league; with no saved state, the rules, the table,
/// the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `GerFirst::GerFirst` (0x5d99b0, ger_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ger_first(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &GER_FIRST_DIVISION, || {})
}

/// Comparer for `construct_ger_first`: only the original runs (`compare_nation_division_ctor`).
unsafe extern "thiscall" fn compare_construct_ger_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_GER_FIRST);
    compare_nation_division_ctor("construct_ger_first", League(this), year, record, &GER_FIRST_DIVISION, || o(this, year, record))
}

/// The rule writes of the Bundesliga (offset, size, value).
pub(crate) unsafe fn ger_first_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 3), (0x1c, 4, 0xffff_ffff), (0x4a, 1, 3), (0x52, 1, 2), (0x20, 4, read_usize(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION) as u32), (0x49, 1, 6)];
    v.push((0xc2, 1, 3)); // mwc11: the Bundesliga points for a win
    v
}

/// Sets up the rules of the Bundesliga: 2 rounds, 3 points for a win and 1 for a draw; 0 go up, 0 to the promotion
/// play-off, 0 to the relegation play-off and 3 go down; above: none; below: `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION`;
/// tie-breaker codes 1, 2, 3 (`+0xc5`..); 6 substitutes named and 3 substitutions; then builds the main-stage round
/// table through virtual +0x3c into `league::ROUND_DATES` (`fra_first_apply_league_rules`). Returns 1.
/// Original: `GerFirst::set_up_rules` (0x5db400, ger_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_first_set_up_rules(this: usize) -> u32 {
    covered!("mwc_11 ger first rules");
    fra_first_apply_league_rules(League(this), &ger_first_rules(this))
}

/// Comparer for `ger_first_set_up_rules`: a dry run (`compare_nation_league_rules`).
unsafe extern "thiscall" fn compare_ger_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GER_FIRST_SET_UP_RULES);
    compare_nation_league_rules("ger_first_set_up_rules", League(this), ger_first_rules(this), || o(this), || ger_first_set_up_rules(this))
}

/// The Bundesliga 2001/02: 34 rounds (the first date with the literal year 2001).
pub(crate) const GER_FIRST_2001_ROUNDS: [FraThirdLeagueRound; 34] = [
    (29, 6, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 0: 29 Jul
    (5, 7, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 1: 5 Aug
    (12, 7, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 2: 12 Aug
    (19, 7, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 3: 19 Aug
    (6, 8, 0, 2, 2, &[(-1, -1, -1)]), // 4: 6 Sep
    (9, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 5: 9 Sep (mwc11 ger_first)
    (16, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 6: 16 Sep
    (23, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 7: 23 Sep
    (30, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 8: 30 Sep
    (14, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 9: 14 Oct
    (21, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 10: 21 Oct
    (28, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 11: 28 Oct
    (4, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 12: 4 Nov
    (11, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 13: 11 Nov
    (18, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 14: 18 Nov
    (25, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 15: 25 Nov
    (2, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 16: 2 Dec
    (9, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 17: 9 Dec
    (13, 11, 0, 2, 2, &[(-1, -1, -1)]), // 18: 13 Dec
    (16, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 19: 16 Dec
    (27, 0, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 20: 27 Jan (+1 year)
    (3, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 21: 3 Feb (+1 year)
    (10, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 22: 10 Feb (+1 year)
    (17, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 23: 17 Feb (+1 year)
    (24, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 24: 24 Feb (+1 year)
    (3, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 25: 3 Mar (+1 year)
    (10, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 26: 10 Mar (+1 year)
    (17, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 27: 17 Mar (+1 year)
    (31, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 28: 31 Mar (+1 year)
    (7, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 29: 7 Apr (+1 year)
    (14, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 30: 14 Apr (+1 year)
    (21, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 31: 21 Apr (+1 year)
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 32: 28 Apr (+1 year)
    (5, 4, 1, 5, 1, &[(-1, -1, -1)]), // 33: 5 May (+1 year)
];
pub(crate) const GER_FIRST_2001: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 34, file: 0x9cd414, row: 0xa1, table: &GER_FIRST_2001_ROUNDS, branch: "mwc_11 ger first schedule" };

/// The Bundesliga from 2002/03: 34 rounds.
pub(crate) const GER_FIRST_LATER_ROUNDS: [FraThirdLeagueRound; 34] = [
    (12, 7, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 0: 12 Aug
    (19, 7, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 1: 19 Aug
    (6, 8, 0, 2, 2, &[(-1, -1, -1)]), // 2: 6 Sep
    (9, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 3: 9 Sep
    (16, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 4: 16 Sep
    (23, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 5: 23 Sep
    (30, 8, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 6: 30 Sep
    (14, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 7: 14 Oct
    (21, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 8: 21 Oct
    (28, 9, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 9: 28 Oct
    (4, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 10: 4 Nov
    (11, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 11: 11 Nov
    (18, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 12: 18 Nov
    (25, 10, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 13: 25 Nov
    (2, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 14: 2 Dec
    (9, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 15: 9 Dec
    (13, 11, 0, 2, 2, &[(-1, -1, -1)]), // 16: 13 Dec
    (16, 11, 0, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 17: 16 Dec
    (27, 0, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 18: 27 Jan (+1 year)
    (3, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 19: 3 Feb (+1 year)
    (10, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 20: 10 Feb (+1 year)
    (17, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 21: 17 Feb (+1 year)
    (24, 1, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 22: 24 Feb (+1 year)
    (3, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 23: 3 Mar (+1 year)
    (10, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 24: 10 Mar (+1 year)
    (17, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 25: 17 Mar (+1 year)
    (31, 2, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 26: 31 Mar (+1 year)
    (7, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 27: 7 Apr (+1 year)
    (14, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 28: 14 Apr (+1 year)
    (21, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 29: 21 Apr (+1 year)
    (28, 3, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 30: 28 Apr (+1 year)
    (5, 4, 1, 5, 1, &[(6, 1, 1), (6, 1, 1), (-1, -1, -1)]), // 31: 5 May (+1 year)
    (12, 4, 1, 5, 1, &[(-1, -1, -1)]), // 32: 12 May (+1 year)
    (19, 4, 1, 5, 1, &[(-1, -1, -1)]), // 33: 19 May (+1 year)
];
pub(crate) const GER_FIRST_LATER: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 34, file: 0x9cd414, row: 0xa1, table: &GER_FIRST_LATER_ROUNDS, branch: "mwc_11 ger first schedule" };

/// The Bundesliga schedule: 34 rounds, from 29 Jul to 5 May; 30 with matches in TV slots; in 2001/02 its own table, in
/// the following seasons another.
///
/// `GerFirst::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage (-1;
/// the others return 0 without writing); is_league = 1, rounds = 34, extra = 0 and the table of 34 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa1 and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `GerFirst::create_schedule` (0x5d9c70, ger_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ger_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let year = League(this).year();
    let p = if year as u16 == 0x7d1 { &GER_FIRST_2001 } else { &GER_FIRST_LATER };
    create_league_schedule(League(this), stage, rounds, extra, league, p)
}

/// Comparer for `ger_first_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
unsafe extern "thiscall" fn compare_ger_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GER_FIRST_CREATE_SCHEDULE);
    compare_cfa_schedule("ger_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| ger_first_create_schedule(this, stage, r, e, l))
}

// ---------- batch 299: the round tables of the Belgian and Danish cups and the Belgian Super Cup (bel_fa_cup.cpp, den_cup.cpp, bel_super.cpp) ----------

/// The Belgian Cup: 8 rounds.
pub(crate) const BEL_FA_CUP_ROUNDS: [FraCupCupRound; 8] = [
    cql11r((6, 7, 0, 6), (0xc, 7, 0, 5, 2, 0), 0x1e, (0, 0x401, 0), 4, (0x4c, 0x26, 0x4c, 0), (0, 1, 0), (0, 0, 0)),
    cql11r((0xd, 7, 0, 6), (0x13, 7, 0, 5, 2, 0), 0x28, (0, 0x403, 0), 4, (0x38, 0x1c, 0x12, 0x4c), (0, 1, 0), (0, 0, 0)),
    cql11r((0x14, 7, 0, 6), (0x1a, 7, 0, 5, 2, 0), 0x32, (0, 0x403, 0), 4, (0x1c, 0xe, 0, 0), (0, 1, 0), (0, 0, 0)),
    cql11r((0xf, 8, 0, 6), (5, 0xa, 0, 6, 2, 0), 0x3c, (0, 3, 0), 4, (0x20, 0x10, 0x12, 0x5e), (0, 1, 0), (0, 0xffffffff, 0)), // mwc12 bel_fa_cup round 3
    cql11r((6, 0xa, 0, 0), (0x1d, 0xa, 0, 2, 2, 0), 0x46, (0, 3, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1e, 0xa, 0, 3), (0x18, 0, 1, 2, 2, 0), 0x78, (0, 2, 3), 6, (8, 4, 0, 0), (1, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x19, 0, 1, 3), (0x1c, 2, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffffffff, 0)),
    cql11r((0xc, 3, 1, 3), (0x1b, 4, 1, 6, 2, 4), 0x96, (0, 2, 3), 6, (2, 1, 0, 0), (1, 1, 6), (0, 0xffffffff, 0)),
];

/// The Belgian Cup has 8 round(s): code 30: draw 6 Aug, match 12 Aug, 76 teams (76 new); code 40: draw 13 Aug, match 19
/// Aug, 56 teams (18 new); code 50: draw 20 Aug, match 26 Aug, 28 teams (0 new); code 60: draw 15 Sep, match 5 Nov, 32
/// teams (18 new); code 70: draw 6 Nov, match 29 Nov, 16 teams (0 new); code 120: draw 30 Nov, match 24 Jan (+1 year),
/// 8 teams (0 new); code 130: draw 25 Jan (+1 year), match 28 Mar (+1 year), 4 teams (0 new), two legs; code 150: draw
/// 12 Apr (+1 year), match 27 May (+1 year), 2 teams (0 new).
///
/// `BelFaCup::create_round_table(stage, &rounds, &extra, &out)` (`single_stage_round_table`): only stage -1 (the others
/// return 0 without writing); out = 0, rounds = the table size, extra = 0 and the table of 0x68-byte rounds
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa9 and 0), each with its draw and match dates
/// (`cup_set_round_draw_date`, `cup_set_round_match_date`, cdecl) and its fields (`fra_cup_write_cup_round`).
/// Original: `BelFaCup::create_round_table` (0x41d310, bel_fa_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bel_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(Cup(this), stage, rounds, extra, out, &BEL_FA_CUP_ROUNDS, 0x989ae0, 0xa9, "mwc_12 bel fa cup round table")
}

/// Comparer for `bel_fa_cup_create_round_table`: side by side (`compare_cup_round_table`).
unsafe extern "thiscall" fn compare_bel_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BEL_FA_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("bel_fa_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| bel_fa_cup_create_round_table(this, stage, r, e, s))
}

/// The Danish Cup: 8 rounds.
pub(crate) const DEN_CUP_ROUNDS: [FraCupCupRound; 8] = [
    cql11r((1, 7, 0, 4), (0xc, 7, 0, 2, 2, 0), 0xa, (1, 3, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0xe, 7, 0, 4), (0x13, 7, 0, 2, 2, 0), 0x14, (1, 3, 0), 4, (0x28, 0x14, 8, 0x40), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x15, 7, 0, 4), (0x1a, 7, 0, 2, 2, 0), 0x1e, (1, 3, 0), 4, (0x1c, 0xe, 8, 0x48), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1c, 7, 0, 4), (5, 8, 0, 2, 2, 0), 0x28, (1, 3, 0), 4, (0x14, 0xa, 6, 0x50), (0, 1, 0), (0, 0xffffffff, 0)), // mwc12 den_cup round 3
    cql11r((7, 8, 0, 4), (0xc, 8, 0, 2, 2, 0), 0x32, (1, 3, 0), 4, (0x10, 8, 6, 0x56), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0xe, 8, 0, 4), (0x13, 8, 0, 2, 2, 0), 0x78, (1, 3, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x15, 8, 0, 4), (0x14, 2, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 7), (0, 0xffffffff, 0)),
    cql11r((0x1d, 2, 1, 4), (1, 5, 1, 3, 2, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];

/// The Danish Cup has 8 round(s): code 10: draw 1 Aug, match 12 Aug, 64 teams (64 new); code 20: draw 14 Aug, match 19
/// Aug, 40 teams (8 new); code 30: draw 21 Aug, match 26 Aug, 28 teams (8 new); code 40: draw 28 Aug, match 5 Sep, 20
/// teams (6 new); code 50: draw 7 Sep, match 12 Sep, 16 teams (6 new); code 120: draw 14 Sep, match 19 Sep, 8 teams (0
/// new); code 130: draw 21 Sep, match 20 Mar (+1 year), 4 teams (0 new), two legs; code 150: draw 29 Mar (+1 year),
/// match 1 Jun (+1 year), 2 teams (0 new).
///
/// `DenCup::create_round_table(stage, &rounds, &extra, &out)` (`single_stage_round_table`): only stage -1 (the others
/// return 0 without writing); out = 0, rounds = the table size, extra = 0 and the table of 0x68-byte rounds
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa5 and 0), each with its draw and match dates
/// (`cup_set_round_draw_date`, `cup_set_round_match_date`, cdecl) and its fields (`fra_cup_write_cup_round`).
/// Original: `DenCup::create_round_table` (0x550c70, den_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn den_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(Cup(this), stage, rounds, extra, out, &DEN_CUP_ROUNDS, 0x9b8dfc, 0xa5, "mwc_12 den cup round table")
}

/// Comparer for `den_cup_create_round_table`: side by side (`compare_cup_round_table`).
unsafe extern "thiscall" fn compare_den_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_DEN_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("den_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| den_cup_create_round_table(this, stage, r, e, s))
}

/// The Belgian Super Cup: one match.
pub(crate) const BEL_SUPER_ROUNDS: [FraCupCupRound; 1] = [
    cql11r((5, 6, 0, 5), (5, 7, 0, 5, 2, 0), 0, (8, 0x201, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwc12 bel_super round 0
];

/// The Belgian Super Cup has 1 round(s): code 0: draw 5 Jul, match 5 Aug, 2 teams (2 new).
///
/// `BelSuper::create_round_table(stage, &rounds, &extra, &out)` (`single_stage_round_table`): only stage -1 (the others
/// return 0 without writing); out = 0, rounds = the table size, extra = 0 and the table of 0x68-byte rounds
/// (`exe::crt_malloc`; out of memory, the error box of line 0x9d and 0), each with its draw and match dates
/// (`cup_set_round_draw_date`, `cup_set_round_match_date`, cdecl) and its fields (`fra_cup_write_cup_round`).
/// Original: `BelSuper::create_round_table` (0x4211b0, bel_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bel_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(Cup(this), stage, rounds, extra, out, &BEL_SUPER_ROUNDS, 0x989ccc, 0x9d, "mwc_12 bel super round table")
}

/// Comparer for `bel_super_create_round_table`: side by side (`compare_cup_round_table`).
unsafe extern "thiscall" fn compare_bel_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BEL_SUPER_CREATE_ROUND_TABLE);
    compare_cup_round_table("bel_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| bel_super_create_round_table(this, stage, r, e, s))
}

// ---------- batch 299: the round tables of the Dutch Super Cup and the German Cup (hol_super.cpp, ger_cup.cpp) ----------

/// The Dutch Super Cup: one match.
pub(crate) const HOL_SUPER_ROUNDS: [FraCupCupRound; 1] = [
    cql11r((5, 6, 0, 5), (8, 7, 0, 6, 1, 4), 0, (0, 0x83, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwc13 hol_super round 0
];

/// The Dutch Super Cup has 1 round(s): code 0: draw 5 Jul, match 8 Aug, 2 teams (2 new).
///
/// `HolSuper::create_round_table(stage, &rounds, &extra, &out)` (`single_stage_round_table`): only stage -1 (the others
/// return 0 without writing); out = 0, rounds = the table size, extra = 0 and the table of 0x68-byte rounds
/// (`exe::crt_malloc`; out of memory, the error box of line 0x95 and 0), each with its draw and match dates
/// (`cup_set_round_draw_date`, `cup_set_round_match_date`, cdecl) and its fields (`fra_cup_write_cup_round`).
/// Original: `HolSuper::create_round_table` (0x5f8ad0, hol_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn hol_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(Cup(this), stage, rounds, extra, out, &HOL_SUPER_ROUNDS, 0x9cefdc, 0x95, "mwc_13 hol super round table")
}

/// Comparer for `hol_super_create_round_table`: side by side (`compare_cup_round_table`).
unsafe extern "thiscall" fn compare_hol_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_HOL_SUPER_CREATE_ROUND_TABLE);
    compare_cup_round_table("hol_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| hol_super_create_round_table(this, stage, r, e, s))
}

/// The German Cup 2001/02 (the final on 11 May 2002).
pub(crate) const GER_CUP_2001_ROUNDS: [FraCupCupRound; 6] = [
    cql11r((0xa, 6, 0, 5), (0x1a, 7, 0, 5, 1, 0), 0xa, (1, 0x403, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1b, 7, 0, 6), (0xb, 9, 0, 2, 2, 0), 0x14, (1, 0x403, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0xc, 0xa, 0, 6), (0x1d, 0xa, 0, 2, 2, 0), 0x1e, (1, 0x403, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1e, 0xa, 0, 3), (0x14, 0xb, 0, 2, 2, 0), 0x78, (1, 3, 0), 4, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwc13 ger_cup round 3
    cql11r((0x15, 0xb, 0, 3), (7, 1, 1, 2, 2, 0), 0x82, (1, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((8, 1, 1, 3), (0xb, 4, 1, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];

/// The German Cup from 2002/03 (the final on 26 May).
pub(crate) const GER_CUP_LATER_ROUNDS: [FraCupCupRound; 6] = [
    cql11r((0xa, 6, 0, 5), (0x1a, 7, 0, 5, 1, 0), 0xa, (1, 0x403, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1b, 7, 0, 6), (0xb, 9, 0, 2, 2, 0), 0x14, (1, 0x403, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0xc, 0xa, 0, 6), (0x1d, 0xa, 0, 2, 2, 0), 0x1e, (1, 0x403, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x1e, 0xa, 0, 3), (0x14, 0xb, 0, 2, 2, 0), 0x78, (1, 3, 0), 4, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((0x15, 0xb, 0, 3), (7, 1, 1, 2, 2, 0), 0x82, (1, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql11r((8, 1, 1, 3), (0x1a, 4, 1, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];

/// The German Cup has 6 round(s): code 10: draw 10 Jul, match 26 Aug, 64 teams (64 new); code 20: draw 27 Aug, match 11
/// Oct, 32 teams (0 new); code 30: draw 12 Nov, match 29 Nov, 16 teams (0 new); code 120: draw 30 Nov, match 20 Dec, 8
/// teams (0 new); code 130: draw 21 Dec, match 7 Feb (+1 year), 4 teams (0 new); code 150: draw 8 Feb (+1 year), match
/// 11 May (+1 year), 2 teams (0 new); in 2001/02 the final is on 11 May, in the following seasons on 26 May.
///
/// `GerCup::create_round_table(stage, &rounds, &extra, &out)` (`single_stage_round_table`): only stage -1 (the others
/// return 0 without writing); out = 0, rounds = the table size, extra = 0 and the table of 0x68-byte rounds
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa6 and 0), each with its draw and match dates
/// (`cup_set_round_draw_date`, `cup_set_round_match_date`, cdecl) and its fields (`fra_cup_write_cup_round`).
/// Original: `GerCup::create_round_table` (0x5d8ee0, ger_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ger_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let table: &[FraCupCupRound] = if Cup(this).year() as u16 == 0x7d1 { &GER_CUP_2001_ROUNDS } else { &GER_CUP_LATER_ROUNDS };
    single_stage_round_table(Cup(this), stage, rounds, extra, out, table, 0x9cd3e0, 0xa6, "mwc_13 ger cup round table")
}

/// Comparer for `ger_cup_create_round_table`: side by side (`compare_cup_round_table`).
unsafe extern "thiscall" fn compare_ger_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GER_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("ger_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| ger_cup_create_round_table(this, stage, r, e, s))
}
