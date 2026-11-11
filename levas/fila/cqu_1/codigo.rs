// ---------- batch %%LEVA%%: the constructors of the national leagues outside Europe: the Brazilian first and second divisions, J1 and the NSL (the rules, the table, the fixtures, the statistics and the predicted positions at creation) (bra_nat_first.cpp, bra_nat_second.cpp, jap_j1.cpp, aus_nsl.cpp) ----------

/// Describes one national league built by `bra_nat_first_construct_league`.
///
/// It holds the vtable, the importance (+0x51) and whether it is written before loading the saved state
/// (`importance_before_load`: on every creation, as in the NSL; otherwise only on a new league), the stages beyond the
/// league (+0x2c; the array at +0xc = malloc of 4 × stages), the rules (thiscall, ret 0), the table (thiscall, ret 0),
/// the source file and the error-box lines (rules, table, fixtures, order, statistics). The predicted positions are
/// League's (`exe::league_set_predicted_positions`).
pub(crate) struct BraNatFirstNationalLeague {
    pub(crate) vtable: usize,
    pub(crate) importance: u8,
    pub(crate) importance_before_load: bool,
    pub(crate) stages: u32,
    pub(crate) rules: usize,
    pub(crate) table: usize,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 5],
}

/// The constructor of a national league (the same 544-byte template as the state leagues, checked instruction by
/// instruction: only the vtable, the discipline rule, the importance and when it is written, the stages, the rules, the
/// table, the source file and the lines change; the predicted positions are League's).
///
/// League::League (`exe::new_league`, thiscall, ret 0); `league::COMPETITION` = the competition record; the vtable; the
/// year (`league::YEAR`, word) = `year`; the discipline rule (`league::DISCIPLINE_RULE`) = `discipline` (and, in the NSL,
/// the importance `league::IMPORTANCE`). If the saved state loads (League::load_from_temp_file
/// `exe::league_load_from_temp_file`, thiscall(this, 1), ret 4; returns ≠ 0), returns this.
///
/// Otherwise (new career): the importance (outside the NSL), `league::STAGE_INDEX` = `stage_index::MAIN` (0xff),
/// `league::CURRENT_STAGE` = −1, `league::STAGE_COUNT` = the stages and their array `league::STAGES` (malloc
/// `exe::crt_malloc`, unchecked); then, each with its own error box (returning this) if it returns 0: the rules, the
/// table, the fixtures (League::create_fixtures `exe::league_create_fixtures`, thiscall, ret 0) and the order
/// (League::sort_table `exe::league_sort_table`, thiscall(this, 0), ret 4); the statistics (operator new
/// `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats`, thiscall(new, this), ret 4; the
/// original's SEH frame covers only that new) at `league::COMP_STATS` (without them: the error box); finally the
/// predicted positions (League::set_predicted_positions `exe::league_set_predicted_positions`, thiscall, ret 0).
pub(crate) unsafe fn bra_nat_first_construct_league(this: League, year: u32, record: usize, e: &BraNatFirstNationalLeague, discipline: u8, branch: &'static str) -> usize {
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.rules);
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.table);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), e.vtable);
    write_u16(this.addr() + league::YEAR, year as u16);
    this.set_discipline_rule(discipline);
    if e.importance_before_load {
        this.set_importance(e.importance);
    }
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    if !e.importance_before_load {
        this.set_importance(e.importance);
    }
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    write_u32(this.addr() + league::STAGE_COUNT, e.stages);
    let v = exe::crt_malloc(e.stages * 4);
    this.set_stages(v);
    if rules(this.addr()) == 0 {
        show_error_at(e.file, e.rows[0]);
        return this.addr();
    }
    if table(this.addr()) == 0 {
        show_error_at(e.file, e.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(e.file, e.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(e.file, e.rows[3]);
        return this.addr();
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(e.file, e.rows[4]);
        return this.addr();
    }
    exe::league_set_predicted_positions(this);
    this.addr()
}

/// Comparer of the four national league constructors: they allocate and draw the fixtures, so only the original runs,
/// and what it writes is checked: the return value, the vtable, the year, the record and the discipline rule; the
/// importance when it is always written (the NSL); on a new league (`stage_index::MAIN`, with the statistics), the
/// stage count, the stage array and the importance.
pub(crate) unsafe fn bra_nat_first_compare_league_ctor(name: &str, this: League, year: u32, record: usize, e: &BraNatFirstNationalLeague, discipline: u8, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == e.vtable && read_u16(this.addr() + league::YEAR) == year as u16
        && read_usize(this.addr() + league::COMPETITION) == record && this.discipline_rule() == discipline;
    if e.importance_before_load {
        ok = ok && this.importance() == e.importance;
    }
    if this.stage_index() == stage_index::MAIN && this.comp_stats() != 0 {
        ok = ok && this.stage_count() == e.stages as usize && this.stages() != 0 && this.importance() == e.importance;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): returned {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}, +0x51 = {}", name, this.addr(), r, read_usize(this.addr()), this.stage_count(), this.stages(), this.comp_stats(), this.importance()));
    r
}

pub(crate) const BRA_NAT_FIRST_LEAGUE: BraNatFirstNationalLeague = BraNatFirstNationalLeague {
    vtable: 0x967d98,
    importance: 0xa,
    importance_before_load: false,
    stages: 1,
    rules: 0x42a5a0,
    table: 0x42a480,
    file: 0x98aa30,
    rows: [0x56, 0x5d, 0x64, 0x6d, 0x74],
};

pub(crate) const BRA_NAT_SECOND_LEAGUE: BraNatFirstNationalLeague = BraNatFirstNationalLeague {
    vtable: 0x967e4c,
    importance: 0xa,
    importance_before_load: false,
    stages: 1,
    rules: 0x42cb70,
    table: 0x42ca70,
    file: 0x98ab64,
    rows: [0x50, 0x57, 0x5e, 0x67, 0x6e],
};

pub(crate) const JAP_J1_LEAGUE: BraNatFirstNationalLeague = BraNatFirstNationalLeague {
    vtable: 0x96ca40,
    importance: 0xf,
    importance_before_load: false,
    stages: 3,
    rules: 0x663620,
    table: 0x6634e0,
    file: 0x9eabc8,
    rows: [0x4b, 0x52, 0x59, 0x62, 0x69],
};

pub(crate) const AUS_NSL_LEAGUE: BraNatFirstNationalLeague = BraNatFirstNationalLeague {
    vtable: 0x967768,
    importance: 0xa,
    importance_before_load: true,
    stages: 2,
    rules: 0x411ce0,
    table: 0x411b70,
    file: 0x987c20,
    rows: [0x46, 0x4d, 0x53, 0x5a, 0x61],
};

/// The Brazilian first division at career creation: the year and the database competition, discipline rule 6
/// (`discipline_rule::BRAZIL_NATIONAL`) and, with no saved state, importance 10, a round robin and the knockout (one
/// stage besides the league), the rules (no league above, the second division below, eight relegated [inference:
/// maxevans60 map]), the table (the 28 national first division clubs [inference: map]), the fixtures, the order, the
/// statistics and the predicted positions by club strength (`bra_nat_first_construct_league` with
/// `BRA_NAT_FIRST_LEAGUE`).
/// Original: `BraNatFirst::BraNatFirst` (0x429850, bra_nat_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_nat_first(this: usize, year: u32, record: usize) -> usize {
    bra_nat_first_construct_league(League(this), year, record, &BRA_NAT_FIRST_LEAGUE, discipline_rule::BRAZIL_NATIONAL, "cqu_1 bra_nat_first new") // cqu1: the Brazilian first division
}

