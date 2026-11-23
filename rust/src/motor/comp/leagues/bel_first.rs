use super::*;

/// Rule: the Belgian turnover, led by the First Division: the season reopened; promotion and relegation along the
/// chain; the Third Division's movements and its split into groups A and B; the division tier of the clubs capped (the
/// First Division with (1, 3, −3, 1), the Second with (2, −3, 3, 0)); the First Division rebuilt for the next year
/// (rules, table, fixtures, order); the Second and Third Divisions turned over; the predicted positions, the board
/// expectations and the officials.
///
/// `top_division_turnover` with, before, virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1),
/// bel_first_move_third_division_clubs (0x41ec90) and bel_first_split_third_division_groups (0x41ef70) (called with ecx
/// = this, not ported) and League::limit_club_division_tier (`exe::league_limit_club_division_tier`) on this and on the
/// object of `KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION`; the table BelFirst::create_table (0x41e8b0) and the boxes of
/// bel_first.cpp (0x989b18) at 0x1af, 0x1b6, 0x1bc, 0x1c3; after, the start_new_season (virtual
/// `comp_vtable::START_NEW_SEASON`, no null check) of the Second Division and of `KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION`.
/// After a box: 0.
/// Original: `BelFirst::start_new_season` (0x41ea90, bel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        promotion_relegation_chain(this, 1);
        call_on_league(0x41ec90, this);
        call_on_league(0x41ef70, this);
        exe::league_limit_club_division_tier(this, 1, 3, -3, 1);
        exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION), 2, -3i32 as u32, 3, 0);
    }, 0x41e8b0, 0x989b18, [0x1af, 0x1b6, 0x1bc, 0x1c3], || {
        covered!("mwc_25 belgian first division turnover");
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION), comp_vtable::START_NEW_SEASON);
    })
}

/// Comparer for `bel_first_start_new_season`: only the original runs (it moves clubs and rebuilds); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_bel_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BEL_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("bel_first_start_new_season", League(this), || o(this))
}

/// The Belgian First Division: discipline rule 4 (`BELGIUM_LEAGUE`), the bench (7 substitutes named, 3 substitutions),
/// only the league; with no saved state, the rules, the table, the fixtures, the order, the statistics and the
/// predicted positions (`construct_nation_division`).
/// Original: `BelFirst::BelFirst` (0x41dd40, bel_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bel_first(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &BEL_FIRST_DIVISION, || {})
}

/// Comparer for `construct_bel_first`: only the original runs (`compare_nation_division_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_bel_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BEL_FIRST);
    compare_nation_division_ctor("construct_bel_first", League(this), year, record, &BEL_FIRST_DIVISION, || o(this, year, record))
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
pub(crate) unsafe extern "thiscall" fn compare_bel_first_set_up_rules(this: usize) -> u32 {
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
pub(crate) unsafe extern "thiscall" fn compare_bel_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BEL_FIRST_CREATE_TABLE);
    compare_nation_league_table("bel_first_create_table", League(this), 0x12, KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION, Some(0x4e20), || o(this))
}

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
pub(crate) unsafe extern "thiscall" fn compare_bel_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BEL_FIRST_CREATE_SCHEDULE);
    compare_cfa_schedule("bel_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| bel_first_create_schedule(this, stage, r, e, l))
}

/// The promotion and relegation chain of the league (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`, push
/// `pairs`; mov ecx -> thiscall, ret 4).
pub(crate) unsafe fn promotion_relegation_chain(this: League, pairs: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    f(this.addr(), pairs);
}

/// The common body of the top-division turnovers (no extra stages): `pre` (the promotion and relegation steps), then
/// the table freed (`exe::league_free_table`(0)), the round dates freed and zeroed, the season archived in the
/// statistics (`exe::comp_stats_archive_season_and_reset`(1), when there are statistics), the year + 1; then, each with
/// the box of `file` at its line when it returns 0 (and 0 returned): the rules (virtual `comp_vtable::SET_UP_RULES`), the
/// table (`table`, direct), `exe::league_create_fixtures` and `exe::league_sort_table`(0); then `post` (the lower
/// divisions' turnovers), the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), the board
/// expectations (League::set_club_season_expectations) and the referee renewal (OfficialsManager::renew_officials on
/// `G_OFFICIALS_MANAGER` with the record), whose eax is returned. Unlike the lower divisions' rebuild, the stages are not
/// destroyed and the current stage is not reset (these leagues have none).
pub(crate) unsafe fn top_division_turnover(this: League, pre: impl FnOnce(), table: usize, file: usize, rows: [u32; 4], post: impl FnOnce()) -> u32 {
    this.set_season_closed(0);
    pre();
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    this.set_year(this.year().wrapping_add(1)); // mwe14: the top division next year
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    let t: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if t(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, rows[3]);
        return 0;
    }
    post();
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
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
    let mut ok = r == this.addr() && read_usize(this.addr()) == p.vtable && built_year_ok(this.year() as u16, year as u16)
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

/// The rule writes of the Belgian First Division (offset, size, value).
pub(crate) unsafe fn bel_first_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0xc5, 1, 3), (0xc6, 1, 1), (0xc7, 1, 2), (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 2), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x20, 4, read_usize(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION) as u32), (0x42, 1, 1), (0xd9, 2, 0x43)];
    v.push((0xc2, 1, 3)); // mwc1: the Belgian First Division points for a win
    v
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

pub(crate) const BEL_FIRST_SCHEDULE: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 34, file: 0x989b18, row: 0xa5, table: &BEL_FIRST_SCHEDULE_ROUNDS, branch: "mwc_2 bel first schedule" };

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
