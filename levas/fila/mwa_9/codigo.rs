// ---------- batch %%LEVA%%: the Northern Irish Premier Division: the constructor, the rules and the table (nir_prm.cpp) ----------

/// What changes between the Irish leagues' constructors (the League template of EngFirst and JapJ2, read instruction by
/// instruction): the vtable, the discipline rule, whether there is one extra stage (the promotion play-off: the stage
/// count 1 and a 4-byte stage array, unchecked; otherwise 0 and none), whether the IrePrm block from +0xee is reset,
/// the rules and table routines (thiscall, ret 0; 0 = failure), the source file and the five error box lines (rules,
/// table, fixtures, order, statistics).
pub(crate) struct IrishLeagueConstructor {
    pub(crate) vtable: usize,
    pub(crate) discipline: u8,
    pub(crate) play_off_stage: bool,
    pub(crate) reset_ten_slots: bool,
    pub(crate) rules: usize,
    pub(crate) table: usize,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 5],
}

/// The body of the Irish leagues' constructors (thiscall, ret 8: this, year, the competition record).
///
/// League::League (`exe::new_league`, thiscall, ret 0); `league::COMPETITION` = the record; the vtable; `league::YEAR`
/// (word); `league::DISCIPLINE_RULE`; in the Premier Division, the ten 5-byte entries from +0xee get −1 in their first
/// dword (its own state [inference: the last seasons' places]). If the saved state loads (League::load_from_temp_file
/// `exe::league_load_from_temp_file`(this, 1), thiscall, ret 4; ≠ 0) it returns this. Otherwise `league::STAGE_INDEX` =
/// `stage_index::MAIN`, `league::CURRENT_STAGE` = −1, the stage count and array; then, each with its own error box
/// (returning this) when it returns 0: the rules, the table, the fixtures (League::create_fixtures
/// `exe::league_create_fixtures`), the order (League::sort_table `exe::league_sort_table`(this, 0)); the statistics
/// (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats`) at
/// `league::COMP_STATS` (without them, the box); finally the predicted positions (League::set_predicted_positions
/// `exe::league_set_predicted_positions`). Returns this.
pub(crate) unsafe fn construct_irish_league(this: League, year: u32, record: usize, k: &IrishLeagueConstructor, branch: &'static str) -> usize {
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), k.vtable);
    this.set_year(year as u16 as i16);
    this.set_discipline_rule(k.discipline);
    if k.reset_ten_slots {
        for s in 0..10usize {
            write_i32(this.addr() + 0xee + s * 5, -1);
        }
    }
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    if k.play_off_stage {
        this.set_stage_count(1);
        this.set_stages(exe::crt_malloc(4));
    } else {
        this.set_stage_count(0);
        this.set_stages(0);
    }
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(k.rules);
    if rules(this.addr()) == 0 {
        show_error_at(k.file, k.rows[0]);
        return this.addr();
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(k.table);
    if table(this.addr()) == 0 {
        show_error_at(k.file, k.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(k.file, k.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(k.file, k.rows[3]);
        return this.addr();
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(k.file, k.rows[4]);
        return this.addr();
    }
    exe::league_set_predicted_positions(this);
    this.addr()
}

/// The comparer of the Irish leagues' constructors: only the original runs (it allocates, draws the fixtures and reads
/// the save file); the plan: the return (this), the vtable, the year, the record, the discipline rule and, on a new
/// league (`stage_index::MAIN`), the current stage −1, the stage count, the stage array (none without stages) and the
/// statistics.
pub(crate) unsafe fn compare_irish_league_constructor(name: &str, this: League, year: u32, record: usize, k: &IrishLeagueConstructor, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == k.vtable && this.year() as u16 == year as u16
        && read_usize(this.addr() + league::COMPETITION) == record && this.discipline_rule() == k.discipline;
    if ok && this.stage_index() == stage_index::MAIN {
        ok = read_i32(this.addr() + league::CURRENT_STAGE) == -1 && read_u32(this.addr() + league::STAGE_COUNT) == k.play_off_stage as u32
            && (this.stages() != 0) == k.play_off_stage && read_usize(this.addr() + league::COMP_STATS) != 0;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}, {}, {:#x}) returned {:#x}, vtable {:#x}, year {}, stages {} {:#x}, statistics {:#x}", name, this.addr(), year as u16, record, r,
        read_usize(this.addr()), this.year(), read_u32(this.addr() + league::STAGE_COUNT), this.stages(), read_usize(this.addr() + league::COMP_STATS)));
    r
}

/// The rule fields of an Irish league's set_up_rules, written in the original's order (offset, width, value), the
/// division above and below (the key global of its id; None = −1).
pub(crate) struct IrishLeagueRules {
    pub(crate) writes: &'static [(usize, u8, u32)],
    pub(crate) upper: Option<usize>,
    pub(crate) lower: Option<usize>,
}

/// The fields `set_up_rules` writes before the round table: `writes`, then `league::UPPER_DIVISION_ID` and
/// `league::LOWER_DIVISION_ID` (the id of the key global, or −1).
pub(crate) unsafe fn write_irish_league_rules(this: League, k: &IrishLeagueRules) {
    for &(o, n, x) in k.writes {
        match n {
            1 => write_u8(this.addr() + o, x as u8),
            2 => write_u16(this.addr() + o, x as u16),
            _ => write_u32(this.addr() + o, x),
        }
    }
    write_u32(this.addr() + league::UPPER_DIVISION_ID, k.upper.map_or(0xffff_ffff, |g| read_u32(g)));
    write_u32(this.addr() + league::LOWER_DIVISION_ID, k.lower.map_or(0xffff_ffff, |g| read_u32(g)));
}

/// The body of the Irish leagues' set_up_rules (thiscall, ret 0): the fields (`write_irish_league_rules`) and the round
/// table of the main stage (the league's own virtual `comp_vtable::CREATE_SCHEDULE`(−1, &`league::ROUND_COUNT`,
/// &`league::STAGE_ID`, null), thiscall, ret 0x10) into `league::ROUND_DATES`. Returns 1.
pub(crate) unsafe fn set_up_irish_league_rules(this: League, k: &IrishLeagueRules) -> u32 {
    write_irish_league_rules(this, k);
    let schedule: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = schedule(this.addr(), 0xffff_ffff, this.addr() + league::ROUND_COUNT, this.addr() + league::STAGE_ID, 0);
    write_usize(this.addr() + league::ROUND_DATES, t);
    1
}

/// The comparer of the Irish leagues' set_up_rules: side by side over the league's fields outside the round table
/// (`league_regions_without_schedule`); the copy writes only the fields (the original's round table stays).
pub(crate) unsafe fn compare_irish_league_rules(name: &str, this: usize, k: &IrishLeagueRules, original: impl FnOnce() -> u32) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&reg, original, || {
        write_irish_league_rules(League(this), k);
        1
    });
    SCOREBOARD.record(equal && r == n, || format!("{}({:#x}) original={} rust={} {:?} {:?}", name, this, r, n, mo, mr));
    r
}