/// The Brazilian second division at career creation: the year and the database competition, discipline rule 6
/// (`discipline_rule::BRAZIL_NATIONAL`) and, with no saved state, importance 10, a round robin and the knockout (one
/// stage besides the league), the rules (the first division above and the third below, two relegated [inference:
/// map]), the table (the national second division clubs), the fixtures, the order, the statistics and the predicted
/// positions by club strength (`bra_nat_first_construct_league` with `BRA_NAT_SECOND_LEAGUE`).
/// Original: `BraNatSecond::BraNatSecond` (0x42be40, bra_nat_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_nat_second(this: usize, year: u32, record: usize) -> usize {
    bra_nat_first_construct_league(League(this), year, record, &BRA_NAT_SECOND_LEAGUE, discipline_rule::BRAZIL_NATIONAL, "cqu_1 bra_nat_second new") // cqu1: the discipline of the second division
}

/// The J1 League at career creation: the year and the database competition, discipline rule 0x13
/// (`discipline_rule::JAPAN_LEAGUE`) and, with no saved state, importance 15, the two halves and the final between the
/// half winners (three stages besides the league), the rules (points, tie-breaks and match rules, with J2 as the
/// relegation division [inference: map]), the table (the 16 J1 clubs and the prize fund [inference: map]), the
/// fixtures, the order, the statistics and the predicted positions by club strength (`bra_nat_first_construct_league`
/// with `JAP_J1_LEAGUE`).
/// Original: `JapJ1::JapJ1` (0x662b30, jap_j1.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_jap_j1(this: usize, year: u32, record: usize) -> usize {
    bra_nat_first_construct_league(League(this), year, record, &JAP_J1_LEAGUE, discipline_rule::JAPAN_LEAGUE, "cqu_1 jap_j1 new") // cqu1: J1
}