/// The Northern Irish Premier Division's constructor: the vtable 0x96dccc, discipline rule
/// `discipline_rule::NORTHERN_IRELAND`, no extra stage, NirPrm::set_up_rules (0x78ed80) and NirPrm::create_table
/// (0x78ec60), the lines 0x45, 0x4c, 0x53, 0x5c and 0x63 of nir_prm.cpp.
pub(crate) const NORTHERN_IRISH_PREMIER_CONSTRUCTOR: IrishLeagueConstructor = IrishLeagueConstructor { vtable: 0x96dccc, discipline: discipline_rule::NORTHERN_IRELAND,
    play_off_stage: false, reset_ten_slots: false, rules: 0x78ed80, table: 0x78ec60, file: 0xa696d0, rows: [0x45, 0x4c, 0x53, 0x5c, 0x63] };

/// Rule: the Northern Irish Premier Division at career creation: the year and the database competition, the Northern
/// Irish discipline rules and, with nothing saved, a plain league (no extra stage): the rules, the table, the fixtures,
/// the order, the statistics and the predicted positions.
///
/// `construct_irish_league` with `NORTHERN_IRISH_PREMIER_CONSTRUCTOR` (checked instruction by instruction against
/// JapJ2::JapJ2 0x6644c0: only the vtable, the discipline rule, the two routines, the file and the lines change).
/// Original: `NirPrm::NirPrm` (0x78e070, nir_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nir_prm(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league(League(this), year, record, &NORTHERN_IRISH_PREMIER_CONSTRUCTOR, "mwa_9 northern irish premier new") // mwa9: the Northern Irish Premier
}