/// The Australian NSL at career creation: the year and the database competition, discipline rule 0x22
/// (`discipline_rule::AUSTRALIA`) and importance 10 (written before loading the saved state) and, with no saved state,
/// the finals series and the grand final (two stages besides the league), the rules (among them the substitute
/// counts [inference: map]), the table (the NSL clubs), the fixtures, the order, the statistics and the predicted
/// positions by club strength (`bra_nat_first_construct_league` with `AUS_NSL_LEAGUE`).
/// Original: `AusNsl::AusNsl` (0x410e90, aus_nsl.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_aus_nsl(this: usize, year: u32, record: usize) -> usize {
    bra_nat_first_construct_league(League(this), year, record, &AUS_NSL_LEAGUE, discipline_rule::AUSTRALIA, "cqu_1 aus_nsl new") // cqu1: the discipline of the NSL
}

/// Comparer for `construct_bra_nat_first`.
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_nat_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    bra_nat_first_compare_league_ctor("construct_bra_nat_first", League(this), year, record, &BRA_NAT_FIRST_LEAGUE, discipline_rule::BRAZIL_NATIONAL, || o(this, year, record))
}

/// Comparer for `construct_bra_nat_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_nat_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    bra_nat_first_compare_league_ctor("construct_bra_nat_second", League(this), year, record, &BRA_NAT_SECOND_LEAGUE, discipline_rule::BRAZIL_NATIONAL, || o(this, year, record))
}

/// Comparer for `construct_jap_j1`.
pub(crate) unsafe extern "thiscall" fn compare_construct_jap_j1(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    bra_nat_first_compare_league_ctor("construct_jap_j1", League(this), year, record, &JAP_J1_LEAGUE, discipline_rule::JAPAN_LEAGUE, || o(this, year, record))
}

/// Comparer for `construct_aus_nsl`.
pub(crate) unsafe extern "thiscall" fn compare_construct_aus_nsl(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I3%%);
    bra_nat_first_compare_league_ctor("construct_aus_nsl", League(this), year, record, &AUS_NSL_LEAGUE, discipline_rule::AUSTRALIA, || o(this, year, record))
}