unsafe extern "thiscall" fn compare_construct_nir_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_irish_league_constructor("construct_nir_prm", League(this), year, record, &NORTHERN_IRISH_PREMIER_CONSTRUCTOR, || o(this, year, record))
}

/// The Northern Irish Premier Division's rules: four meetings (`league::LEGS` = 4), 3 points for a win and 1 for a
/// draw, match category 2, `league::MATCH_RULES` and `league::ROUND_DRAW_RULES` 0, club sides, the tie-breaks 1, 2
/// (goal difference, goals for) and 4, no promotion, one club to the relegation play-off and one relegated, the squad
/// flags 0, 3 substitutes on the bench and 3 substitutions, `league::LEAGUE_FLAGS` 3 (champion and promotion status
/// bits [inference]); no division above, the First Division below.
pub(crate) const NORTHERN_IRISH_PREMIER_RULES: IrishLeagueRules = IrishLeagueRules {
    writes: &[(league::LEGS, 2, 4), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0),
        (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 4),
        (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 1), // mwa9: one relegated, one in the play-off
        (league::SQUAD_FLAGS, 1, 0), (league::SUBSTITUTES_NAMED, 1, 3), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::LEAGUE_FLAGS, 2, 3)],
    upper: None,
    lower: Some(KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION),
};

/// Rule: the Northern Irish Premier Division: ten clubs meeting four times, 3 points for a win and 1 for a draw, ties
/// split by goal difference and goals for; the last goes down to the First Division and the second last plays the
/// promotion/relegation play-off; 3 substitutes on the bench and 3 substitutions.
///
/// `set_up_irish_league_rules` with `NORTHERN_IRISH_PREMIER_RULES` (the fields, then the round table of the main stage
/// into `league::ROUND_DATES`). Returns 1.
/// Original: `NirPrm::set_up_rules` (0x78ed80, nir_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwa_9 northern irish premier rules");
    set_up_irish_league_rules(League(this), &NORTHERN_IRISH_PREMIER_RULES)
}

unsafe extern "thiscall" fn compare_nir_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_irish_league_rules("nir_prm_set_up_rules", this, &NORTHERN_IRISH_PREMIER_RULES, || o(this))
}

/// Rule: the Northern Irish Premier Division table has ten clubs, the Premier Division clubs in database order; when
/// full, the prize fund for a 500 share per place, paid by table position at the season's end.
///
/// `ita_ser_a_fill_league_table`(10 clubs, `KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION`; out of memory, the error
/// box at line 0x15a of nir_prm.cpp) (ItaSerA::create_table's template, checked instruction by instruction: the end
/// without the TV money); not full, 0; full, the prize fund (League::calc_prize_fund `exe::league_calc_prize_fund`(this,
/// 500), thiscall, ret 4) in `league::PRIZE_FUND`, `league::PRIZE_MODE` = 1 and 1.
/// Original: `NirPrm::create_table` (0x78ec60, nir_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_prm_create_table(this: usize) -> u32 {
    if ita_ser_a_fill_league_table(League(this), 0xa, 0xa696d0, 0x15a, KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION) != Some(true) {
        return 0;
    }
    covered!("mwa_9 northern irish premier table");
    set_prize_fund(League(this), 0x1f4); // mwa9: the Northern Irish Premier quota
    1
}

unsafe extern "thiscall" fn compare_nir_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    ita_ser_a_compare_league_table("nir_prm_create_table", League(this), 0xa, KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, Some(0x1f4), || o(this))
}
