// ---------- batch 285: the constructors of the national leagues outside Europe: the Brazilian first and second divisions, J1 and the NSL (the rules, the table, the fixtures, the statistics and the predicted positions at creation) (bra_nat_first.cpp, bra_nat_second.cpp, jap_j1.cpp, aus_nsl.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_NAT_FIRST);
    bra_nat_first_compare_league_ctor("construct_bra_nat_first", League(this), year, record, &BRA_NAT_FIRST_LEAGUE, discipline_rule::BRAZIL_NATIONAL, || o(this, year, record))
}

/// Comparer for `construct_bra_nat_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_nat_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_NAT_SECOND);
    bra_nat_first_compare_league_ctor("construct_bra_nat_second", League(this), year, record, &BRA_NAT_SECOND_LEAGUE, discipline_rule::BRAZIL_NATIONAL, || o(this, year, record))
}

/// Comparer for `construct_jap_j1`.
pub(crate) unsafe extern "thiscall" fn compare_construct_jap_j1(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_JAP_J1);
    bra_nat_first_compare_league_ctor("construct_jap_j1", League(this), year, record, &JAP_J1_LEAGUE, discipline_rule::JAPAN_LEAGUE, || o(this, year, record))
}

/// Comparer for `construct_aus_nsl`.
pub(crate) unsafe extern "thiscall" fn compare_construct_aus_nsl(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_AUS_NSL);
    bra_nat_first_compare_league_ctor("construct_aus_nsl", League(this), year, record, &AUS_NSL_LEAGUE, discipline_rule::AUSTRALIA, || o(this, year, record))
}

// ---------- batch 285: the constructors of the Brazilian third division, J2 and MLS (the rules, the table, the fixtures, the statistics, the groups and the predicted positions at creation) (bra_nat_third.cpp, jap_j2.cpp, usa_mls.cpp) ----------

/// Describes one league of the constructor template (the state leagues' and the national leagues'), with the
/// variations of these three.
///
/// The importance (`league::IMPORTANCE`; `Some((value, before_load))`: written before loading the saved state, as in
/// MLS, or only on a new league; `None`: not written), the stages beyond the league (`league::STAGE_COUNT`; 0: no array,
/// `league::STAGES` = 0, as in J2; otherwise the array = malloc of 4 × stages), the rules written inline (offset, width
/// 1/2/4, value; MLS), the rules routine (thiscall, ret 0; `None` in MLS), the table and the fixtures (thiscall, ret 0),
/// the predicted positions (thiscall, ret 0), the third division (`serie_c`: `bra_nat_third::CLUB_LIST` zeroed at the
/// start and, at the end, the seven groups, the predicted positions and the list of 56 freed), the source file and the
/// error-box lines (the rules, if any, the table, the fixtures, the order, the statistics).
pub(crate) struct BraNatThirdLeagueVariant {
    pub(crate) vtable: usize,
    pub(crate) importance: Option<(u8, bool)>,
    pub(crate) stages: u32,
    pub(crate) writes: &'static [(usize, u8, u32)],
    pub(crate) rules: Option<usize>,
    pub(crate) table: usize,
    pub(crate) fixtures: usize,
    pub(crate) predicted: usize,
    pub(crate) serie_c: bool,
    pub(crate) file: usize,
    pub(crate) rows: &'static [u32],
}

/// The constructor (the 544-byte template of the state leagues, with the variations of `BraNatThirdLeagueVariant`).
///
/// League::League (`exe::new_league`, thiscall, ret 0); `league::COMPETITION` = the record; the vtable; in the third
/// division, `bra_nat_third::CLUB_LIST` = 0; the year (`league::YEAR`, word); the discipline rule
/// (`league::DISCIPLINE_RULE`); in MLS, the importance. If the saved state loads (League::load_from_temp_file
/// `exe::league_load_from_temp_file`, thiscall(this, 1), ret 4; ≠ 0), returns this.
///
/// Otherwise: the importance (if only on a new league), `league::STAGE_INDEX` = `stage_index::MAIN` (0xff),
/// `league::CURRENT_STAGE` = −1, `league::STAGE_COUNT` = the stages and their array `league::STAGES` (malloc
/// `exe::crt_malloc`, unchecked; no stages: 0), the inline rules; then, each with its own error box (returning this) if
/// it returns 0: the rules, the table, the fixtures and the order (League::sort_table `exe::league_sort_table`,
/// thiscall(this, 0), ret 4); the statistics (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats
/// `exe::new_comp_stats`, thiscall(new, this), ret 4) at `league::COMP_STATS` (without them: the error box). In the
/// third division: the seven first-round groups (BraNatThird::create_group_stage
/// `exe::branatthird_create_group_stage`(g), thiscall, ret 4, g from 0 to 6; the original pushes the dword with g in the
/// low byte and, above it, the high bytes of the new's pointer; the routine only uses g: the copy passes g clean), the
/// predicted positions and the list of 56 (`bra_nat_third::CLUB_LIST`) freed (free `exe::crt_free`, if there is one)
/// and zeroed. In the others, the predicted positions.
pub(crate) unsafe fn bra_nat_third_construct_league(this: League, year: u32, record: usize, e: &BraNatThirdLeagueVariant, discipline: u8, branch: &'static str) -> usize {
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.table);
    let fixtures: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.fixtures);
    let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(e.predicted);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), e.vtable);
    if e.serie_c {
        write_usize(this.addr() + bra_nat_third::CLUB_LIST, 0);
    }
    write_u16(this.addr() + league::YEAR, year as u16);
    this.set_discipline_rule(discipline);
    if let Some((v, true)) = e.importance {
        this.set_importance(v);
    }
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    if let Some((v, false)) = e.importance {
        this.set_importance(v);
    }
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    write_u32(this.addr() + league::STAGE_COUNT, e.stages);
    let v = if e.stages != 0 { exe::crt_malloc(e.stages * 4) } else { 0 };
    this.set_stages(v);
    for &(o, n, x) in e.writes {
        match n {
            1 => write_u8(this.addr() + o, x as u8),
            2 => write_u16(this.addr() + o, x as u16),
            _ => write_u32(this.addr() + o, x),
        }
    }
    let mut rows = e.rows.iter();
    if let Some(r) = e.rules {
        let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(r);
        let row = *rows.next().unwrap();
        if rules(this.addr()) == 0 {
            show_error_at(e.file, row);
            return this.addr();
        }
    }
    let (row_table, row_fixtures, row_order, row_stats) = (*rows.next().unwrap(), *rows.next().unwrap(), *rows.next().unwrap(), *rows.next().unwrap());
    if table(this.addr()) == 0 {
        show_error_at(e.file, row_table);
        return this.addr();
    }
    if fixtures(this.addr()) == 0 {
        show_error_at(e.file, row_fixtures);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(e.file, row_order);
        return this.addr();
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(e.file, row_stats);
        return this.addr();
    }
    if e.serie_c {
        for g in 0..7u32 {
            exe::branatthird_create_group_stage(this, g); // cqu2: the seven groups of the third division
        }
        predicted(this.addr());
        let list = read_usize(this.addr() + bra_nat_third::CLUB_LIST);
        if list != 0 {
            exe::crt_free(list);
        }
        write_usize(this.addr() + bra_nat_third::CLUB_LIST, 0);
        return this.addr();
    }
    predicted(this.addr());
    this.addr()
}

/// Comparer of the three constructors: they allocate and draw the fixtures, so only the original runs, and what it
/// writes is checked: the return value, the vtable, the year, the record, the discipline rule and the importance when
/// it is always written (MLS); on a new league (`stage_index::MAIN`, with the statistics), the stages, their array (null
/// without stages), the importance, the inline rules and, in the third division, the seven groups in slots 0 to 6 and
/// the list of 56 zeroed.
pub(crate) unsafe fn bra_nat_third_compare_league_ctor(name: &str, this: League, year: u32, record: usize, e: &BraNatThirdLeagueVariant, discipline: u8, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == e.vtable && read_u16(this.addr() + league::YEAR) == year as u16
        && read_usize(this.addr() + league::COMPETITION) == record && this.discipline_rule() == discipline;
    if let Some((v, true)) = e.importance {
        ok = ok && this.importance() == v;
    }
    if this.stage_index() == stage_index::MAIN && this.comp_stats() != 0 {
        ok = ok && this.stage_count() == e.stages as usize && (this.stages() != 0) == (e.stages != 0);
        if let Some((v, _)) = e.importance {
            ok = ok && this.importance() == v;
        }
        // ponytail: only the points and tie-break rules (PROMOTED_COUNT..=TIEBREAKS + 3, +0xbe..+0xc8); the order
        // (sort_table) rewrites GAMES_PER_CLUB and the others may change with the fixtures
        for &(o, n, x) in e.writes.iter().filter(|w| (league::PROMOTED_COUNT..=league::TIEBREAKS + 3).contains(&w.0)) {
            let value = match n {
                1 => read_u8(this.addr() + o) as u32,
                2 => read_u16(this.addr() + o) as u32,
                _ => read_usize(this.addr() + o) as u32,
            };
            ok = ok && value == x;
        }
        if e.serie_c {
            ok = ok && read_usize(this.addr() + bra_nat_third::CLUB_LIST) == 0 && (0..7).all(|g| read_usize(this.stages() + 4 * g) != 0);
        }
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): returned {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}, +0x51 = {}", name, this.addr(), r, read_usize(this.addr()), this.stage_count(), this.stages(), this.comp_stats(), this.importance()));
    r
}

pub(crate) const BRA_NAT_THIRD_LEAGUE: BraNatThirdLeagueVariant = BraNatThirdLeagueVariant {
    vtable: 0x967f00,
    importance: None,
    stages: 0xb,
    writes: &[],
    rules: Some(0x42de50),
    table: FN_BRANATTHIRD_CREATE_TABLE,
    fixtures: FN_LEAGUE_CREATE_FIXTURES,
    predicted: 0x42f0a0,
    serie_c: true,
    file: 0x98aba4,
    rows: &[0x4a, 0x51, 0x58, 0x61, 0x68],
};

pub(crate) const JAP_J2_LEAGUE: BraNatThirdLeagueVariant = BraNatThirdLeagueVariant {
    vtable: 0x96caf4,
    importance: None,
    stages: 0,
    writes: &[],
    rules: Some(0x665140),
    table: 0x665030,
    fixtures: FN_LEAGUE_CREATE_FIXTURES,
    predicted: FN_LEAGUE_SET_PREDICTED_POSITIONS,
    serie_c: false,
    file: 0x9eaedc,
    rows: &[0x46, 0x4d, 0x53, 0x5a, 0x61],
};

/// The MLS rules, written inline in the original's order (offset, width, value).
pub(crate) const MLS_INLINE_RULES: [(usize, u8, u32); 18] = [
    (league::WIN_POINTS, 1, 3), // cqu2: the MLS points for a win
    (league::TIEBREAKS + 2, 1, 3),
    (league::DRAW_POINTS, 1, 1),
    (league::MATCH_RULES, 2, 0xc2),
    (league::MATCH_CATEGORY, 1, 2),
    (league::CLUB_SIDES, 1, 1),
    (league::TIEBREAKS, 1, 1),
    (league::TIEBREAKS + 1, 1, 2),
    (league::TIEBREAKS + 3, 1, 0),
    (league::PROMOTED_COUNT, 1, 1),
    (league::PROMOTION_PLAYOFF_COUNT, 1, 0),
    (league::RELEGATION_PLAYOFF_COUNT, 1, 0),
    (league::RELEGATED_COUNT, 1, 0),
    (league::UPPER_DIVISION_ID, 4, 0xffff_ffff),
    (league::LOWER_DIVISION_ID, 4, 0xffff_ffff),
    (league::LEAGUE_FLAGS, 2, 0x48),
    (league::LEGS, 2, 4),
    (league::GAMES_PER_CLUB, 2, 0x20),
];

pub(crate) const USA_MLS_LEAGUE: BraNatThirdLeagueVariant = BraNatThirdLeagueVariant {
    vtable: 0x970844,
    importance: Some((0xa, true)),
    stages: 4,
    writes: &MLS_INLINE_RULES,
    rules: None,
    table: FN_USAMLS_CREATE_TABLE,
    fixtures: FN_USAMLS_CREATE_SEASON_FIXTURES,
    predicted: 0x909bb0,
    serie_c: false,
    file: 0xaa1fb4,
    rows: &[0x59, 0x5f, 0x66, 0x6d],
};

/// The Brazilian third division at career creation: the year and the database competition, discipline rule 7
/// (`discipline_rule::BRAZIL_REGIONAL`) and, with no saved state, eleven stages besides the league (the seven
/// first-round groups of seven, the second and third rounds and the final), the rules, the table, the fixtures, the
/// order, the statistics, the seven first-round groups with the 56 clubs, the predicted positions (by reputation) and
/// the list of 56 freed (`bra_nat_third_construct_league` with `BRA_NAT_THIRD_LEAGUE`).
/// Original: `BraNatThird::BraNatThird` (0x42d2c0, bra_nat_third.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_nat_third(this: usize, year: u32, record: usize) -> usize {
    bra_nat_third_construct_league(League(this), year, record, &BRA_NAT_THIRD_LEAGUE, discipline_rule::BRAZIL_REGIONAL, "cqu_2 bra_nat_third new") // cqu2: the third division
}

/// J2 at career creation: the year and the database competition, discipline rule 0x13 (`discipline_rule::JAPAN_LEAGUE`)
/// and, with no saved state, a plain round robin (no stage besides the league, no stage array), the rules, the table,
/// the fixtures, the order, the statistics and the predicted positions by club strength (`bra_nat_third_construct_league`
/// with `JAP_J2_LEAGUE`).
/// Original: `JapJ2::JapJ2` (0x6644c0, jap_j2.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_jap_j2(this: usize, year: u32, record: usize) -> usize {
    bra_nat_third_construct_league(League(this), year, record, &JAP_J2_LEAGUE, discipline_rule::JAPAN_LEAGUE, "cqu_2 jap_j2 new") // cqu2: the discipline of J2
}

/// MLS at career creation: the year and the database competition, discipline rule 0x1e (`discipline_rule::USA`) and
/// importance 10 (before loading the saved state) and, with no saved state, four stages besides the league (the
/// conferences and the knockout), the rules written inline (`MLS_INLINE_RULES`: 3 points for a win and 1 for a draw,
/// the tie-breakers and the match category), the table of the three divisions, the season fixtures
/// (UsaMls::create_season_fixtures instead of League::create_fixtures), the order, the statistics and the predicted
/// positions (`bra_nat_third_construct_league` with `USA_MLS_LEAGUE`).
/// Original: `UsaMls::UsaMls` (0x906f80, usa_mls.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_usa_mls(this: usize, year: u32, record: usize) -> usize {
    bra_nat_third_construct_league(League(this), year, record, &USA_MLS_LEAGUE, discipline_rule::USA, "cqu_2 usa_mls new") // cqu2: MLS
}

/// Comparer for `construct_bra_nat_third`.
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_nat_third(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_NAT_THIRD);
    bra_nat_third_compare_league_ctor("construct_bra_nat_third", League(this), year, record, &BRA_NAT_THIRD_LEAGUE, discipline_rule::BRAZIL_REGIONAL, || o(this, year, record))
}

/// Comparer for `construct_jap_j2`.
pub(crate) unsafe extern "thiscall" fn compare_construct_jap_j2(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_JAP_J2);
    bra_nat_third_compare_league_ctor("construct_jap_j2", League(this), year, record, &JAP_J2_LEAGUE, discipline_rule::JAPAN_LEAGUE, || o(this, year, record))
}

/// Comparer for `construct_usa_mls`.
pub(crate) unsafe extern "thiscall" fn compare_construct_usa_mls(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_USA_MLS);
    bra_nat_third_compare_league_ctor("construct_usa_mls", League(this), year, record, &USA_MLS_LEAGUE, discipline_rule::USA, || o(this, year, record))
}

// ---------- batch 285: each game result of the Minas Gerais, Paraná and Rio state leagues and J1 (the overall and the half table) (bra_reg_minas_gerais.cpp, bra_reg_parana.cpp, bra_reg_rio.cpp, jap_j1.cpp) ----------

/// The result of a game in a two-half league (the same 128-byte template in all four, checked instruction by
/// instruction: only the round where the second half starts, 11 in the state leagues and 15 in J1, and the half routine
/// change).
///
/// 1) The game goes into the overall table (League::add_result `exe::league_add_result`, thiscall(this, game, stats),
///    ret 8).
/// 2) If the game has no stage yet (`fixture::STAGE_INDEX` = 0xff): its stage is the half, 0 up to round `second` − 1
///    and 1 from it on (the round is the signed word `fixture::ROUND_NUMBER`), written to `fixture::STAGE_INDEX`; while
///    the league's current stage (`league::CURRENT_STAGE`) is below it, the next half is built (the half routine `half`,
///    e.g. `jap_j1_create_half_season_stage`: thiscall(this, current stage + 1), ret 4; the original pushes the dword
///    with the byte in the low part and register garbage above; the half routine only uses the byte: the copy passes the
///    clean byte); then the game also goes into the half table (the `comp_vtable::ADD_RESULT` slot of stage
///    [`league::STAGES`][stage]: thiscall(stage, game, stats), ret 8) with the league's statistics block
///    (`league::COMP_STATS`) saved and zeroed (so the game is not counted twice) and restored; the game's
///    `fixture::STAGE_INDEX` goes back to 0xff.
///
/// Returns 1 on both paths.
pub(crate) unsafe fn bra_reg_minas_gerais_add_two_half_result(this: League, game: usize, stats: u32, second: i16, half: usize, branch: &'static str) -> u32 {
    let build: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(half);
    exe::league_add_result(this, game, stats);
    if read_u8(game + fixture::STAGE_INDEX) != 0xff {
        return 1;
    }
    let round = read_i16(game + fixture::ROUND_NUMBER);
    let stage: i8 = (round >= second) as i8; // cqu3: the half of the game
    write_i8(game + fixture::STAGE_INDEX, stage);
    while (this.current_stage() as i32) < stage as i32 {
        build(this.addr(), read_u8(this.addr() + league::CURRENT_STAGE).wrapping_add(1) as u32);
    }
    covered!(branch);
    let stored = this.comp_stats();
    this.set_comp_stats(0);
    let f = read_usize(this.stages() + 4 * read_i8(game + fixture::STAGE_INDEX) as i32 as usize);
    let add: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(f) + comp_vtable::ADD_RESULT));
    add(f, game, stats); // cqu3: the game in the half table
    this.set_comp_stats(stored);
    write_u8(game + fixture::STAGE_INDEX, 0xff); // cqu3: the game mark back
    1
}

/// Comparer of the four: only the original runs (the game goes into the tables and the half may be built; it cannot run
/// twice), with the plan made before: the return value 1, the game's `fixture::STAGE_INDEX` as it was (0xff back, if it
/// was), the league's `league::COMP_STATS` unchanged and, when the game had no stage, the league's current stage
/// (`league::CURRENT_STAGE`) already at the game's half, with that stage built.
pub(crate) unsafe fn bra_reg_minas_gerais_compare_two_half_result(name: &str, this: League, game: usize, second: i16, original: impl FnOnce() -> u32) -> u32 {
    let mark = read_u8(game + fixture::STAGE_INDEX);
    let block = this.comp_stats();
    let stage = (read_i16(game + fixture::ROUND_NUMBER) >= second) as i32;
    let r = original();
    let mut ok = r == 1 && read_u8(game + fixture::STAGE_INDEX) == mark && this.comp_stats() == block;
    if mark == 0xff {
        ok = ok && this.current_stage() as i32 >= stage && read_usize(this.stages() + 4 * stage as usize) != 0;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}, {:#x}): returned {}, mark {:#x} -> {:#x}, +0x30 = {}, game stage {}", name, this.addr(), game, r, mark, read_u8(game + fixture::STAGE_INDEX), this.current_stage() as i32, stage));
    r
}

/// Each game result of the Minas Gerais state league: the game goes into the overall table and, if it has no stage
/// yet, gets its half's (the first up to round 10, the second from round 11); the half is built when its first game
/// arrives (`bra_reg_minas_gerais_create_half_season_stage`, 0x4351b0), and the game also goes into the half table, without
/// counting again in the league statistics; each half winner goes to the final (`bra_reg_minas_gerais_add_two_half_result`).
/// Original: `BraRegMinasGerais::add_result` (0x435130, bra_reg_minas_gerais.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_add_result(this: usize, game: usize, stats: u32) -> u32 {
    bra_reg_minas_gerais_add_two_half_result(League(this), game, stats, 0xb, 0x4351b0, "cqu_3 minas gerais half") // cqu3: the second-half round of the Minas Gerais league
}

/// Each game result of the Paraná state league: the game goes into the overall table and, if it has no stage yet, gets
/// its half's (the first up to round 10, the second from round 11); the half is built when its first game arrives
/// (BraRegParana::create_half_stage, `create_half_stage`, 0x438760), and the game also goes into the half table, without counting
/// again in the league statistics; each half winner goes to the final (`bra_reg_minas_gerais_add_two_half_result`).
/// Original: `BraRegParana::add_result` (0x4386e0, bra_reg_parana.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_add_result(this: usize, game: usize, stats: u32) -> u32 {
    bra_reg_minas_gerais_add_two_half_result(League(this), game, stats, 0xb, 0x438760, "cqu_3 parana half") // cqu3: the Paraná league
}

/// Each game result of the Rio state league: the game goes into the overall table and, if it has no stage yet, gets its
/// half's (the first up to round 10, the second from round 11); the half is built when its first game arrives
/// (`bra_reg_rio_create_half_season_stage`, 0x43afa0), and the game also goes into the half table, without counting again
/// in the league statistics; each half winner goes to the final (`bra_reg_minas_gerais_add_two_half_result`).
/// Original: `BraRegRio::add_result` (0x43af20, bra_reg_rio.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_add_result(this: usize, game: usize, stats: u32) -> u32 {
    bra_reg_minas_gerais_add_two_half_result(League(this), game, stats, 0xb, 0x43afa0, "cqu_3 rio half") // cqu3: the Rio league
}

/// Each game result of J1: the game goes into the overall table and, if it has no stage yet, gets its half's (the first
/// up to round 14, the second from round 15); the half is built when its first game arrives
/// (`jap_j1_create_half_season_stage`, 0x663c20), and the game also goes into the half table, without counting again in
/// the league statistics; each half winner goes to the final (`bra_reg_minas_gerais_add_two_half_result`).
/// Original: `JapJ1::add_result` (0x663940, jap_j1.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn jap_j1_add_result(this: usize, game: usize, stats: u32) -> u32 {
    bra_reg_minas_gerais_add_two_half_result(League(this), game, stats, 0xf, 0x663c20, "cqu_3 j1 half") // cqu3: the second-half round of J1
}

/// Comparer for `bra_reg_minas_gerais_add_result`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_minas_gerais_add_result(this: usize, game: usize, stats: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_BRA_REG_MINAS_GERAIS_ADD_RESULT);
    bra_reg_minas_gerais_compare_two_half_result("bra_reg_minas_gerais_add_result", League(this), game, 0xb, || o(this, game, stats))
}

/// Comparer for `bra_reg_parana_add_result`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_parana_add_result(this: usize, game: usize, stats: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_BRA_REG_PARANA_ADD_RESULT);
    bra_reg_minas_gerais_compare_two_half_result("bra_reg_parana_add_result", League(this), game, 0xb, || o(this, game, stats))
}

/// Comparer for `bra_reg_rio_add_result`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_rio_add_result(this: usize, game: usize, stats: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_BRA_REG_RIO_ADD_RESULT);
    bra_reg_minas_gerais_compare_two_half_result("bra_reg_rio_add_result", League(this), game, 0xb, || o(this, game, stats))
}

/// Comparer for `jap_j1_add_result`.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_add_result(this: usize, game: usize, stats: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_JAP_J1_ADD_RESULT);
    bra_reg_minas_gerais_compare_two_half_result("jap_j1_add_result", League(this), game, 0xf, || o(this, game, stats))
}

// ---------- batch 285: the J2 schedule (the dates and the games of each round of the round-robin stage) (jap_j2.cpp) ----------

/// J2, the league: 44 rounds (from 12 February to 26 October), 2860 bytes, 88 operations.
pub(crate) const J2_LEAGUE_ROUNDS: [AsiaClubChampScheduleOp; 88] = [
    Cqp11D(0, 0xc, 2, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x13, 2, 0, 6, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0x1a, 2, 0, 6, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 2, 3, 0, 6, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 9, 3, 0, 6, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0x10, 3, 0, 6, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 0x17, 3, 0, 6, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x1b, 3, 0, 3, 2, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 0x1f, 3, 0, 6, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 3, 4, 0, 2, 2, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 7, 4, 0, 6, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0xe, 4, 0, 6, 1, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 0x15, 4, 0, 6, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 0x1b, 4, 0, 5, 1, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0), Cqp11D(0xe, 0x19, 5, 0, 6, 1, 0), Cqp11L(0xe, 0, -1, -1, -1, 0),
    Cqp11D(0xf, 2, 6, 0, 6, 1, 0), Cqp11L(0xf, 0, -1, -1, -1, 0), Cqp11D(0x10, 9, 6, 0, 6, 1, 0),
    Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 0xf, 6, 0, 5, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
    Cqp11D(0x12, 0x1e, 6, 0, 6, 1, 0), Cqp11L(0x12, 0, -1, -1, -1, 0), Cqp11D(0x13, 2, 7, 0, 2, 2, 0),
    Cqp11L(0x13, 0, -1, -1, -1, 0), Cqp11D(0x14, 6, 7, 0, 6, 1, 0), Cqp11L(0x14, 0, -1, -1, -1, 0),
    Cqp11D(0x15, 0xd, 7, 0, 6, 1, 0), Cqp11L(0x15, 0, -1, -1, -1, 0), Cqp11D(0x16, 0x14, 7, 0, 6, 1, 0),
    Cqp11L(0x16, 0, -1, -1, -1, 0), Cqp11D(0x17, 0x17, 7, 0, 2, 2, 0), Cqp11L(0x17, 0, -1, -1, -1, 0),
    Cqp11D(0x18, 0x1b, 7, 0, 6, 1, 0), Cqp11L(0x18, 0, -1, -1, -1, 0), Cqp11D(0x19, 3, 8, 0, 6, 1, 0),
    Cqp11L(0x19, 0, -1, -1, -1, 0), Cqp11D(0x1a, 6, 8, 0, 2, 2, 0), Cqp11L(0x1a, 0, -1, -1, -1, 0),
    Cqp11D(0x1b, 0xa, 8, 0, 6, 1, 0), Cqp11L(0x1b, 0, -1, -1, -1, 0), Cqp11D(0x1c, 0xd, 8, 0, 2, 2, 0),
    Cqp11L(0x1c, 0, -1, -1, -1, 0), Cqp11D(0x1d, 0x11, 8, 0, 6, 1, 0), Cqp11L(0x1d, 0, -1, -1, -1, 0),
    Cqp11D(0x1e, 0x18, 8, 0, 6, 1, 0), Cqp11L(0x1e, 0, -1, -1, -1, 0), Cqp11D(0x1f, 1, 9, 0, 6, 1, 0),
    Cqp11L(0x1f, 0, -1, -1, -1, 0), Cqp11D(0x20, 4, 9, 0, 2, 2, 0), Cqp11L(0x20, 0, -1, -1, -1, 0),
    Cqp11D(0x21, 7, 9, 0, 5, 1, 0), Cqp11L(0x21, 0, -1, -1, -1, 0), Cqp11D(0x22, 0xb, 9, 0, 2, 2, 0),
    Cqp11L(0x22, 0, -1, -1, -1, 0), Cqp11D(0x23, 0xf, 9, 0, 6, 1, 0), Cqp11L(0x23, 0, -1, -1, -1, 0),
    Cqp11D(0x24, 0x16, 9, 0, 6, 1, 0), Cqp11L(0x24, 0, -1, -1, -1, 0), Cqp11D(0x25, 0x1d, 9, 0, 6, 1, 0),
    Cqp11L(0x25, 0, -1, -1, -1, 0), Cqp11D(0x26, 1, 0xa, 0, 2, 2, 0), Cqp11L(0x26, 0, -1, -1, -1, 0),
    Cqp11D(0x27, 5, 0xa, 0, 6, 1, 0), Cqp11L(0x27, 0, -1, -1, -1, 0), Cqp11D(0x28, 8, 0xa, 0, 2, 2, 0),
    Cqp11L(0x28, 0, -1, -1, -1, 0), Cqp11D(0x29, 0xc, 0xa, 0, 6, 1, 0), Cqp11L(0x29, 0, -1, -1, -1, 0),
    Cqp11D(0x2a, 0x13, 0xa, 0, 6, 1, 0), Cqp11L(0x2a, 0, -1, -1, -1, 0), Cqp11D(0x2b, 0x1a, 0xa, 0, 6, 1, 0),
    Cqp11L(0x2b, 0, -1, -1, -1, 0),
];

/// The stages of the J2 round table: the round-robin stage (−1) (a league): 44 rounds, from 12 February to 26 October;
/// every other stage has no table (`None`: returns 0 without writing anything).
pub(crate) fn jap_j2_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 44, extra: 0, extra_check: false, size: 0xb2c, row: 0x9d,
            ops: &J2_LEAGUE_ROUNDS, name: "cqu_4 j2 league" }),
        _ => None,
    }
}

/// The J2 schedule: the round-robin stage (−1) (a league): 44 rounds, from 12 February to 26 October; each round has
/// the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left
/// as allocated. Virtual `comp_vtable::CREATE_SCHEDULE` (`create_schedule(stage, &rounds, &extra, &is_league)`): the
/// stage of `jap_j2_schedule_stage` through the cup schedule machine (`create_round_table`: the outputs, the table of
/// 0x41 bytes per round from `exe::crt_malloc`, the error box of line 0x9d of jap_j2.cpp without memory, and the dates
/// and games of each round; the league's year at `league::YEAR` is read as `cup::YEAR`, the same word +0x40).
/// Original: `JapJ2::create_schedule` (0x664780, jap_j2.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn jap_j2_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), jap_j2_schedule_stage(stage), rounds, extra, league, 0x9eaedc)
}

/// Comparer for `jap_j2_create_schedule`: side by side (`asia_club_champ_compare_round_table`).
pub(crate) unsafe extern "thiscall" fn compare_jap_j2_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_JAP_J2_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("jap_j2_create_schedule", stage, jap_j2_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| jap_j2_create_schedule(this, stage, r, e, l))
}

// ---------- batch 285: the English cup constructors: the FA Cup, the League Cup and the Charity Shield (eng_fa_cup.cpp, eng_cc_cup.cpp, eng_charity.cpp) ----------

/// Builds the cup's round table through its own virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(−1,
/// &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, null): push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this ->
/// thiscall, ret 0x10) and stores it in `cup::ROUND_TABLE`.
pub(crate) unsafe fn eng_fa_cup_create_rounds(this: Cup) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table(t);
}

/// Creates the cup's statistics at Comp +8 (operator new `exe::crt_operator_new` of 0x5ce, cdecl, and
/// Comp_Stats::Comp_Stats `exe::new_comp_stats`: push this; mov ecx, new -> thiscall, ret 4; the original's SEH frame
/// covers only that new; 0 without memory). Returns whether they were created.
pub(crate) unsafe fn eng_fa_cup_create_comp_stats(this: Cup) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this.addr() + 8, s); // Comp +8: the Comp_Stats object
    s != 0
}

/// The plan the comparer checks after the original constructor: the return value (this), the vtable, the competition
/// record (Comp +4) and `cup::YEAR`; on a new cup (`cup::STAGE_INDEX` = `stage_index::MAIN`), every fixed field the
/// constructor writes (offset, width, value) and the round table and statistics created. `None` = as planned.
pub(crate) unsafe fn eng_fa_cup_check_constructor_plan(this: Cup, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this.addr() || read_usize(this.addr()) != vtable || read_usize(this.addr() + 4) != record || read_u16(this.addr() + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this.addr()), read_usize(this.addr() + 4), read_u16(this.addr() + cup::YEAR)));
    }
    if this.stage_index() != stage_index::MAIN {
        return None;
    }
    for &(o, w, v) in fields {
        let x = match w {
            1 => read_u8(this.addr() + o) as u32,
            2 => read_u16(this.addr() + o) as u32,
            _ => read_u32(this.addr() + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && this.round_table() == 0) || read_usize(this.addr() + 8) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", this.round_table(), read_usize(this.addr() + 8)));
    }
    None
}

/// The body of `construct_eng_fa_cup` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_eng_fa_cup_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x969b28); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_stage_count(0);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_club_sides(1);
    this.set_discipline_index(discipline_rule::ENGLAND);
    this.set_run_by_stages(0);
    this.set_season_set_up(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_5 fa cup new");
    write_u8(this.addr() + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 0);
    write_u8(this.addr() + eng_fa_cup::FIFTH_ROUND_ENTRY, 1);
    write_u8(this.addr() + eng_fa_cup::EXEMPTION_COUNT, 0);
    this.set_team_count(0x98);
    let v = exe::crt_calloc(0x98, 6);
    this.set_team_table(v);
    if v == 0 {
        show_error_at(0x9c42cc, 0x63);
        return this.addr();
    }
    this.set_prize_fund(0x1e8480);
    eng_fa_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0x9c42cc, 0x71);
        return this.addr();
    }
    if !eng_fa_cup_create_comp_stats(this) {
        show_error_at(0x9c42cc, 0x79);
        return this.addr();
    }
    this.addr()
}

/// The fixed fields `construct_eng_fa_cup` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const FA_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 13] = [(cup::PRIZE_FUND, 4, 0x1e8480), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::TEAM_COUNT, 2, 0x98), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SEASON_SET_UP, 4, 0x0), (cup::DISCIPLINE_INDEX, 1, 0x9), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0), (eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 1, 0x0), (eng_fa_cup::EXEMPTION_COUNT, 1, 0x0), (eng_fa_cup::FIFTH_ROUND_ENTRY, 1, 0x1)];

/// The constructor of the FA Cup: with a table of 152 entrants (6-byte records, zeroed); type 1 (clubs of one nation),
/// 152 teams, prize fund of 2,000,000, bench and substitutions from the base, discipline rule 9; on a new cup, in
/// order: the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the tie tree
/// (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error box of line 0x71 if it returns a
/// negative value); the statistics at Comp +8 (the error box of line 0x79 without them). Over the base Cup::Cup
/// (`exe::new_cup`, thiscall, ret 0) it writes the fixed fields in the original's order (the vtable 0x969b28, the
/// competition record at Comp +4, `cup::YEAR` and those of `FA_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads
/// (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step
/// that fails shows its own error box (`show_error_at`) and returns this. Returns this.
/// Original: `EngFaCup::EngFaCup` (0x56f040, eng_fa_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_eng_fa_cup(this: usize, year: u32, record: usize) -> usize {
    construct_eng_fa_cup_body(Cup(this), year, record) // cqu5: the FA Cup
}

/// Comparer for `construct_eng_fa_cup`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`eng_fa_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_eng_fa_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_FA_CUP)(this, year, record);
    let error = eng_fa_cup_check_constructor_plan(Cup(this), r, year, record, 0x969b28, &FA_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_eng_fa_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of `construct_eng_cc_cup` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_eng_cc_cup_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x969934); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_stage_count(0);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_club_sides(1);
    this.set_discipline_index(discipline_rule::ENGLAND);
    this.set_run_by_stages(0);
    this.set_season_set_up(0);
    write_u8(this.addr() + eng_cc_cup::EUROPEAN_ENTRANT_COUNT, 0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_5 league cup new");
    this.set_team_count(0x5c);
    let v = exe::crt_calloc(0x5c, 6);
    this.set_team_table(v);
    if v == 0 {
        show_error_at(0x9c4224, 0x50);
        return this.addr();
    }
    this.set_prize_fund(0xf4240);
    eng_fa_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0x9c4224, 0x5e);
        return this.addr();
    }
    if !eng_fa_cup_create_comp_stats(this) {
        show_error_at(0x9c4224, 0x65);
        return this.addr();
    }
    this.addr()
}

/// The fixed fields `construct_eng_cc_cup` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const LEAGUE_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 11] = [(cup::PRIZE_FUND, 4, 0xf4240), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::TEAM_COUNT, 2, 0x5c), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SEASON_SET_UP, 4, 0x0), (cup::DISCIPLINE_INDEX, 1, 0x9), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0), (eng_cc_cup::EUROPEAN_ENTRANT_COUNT, 1, 0x0)];

/// The constructor of the English League Cup: with a table of 92 entrants (6-byte records, zeroed); type 1 (clubs of
/// one nation), 92 teams, prize fund of 1,000,000, bench and substitutions from the base, discipline rule 9; on a new
/// cup, in order: the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the tie
/// tree (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error box of line 0x5e if it returns
/// a negative value); the statistics at Comp +8 (the error box of line 0x65 without them). Over the base Cup::Cup
/// (`exe::new_cup`, thiscall, ret 0) it writes the fixed fields in the original's order (the vtable 0x969934, the
/// competition record at Comp +4, `cup::YEAR` and those of `LEAGUE_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads
/// (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step
/// that fails shows its own error box (`show_error_at`) and returns this. Returns this.
/// Original: `EngCcCup::EngCcCup` (0x56c2a0, eng_cc_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_eng_cc_cup(this: usize, year: u32, record: usize) -> usize {
    construct_eng_cc_cup_body(Cup(this), year, record) // cqu5: the League Cup
}

/// Comparer for `construct_eng_cc_cup`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`eng_fa_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_eng_cc_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_CC_CUP)(this, year, record);
    let error = eng_fa_cup_check_constructor_plan(Cup(this), r, year, record, 0x969934, &LEAGUE_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_eng_cc_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of `construct_eng_charity` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_eng_charity_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x9699d4); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_stage_count(0);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_club_sides(1);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(5);
    this.set_discipline_index(discipline_rule::ENGLAND);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_5 charity shield new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56d780usize); // EngCharity::select_teams
    let r = select_teams(this.addr());
    if r < 0 {
        show_error_at(0x9c425c, 0x43);
        return this.addr();
    }
    this.set_prize_fund(0x493e0);
    eng_fa_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0x9c425c, 0x51);
        return this.addr();
    }
    if !eng_fa_cup_create_comp_stats(this) {
        show_error_at(0x9c425c, 0x58);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields `construct_eng_charity` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const CHARITY_SHIELD_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::PRIZE_FUND, 4, 0x493e0), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x5), (cup::DISCIPLINE_INDEX, 1, 0x9), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The constructor of the Charity Shield: with no entrant table of its own; type 1 (clubs of one nation), prize fund of
/// 300,000, 7 substitutes on the bench and 5 substitutions, discipline rule 9; on a new cup, in order: the two clubs
/// (EngCharity::select_teams, `eng_charity_select_teams`, 0x56d780, thiscall, ret 0; the error box of line 0x43 if it
/// returns a negative value); the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into
/// `cup::ROUND_TABLE`); the tie tree (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error
/// box of line 0x51 if it returns a negative value); the statistics at Comp +8 (the error box of line 0x58 without
/// them); the final ranking (Cup::rank_teams, `cup_rank_teams`, 0x5223a0, thiscall, ret 0, unchecked). Over the base
/// Cup::Cup (`exe::new_cup`, thiscall, ret 0) it writes the fixed fields in the original's order (the vtable 0x9699d4,
/// the competition record at Comp +4, `cup::YEAR` and those of `CHARITY_SHIELD_CONSTRUCTOR_FIELDS`); if the saved state
/// loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each
/// step that fails shows its own error box (`show_error_at`) and returns this. Returns this.
/// Original: `EngCharity::EngCharity` (0x56d380, eng_charity.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_eng_charity(this: usize, year: u32, record: usize) -> usize {
    construct_eng_charity_body(Cup(this), year, record) // cqu5: the Charity Shield
}

/// Comparer for `construct_eng_charity`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`eng_fa_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_eng_charity(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_CHARITY)(this, year, record);
    let error = eng_fa_cup_check_constructor_plan(Cup(this), r, year, record, 0x9699d4, &CHARITY_SHIELD_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_eng_charity({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

// ---------- batch 285: the Copa del Rey and the Spanish Super Cup: the two constructors and the 83 Copa del Rey entrants (spa_cup.cpp, spa_super.cpp) ----------

/// Builds the cup's round table through its own virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(−1,
/// &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, null): push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this ->
/// thiscall, ret 0x10) and stores it in `cup::ROUND_TABLE`.
pub(crate) unsafe fn spa_cup_create_rounds(this: Cup) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table(t);
}

/// Creates the cup's statistics at Comp +8 (operator new `exe::crt_operator_new` of 0x5ce, cdecl, and
/// Comp_Stats::Comp_Stats `exe::new_comp_stats`: push this; mov ecx, new -> thiscall, ret 4; the original's SEH frame
/// covers only that new; 0 without memory). Returns whether they were created.
pub(crate) unsafe fn spa_cup_create_comp_stats(this: Cup) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this.addr() + 8, s); // Comp +8: the Comp_Stats object
    s != 0
}

/// The plan the comparer checks after the original constructor: the return value (this), the vtable, the competition
/// record (Comp +4) and `cup::YEAR`; on a new cup (`cup::STAGE_INDEX` = `stage_index::MAIN`), every fixed field the
/// constructor writes (offset, width, value) and the round table and statistics created. `None` = as planned.
pub(crate) unsafe fn spa_cup_check_constructor_plan(this: Cup, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this.addr() || read_usize(this.addr()) != vtable || read_usize(this.addr() + 4) != record || read_u16(this.addr() + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this.addr()), read_usize(this.addr() + 4), read_u16(this.addr() + cup::YEAR)));
    }
    if this.stage_index() != stage_index::MAIN {
        return None;
    }
    for &(o, w, v) in fields {
        let x = match w {
            1 => read_u8(this.addr() + o) as u32,
            2 => read_u16(this.addr() + o) as u32,
            _ => read_u32(this.addr() + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && this.round_table() == 0) || read_usize(this.addr() + 8) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", this.round_table(), read_usize(this.addr() + 8)));
    }
    None
}

/// The body of `construct_spa_cup` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_spa_cup_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96f9c8); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_stage_count(0);
    this.set_club_sides(1);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(discipline_rule::SPAIN_CUP);
    this.set_run_by_stages(0);
    this.set_season_set_up(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_6 copa del rey new");
    this.set_team_count(0x53);
    let v = exe::crt_calloc(0x53, 6);
    this.set_team_table(v);
    if v == 0 {
        show_error_at(0xa82810, 0x52);
        return this.addr();
    }
    this.set_prize_fund(0xf4240);
    spa_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r == 0 {
        show_error_at(0xa82810, 0x60);
        return this.addr();
    }
    if !spa_cup_create_comp_stats(this) {
        show_error_at(0xa82810, 0x67);
        return this.addr();
    }
    this.addr()
}

/// The fixed fields `construct_spa_cup` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const COPA_DEL_REY_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 12] = [(cup::PRIZE_FUND, 4, 0xf4240), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::TEAM_COUNT, 2, 0x53), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SEASON_SET_UP, 4, 0x0), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1b), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The constructor of the Copa del Rey: with a table of 83 entrants (6-byte records, zeroed); type 1 (clubs of one
/// nation), 83 teams, prize fund of 1,000,000, 7 substitutes on the bench and 3 substitutions, discipline rule 27; on a
/// new cup, in order: the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the
/// tie tree (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error box of line 0x60 if it
/// returns 0); the statistics at Comp +8 (the error box of line 0x67 without them). Over the base Cup::Cup
/// (`exe::new_cup`, thiscall, ret 0) it writes the fixed fields in the original's order (the vtable 0x96f9c8, the
/// competition record at Comp +4, `cup::YEAR` and those of `COPA_DEL_REY_CONSTRUCTOR_FIELDS`); if the saved state loads
/// (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step
/// that fails shows its own error box (`show_error_at`) and returns this. Returns this.
/// Original: `SpaCup::SpaCup` (0x84cf50, spa_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_spa_cup(this: usize, year: u32, record: usize) -> usize {
    construct_spa_cup_body(Cup(this), year, record) // cqu6: the Copa del Rey
}

/// Comparer for `construct_spa_cup`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`spa_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_spa_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SPA_CUP)(this, year, record);
    let error = spa_cup_check_constructor_plan(Cup(this), r, year, record, 0x96f9c8, &COPA_DEL_REY_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_spa_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of `construct_spa_super` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_spa_super_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96fd38); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_stage_count(0);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_club_sides(1);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(discipline_rule::SPAIN_LEAGUE);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_6 spanish super cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x858aa0usize); // SpaSuper::select_teams
    let r = select_teams(this.addr());
    if r < 0 {
        show_error_at(0xa82a58, 0x43);
        return this.addr();
    }
    spa_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0xa82a58, 0x51);
        return this.addr();
    }
    if !spa_cup_create_comp_stats(this) {
        show_error_at(0xa82a58, 0x58);
        return this.addr();
    }
    this.addr()
}

/// The fixed fields `construct_spa_super` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const SPANISH_SUPER_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 9] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1a), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The constructor of the Spanish Super Cup: with no entrant table of its own; type 1 (clubs of one nation), no prize
/// fund (`cup::PRIZE_FUND` stays as the base left it), 7 substitutes on the bench and 3 substitutions, discipline rule
/// 26; on a new cup, in order: the two clubs (SpaSuper::select_teams, `spa_super_select_teams`, 0x858aa0, thiscall, ret
/// 0; the error box of line 0x43 if it returns a negative value); the rounds (the cup's own virtual
/// `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the tie tree (Cup::map_fixture_tree, `map_fixture_tree`,
/// 0x518790, thiscall, ret 0; the error box of line 0x51 if it returns a negative value); the statistics at Comp +8
/// (the error box of line 0x58 without them). Over the base Cup::Cup (`exe::new_cup`, thiscall, ret 0) it writes the
/// fixed fields in the original's order (the vtable 0x96fd38, the competition record at Comp +4, `cup::YEAR` and those
/// of `SPANISH_SUPER_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx,
/// this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step that fails shows its own error box
/// (`show_error_at`) and returns this. Returns this.
/// Original: `SpaSuper::SpaSuper` (0x8586b0, spa_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_spa_super(this: usize, year: u32, record: usize) -> usize {
    construct_spa_super_body(Cup(this), year, record) // cqu6: the Spanish Super Cup
}

/// Comparer for `construct_spa_super`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`spa_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_spa_super(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SPA_SUPER)(this, year, record);
    let error = spa_cup_check_constructor_plan(Cup(this), r, year, record, 0x96fd38, &SPANISH_SUPER_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_spa_super({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// Whether club `c` is left out of the Copa del Rey by its link to a B team (club_get_linked_b_team
/// `exe::club_get_linked_b_team`: push 1; push &senior; push club; add esp, 0xc -> cdecl): it is when there is a link
/// and the output says it is not the senior side (0).
pub(crate) unsafe fn spa_cup_is_linked_b_team(c: Club) -> bool {
    let mut senior = 0u32;
    let r = exe::club_get_linked_b_team(c, &mut senior, 1);
    r != 0 && senior == 0
}

/// Whether the club's division (`club::LAST_DIVISION`, the competition record; +0 the id) is the one of key `key`.
pub(crate) unsafe fn spa_cup_in_division(c: Club, key: usize) -> bool {
    let l = c.last_division();
    !l.is_null() && read_usize(l.addr()) == read_usize(key)
}

/// Whether the club, with a nation (`club::NATION`) and a division (`club::LAST_DIVISION`), is in the lower Spanish
/// divisions (`KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B` and the four groups `KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1`
/// to `_B4`: the Segunda B and the Tercera [inference]).
pub(crate) unsafe fn spa_cup_is_lower_division_club(c: Club) -> bool {
    if c.nation().is_null() || c.last_division().is_null() {
        return false;
    }
    [KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B2,
        KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B3, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B4].iter().any(|&g| spa_cup_in_division(c, g))
}

/// Writes club `c` into slot `k` of the entrant table (`cup::TEAM_TABLE`, 6-byte `cup_team_entry` records: the club,
/// the seed level and the status zeroed).
pub(crate) unsafe fn spa_cup_put_entrant(this: Cup, k: i32, c: usize) {
    let t = this.team_table() + k as usize * 6;
    write_usize(t + cup_team_entry::CLUB, c);
    write_u8(t + cup_team_entry::SEED_LEVEL, 0);
    write_u8(t + cup_team_entry::STATUS, 0);
}

/// Copies the `n` clubs of `list` into the slots from `from` on.
pub(crate) unsafe fn spa_cup_put_block(this: Cup, from: i32, list: &[usize], n: i32) {
    for j in 0..n.max(0) {
        spa_cup_put_entrant(this, from + j, list[j as usize]);
    }
}

/// The body of `spa_cup_collect_entrants` (the comparer runs it on its own copy of the table).
pub(crate) unsafe fn spa_cup_collect_entrants_body(this: Cup) -> i32 {
    let base = read_usize(G_CLUB_TABLE);
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut continental = [0usize; 20];
    let mut first = [0usize; 20];
    let mut second = [0usize; 22];
    let (mut nc, mut nf, mut ns) = (0i16, 0i16, 0i16);
    let mut i = 0i32;
    while i < limit() {
        let c = Club(base + i as usize * club::SIZE);
        let nation = c.nation();
        if !nation.is_null() && read_usize(nation.addr()) == read_usize(KEY_NATION_SPAIN) && !spa_cup_is_linked_b_team(c) {
            let cont = c.continental_comp_id() as i32;
            if cont != -1 && cont != read_usize(KEY_CLUB_COMP_INTER_TOTO_CUP) as i32 && nc < 0x14 && nc < 7 {
                continental[nc as usize] = c.addr();
                nc += 1;
            } else if !c.last_division().is_null() {
                if spa_cup_in_division(c, KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) && nf < 0x14 {
                    first[nf as usize] = c.addr();
                    nf += 1;
                } else if spa_cup_in_division(c, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION) && ns < 0x16 {
                    covered!("cqu_6 second division");
                    second[ns as usize] = c.addr();
                    ns += 1;
                }
            }
        }
        i += 1;
    }
    let missing = 0x53i32 - ns as i32 - nf as i32 - nc as i32;
    this.set_team_count(0x53);
    let t = exe::crt_calloc(0x53, 6);
    this.set_team_table(t);
    if t == 0 {
        show_error_at(0xa82810, 0x1e0);
        return -1;
    }
    let missing_w = missing as i16 as i32;
    let mut k = 0i32;
    let nation = Competition(read_usize(this.addr() + 4)).nation();
    if nation.league_selected() & 4 != 0 {
        let list = exe::crt_malloc(0x2d0);
        if list == 0 {
            show_error_at(0xa82810, 0x1ee);
            return 0;
        }
        let mut n = 0i32;
        let mut i = 0i32;
        while i < limit() {
            if n >= 0x78 {
                break;
            }
            let c = Club(base + i as usize * club::SIZE);
            if spa_cup_is_lower_division_club(c) && !spa_cup_is_linked_b_team(c) && c.continental_comp_id() as i32 == -1 {
                write_usize(list + n as usize * 6, c.addr());
                write_u8(list + n as usize * 6 + 4, c.last_position()); // the sort key
                n += 1;
            }
            i += 1;
        }
        exe::crt_qsort(list, n as u32, 6, 0x92f660usize);
        let mut j = 0i32;
        while j < missing_w && j < n {
            k += 1;
            spa_cup_put_entrant(this, missing_w - 1 - j, read_usize(list + j as usize * 6));
            j += 1;
        }
        exe::crt_free(list);
    } else {
        let mut i = 0i32;
        while i < limit() {
            let c = Club(base + i as usize * club::SIZE);
            if spa_cup_is_lower_division_club(c) && !spa_cup_is_linked_b_team(c) && c.continental_comp_id() as i32 == -1 {
                k += 1;
                spa_cup_put_entrant(this, missing_w - k, c.addr());
                if k >= missing_w {
                    break;
                }
            }
            i += 1;
        }
    }
    if missing_w - k > 0 && limit() > 0 {
        covered!("cqu_6 fill with clubs without division");
        let mut i = 0i32;
        while i < limit() {
            let c = Club(base + i as usize * club::SIZE);
            let nation = c.nation();
            if !nation.is_null() && read_usize(nation.addr()) == read_usize(KEY_NATION_SPAIN) && !spa_cup_is_linked_b_team(c) && c.continental_comp_id() as i32 == -1
                && (c.last_division().is_null() || spa_cup_in_division(c, KEY_CLUB_COMP_A_LOWER_DIVISION) || spa_cup_in_division(c, KEY_CLUB_COMP_SPANISH_LOWER_DIVISION)) {
                k += 1;
                spa_cup_put_entrant(this, missing_w - k, c.addr());
                if k >= missing_w {
                    break;
                }
            }
            i += 1;
        }
    }
    let (ns, nf, nc) = (ns as i32, nf as i32, nc as i32);
    spa_cup_put_block(this, missing_w, &second, ns); spa_cup_put_block(this, missing_w + ns.max(0), &first, nf); // cqu6: the Segunda and then the Primera
    spa_cup_put_block(this, missing_w + ns.max(0) + nf.max(0), &continental, nc);
    1
}

/// The 83 Copa del Rey entrants (called by the Copa del Rey day routine, 0x84d740, once up to 2002-06-01). In club
/// table order (`G_CLUB_TABLE`, the first `G_CLUB_COUNT` − 2 × `G_NATION_COUNT`, the limit re-read on every pass), the
/// Spanish clubs (`club::NATION` = `KEY_NATION_SPAIN`) that are not a linked B team (`spa_cup_is_linked_b_team`) go to
/// three groups: those in a continental competition (`club::CONTINENTAL_COMP_ID` ≠ −1 and ≠
/// `KEY_CLUB_COMP_INTER_TOTO_CUP`), up to 7; otherwise those of the Primera (`club::LAST_DIVISION` =
/// `KEY_CLUB_COMP_SPANISH_FIRST_DIVISION`, up to 20) and of the Segunda (`KEY_CLUB_COMP_SPANISH_SECOND_DIVISION`, up to
/// 22). Missing = 83 − the three groups; `cup::TEAM_COUNT` = 83 and the table (calloc `exe::crt_calloc` of 83 × 6:
/// push 6; push 0x53; add esp, 8 -> cdecl) in `cup::TEAM_TABLE` (without memory: the error box of line 0x1e0 of
/// spa_cup.cpp and −1). Slots 0 to missing − 1 go to the lower clubs (`spa_cup_is_lower_division_club`, not a B team,
/// `club::CONTINENTAL_COMP_ID` = −1), from the last to the first: if the cup's nation (the competition record at Comp
/// +4, `competition::NATION`) has bit 4 in `nation::LEAGUE_SELECTED`, up to 120 of them in a list (malloc
/// `exe::crt_malloc` of 0x2d0; without memory: the error box of line 0x1ee and 0) with `club::LAST_POSITION` as the
/// key, sorted (qsort `exe::crt_qsort` with 0x92f660: push 0x92f660; push 6; push n; push list; add esp, 0x10 ->
/// cdecl; the list freed with `exe::crt_free`), and the first of them; otherwise in table order. If some are still
/// missing, the Spanish clubs with no division or in the generic divisions `KEY_CLUB_COMP_A_LOWER_DIVISION` or
/// `KEY_CLUB_COMP_SPANISH_LOWER_DIVISION` (not a B team, `club::CONTINENTAL_COMP_ID` = −1). After the lower slots come
/// the Segunda, the Primera and the continental clubs, in that order (the cup rounds make the top ones enter later).
/// Returns 1.
/// Original: `SpaCup::collect_entrants` (0x84d910, spa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_cup_collect_entrants(this: usize) -> i32 {
    spa_cup_collect_entrants_body(Cup(this))
}

/// Comparer for `spa_cup_collect_entrants`: side by side from the same state (`cup::TEAM_COUNT`; it does not draw): the
/// original leaves its table in `cup::TEAM_TABLE`, the copy makes its own, compared over the 83 slots (6 bytes each)
/// and freed; the return value too.
pub(crate) unsafe extern "thiscall" fn compare_spa_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SPA_CUP_COLLECT_ENTRANTS);
    let before = this.team_table();
    let table_original = core::cell::Cell::new(0usize);
    let table_rust = core::cell::Cell::new(0usize);
    let (a, b, same, _, _) = crate::comparison::side_by_side(&[(this.addr() + cup::TEAM_COUNT, 2usize)], || {
        let r = o(this.addr());
        table_original.set(this.team_table());
        this.set_team_table(before);
        r
    }, || {
        let r = spa_cup_collect_entrants_body(this);
        table_rust.set(this.team_table());
        this.set_team_table(table_original.get());
        r
    });
    let (lo, lr) = (table_original.get(), table_rust.get());
    let table = lo == 0 || lr == 0 || core::slice::from_raw_parts(lo as *const u8, 0x53 * 6) == core::slice::from_raw_parts(lr as *const u8, 0x53 * 6);
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(same && a == b && table, || format!("spa_cup_collect_entrants({:#x}) returned {} x {} memory {} table {}", this.addr(), a, b, same, table));
    a
}

// ---------- batch 285: the French cup constructors: the Coupe de France, the League Cup and the Super Cup (fra_cup.cpp, fra_lge_cup.cpp, fra_super.cpp) ----------

/// Builds the cup's round table through its own virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(−1,
/// &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, null): push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this ->
/// thiscall, ret 0x10) and stores it in `cup::ROUND_TABLE`.
pub(crate) unsafe fn fra_cup_create_rounds(this: Cup) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table(t);
}

/// Creates the cup's statistics at Comp +8 (operator new `exe::crt_operator_new` of 0x5ce, cdecl, and
/// Comp_Stats::Comp_Stats `exe::new_comp_stats`: push this; mov ecx, new -> thiscall, ret 4; the original's SEH frame
/// covers only that new; 0 without memory). Returns whether they were created.
pub(crate) unsafe fn fra_cup_create_comp_stats(this: Cup) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this.addr() + 8, s); // Comp +8: the Comp_Stats object
    s != 0
}

/// The plan the comparer checks after the original constructor: the return value (this), the vtable, the competition
/// record (Comp +4) and `cup::YEAR`; on a new cup (`cup::STAGE_INDEX` = `stage_index::MAIN`), every fixed field the
/// constructor writes (offset, width, value) and the round table and statistics created. `None` = as planned.
pub(crate) unsafe fn fra_cup_check_constructor_plan(this: Cup, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this.addr() || read_usize(this.addr()) != vtable || read_usize(this.addr() + 4) != record || read_u16(this.addr() + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this.addr()), read_usize(this.addr() + 4), read_u16(this.addr() + cup::YEAR)));
    }
    if this.stage_index() != stage_index::MAIN {
        return None;
    }
    for &(o, w, v) in fields {
        let x = match w {
            1 => read_u8(this.addr() + o) as u32,
            2 => read_u16(this.addr() + o) as u32,
            _ => read_u32(this.addr() + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && this.round_table() == 0) || read_usize(this.addr() + 8) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", this.round_table(), read_usize(this.addr() + 8)));
    }
    None
}

/// The body of `construct_fra_cup` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_fra_cup_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_u16(this.addr() + cup::YEAR, year as u16);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96a864); // the vtable
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_stage_count(0);
    this.set_club_sides(1);
    this.set_substitutes_named(3);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(discipline_rule::FRANCE);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_7 coupe de france new");
    let collect_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5baa50usize); // FraCup::collect_entrants
    let r = collect_entrants(this.addr());
    if r == 0 {
        show_error_at(0x9ccb28, 0x4c);
        return this.addr();
    }
    this.set_prize_fund(0xf4240);
    fra_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r == 0 {
        show_error_at(0x9ccb28, 0x5b);
        return this.addr();
    }
    if !fra_cup_create_comp_stats(this) {
        show_error_at(0x9ccb28, 0x62);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields `construct_fra_cup` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const COUPE_DE_FRANCE_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::PRIZE_FUND, 4, 0xf4240), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x3), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0xc), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The constructor of the Coupe de France: with no entrant table of its own; type 1 (clubs of one nation), prize fund
/// of 1,000,000, 3 substitutes on the bench and 3 substitutions, discipline rule 12; on a new cup, in order: the 222
/// entrants (FraCup::collect_entrants, `fra_cup_collect_entrants`, 0x5baa50, thiscall, ret 0; the error box of line
/// 0x4c if it returns 0); the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`);
/// the tie tree (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error box of line 0x5b if it
/// returns 0); the statistics at Comp +8 (the error box of line 0x62 without them); the final ranking (Cup::rank_teams,
/// `cup_rank_teams`, 0x5223a0, thiscall, ret 0, unchecked). Over the base Cup::Cup (`exe::new_cup`, thiscall, ret 0) it
/// writes the fixed fields in the original's order (the vtable 0x96a864, the competition record at Comp +4, `cup::YEAR`
/// and those of `COUPE_DE_FRANCE_CONSTRUCTOR_FIELDS`); if the saved state loads (Cup::load_from_disk 0x51fc00: push 1;
/// mov ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step that fails shows its own error box
/// (`show_error_at`) and returns this. Returns this.
/// Original: `FraCup::FraCup` (0x5ba130, fra_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_fra_cup(this: usize, year: u32, record: usize) -> usize {
    construct_fra_cup_body(Cup(this), year, record) // cqu7: the Coupe de France
}

/// Comparer for `construct_fra_cup`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`fra_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_fra_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_FRA_CUP)(this, year, record);
    let error = fra_cup_check_constructor_plan(Cup(this), r, year, record, 0x96a864, &COUPE_DE_FRANCE_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_fra_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of `construct_fra_lge_cup` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_fra_lge_cup_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_u16(this.addr() + cup::YEAR, year as u16);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96aad8); // the vtable
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_stage_count(0);
    this.set_club_sides(1);
    this.set_substitutes_named(3);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(discipline_rule::FRANCE);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_7 french league cup new");
    let collect_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5bd160usize); // FraLgeCup::collect_entrants
    let r = collect_entrants(this.addr());
    if r == 0 {
        show_error_at(0x9ccb98, 0x49);
        return this.addr();
    }
    this.set_prize_fund(0x7a120);
    fra_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r == 0 {
        show_error_at(0x9ccb98, 0x57);
        return this.addr();
    }
    if !fra_cup_create_comp_stats(this) {
        show_error_at(0x9ccb98, 0x5e);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields `construct_fra_lge_cup` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const FRENCH_LEAGUE_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::PRIZE_FUND, 4, 0x7a120), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x3), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0xc), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The constructor of the French League Cup: with no entrant table of its own; type 1 (clubs of one nation), prize fund
/// of 500,000, 3 substitutes on the bench and 3 substitutions, discipline rule 12; on a new cup, in order: the 54
/// entrants (FraLgeCup::collect_entrants, `fra_lge_cup_collect_entrants`, 0x5bd160, thiscall, ret 0; the error box of
/// line 0x49 if it returns 0); the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into
/// `cup::ROUND_TABLE`); the tie tree (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error
/// box of line 0x57 if it returns 0); the statistics at Comp +8 (the error box of line 0x5e without them); the final
/// ranking (Cup::rank_teams, `cup_rank_teams`, 0x5223a0, thiscall, ret 0, unchecked). Over the base Cup::Cup
/// (`exe::new_cup`, thiscall, ret 0) it writes the fixed fields in the original's order (the vtable 0x96aad8, the
/// competition record at Comp +4, `cup::YEAR` and those of `FRENCH_LEAGUE_CUP_CONSTRUCTOR_FIELDS`); if the saved state
/// loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each
/// step that fails shows its own error box (`show_error_at`) and returns this. Returns this.
/// Original: `FraLgeCup::FraLgeCup` (0x5bc980, fra_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_fra_lge_cup(this: usize, year: u32, record: usize) -> usize {
    construct_fra_lge_cup_body(Cup(this), year, record) // cqu7: the French League Cup
}

/// Comparer for `construct_fra_lge_cup`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`fra_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_fra_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_FRA_LGE_CUP)(this, year, record);
    let error = fra_cup_check_constructor_plan(Cup(this), r, year, record, 0x96aad8, &FRENCH_LEAGUE_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_fra_lge_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of `construct_fra_super` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_fra_super_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96ace0); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_stage_count(0);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_club_sides(1);
    this.set_discipline_index(discipline_rule::FRANCE);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_7 french super cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5bf770usize); // FraSuper::select_teams
    let r = select_teams(this.addr());
    if r < 0 {
        show_error_at(0x9ccc48, 0x3f);
        return this.addr();
    }
    fra_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0x9ccc48, 0x4d);
        return this.addr();
    }
    if !fra_cup_create_comp_stats(this) {
        show_error_at(0x9ccc48, 0x54);
        return this.addr();
    }
    this.addr()
}

/// The fixed fields `construct_fra_super` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const FRENCH_SUPER_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 7] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::DISCIPLINE_INDEX, 1, 0xc), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The constructor of the French Super Cup: with no entrant table of its own; type 1 (clubs of one nation), no prize
/// fund (`cup::PRIZE_FUND` stays as the base left it), bench and substitutions from the base, discipline rule 12; on a
/// new cup, in order: the two clubs (FraSuper::select_teams, `fra_super_select_teams`, 0x5bf770, thiscall, ret 0; the
/// error box of line 0x3f if it returns a negative value); the rounds (the cup's own virtual
/// `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the tie tree (Cup::map_fixture_tree, `map_fixture_tree`,
/// 0x518790, thiscall, ret 0; the error box of line 0x4d if it returns a negative value); the statistics at Comp +8
/// (the error box of line 0x54 without them). Over the base Cup::Cup (`exe::new_cup`, thiscall, ret 0) it writes the
/// fixed fields in the original's order (the vtable 0x96ace0, the competition record at Comp +4, `cup::YEAR` and those
/// of `FRENCH_SUPER_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx,
/// this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step that fails shows its own error box
/// (`show_error_at`) and returns this. Returns this.
/// Original: `FraSuper::FraSuper` (0x5bf320, fra_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_fra_super(this: usize, year: u32, record: usize) -> usize {
    construct_fra_super_body(Cup(this), year, record) // cqu7: the French Super Cup
}

/// Comparer for `construct_fra_super`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`fra_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_fra_super(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_FRA_SUPER)(this, year, record);
    let error = fra_cup_check_constructor_plan(Cup(this), r, year, record, 0x96ace0, &FRENCH_SUPER_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_fra_super({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

// ---------- batch 285: the Japanese cup constructors: the Emperor's Cup, the J League Cup and the Super Cup (jap_emp_cup.cpp, jap_j_cup.cpp, jap_super.cpp) ----------

/// Builds the cup's round table through its own virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(−1,
/// &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, null): push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this ->
/// thiscall, ret 0x10) and stores it in `cup::ROUND_TABLE`.
pub(crate) unsafe fn jap_emp_cup_create_rounds(this: Cup) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table(t);
}

/// Creates the cup's statistics at Comp +8 (operator new `exe::crt_operator_new` of 0x5ce, cdecl, and
/// Comp_Stats::Comp_Stats `exe::new_comp_stats`: push this; mov ecx, new -> thiscall, ret 4; the original's SEH frame
/// covers only that new; 0 without memory). Returns whether they were created.
pub(crate) unsafe fn jap_emp_cup_create_comp_stats(this: Cup) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this.addr() + 8, s); // Comp +8: the Comp_Stats object
    s != 0
}

/// The plan the comparer checks after the original constructor: the return value (this), the vtable, the competition
/// record (Comp +4) and `cup::YEAR`; on a new cup (`cup::STAGE_INDEX` = `stage_index::MAIN`), every fixed field the
/// constructor writes (offset, width, value) and the round table and statistics created. `None` = as planned.
pub(crate) unsafe fn jap_emp_cup_check_constructor_plan(this: Cup, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this.addr() || read_usize(this.addr()) != vtable || read_usize(this.addr() + 4) != record || read_u16(this.addr() + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this.addr()), read_usize(this.addr() + 4), read_u16(this.addr() + cup::YEAR)));
    }
    if this.stage_index() != stage_index::MAIN {
        return None;
    }
    for &(o, w, v) in fields {
        let x = match w {
            1 => read_u8(this.addr() + o) as u32,
            2 => read_u16(this.addr() + o) as u32,
            _ => read_u32(this.addr() + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && this.round_table() == 0) || read_usize(this.addr() + 8) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", this.round_table(), read_usize(this.addr() + 8)));
    }
    None
}

/// The body of `construct_jap_emp_cup` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_jap_emp_cup_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96c9a0); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_stage_count(0);
    this.set_club_sides(1);
    this.set_substitutes_named(5);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(discipline_rule::JAPAN_CUP);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_8 emperors cup new");
    let collect_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x6624f0usize); // JapEmpCup::collect_entrants
    let r = collect_entrants(this.addr());
    if r == 0 {
        show_error_at(0x9eab90, 0x47);
        return this.addr();
    }
    this.set_prize_fund(0x6acfc0);
    jap_emp_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r == 0 {
        show_error_at(0x9eab90, 0x55);
        return this.addr();
    }
    if !jap_emp_cup_create_comp_stats(this) {
        show_error_at(0x9eab90, 0x5c);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields `construct_jap_emp_cup` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const EMPERORS_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::PRIZE_FUND, 4, 0x6acfc0), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x5), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x14), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The constructor of the Emperor's Cup: with no entrant table of its own; type 1 (clubs of one nation), prize fund of
/// 7,000,000, 5 substitutes on the bench and 3 substitutions, discipline rule 20; on a new cup, in order: the entrants
/// (JapEmpCup::collect_entrants, `jap_emp_cup_collect_entrants`, 0x6624f0, thiscall, ret 0; the error box of line 0x47
/// if it returns 0); the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the
/// tie tree (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error box of line 0x55 if it
/// returns 0); the statistics at Comp +8 (the error box of line 0x5c without them); the final ranking (Cup::rank_teams,
/// `cup_rank_teams`, 0x5223a0, thiscall, ret 0, unchecked). Over the base Cup::Cup (`exe::new_cup`, thiscall, ret 0) it
/// writes the fixed fields in the original's order (the vtable 0x96c9a0, the competition record at Comp +4, `cup::YEAR`
/// and those of `EMPERORS_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov
/// ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step that fails shows its own error box
/// (`show_error_at`) and returns this. Returns this.
/// Original: `JapEmpCup::JapEmpCup` (0x661d30, jap_emp_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_jap_emp_cup(this: usize, year: u32, record: usize) -> usize {
    construct_jap_emp_cup_body(Cup(this), year, record) // cqu8: the Emperor's Cup
}

/// Comparer for `construct_jap_emp_cup`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`jap_emp_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_jap_emp_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_JAP_EMP_CUP)(this, year, record);
    let error = jap_emp_cup_check_constructor_plan(Cup(this), r, year, record, 0x96c9a0, &EMPERORS_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_jap_emp_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of `construct_jap_j_cup` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_jap_j_cup_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96cba8); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_stage_count(0);
    this.set_club_sides(1);
    this.set_substitutes_named(5);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(discipline_rule::JAPAN_CUP);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_8 j league cup new");
    let collect_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x665a00usize); // JapJCup::collect_entrants
    let r = collect_entrants(this.addr());
    if r == 0 {
        show_error_at(0x9eaf14, 0x45);
        return this.addr();
    }
    this.set_prize_fund(0x1e8480);
    jap_emp_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r == 0 {
        show_error_at(0x9eaf14, 0x53);
        return this.addr();
    }
    if !jap_emp_cup_create_comp_stats(this) {
        show_error_at(0x9eaf14, 0x5a);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields `construct_jap_j_cup` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const J_LEAGUE_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::PRIZE_FUND, 4, 0x1e8480), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x5), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x14), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The constructor of the J League Cup: with no entrant table of its own; type 1 (clubs of one nation), prize fund of
/// 2,000,000, 5 substitutes on the bench and 3 substitutions, discipline rule 20; on a new cup, in order: the 28
/// entrants (JapJCup::collect_entrants, `jap_j_cup_collect_entrants`, 0x665a00, thiscall, ret 0; the error box of line
/// 0x45 if it returns 0); the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`);
/// the tie tree (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error box of line 0x53 if it
/// returns 0); the statistics at Comp +8 (the error box of line 0x5a without them); the final ranking (Cup::rank_teams,
/// `cup_rank_teams`, 0x5223a0, thiscall, ret 0, unchecked). Over the base Cup::Cup (`exe::new_cup`, thiscall, ret 0) it
/// writes the fixed fields in the original's order (the vtable 0x96cba8, the competition record at Comp +4, `cup::YEAR`
/// and those of `J_LEAGUE_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov
/// ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step that fails shows its own error box
/// (`show_error_at`) and returns this. Returns this.
/// Original: `JapJCup::JapJCup` (0x665380, jap_j_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_jap_j_cup(this: usize, year: u32, record: usize) -> usize {
    construct_jap_j_cup_body(Cup(this), year, record) // cqu8: the J League Cup
}

/// Comparer for `construct_jap_j_cup`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`jap_emp_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_jap_j_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_JAP_J_CUP)(this, year, record);
    let error = jap_emp_cup_check_constructor_plan(Cup(this), r, year, record, 0x96cba8, &J_LEAGUE_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_jap_j_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of `construct_jap_super` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_jap_super_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96cc48); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_stage_count(0);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_club_sides(1);
    this.set_discipline_index(discipline_rule::JAPAN_CUP);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_8 japanese super cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x666160usize); // JapSuper::select_teams
    let r = select_teams(this.addr());
    if r < 0 {
        show_error_at(0x9eaf4c, 0x3f);
        return this.addr();
    }
    jap_emp_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0x9eaf4c, 0x4d);
        return this.addr();
    }
    if !jap_emp_cup_create_comp_stats(this) {
        show_error_at(0x9eaf4c, 0x54);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields `construct_jap_super` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const JAPANESE_SUPER_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 7] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::DISCIPLINE_INDEX, 1, 0x14), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The constructor of the Japanese Super Cup: with no entrant table of its own; type 1 (clubs of one nation), no prize
/// fund (`cup::PRIZE_FUND` stays as the base left it), bench and substitutions from the base, discipline rule 20; on a
/// new cup, in order: the two clubs (JapSuper::select_teams, `jap_super_select_teams`, 0x666160, thiscall, ret 0; the
/// error box of line 0x3f if it returns a negative value); the rounds (the cup's own virtual
/// `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the tie tree (Cup::map_fixture_tree, `map_fixture_tree`,
/// 0x518790, thiscall, ret 0; the error box of line 0x4d if it returns a negative value); the statistics at Comp +8
/// (the error box of line 0x54 without them); the final ranking (Cup::rank_teams, `cup_rank_teams`, 0x5223a0, thiscall,
/// ret 0, unchecked). Over the base Cup::Cup (`exe::new_cup`, thiscall, ret 0) it writes the fixed fields in the
/// original's order (the vtable 0x96cc48, the competition record at Comp +4, `cup::YEAR` and those of
/// `JAPANESE_SUPER_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx,
/// this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step that fails shows its own error box
/// (`show_error_at`) and returns this. Returns this.
/// Original: `JapSuper::JapSuper` (0x665d70, jap_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_jap_super(this: usize, year: u32, record: usize) -> usize {
    construct_jap_super_body(Cup(this), year, record) // cqu8: the Japanese Super Cup
}

/// Comparer for `construct_jap_super`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`jap_emp_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_jap_super(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_JAP_SUPER)(this, year, record);
    let error = jap_emp_cup_check_constructor_plan(Cup(this), r, year, record, 0x96cc48, &JAPANESE_SUPER_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_jap_super({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

// ---------- 285a leva: the European cup constructors: the UEFA Cup, the European Super Cup and the Intertoto Cup (eur_super_cup.cpp, intertoto_cup.cpp, uefa_cup.cpp) ----------

/// The cup's rounds through its `comp_vtable::CREATE_SCHEDULE` method (push 0; push &`cup::STAGE_LABEL`; push
/// &`cup::ROUND_COUNT`; push −1; mov ecx, this -> thiscall, ret 0x10): the round table, stored in `round_table()`.
unsafe fn uefa_cup_ctor_create_rounds(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this, 0xffff_ffff, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0);
    Cup(this).set_round_table(t);
}

/// The statistics (`exe::crt_operator_new`(0x5ce), cdecl; `exe::new_comp_stats`(new, this), thiscall, ret 4) at +8; the
/// original's SEH frame covers only that new. Returns whether they were created.
unsafe fn uefa_cup_ctor_create_stats(this: usize) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this + 8, s);
    s != 0
}

/// The plan the comparer checks after the original: the return value (this), the vtable, the competition record (+4)
/// and the year (`cup::YEAR`); on a new cup (`stage_index()` = 0xff), each fixed field the constructor writes
/// (offset, width in bytes, value) and the round table and statistics it created. Returns the first difference.
unsafe fn uefa_cup_ctor_plan_error(this: usize, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this || read_usize(this) != vtable || read_usize(this + 4) != record || read_u16(this + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this), read_usize(this + 4), read_u16(this + cup::YEAR)));
    }
    if read_u8(this + cup::STAGE_INDEX) != 0xff {
        return None;
    }
    for &(o, t, v) in fields {
        let x = match t {
            1 => read_u8(this + o) as u32,
            2 => read_u16(this + o) as u32,
            _ => read_u32(this + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && read_usize(this + cup::ROUND_TABLE) == 0) || read_usize(this + 8) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", read_usize(this + cup::ROUND_TABLE), read_usize(this + 8)));
    }
    None
}

const UEFA_CUP_VTABLE: usize = 0x970790;
const EUR_SUPER_CUP_VTABLE: usize = 0x969f80;
const INTERTOTO_CUP_VTABLE: usize = 0x96bb8c;

/// The body of the UEFA Cup constructor (see `construct_uefa_cup`).
unsafe fn construct_uefa_cup_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    cup.set_year(year as i16);
    write_usize(this + 4, record);
    write_usize(this, UEFA_CUP_VTABLE);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_current_stage(0xffff_ffff);
    cup.set_stage_count(0);
    cup.set_club_sides(2);
    write_u8(this + 0x52, 3);
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    cup.set_discipline_index(1);
    cup.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqu_9 uefa cup new");
    // the yearly European seeding (UefaSeeding::update_season 0x9035a0: push 0; mov ecx, [G_UEFA_SEEDING] -> thiscall, ret 4)
    let seeding = read_usize(G_UEFA_SEEDING);
    if seeding != 0 {
        let update_season: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x9035a0usize);
        update_season(seeding, 0);
    }
    let build_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x902b90usize);
    if build_entrants(this) == 0 {
        show_error_at(0xaa19e0, 0x54);
        return this;
    }
    cup.set_prize_fund(0x2dc6c0);
    uefa_cup_ctor_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if map_fixture_tree(this) == 0 {
        show_error_at(0xaa19e0, 0x62);
        return this;
    }
    if !uefa_cup_ctor_create_stats(this) {
        show_error_at(0xaa19e0, 0x69);
        return this;
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    rank_teams(this);
    this
}

/// The fixed fields the UEFA Cup constructor writes on a new cup (offset, width, value).
const UEFA_CUP_CTOR_FIELDS: [(usize, u8, u32); 11] = [(cup::PRIZE_FUND, 4, 0x2dc6c0), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1), (0x52, 1, 0x3), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The UEFA Cup constructor `UefaCup::UefaCup(year, record)`, run once at career creation. On the Cup::Cup base
/// (`exe::new_cup`): the year (`cup::YEAR`), the competition record at +4, the vtable (0x970790), `current_round()` =
/// 0, `stage_index()` = 0xff, `current_stage()` = −1, `stage_count()` = 0, `club_sides()` = 2 (clubs of several
/// nations), +0x52 = 3, 7 substitutes named, 3 substitutions allowed, discipline rule 1 and `run_by_stages()` = 0. If
/// the saved state loads (Cup::load_from_disk 0x51fc00, thiscall(this, 1), ret 4; returns ≠ 0), it returns this. On a
/// new cup, in order: the yearly European seeding (UefaSeeding::update_season 0x9035a0 on [`G_UEFA_SEEDING`], if any;
/// thiscall, ret 4); the entrants (UefaCup::build_entrants 0x902b90, thiscall, ret 0; the error box of line 0x54 if it
/// returns 0); the prize fund (`cup::PRIZE_FUND`) = 3,000,000; the rounds (`comp_vtable::CREATE_SCHEDULE`, into
/// `round_table()`); the tie tree (Cup::map_fixture_tree 0x518790, thiscall, ret 0; the error box of line 0x62 if it
/// returns 0); the statistics at +8 (the error box of line 0x69 without them); the final ranking (Cup::rank_teams
/// 0x5223a0, thiscall, ret 0, unchecked). Every failure returns this. [the bench, substitutions, discipline and type
/// fields read from the competitions reference, §2.1]
/// Original: `UefaCup::UefaCup` (0x9022e0, uefa_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_uefa_cup(this: usize, year: u32, record: usize) -> usize {
    construct_uefa_cup_body(this, year, record) // cqu9: the UEFA Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`uefa_cup_ctor_plan_error`).
unsafe extern "thiscall" fn compare_construct_uefa_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_UEFA_CUP)(this, year, record);
    let error = uefa_cup_ctor_plan_error(this, r, year, record, UEFA_CUP_VTABLE, &UEFA_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_uefa_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of the European Super Cup constructor (see `construct_eur_super_cup`).
unsafe fn construct_eur_super_cup_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    cup.set_year(year as i16);
    write_usize(this + 4, record);
    write_usize(this, EUR_SUPER_CUP_VTABLE);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_stage_count(0);
    cup.set_current_stage(0xffff_ffff);
    cup.set_club_sides(2);
    write_u8(this + 0x52, 3);
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    cup.set_discipline_index(1);
    cup.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqu_9 european super cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x57a500usize);
    if select_teams(this) < 0 {
        show_error_at(0x9c45f8, 0x3f);
        return this;
    }
    uefa_cup_ctor_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if map_fixture_tree(this) < 0 {
        show_error_at(0x9c45f8, 0x4d);
        return this;
    }
    if !uefa_cup_ctor_create_stats(this) {
        show_error_at(0x9c45f8, 0x54);
        return this;
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    rank_teams(this);
    this
}

/// The fixed fields the European Super Cup constructor writes on a new cup (offset, width, value).
const EUR_SUPER_CUP_CTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1), (0x52, 1, 0x3), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The European Super Cup constructor `EurSuperCup::EurSuperCup(year, record)`, run once at career creation. On the
/// Cup::Cup base (`exe::new_cup`): the year (`cup::YEAR`), the competition record at +4, the vtable (0x969f80),
/// `current_round()` = 0, `stage_index()` = 0xff, `stage_count()` = 0, `current_stage()` = −1, `club_sides()` = 2
/// (clubs of several nations), +0x52 = 3, 7 substitutes named, 3 substitutions allowed, discipline rule 1 and
/// `run_by_stages()` = 0; no prize fund (`cup::PRIZE_FUND` stays as the base left it). If the saved state loads
/// (Cup::load_from_disk 0x51fc00, thiscall(this, 1), ret 4; returns ≠ 0), it returns this. On a new cup, in order: the
/// two clubs (EurSuperCup::select_teams 0x57a500, thiscall, ret 0; the error box of line 0x3f if it returns a negative
/// value); the rounds (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the tie tree (Cup::map_fixture_tree
/// 0x518790, thiscall, ret 0; the error box of line 0x4d if it returns a negative value); the statistics at +8 (the
/// error box of line 0x54 without them); the final ranking (Cup::rank_teams 0x5223a0, thiscall, ret 0, unchecked).
/// Every failure returns this.
/// Original: `EurSuperCup::EurSuperCup` (0x57a100, eur_super_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_eur_super_cup(this: usize, year: u32, record: usize) -> usize {
    construct_eur_super_cup_body(this, year, record) // cqu9: the European Super Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`uefa_cup_ctor_plan_error`).
unsafe extern "thiscall" fn compare_construct_eur_super_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_EUR_SUPER_CUP)(this, year, record);
    let error = uefa_cup_ctor_plan_error(this, r, year, record, EUR_SUPER_CUP_VTABLE, &EUR_SUPER_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_eur_super_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of the Intertoto Cup constructor (see `construct_intertoto_cup`).
unsafe fn construct_intertoto_cup_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    cup.set_year(year as i16);
    write_usize(this + 4, record);
    write_usize(this, INTERTOTO_CUP_VTABLE);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_current_stage(0xffff_ffff);
    cup.set_stage_count(0);
    cup.set_club_sides(2);
    write_u8(this + 0x52, 3);
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    write_u32(this + intertoto_cup::WINNERS_SENT_TO_UEFA, 0);
    cup.set_discipline_index(1);
    cup.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqu_9 intertoto cup new");
    // the yearly European seeding (UefaSeeding::update_season 0x9035a0: push 0; mov ecx, [G_UEFA_SEEDING] -> thiscall, ret 4)
    let seeding = read_usize(G_UEFA_SEEDING);
    if seeding != 0 {
        let update_season: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x9035a0usize);
        update_season(seeding, 0);
    }
    uefa_cup_ctor_create_rounds(this);
    let collect_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x6336c0usize);
    if collect_entrants(this) == 0 {
        show_error_at(0x9e9e14, 0x60);
        return this;
    }
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if map_fixture_tree(this) == 0 {
        show_error_at(0x9e9e14, 0x67);
        return this;
    }
    if !uefa_cup_ctor_create_stats(this) {
        show_error_at(0x9e9e14, 0x6e);
        return this;
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    rank_teams(this);
    this
}

/// The fixed fields the Intertoto Cup constructor writes on a new cup (offset, width, value).
const INTERTOTO_CUP_CTOR_FIELDS: [(usize, u8, u32); 11] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1), (0x52, 1, 0x3), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0), (intertoto_cup::WINNERS_SENT_TO_UEFA, 4, 0x0)];

/// The Intertoto Cup constructor `IntertotoCup::IntertotoCup(year, record)`, run once at career creation. On the
/// Cup::Cup base (`exe::new_cup`): the year (`cup::YEAR`), the competition record at +4, the vtable (0x96bb8c),
/// `current_round()` = 0, `stage_index()` = 0xff, `current_stage()` = −1, `stage_count()` = 0, `club_sides()` = 2
/// (clubs of several nations), +0x52 = 3, 7 substitutes named, 3 substitutions allowed,
/// `intertoto_cup::WINNERS_SENT_TO_UEFA` = 0, discipline rule 1 and `run_by_stages()` = 0; no prize fund
/// (`cup::PRIZE_FUND` stays as the base left it). If the saved state loads (Cup::load_from_disk 0x51fc00,
/// thiscall(this, 1), ret 4; returns ≠ 0), it returns this. On a new cup, in order: the yearly European seeding
/// (UefaSeeding::update_season 0x9035a0 on [`G_UEFA_SEEDING`], if any; thiscall, ret 4); the rounds
/// (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the 60 entrants (IntertotoCup::collect_entrants 0x6336c0,
/// thiscall, ret 0; the error box of line 0x60 if it returns 0); the tie tree (Cup::map_fixture_tree 0x518790,
/// thiscall, ret 0; the error box of line 0x67 if it returns 0); the statistics at +8 (the error box of line 0x6e
/// without them); the final ranking (Cup::rank_teams 0x5223a0, thiscall, ret 0, unchecked). Every failure returns this.
/// Original: `IntertotoCup::IntertotoCup` (0x632fb0, intertoto_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_intertoto_cup(this: usize, year: u32, record: usize) -> usize {
    construct_intertoto_cup_body(this, year, record) // cqu9: the Intertoto Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`uefa_cup_ctor_plan_error`).
unsafe extern "thiscall" fn compare_construct_intertoto_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_INTERTOTO_CUP)(this, year, record);
    let error = uefa_cup_ctor_plan_error(this, r, year, record, INTERTOTO_CUP_VTABLE, &INTERTOTO_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_intertoto_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

// ---------- 285a leva: the Asian Cup Winners' Cup: the 32 qualifiers of the two zones, the west zone stage and the predicted positions (asia_cup_winner.cpp) ----------

/// The nation id of the club ([[`club::NATION`]], dword); without a club or a nation, −1.
unsafe fn asia_cup_winner_club_nation_id(c: usize) -> i32 {
    if c == 0 || read_usize(c + club::NATION) == 0 {
        return -1;
    }
    read_usize(read_usize(c + club::NATION)) as i32
}

/// The Cup Winners' Cup east zone (23 nations, in the original's order; Pakistan comes after Laos).
const ASIAN_CUP_WINNERS_EAST_ZONE_KEYS: [usize; 23] = [KEY_NATION_BANGLADESH, KEY_NATION_BRUNEI_DARUSSALAM, KEY_NATION_CHINA, KEY_NATION_HONG_KONG, KEY_NATION_INDIA,
    KEY_NATION_INDONESIA, KEY_NATION_JAPAN, KEY_NATION_KAMPUCHEA, KEY_NATION_LAOS, KEY_NATION_PAKISTAN, KEY_NATION_MACAU, KEY_NATION_MALAYSIA, KEY_NATION_MALDIVES,
    KEY_NATION_MYANMAR, KEY_NATION_NEPAL, KEY_NATION_NORTH_KOREA, KEY_NATION_SINGAPORE, KEY_NATION_SOUTH_KOREA, KEY_NATION_SRI_LANKA, KEY_NATION_TAIWAN,
    KEY_NATION_THAILAND, KEY_NATION_THE_PHILIPPINES, KEY_NATION_VIETNAM];

/// The west zone (19 nations, as in the Asian Club Championship).
const ASIAN_CUP_WINNERS_WEST_ZONE_KEYS: [usize; 19] = [KEY_NATION_AFGHANISTAN, KEY_NATION_BAHRAIN, KEY_NATION_IRAN, KEY_NATION_IRAQ, KEY_NATION_JORDAN,
    KEY_NATION_KAZAKHSTAN, KEY_NATION_KUWAIT, KEY_NATION_KYRGYZSTAN, KEY_NATION_LEBANON, KEY_NATION_OMAN, KEY_NATION_PALESTINE, KEY_NATION_QATAR,
    KEY_NATION_SAUDI_ARABIA, KEY_NATION_SYRIA, KEY_NATION_TAJIKISTAN, KEY_NATION_TURKMENISTAN, KEY_NATION_UNITED_ARAB_EMIRATES, KEY_NATION_UZBEKISTAN,
    KEY_NATION_YEMEN];

/// The index of the club's nation in a list of nation ids (0x40cba0(club, list, n): 3 pushes; add esp, 0xc -> cdecl;
/// returns the index byte or −1).
unsafe fn asia_cup_winner_zone_index(club: usize, list: &[u32]) -> i8 {
    let f: unsafe extern "C" fn(usize, *const u32, u32) -> u32 = core::mem::transmute(0x40cba0usize);
    f(club, list.as_ptr(), list.len() as u32) as u8 as i8
}

/// Does the club already play the Asian Club Championship (0x40e7b0(club): push; add esp, 4 -> cdecl): in that cup's
/// team table or in that of its stage 0.
unsafe fn asia_cup_winner_in_champions_league(club: usize) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x40e7b0usize);
    f(club) != 0
}

/// The winner of the cup `cup_key` of the nation `nation_key` (when the nation's `nation::LEAGUE_SELECTED` byte is set,
/// in its [`G_NATION_TABLE`] record): the winner, or the runner-up if the winner is already in place 0 or 16 or plays
/// the Champions League; the runner-up in those conditions (or null) does not enter.
unsafe fn asia_cup_winner_national_cup_club(this: usize, nation_key: usize, cup_key: usize) -> Option<usize> {
    let n = read_usize(nation_key);
    if read_u8(read_usize(G_NATION_TABLE).wrapping_add(n.wrapping_mul(nation::SIZE)).wrapping_add(nation::LEAGUE_SELECTED)) == 0 {
        return None;
    }
    let cup_record = comp_record_of_key(cup_key);
    let list = read_usize(this + 0xb2);
    let c = exe::comp_util_get_latest_placed_club(cup_record, 0); // cqu11: the cup winner
    if c != read_usize(list) && c != read_usize(list + 0x40) && !asia_cup_winner_in_champions_league(c) {
        return if c != 0 { Some(c) } else { None };
    }
    let c = exe::comp_util_get_latest_placed_club(cup_record, 1);
    if c == read_usize(list) || c == read_usize(list + 0x40) || asia_cup_winner_in_champions_league(c) || c == 0 {
        return None;
    }
    Some(c)
}

/// The 32 Asian Cup Winners' Cup qualifiers: +0xb6 = 32 and the 32-place list +0xb2 (`exe::crt_calloc`(0x20, 4), cdecl;
/// out of memory, the error box of line 0x241 and 0). The latest winner of the Cup Winners' Cup itself
/// (`exe::comp_util_get_latest_placed_club`([+4], 0), cdecl), if it does not play the Champions League, goes to place
/// 0 if its nation is in the east zone, otherwise to 16. In a local list of up to 1,200 clubs: the Emperor's Cup winner
/// ([`KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP`]; the runner-up if the winner is already in places 0/16 or plays the
/// Champions League), if Japan has its `nation::LEAGUE_SELECTED` byte, and the Korean cup's
/// ([`KEY_CLUB_COMP_KOREAN_FA_CUP`]) in the same way; and every Asian club ([`KEY_CONTINENT_ASIA`]) outside places 0
/// and 16. The list is sorted and shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n − 1, 40), cdecl). In
/// its order, each club whose nation has not had a place yet and that does not play the Champions League takes the
/// first free place of its zone (0..15 east, 16..31 west), while the zone's last place is free; the nation is marked
/// even when the zone is full. Returns 1. (Stack frame 0x1598.)
/// Original: `AsiaCupWinner::select_qualified_clubs` (0x40d830, asia_cup_winner.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn asia_cup_winner_select_qualified_clubs(this: usize) -> u32 {
    write_u32(this + 0xb6, 0x20);
    let east: [u32; 23] = core::array::from_fn(|i| read_usize(ASIAN_CUP_WINNERS_EAST_ZONE_KEYS[i]) as u32);
    let west: [u32; 19] = core::array::from_fn(|i| read_usize(ASIAN_CUP_WINNERS_WEST_ZONE_KEYS[i]) as u32);
    let list = exe::crt_calloc(0x20, 4);
    write_usize(this + 0xb2, list);
    if list == 0 {
        show_error_at(0x987b60, 0x241);
        return 0;
    }
    let holder = exe::comp_util_get_latest_placed_club(read_usize(this + 4), 0);
    if !asia_cup_winner_in_champions_league(holder) {
        if asia_cup_winner_zone_index(holder, &east) > -1 {
            write_usize(list, holder);
        } else {
            write_usize(list + 0x40, holder);
        }
    }
    let mut clubs = [0usize; 0x4b0];
    let mut n = 0usize;
    if let Some(c) = asia_cup_winner_national_cup_club(this, KEY_NATION_JAPAN, KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP) {
        covered!("cqu_11 emperors cup club");
        clubs[0] = c;
        n = 1;
    }
    if let Some(c) = asia_cup_winner_national_cup_club(this, KEY_NATION_SOUTH_KOREA, KEY_CLUB_COMP_KOREAN_FA_CUP) {
        clubs[n] = c;
        n += 1;
    }
    let total = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let asia = read_usize(KEY_CONTINENT_ASIA);
    let mut c = read_usize(G_CLUB_TABLE);
    for _ in 0..total.max(0) {
        let nation = read_usize(c + club::NATION);
        if nation != 0 {
            let continent = read_usize(nation + nation::CONTINENT);
            if continent != 0 && read_usize(continent) == asia && (n as i16) < 0x4b0 && c != read_usize(list) && c != read_usize(list + 0x40) {
                clubs[n] = c;
                n += 1;
            }
        }
        c += club::SIZE;
    }
    covered!("cqu_11 club draw");
    exe::comp_util_sort_and_shuffle_top_clubs(clubs.as_mut_ptr() as usize, (n as u32).wrapping_sub(1), 0x28); // cqu11: 40 picks in the shuffle
    let (mut east_marks, mut west_marks) = ([0u8; 23], [0u8; 19]);
    for &club in clubs.iter().take((n as i16).max(0) as usize) {
        let i = asia_cup_winner_zone_index(club, &east);
        let (mark, first) = if i > -1 {
            (&mut east_marks[i as usize], 0usize)
        } else {
            let j = asia_cup_winner_zone_index(club, &west);
            if j <= -1 {
                continue;
            }
            (&mut west_marks[j as usize], 16usize)
        };
        if *mark != 0 || asia_cup_winner_in_champions_league(club) {
            continue;
        }
        *mark = 1;
        if read_usize(list + (first + 15) * 4) != 0 {
            continue;
        }
        if let Some(k) = (first..first + 16).find(|&k| read_usize(list + k * 4) == 0) {
            write_usize(list + k * 4, club);
        }
    }
    1
}

// ponytail: the list is allocated and the shuffle draws: it cannot run twice. Only the original; the plan: returns 1,
// +0xb6 = 32, the list at +0xb2, none of its clubs in the Asian Club Championship and, outside places 0 and 16 (the Cup
// Winners' Cup holder's), each club in its nation's zone and no nation repeated.
unsafe extern "thiscall" fn compare_asia_cup_winner_select_qualified_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ASIA_CUP_WINNER_SELECT_QUALIFIED_CLUBS);
    let r = o(this);
    if r == 1 {
        let list = read_usize(this + 0xb2);
        let mut nations: Vec<i32> = Vec::new();
        let mut error = 0usize;
        for k in 0..32usize {
            let c = if list != 0 { read_usize(list + k * 4) } else { 0 };
            if c == 0 {
                continue;
            }
            if asia_cup_winner_in_champions_league(c) {
                error = 100 + k;
            }
            if k == 0 || k == 16 {
                continue;
            }
            let nation = asia_cup_winner_club_nation_id(c);
            let zone = if k < 16 { &ASIAN_CUP_WINNERS_EAST_ZONE_KEYS[..] } else { &ASIAN_CUP_WINNERS_WEST_ZONE_KEYS[..] };
            if !zone.iter().any(|&g| read_usize(g) as i32 == nation) || nations.contains(&nation) {
                error = k;
            }
            nations.push(nation);
        }
        let ok = list != 0 && read_usize(this + 0xb6) == 0x20 && error == 0;
        SCOREBOARD.record(ok, || format!("asia_cup_winner_select_qualified_clubs({:#x}): list {:#x} +0xb6 {} wrong place {}", this, list, read_usize(this + 0xb6), error));
    }
    r
}

/// The knockout stage of one zone of the Asian cups (0x40bec0 and 0x40ddf0, the same body; only the source file and
/// the box lines change): the `cup::TEAM_COUNT` teams (`exe::crt_malloc`(4 × n), cdecl; out of memory, the box
/// `lines.0`) are qualifiers 16.. of the list +0xb2; the round table through `comp_vtable::CREATE_SCHEDULE` (push 0;
/// push &extra; push &rounds; push 0; mov ecx, this -> thiscall, ret 0x10); the stage (`exe::crt_operator_new`(0xb2)
/// and the Cup_Stage constructor `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c, 15 arguments: the cup, n, the teams,
/// the rounds, +4, the table, the year `cup::YEAR`, 0, 2, the extra, `cup::IMPORTANCE_BIAS`, 0, 0, 0, 0; the original
/// pushes the words and the byte with garbage in the high bytes, which the constructor does not read: the copy passes
/// clean values) in [`cup::STAGES`][0]; the teams and the table freed (`exe::crt_free`). Without the stage, the box
/// `lines.1`; with it, `cup::CURRENT_STAGE` = 0.
unsafe fn asia_cup_winner_zone_knockout_stage(this: usize, file: usize, lines: (u32, u32), label: &'static str) {
    #[allow(clippy::type_complexity)]
    let construct_stage: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let n = read_i16(this + cup::TEAM_COUNT);
    let teams = exe::crt_malloc((n as i32 as u32).wrapping_mul(4));
    if teams == 0 {
        return show_error_at(file, lines.0);
    }
    for i in 0..n.max(0) as usize {
        covered!(label);
        write_usize(teams + i * 4, read_usize(read_usize(this + 0xb2) + 0x40 + i * 4)); // cqu11: team i of the west zone
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = create_schedule(this, 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        construct_stage(p, this, n as u16 as u32, teams, rounds as u16 as u32, read_usize(this + 4), table, read_i16(this + cup::YEAR) as u16 as u32, 0, 2,
                        extra as u16 as u32, read_u8(this + cup::IMPORTANCE_BIAS) as u32, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(read_usize(this + cup::STAGES), stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    if read_usize(read_usize(this + cup::STAGES)) == 0 {
        return show_error_at(file, lines.1);
    }
    write_u32(this + cup::CURRENT_STAGE, 0);
}

/// Stage 0 of the Asian Cup Winners' Cup, a knockout of the 16 west zone qualifiers (boxes at lines 0x2fd and 0x314).
/// (The original's SEH frame is only for the stage's new: no destructor on the normal path.)
/// Original: `AsiaCupWinner::create_zone_stage` (0x40ddf0, asia_cup_winner.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn asia_cup_winner_create_zone_stage(this: usize) {
    asia_cup_winner_zone_knockout_stage(this, 0x987b60, (0x2fd, 0x314), "cqu_11 west zone team")
}

// ponytail: the stage is allocated: it cannot run twice. Only the original; the plan: the stage in [`cup::STAGES`][0],
// `cup::CURRENT_STAGE` = 0 and, in its team table (`cup::TEAM_TABLE`, 6-byte rows), qualifiers 16.. of the list (in
// any order).
unsafe fn asia_cup_winner_check_zone_stage(name: &str, this: usize, o: impl FnOnce()) {
    let list = read_usize(this + 0xb2);
    let n = read_i16(this + cup::TEAM_COUNT).max(0) as usize;
    if list == 0 || n > 16 {
        return o();
    }
    let mut plan: Vec<usize> = (0..n).map(|i| read_usize(list + 0x40 + i * 4)).collect();
    o();
    let f = read_usize(read_usize(this + cup::STAGES));
    if f == 0 {
        return;
    }
    let t = read_usize(f + cup::TEAM_TABLE);
    let mut read: Vec<usize> = if t != 0 { (0..n).map(|i| read_usize(t + i * 6)).collect() } else { Vec::new() };
    read.sort_unstable();
    plan.sort_unstable();
    SCOREBOARD.record(read_usize(this + cup::CURRENT_STAGE) == 0 && read == plan, || format!("{}({:#x}): teams {:x?} plan {:x?} +0x30 {}", name, this, read, plan, read_usize(this + cup::CURRENT_STAGE) as i32));
}

unsafe extern "thiscall" fn compare_asia_cup_winner_create_zone_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CUP_WINNER_CREATE_ZONE_STAGE);
    asia_cup_winner_check_zone_stage("asia_cup_winner_create_zone_stage", this, || o(this))
}

/// The predicted positions of the Asian cups (0x40c720 and 0x40e3b0, the same body): without statistics (+8), the box
/// `lines.0`; an array of 2n clubs (`exe::crt_malloc`(8 × `cup::TEAM_COUNT`); out of memory, the box `lines.1`): the n
/// of the cup's team table (`cup::TEAM_TABLE`, 6-byte rows) and the n of stage 0's team table; each half sorted by
/// reputation (`exe::crt_qsort`(v, n, 4, 0x4b07d0), cdecl); the 32 positions in order
/// (`exe::comp_stats_set_club_final_position`(stats, club, position), thiscall, ret 8): in each half 1, 3, 5, 7, 9, 9,
/// 9, 9 and eight 17s; the array freed.
unsafe fn asia_cup_winner_predicted_positions(this: usize, file: usize, lines: (u32, u32)) {
    if read_usize(this + 8) == 0 {
        return show_error_at(file, lines.0);
    }
    let v = exe::crt_malloc((read_i16(this + cup::TEAM_COUNT) as i32 as u32).wrapping_mul(8));
    if v == 0 {
        return show_error_at(file, lines.1);
    }
    let mut i = 0i16;
    while i < read_i16(this + cup::TEAM_COUNT) {
        let k = i as usize;
        write_usize(v + k * 4, read_usize(read_usize(this + cup::TEAM_TABLE) + k * 6));
        let n = read_i16(this + cup::TEAM_COUNT) as isize as usize;
        write_usize(v + (n + k) * 4, read_usize(read_usize(read_usize(read_usize(this + cup::STAGES)) + cup::TEAM_TABLE) + k * 6));
        i += 1;
    }
    let n = read_i16(this + cup::TEAM_COUNT) as i32;
    exe::crt_qsort(v, n as u32, 4, 0x4b07d0usize);
    let n = read_i16(this + cup::TEAM_COUNT) as i32;
    exe::crt_qsort(v + (n as usize).wrapping_mul(4), n as u32, 4, 0x4b07d0usize);
    for (k, &p) in ASIAN_CUP_WINNERS_PREDICTED_POSITIONS.iter().enumerate() {
        exe::comp_stats_set_club_final_position(read_usize(this + 8), read_usize(v + k * 4), p);
    }
    exe::crt_free(v);
}

/// The 32 predicted positions, in array order (the two halves).
const ASIAN_CUP_WINNERS_PREDICTED_POSITIONS: [u32; 32] = [1, 3, 5, 7, 9, 9, 9, 9, 17, 17, 17, 17, 17, 17, 17, 17, 1, 3, 5, 7, 9, 9, 9, 9, 17, 17, 17, 17, 17, 17, 17, 17];

/// The predicted positions of the two Asian Cup Winners' Cup zones (boxes at lines 0x3db and 0x3e3).
/// Original: `AsiaCupWinner::rank_teams` (0x40e3b0, asia_cup_winner.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn asia_cup_winner_rank_teams(this: usize) {
    asia_cup_winner_predicted_positions(this, 0x987b60, (0x3db, 0x3e3))
}

// ponytail: Comp_Stats::set_club_final_position adds the club's entry (it reallocates): it cannot run twice. Only the
// original; the plan: in each half (the cup's team table and stage 0's, 16 clubs each), the written positions
// (`comp_stats_club::FINAL_POSITION`, `exe::comp_stats_find_club_entry`) by reputation (`club::REPUTATION`) in
// descending order make 1, 3, 5, 7, 9, 9, 9, 9 and eight 17s; reputation ties may swap (qsort is not stable).
unsafe fn asia_cup_winner_check_positions(name: &str, this: usize, o: impl FnOnce()) {
    let n = read_i16(this + cup::TEAM_COUNT);
    let stats = read_usize(this + 8);
    let stages = read_usize(this + cup::STAGES);
    if n != 16 || stats == 0 || stages == 0 || read_usize(stages) == 0 || read_usize(this + cup::TEAM_TABLE) == 0 || read_usize(read_usize(stages) + cup::TEAM_TABLE) == 0 {
        return o();
    }
    let halves: [Vec<usize>; 2] = [
        (0..16).map(|k| read_usize(read_usize(this + cup::TEAM_TABLE) + k * 6)).collect(),
        (0..16).map(|k| read_usize(read_usize(read_usize(stages) + cup::TEAM_TABLE) + k * 6)).collect(),
    ];
    if halves.iter().flatten().any(|&c| c == 0) {
        return o();
    }
    o();
    let mut error = None;
    for (m, half) in halves.iter().enumerate() {
        let mut v: Vec<(i16, u8)> = half
            .iter()
            .map(|&c| {
                let e = exe::comp_stats_find_club_entry(stats, c);
                (read_i16(c + club::REPUTATION), if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 })
            })
            .collect();
        v.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        let positions: Vec<u8> = v.iter().map(|x| x.1).collect();
        if positions.iter().zip(ASIAN_CUP_WINNERS_PREDICTED_POSITIONS.iter()).any(|(&a, &b)| a as u32 != b) {
            error = Some((m, positions));
        }
    }
    SCOREBOARD.record(error.is_none(), || format!("{}({:#x}): positions by reputation {:?}", name, this, error));
}

unsafe extern "thiscall" fn compare_asia_cup_winner_rank_teams(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CUP_WINNER_RANK_TEAMS);
    asia_cup_winner_check_positions("asia_cup_winner_rank_teams", this, || o(this))
}

// ---------- 285a leva: the Oceania World Cup qualifying: the start day, the first group table and the second group stage (wc_oceania_league.cpp) ----------

/// The qualifying's start date (`exe::new_date` on a local date: push year; push day; lea ecx -> thiscall, ret 8): the
/// day `league::NEXT_DRAW_DAY` and the year `league::NEXT_DRAW_YEAR_OFFSET` + `league::YEAR` (words), returned as (day,
/// year).
unsafe fn wc_oceania_league_start_date(this: usize) -> (u16, u16) {
    // the date is 8 bytes: the day, the year and the leap-year dword at +4, which the routine also writes
    let mut date = [0u32; 2];
    let year = (read_i16(this + league::NEXT_DRAW_YEAR_OFFSET) as u16).wrapping_add(read_i16(this + league::YEAR) as u16);
    exe::new_date(date.as_mut_ptr() as usize, read_i16(this + league::NEXT_DRAW_DAY) as u16 as u32, year as u32);
    (date[0] as u16, (date[0] >> 16) as u16)
}

/// The Oceania qualifying day (`comp_vtable::UPDATE_FOR_DATE`; the original's SEH frame is only for the local dates,
/// with no destructor): `WcOceaniaLeague::update_for_date(today, full)`. Before the start (today's year lower,
/// unsigned; or the same year and the day lower, signed) it returns 0 without the league day. On the start day or on
/// any day of 2001 (a game starting in 2001 sets up the qualifying already under way), with `league::ACTIVE` = 0 and
/// WITH `full`: the Oceania national teams are released from the continent's competitions (0x775070([`G_NATIONAL_TEAMS`],
/// the continent [`G_CONTINENT_TABLE`] + 0xc6 × [`KEY_CONTINENT_OCEANIA`], 0): push 0; push; mov ecx -> thiscall, ret 8);
/// each national team of the first group's table (`league::TABLE`, `league_table_row::SIZE`-byte rows;
/// `league::CLUB_COUNT` rows) and each of the +0xf3 of the list +0xef (the non-null ones) enters the qualifying
/// (`exe::nationalteams_set_current_competition`([`G_NATIONAL_TEAMS`], its `club::NATION`, `league::COMPETITION`),
/// thiscall, ret 8); the fixtures (`exe::league_create_fixtures`; 0: the error box of line 0x3a4 and 0), the sorted
/// table (`exe::league_sort_table`(this, 0); 0: the error box of line 0x3ab and 0), the second group stage
/// (WcOceaniaLeague::create_second_group_stage 0x91f1a0, thiscall, ret 0) and `league::ACTIVE` = 1. Then the league
/// day (League::update_for_date `FN_LEAGUE_UPDATE_FOR_DATE`, thiscall(this, today, full), ret 8), whose return value it
/// passes on.
/// Original: `WcOceaniaLeague::update_for_date` (0x91f3d0, wc_oceania_league.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn wc_oceania_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let release_continent: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x775070usize);
    let create_second_group_stage: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x91f1a0usize);
    // the full eax is passed on, so the u8 wrapper `exe::league_update_for_date` is not used
    let league_update_for_date: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    let (day, year) = wc_oceania_league_start_date(this);
    let today_day = read_u16(today);
    let today_year = read_u16(today + 2);
    // ponytail: on the early returns before the start and on the errors the original returns only al (0); the caller reads the byte
    if today_year < year || (today_year == year && (today_day as i16) < day as i16) {
        return 0;
    }
    let set_up_day = (today_year == year && today_day == day) || today_year == 0x7d1; // cqu12: the 2001 game sets it up at once
    if set_up_day && read_usize(this + league::ACTIVE) == 0 && full != 0 {
        covered!("cqu_12 qualifying set up");
        let national_teams = read_usize(G_NATIONAL_TEAMS);
        release_continent(national_teams, read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_OCEANIA).wrapping_mul(0xc6)), 0);
        let mut i = 0i16;
        while i < read_i16(this + league::CLUB_COUNT) {
            let team = read_usize(read_usize(this + league::TABLE) + i as usize * league_table_row::SIZE);
            exe::nationalteams_set_current_competition(national_teams, read_usize(team + club::NATION), read_usize(this + league::COMPETITION));
            i += 1;
        }
        let mut k = 0i16;
        while (k as i32) < read_usize(this + 0xf3) as i32 {
            let e = read_usize(read_usize(this + 0xef) + k as usize * 4);
            if e != 0 {
                exe::nationalteams_set_current_competition(national_teams, read_usize(e + club::NATION), read_usize(this + league::COMPETITION));
            }
            k = k.wrapping_add(1);
        }
        if exe::league_create_fixtures(this) == 0 {
            show_error_at(0xad4afc, 0x3a4);
            return 0;
        }
        if exe::league_sort_table(this, 0usize) == 0 {
            show_error_at(0xad4afc, 0x3ab);
            return 0;
        }
        create_second_group_stage(this);
        write_u32(this + league::ACTIVE, 1);
    }
    league_update_for_date(this, today, full)
}

// ponytail: the competition entries, the fixtures, the stage and the league day cannot run twice. Only the original;
// the plan on the set-up day (the copy's condition): `league::ACTIVE` = 1, the second group stage in
// [`league::STAGES`][0] and the current stage (`league::CURRENT_STAGE`) = 0.
unsafe extern "thiscall" fn compare_wc_oceania_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_WC_OCEANIA_LEAGUE_UPDATE_FOR_DATE);
    let sets_up = read_usize(this + league::ACTIVE) == 0 && full != 0 && {
        let (day, year) = wc_oceania_league_start_date(this);
        let (td, ty) = (read_u16(today), read_u16(today + 2));
        !(ty < year || (ty == year && (td as i16) < day as i16)) && ((ty == year && td == day) || ty == 0x7d1)
    };
    let r = o(this, today, full);
    if sets_up {
        let stages = read_usize(this + league::STAGES);
        let ok = read_usize(this + league::ACTIVE) == 1 && stages != 0 && read_usize(stages) != 0 && read_usize(this + league::CURRENT_STAGE) == 0;
        SCOREBOARD.record(ok, || format!("wc_oceania_league_update_for_date({:#x}): the set-up did not build the stage (+0x45 {}, +0x30 {})", this, read_usize(this + league::ACTIVE), read_usize(this + league::CURRENT_STAGE) as i32));
    }
    r
}

/// The first Oceania qualifying group has 5 national teams (`league::CLUB_COUNT`), the table `league::TABLE`
/// (`exe::crt_malloc`(0x127), cdecl: 5 rows of `league_table_row::SIZE` bytes; out of memory, the error box of line
/// 0x24b and 0), each row i with the national team of the even places of the list +0xef (2i)
/// (`exe::league_init_table_row`(this, i, team, 0, 0), thiscall, ret 0x10), the group draw news
/// (`exe::league_send_group_draw_news`) and returns 1.
/// Original: `WcOceaniaLeague::init_group_table` (0x91ec00, wc_oceania_league.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn wc_oceania_league_init_group_table(this: usize) -> u32 {
    write_i16(this + league::CLUB_COUNT, 5); // cqu12: 5 national teams in the first group
    let t = exe::crt_malloc(0x127);
    write_usize(this + league::TABLE, t);
    if t == 0 {
        show_error_at(0xad4afc, 0x24b);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < read_i16(this + league::CLUB_COUNT) {
        covered!("cqu_12 first group row");
        // ponytail: the index goes in a dword whose 3 high bytes are stack garbage; the routine reads only the byte (char)
        exe::league_init_table_row(this, i as u8 as u32, read_usize(read_usize(this + 0xef) + i as isize as usize * 8), 0, 0);
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

// ponytail: the table is allocated (and each row allocates its history) and the news goes to the managers: it cannot
// run twice. Only the original; the plan: returns 1, `league::CLUB_COUNT` = 5 and in row i the national team 2i of
// the list with position i (`league_table_row::POSITION`).
unsafe extern "thiscall" fn compare_wc_oceania_league_init_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_OCEANIA_LEAGUE_INIT_GROUP_TABLE);
    let list = read_usize(this + 0xef);
    if list == 0 {
        return o(this);
    }
    let expected: [usize; 5] = core::array::from_fn(|i| read_usize(list + i * 8));
    let r = o(this);
    let t = read_usize(this + league::TABLE);
    let mut read = [(0usize, 0u8); 5];
    if t != 0 {
        for (i, l) in read.iter_mut().enumerate() {
            *l = (read_usize(t + i * league_table_row::SIZE), read_u8(t + i * league_table_row::SIZE + league_table_row::POSITION));
        }
    }
    let same = r == 1 && read_i16(this + league::CLUB_COUNT) == 5 && t != 0 && (0..5).all(|i| read[i] == (expected[i], i as u8));
    SCOREBOARD.record(same, || format!("wc_oceania_league_init_group_table({:#x}) returned {} rows {:x?} expected {:x?}", this, r, read, expected));
    r
}

/// The second Oceania qualifying group has 4 national teams, or 5 when `wc_oceania_league_fields::EXEMPT_COUNT` is
/// not 1; the teams in an array (`exe::crt_malloc`(4 × n), cdecl; out of memory, the error box of line 0x355 and 0)
/// with the national teams of the odd places of the list +0xef (2i + 1); the round table through
/// `comp_vtable::CREATE_SCHEDULE` (push 0; push &id; push &rounds; push 0; mov ecx, this -> thiscall, ret 0x10); the
/// stage (`exe::crt_operator_new`(0xee), cdecl; the League_Stage constructor `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall,
/// ret 0x58, 22 arguments: the qualifying, n, the array, 1, `league::COMPETITION`, the round table, the rounds (word),
/// `league::WIN_POINTS`, `league::DRAW_POINTS`, `league::MATCH_CATEGORY`, &`league::TIEBREAKS`, a local 4-byte array
/// {1, 0, 0, 0} (the places [inference]), `league::YEAR` (word), 0, the id (word), `league::IMPORTANCE`, 4, 0,
/// `league::LEAGUE_FLAGS` (word), −1, 0, 2; the original pushes the words and bytes with garbage in the high bytes,
/// which the constructor does not read: the copy passes clean values) in [`league::STAGES`][0]; the array and the
/// round table freed (`exe::crt_free`). Without the stage: the error box of line 0x36b and 0. With it:
/// `league::CURRENT_STAGE` = 0, the group news (`exe::league_send_group_draw_news` on the stage) and 1. (The original's
/// SEH frame is only for the stage's new: no destructor on the normal path.)
/// Original: `WcOceaniaLeague::create_second_group_stage` (0x91f1a0, wc_oceania_league.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn wc_oceania_league_create_second_group_stage(this: usize) -> u32 {
    #[allow(clippy::type_complexity)]
    let construct_stage: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let places: [u8; 4] = [1, 0, 0, 0];
    let n = 4 + (read_u8(this + wc_oceania_league_fields::EXEMPT_COUNT) != 1) as usize;
    let teams = exe::crt_malloc((n * 4) as u32);
    if teams == 0 {
        show_error_at(0xad4afc, 0x355);
        return 0;
    }
    for i in 0..n {
        covered!("cqu_12 second group team");
        write_usize(teams + i * 4, read_usize(read_usize(this + 0xef) + i * 8 + 4)); // cqu12: the national team in place i of the second group
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let table = create_schedule(this, 0, &mut rounds, &mut id, 0);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        let b = |d: usize| read_u8(this + d) as u32;
        construct_stage(
            p,
            this,
            n as u32,
            teams,
            1,
            read_usize(this + league::COMPETITION),
            table,
            rounds as u16 as u32,
            b(league::WIN_POINTS),
            b(league::DRAW_POINTS),
            b(league::MATCH_CATEGORY),
            this + league::TIEBREAKS,
            places.as_ptr() as usize,
            read_i16(this + league::YEAR) as u16 as u32,
            0,
            id as u16 as u32,
            b(league::IMPORTANCE),
            4,
            0,
            read_i16(this + league::LEAGUE_FLAGS) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_usize(read_usize(this + league::STAGES), stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    let f = read_usize(read_usize(this + league::STAGES));
    if f == 0 {
        show_error_at(0xad4afc, 0x36b);
        return 0;
    }
    write_u32(this + league::CURRENT_STAGE, 0);
    exe::league_send_group_draw_news(f);
    1
}

// ponytail: the stage is allocated and the news goes to the managers: it cannot run twice. Only the original; the
// plan: returns 1 with the stage in [`league::STAGES`][0], `league::CURRENT_STAGE` = 0, the stage with n teams and the
// qualifying's year and, in its table (`league::TABLE`, `league_table_row::SIZE`-byte rows), the national teams of the
// odd places of the list (in any order: the constructor sorts the table).
unsafe extern "thiscall" fn compare_wc_oceania_league_create_second_group_stage(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_OCEANIA_LEAGUE_CREATE_SECOND_GROUP_STAGE);
    let list = read_usize(this + 0xef);
    if list == 0 {
        return o(this);
    }
    let n = 4 + (read_u8(this + wc_oceania_league_fields::EXEMPT_COUNT) != 1) as usize;
    let mut plan: Vec<usize> = (0..n).map(|i| read_usize(list + i * 8 + 4)).collect();
    let r = o(this);
    if r == 1 {
        let f = read_usize(read_usize(this + league::STAGES));
        let t = if f != 0 { read_usize(f + league::TABLE) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..n).map(|k| read_usize(t + k * league_table_row::SIZE)).collect() } else { Vec::new() };
        a.sort_unstable();
        plan.sort_unstable();
        let ok = f != 0 && read_usize(this + league::CURRENT_STAGE) == 0 && read_i16(f + league::CLUB_COUNT) as usize == n && read_i16(f + league::YEAR) == read_i16(this + league::YEAR) && a == plan;
        SCOREBOARD.record(ok, || format!("wc_oceania_league_create_second_group_stage: the stage does not match the plan ({:x?} read {:x?})", plan, a));
    }
    r
}

// ---------- 285a leva: the inter-continental cups: the two clubs of the Intercontinental Cup and of the Interamerican Cup; the Merconorte predicted positions (world_club_cup.cpp, inter_amer_cup.cpp, con_merc_cup.cpp) ----------

/// Is the club's continent ([[`club::NATION`] + `nation::CONTINENT`]] (dword)) the one of the key `continent`?
unsafe fn intercontinental_club_in_continent(c: usize, continent: usize) -> bool {
    let n = read_usize(c + club::NATION);
    if n == 0 {
        return false;
    }
    let k = read_usize(n + nation::CONTINENT);
    k != 0 && read_usize(k) == read_usize(continent)
}

/// One place of an inter-continental cup. If the continent (key `continent`; the record [`G_CONTINENT_TABLE`] + 0xc6 ×
/// key) has a loaded league (0x6699d0(continent): push; add esp, 4 -> cdecl) or the cup's year (`cup::YEAR`) is 2001:
/// the latest club at place `place` of the competition of key `comp` (`exe::comp_util_get_latest_placed_club`(record,
/// place), cdecl; the winner is place 0), returning (club, true, false). Otherwise, the clubs (up to the first national
/// team, `exe::database_is_national_team`, cdecl) of the continent with a reputation (`club::REPUTATION`, word) above
/// `min_reputation`, in a local list; with any, the list sorted and shuffled with 1 pick
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 1), cdecl) and the first one, (club, false, false); with none,
/// the error box of line `line` and the competition's winner, which the original ALWAYS writes in place 1 (+6), even
/// when looking for place 0 (a defect: place 0 keeps whatever the allocated memory had); returns (club, false, true).
#[allow(clippy::too_many_arguments)]
unsafe fn intercontinental_cup_slot(this: usize, continent: usize, min_reputation: i16, comp: usize, place: u32, file: usize, line: u32) -> (usize, bool, bool) {
    let league_loaded: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x6699d0usize);
    let record = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(continent).wrapping_mul(0xc6));
    if league_loaded(record) != 0 || read_i16(this + cup::YEAR) == 0x7d1 {
        return (exe::comp_util_get_latest_placed_club(comp_record_of_key(comp), place), true, false);
    }
    let mut list: Vec<usize> = Vec::new();
    let mut c = read_usize(G_CLUB_TABLE);
    for _ in 0..(read_usize(G_CLUB_COUNT) as i32).max(0) {
        if exe::database_is_national_team(c) != 0 {
            break;
        }
        if read_i16(c + club::REPUTATION) > min_reputation && intercontinental_club_in_continent(c, continent) {
            list.push(c);
        }
        c += club::SIZE;
    }
    if !list.is_empty() {
        exe::comp_util_sort_and_shuffle_top_clubs(list.as_mut_ptr() as usize, list.len() as u32, 1);
        return (list[0], false, false);
    }
    show_error_at(file, line);
    (exe::comp_util_get_latest_placed_club(comp_record_of_key(comp), 0), false, true)
}

/// The start of both cups: `cup::TEAM_COUNT` = 2 and the team table `cup::TEAM_TABLE` of 2 six-byte teams
/// (`exe::crt_malloc`(0xc), cdecl; out of memory, the error box of line 0xc6 and −1).
unsafe fn intercontinental_cup_team_table(this: usize, file: usize) -> Option<usize> {
    write_i16(this + cup::TEAM_COUNT, 2);
    let t = exe::crt_malloc(0xc);
    write_usize(this + cup::TEAM_TABLE, t);
    if t == 0 {
        show_error_at(file, 0xc6);
        return None;
    }
    Some(t)
}

/// The end of both cups: bytes +4, +5, +0xa and +0xb of the team table zeroed; returns 1.
unsafe fn intercontinental_cup_finish(this: usize) -> i32 {
    for k in [4usize, 5, 0xa, 0xb] {
        write_u8(read_usize(this + cup::TEAM_TABLE) + k, 0);
    }
    1
}

/// The two Intercontinental Cup clubs. Place 0 is Europe's: the Champions League winner
/// ([`KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE`]), or a European club with reputation above 5,000 drawn when Europe
/// has no loaded league outside 2001 (the error box of line 0xea with none; a null winner: the error box of line 0xd1
/// and 0). Place 1 is South America's: the Libertadores winner ([`KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES`]) or
/// the draw in the same way (boxes at lines 0x12a and 0xff); when it is the winner and the winner is South American,
/// the Libertadores runner-up takes its place if it is South American too. Returns 1 (−1 out of memory). (Stack frame
/// 0x11a4.)
/// Original: `WorldClubCup::pick_qualifiers` (0x92b8b0, world_club_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn world_club_cup_pick_qualifiers(this: usize) -> i32 {
    let Some(t) = intercontinental_cup_team_table(this, 0xad5848) else {
        return -1;
    };
    let (europe, by_winner, fallback) = intercontinental_cup_slot(this, KEY_CONTINENT_EUROPE, 0x1388, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, 0, 0xad5848, 0xea); // cqu13: the European champion
    write_usize(t + if fallback { 6 } else { 0 }, europe);
    if by_winner {
        covered!("cqu_13 european club");
        if europe == 0 {
            show_error_at(0xad5848, 0xd1);
            return 0;
        }
    }
    let (south, by_winner, _) = intercontinental_cup_slot(this, KEY_CONTINENT_SOUTH_AMERICA, 0x1388, KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES, 0, 0xad5848, 0x12a);
    write_usize(read_usize(this + cup::TEAM_TABLE) + 6, south);
    if by_winner {
        if south == 0 {
            show_error_at(0xad5848, 0xff);
            return 0;
        }
        if intercontinental_club_in_continent(south, KEY_CONTINENT_SOUTH_AMERICA) {
            let runner_up = exe::comp_util_get_latest_placed_club(comp_record_of_key(KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES), 1);
            if intercontinental_club_in_continent(runner_up, KEY_CONTINENT_SOUTH_AMERICA) {
                write_usize(read_usize(this + cup::TEAM_TABLE) + 6, runner_up);
            }
        }
    }
    intercontinental_cup_finish(this)
}

// ponytail: the draw (in Europe or South America without a league) and the allocated table cannot run twice. Only the
// original; the plan: returns 1, `cup::TEAM_COUNT` = 2, neither club null, the first from one continent and the second
// from the other, the status bytes zeroed.
unsafe fn intercontinental_cup_check_clubs(name: &str, this: usize, continents: (usize, usize), o: impl FnOnce() -> i32) -> i32 {
    let r = o();
    if r == 1 {
        let t = read_usize(this + cup::TEAM_TABLE);
        let (a, b) = (read_usize(t), read_usize(t + 6));
        let bytes = [4usize, 5, 0xa, 0xb].iter().all(|&k| read_u8(t + k) == 0);
        let ok = read_i16(this + cup::TEAM_COUNT) == 2 && a != 0 && b != 0 && intercontinental_club_in_continent(a, continents.0) && intercontinental_club_in_continent(b, continents.1) && bytes;
        SCOREBOARD.record(ok, || format!("{}({:#x}): clubs {:#x} {:#x} status zeroed {}", name, this, a, b, bytes));
    }
    r
}

unsafe extern "thiscall" fn compare_world_club_cup_pick_qualifiers(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_WORLD_CLUB_CUP_PICK_QUALIFIERS);
    intercontinental_cup_check_clubs("world_club_cup_pick_qualifiers", this, (KEY_CONTINENT_EUROPE, KEY_CONTINENT_SOUTH_AMERICA), || o(this))
}

/// The two Interamerican Cup clubs. Place 0 is North America's: the CONCACAF Champions Cup winner
/// ([`KEY_CLUB_COMP_CONCACAF_CHAMPIONS_CUP`]), or a North American club with reputation above 1,000 drawn when it has
/// no loaded league outside 2001 (boxes at lines 0xea and 0xd1); place 1 is South America's: the Libertadores winner
/// ([`KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES`]) or the draw above 5,000 (boxes at lines 0x118 and 0xff).
/// Returns 1 (0 with a null winner; −1 out of memory). (Stack frame 0x11a4.)
/// Original: `InterAmerCup::select_teams` (0x632480, inter_amer_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn inter_amer_cup_select_teams(this: usize) -> i32 {
    let Some(t) = intercontinental_cup_team_table(this, 0x9e9d94) else {
        return -1;
    };
    let (north, by_winner, fallback) = intercontinental_cup_slot(this, KEY_CONTINENT_NORTH_AMERICA, 0x3e8, KEY_CLUB_COMP_CONCACAF_CHAMPIONS_CUP, 0, 0x9e9d94, 0xea); // cqu13: the CONCACAF champion
    write_usize(t + if fallback { 6 } else { 0 }, north);
    if by_winner {
        covered!("cqu_13 north american club");
        if north == 0 {
            show_error_at(0x9e9d94, 0xd1);
            return 0;
        }
    }
    let (south, by_winner, _) = intercontinental_cup_slot(this, KEY_CONTINENT_SOUTH_AMERICA, 0x1388, KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES, 0, 0x9e9d94, 0x118); // cqu13: the Libertadores champion in the Interamerican Cup
    write_usize(read_usize(this + cup::TEAM_TABLE) + 6, south);
    if by_winner {
        covered!("cqu_13 interamerican south american club");
        if south == 0 {
            show_error_at(0x9e9d94, 0xff);
            return 0;
        }
    }
    intercontinental_cup_finish(this)
}

unsafe extern "thiscall" fn compare_inter_amer_cup_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_INTER_AMER_CUP_SELECT_TEAMS);
    intercontinental_cup_check_clubs("inter_amer_cup_select_teams", this, (KEY_CONTINENT_NORTH_AMERICA, KEY_CONTINENT_SOUTH_AMERICA), || o(this))
}

/// The calls of the CONMEBOL cups' predicted positions (0x4c2a90, 0x4c4210 and 0x4c5a90, the same body), in order: the
/// invited list (+0xee, +0xf2 clubs) already sorted; the rounds of the table `table` (`cup_round::SIZE`-byte records;
/// the word `cup_round::TIES` of each, those leaving in it), from the last to the first: the last gives position 1 to
/// those leaving in it and, like all of them, the running position (which goes up with each club) to as many again;
/// those remaining go in blocks of `group` with the position of the block's start. Returns (club, position).
unsafe fn con_merc_cup_predicted_plan(this: usize, table: usize, rounds: i16, group: u32) -> Vec<(usize, u8)> {
    let mut v = Vec::new();
    let list = read_usize(this + 0xee);
    let mut i: i16 = 0;
    let mut bl: u8 = 1;
    let mut current: u8 = 1;
    let mut r = rounds as i32 - 1;
    while r >= 0 {
        let p = table + r as usize * cup_round::SIZE + cup_round::TIES;
        if r == rounds as i32 - 1 {
            let mut j: i16 = 0;
            while j < read_i16(p) {
                v.push((read_usize(list + i as isize as usize * 4), current));
                i = i.wrapping_add(1);
                bl = bl.wrapping_add(1);
                j += 1;
            }
        }
        current = bl;
        let mut j: i16 = 0;
        while j < read_i16(p) {
            v.push((read_usize(list + i as isize as usize * 4), current));
            i = i.wrapping_add(1);
            bl = bl.wrapping_add(1);
            j += 1;
        }
        r -= 1;
    }
    let mut pos = bl;
    while (i as i32) < read_usize(this + 0xf2) as i32 {
        v.push((read_usize(list + i as isize as usize * 4), pos));
        bl = bl.wrapping_add(1);
        if bl as u32 == pos as u32 + group {
            pos = bl;
        }
        i = i.wrapping_add(1);
    }
    v
}

/// The predicted positions body: without statistics (+8), the error box `line`; the list +0xee of +0xf2 invited clubs
/// sorted by reputation (`exe::crt_qsort`(list, n, 4, 0x4b07d0), cdecl); the round table of stage `stage` through
/// `comp_vtable::CREATE_SCHEDULE` (push 0; push &extra; push &rounds; push stage; mov ecx, this -> thiscall, ret
/// 0x10); each club of the plan with its position (`exe::comp_stats_set_club_final_position`(stats, club, position),
/// thiscall, ret 8; the original pushes the byte with garbage in the high bytes, the routine reads the byte); the table
/// freed (`exe::crt_free`).
unsafe fn con_merc_cup_predicted_positions(this: usize, stage: u32, group: u32, file: usize, line: u32) {
    if read_usize(this + 8) == 0 {
        return show_error_at(file, line);
    }
    exe::crt_qsort(read_usize(this + 0xee), read_usize(this + 0xf2) as u32, 4, 0x4b07d0usize);
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = create_schedule(this, stage, &mut rounds, &mut extra, 0);
    for (c, p) in con_merc_cup_predicted_plan(this, table, rounds, group) {
        exe::comp_stats_set_club_final_position(read_usize(this + 8), c, p as u32);
    }
    exe::crt_free(table);
}

/// The Merconorte predicted positions, by the rounds of stage 3 (the knockout) and in blocks of 4 for those staying in
/// the groups (the error box at line 0x32e).
/// Original: `ConMercCup::set_predicted_positions` (0x4c2a90, con_merc_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn con_merc_cup_set_predicted_positions(this: usize) {
    con_merc_cup_predicted_positions(this, 3, 4, 0x9a7d6c, 0x32e)
}

// ponytail: Comp_Stats::set_club_final_position adds the club's entry (it reallocates): it cannot run twice. Only the
// original; the plan, with the list already sorted by it: each invited club with an entry and the positions
// (`comp_stats_club::FINAL_POSITION`, `exe::comp_stats_find_club_entry`) rising in list order, the first 1. (Asking
// for the stage's round table again moves the cup's calendar: the 07/11 comparison showed the Cup.cpp line 1278 box;
// the plan calls nothing but the lookup.)
unsafe fn con_merc_cup_check_predicted(name: &str, this: usize, _stage: u32, _group: u32, o: impl FnOnce()) {
    o();
    let stats = read_usize(this + 8);
    let list = read_usize(this + 0xee);
    let n = read_usize(this + 0xf2) as i32;
    if stats == 0 || list == 0 || !(1..=256).contains(&n) {
        return;
    }
    let mut read: Vec<i32> = Vec::new();
    for k in 0..n as usize {
        let c = read_usize(list + k * 4);
        let e = if c != 0 { exe::comp_stats_find_club_entry(stats, c) } else { 0 };
        read.push(if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) as i32 } else { -1 });
    }
    let ok = read[0] == 1 && read.iter().all(|&x| x >= 1) && read.windows(2).all(|w| w[0] <= w[1]);
    SCOREBOARD.record(ok, || format!("{}({:#x}): positions {:?}", name, this, read));
}

unsafe extern "thiscall" fn compare_con_merc_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CON_MERC_CUP_SET_PREDICTED_POSITIONS);
    con_merc_cup_check_predicted("con_merc_cup_set_predicted_positions", this, 3, 4, || o(this))
}

// ---------- 285a leva: the Brazilian and J2 season turnover: promotion and relegation between the state leagues and their lower divisions, the 16 clubs entering the third division and the J2 turnover (bra_nat_first.cpp, jap_j2.cpp) ----------

/// The keys of the twelve state leagues (the league object in [[`G_COMP_OBJECT_TABLE`] + key × 4]) and those of each
/// one's lower division (the record in [`G_CLUB_COMP_TABLE`] + key × 0x6b), in the original's order.
const BRA_STATE_LEAGUE_KEYS: [usize; 12] = [
    KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI,
    KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP,
];
const BRA_STATE_LOWER_DIVISION_KEYS: [usize; 12] = [
    KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_LOWER_DIVISION,
    KEY_CLUB_COMP_BRAZILIAN_PARANA_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_LOWER_DIVISION,
    KEY_CLUB_COMP_BRAZILIAN_BAHIA_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_GOIAS_LOWER_DIVISION,
    KEY_CLUB_COMP_BRAZILIAN_NORTH_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_LOWER_DIVISION,
];

/// The competition object (league) of key `key`: [[`G_COMP_OBJECT_TABLE`] + 4 × [key]].
unsafe fn comp_object_of_key(key: usize) -> usize {
    read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_usize(key))
}

/// What a club leaving a division goes through in the original (both relegations): if it cannot be managed
/// (`exe::club_can_be_managed`, cdecl, returns al = 0) and a human manager decides for it
/// (`exe::human_manager_is_human_club`(club, 1, 1, 0, 0), cdecl), the manager leaves
/// (`exe::manager_manager_remove_club_manager`([`G_MANAGER_MANAGER`], club, 1, 7, 0), thiscall, ret 0x10); then the
/// relegation clauses of the contracts (`exe::contract_manager_trigger_relegation_clauses`([`G_CONTRACT_MANAGER`],
/// club, the record of the division it left), thiscall, ret 8).
unsafe fn bra_nat_first_club_left_division(club: usize, record: usize) {
    if exe::club_can_be_managed(club) == 0 && exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), club, 1, 7, 0);
    }
    exe::contract_manager_trigger_relegation_clauses(read_usize(G_CONTRACT_MANAGER), club, record);
}

/// At the Brazilian season turnover, promotion and relegation between the twelve state leagues
/// (`BRA_STATE_LEAGUE_KEYS`) and their twelve lower divisions (`BRA_STATE_LOWER_DIVISION_KEYS`); as many clubs go up
/// from each lower division as its state league's `league::RELEGATED_COUNT` (signed byte).
/// 1) A list of 200 clubs (`exe::crt_malloc` of 0x320; out of memory: the error box of line 0x3d0 and return) with the
///    database clubs whose regional division (`club::RESERVE_DIVISION`) is one of the twelve lower ones, in database
///    order, up to 200.
/// 2) The list sorted by reputation and the first 75 shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n,
///    25), cdecl: it draws).
/// 3) In list order, each club goes up to its division's state league while there are places
///    (`club::RESERVE_DIVISION` = the state league's record, its `league::COMPETITION`); if it has a national division
///    (`club::DIVISION`) and no manager (`club::MANAGER` = 0), it opens the job (`exe::manager_manager_add_job_vacancy`
///    ([`G_MANAGER_MANAGER`], club, 0), thiscall, ret 8).
/// 4) In each state league, the clubs of the last `league::RELEGATED_COUNT` table rows (`exe::league_get_table_row`)
///    go down to the lower division (`club::RESERVE_DIVISION` = its record) and leave the division
///    (`bra_nat_first_club_left_division`, with the state league's record).
/// 5) The list is freed (`exe::crt_free`).
/// Original: `bra_nat_first_exchange_state_league_clubs` (0x42b130, bra_nat_first.cpp, cdecl).
unsafe extern "C" fn bra_nat_first_exchange_state_league_clubs() {
    let leagues: [usize; 12] = core::array::from_fn(|i| comp_object_of_key(BRA_STATE_LEAGUE_KEYS[i]));
    let lower: [usize; 12] = core::array::from_fn(|i| comp_record_of_key(BRA_STATE_LOWER_DIVISION_KEYS[i]));
    let mut places: [i8; 12] = core::array::from_fn(|i| read_i8(leagues[i] + league::RELEGATED_COUNT));
    let list = exe::crt_malloc(0x320);
    if list == 0 {
        return show_error_at(0x98aa30, 0x3d0);
    }
    let mut n = 0i16;
    let mut i = 0usize;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 && n < 0xc8 {
        let c = club_at(i as isize).addr();
        let d = read_usize(c + club::RESERVE_DIVISION);
        if d != 0 && lower.contains(&d) {
            write_usize(list + 4 * n as usize, c);
            n += 1;
        }
        i += 1;
    }
    exe::comp_util_sort_and_shuffle_top_clubs(list, n as u16 as u32, 0x19);
    for k in 0..n.max(0) as usize { // cqv3: the list in order
        let c = read_usize(list + 4 * k);
        let d = read_usize(c + club::RESERVE_DIVISION);
        let e = match lower.iter().position(|&b| b == d) {
            Some(e) => e,
            None => continue, // ponytail: every club of the list belongs to one of the twelve
        };
        if places[e] > 0 { // cqv3: the state league's promotion places
            covered!("cqv_3 promotion to the state league");
            write_usize(c + club::RESERVE_DIVISION, read_usize(leagues[e] + league::COMPETITION));
            if read_usize(c + club::DIVISION) != 0 && read_usize(c + club::MANAGER) == 0 {
                exe::manager_manager_add_job_vacancy(read_usize(G_MANAGER_MANAGER), c, 0usize);
            }
            places[e] -= 1;
        }
    }
    for e in 0..12 {
        let l = leagues[e];
        let total = read_i16(l + league::CLUB_COUNT) as i32;
        let mut s = total - read_i8(l + league::RELEGATED_COUNT) as i32;
        while s < read_i16(l + league::CLUB_COUNT) as i32 {
            let c = read_usize(exe::league_get_table_row(l, s));
            write_usize(c + club::RESERVE_DIVISION, lower[e]); // cqv3: the relegated club goes to the lower division
            covered!("cqv_3 relegated from the state league");
            bra_nat_first_club_left_division(c, read_usize(l + league::COMPETITION));
            s += 1;
        }
    }
    exe::crt_free(list);
}

/// The promotion comparer: runs only the original (it moves clubs between divisions, draws the shuffle and calls the
/// manager and contract code; it cannot run twice), with the plan made before: the clubs of each lower division in the
/// list (up to 200) and those of the last rows of each state league. Afterwards: exactly min(places, its clubs in the
/// list) clubs go up from each division (`club::RESERVE_DIVISION` = the state league's record) and its others stay;
/// those of the last rows are in the lower division.
unsafe extern "C" fn compare_bra_nat_first_exchange_state_league_clubs() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_BRA_NAT_FIRST_EXCHANGE_STATE_LEAGUE_CLUBS);
    let leagues: [usize; 12] = core::array::from_fn(|i| comp_object_of_key(BRA_STATE_LEAGUE_KEYS[i]));
    let lower: [usize; 12] = core::array::from_fn(|i| comp_record_of_key(BRA_STATE_LOWER_DIVISION_KEYS[i]));
    let mut theirs: Vec<Vec<usize>> = vec![Vec::new(); 12];
    let mut n = 0;
    let mut i = 0usize;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 && n < 0xc8 {
        let c = club_at(i as isize).addr();
        if let Some(e) = lower.iter().position(|&b| b == read_usize(c + club::RESERVE_DIVISION) && b != 0) {
            theirs[e].push(c);
            n += 1;
        }
        i += 1;
    }
    let places: Vec<i32> = leagues.iter().map(|&l| read_i8(l + league::RELEGATED_COUNT) as i32).collect();
    let records: Vec<usize> = leagues.iter().map(|&l| read_usize(l + league::COMPETITION)).collect();
    let mut going_down: Vec<(usize, usize)> = Vec::new();
    for e in 0..12 {
        let l = leagues[e];
        let t = read_i16(l + league::CLUB_COUNT) as i32;
        for s in (t - places[e]).max(0)..t {
            going_down.push((read_usize(exe::league_get_table_row(l, s)), lower[e]));
        }
    }
    o();
    let mut errors = Vec::new();
    for e in 0..12 {
        let went_up = theirs[e].iter().filter(|&&c| read_usize(c + club::RESERVE_DIVISION) == records[e]).count();
        let stayed = theirs[e].iter().filter(|&&c| read_usize(c + club::RESERVE_DIVISION) == lower[e]).count();
        let expected = (places[e].max(0) as usize).min(theirs[e].len());
        if went_up != expected || went_up + stayed != theirs[e].len() {
            errors.push(format!("division {}: {} went up (expected {}), {} of {} stayed", e, went_up, expected, stayed, theirs[e].len()));
        }
    }
    for &(c, b) in &going_down {
        if read_usize(c + club::RESERVE_DIVISION) != b {
            errors.push(format!("relegated {:#x} in {:#x} (expected {:#x})", c, read_usize(c + club::RESERVE_DIVISION), b));
        }
    }
    SCOREBOARD.record(errors.is_empty(), || format!("bra_nat_first_exchange_state_league_clubs: {}", errors.join("; ")));
}

/// The third division league object and its record.
unsafe fn bra_nat_first_third_division() -> (usize, usize) {
    (comp_object_of_key(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION), comp_record_of_key(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION))
}

/// The Brazilian clubs without a national division, in database order, up to 160: a club (outside the national teams:
/// the ids below [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`]) with a nation (`club::NATION`), no division
/// (`club::DIVISION` = 0) and Brazil's nation (the nation id = [`KEY_NATION_BRAZIL`]).
unsafe fn bra_nat_first_clubs_without_division() -> Vec<usize> {
    let mut v = Vec::new();
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 - 2 * read_usize(G_NATION_COUNT) as i32 && v.len() < 0xa0 {
        let c = club_at(i as isize).addr();
        let nation = read_usize(c + club::NATION);
        if nation != 0 && read_usize(c + club::DIVISION) == 0 && read_usize(nation) == read_usize(KEY_NATION_BRAZIL) {
            v.push(c);
        }
        i += 1;
    }
    v
}

/// At the Brazilian season turnover, the third division renews itself.
/// 1) A list of 160 (`exe::crt_malloc` of 0x280; out of memory: the error box of line 0x43a and return) with the
///    Brazilian clubs without a national division (`bra_nat_first_clubs_without_division`). Fewer than 16: the list is
///    freed and nothing changes.
/// 2) The list sorted by reputation and the first 48 shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n,
///    16), cdecl: it draws); the first 16 enter the third division (`club::DIVISION` = the third division's record) and
///    those with a regional division (`club::RESERVE_DIVISION`) and no manager (`club::MANAGER` = 0) open the job
///    (`exe::manager_manager_add_job_vacancy`([`G_MANAGER_MANAGER`], club, 0), thiscall, ret 8).
/// 3) In the third division's table and in those of its seven groups (stages 0 to 6 of `league::STAGES`), the clubs of
///    the rows marked `league_table_row::STATUS` = 3 (eliminated in the first stage [inference]) leave the national
///    division (`club::DIVISION` = 0) and leave the division (`bra_nat_first_club_left_division`, with the record of the
///    table they leave).
/// 4) The list is freed (`exe::crt_free`).
/// Original: `bra_nat_first_refresh_third_division_clubs` (0x42b520, bra_nat_first.cpp, cdecl).
unsafe extern "C" fn bra_nat_first_refresh_third_division_clubs() {
    let (t, record) = bra_nat_first_third_division();
    let list = exe::crt_malloc(0x280);
    if list == 0 {
        return show_error_at(0x98aa30, 0x43a);
    }
    let v = bra_nat_first_clubs_without_division();
    for (k, &c) in v.iter().enumerate() {
        write_usize(list + 4 * k, c);
    }
    if v.len() < 0x10 {
        return exe::crt_free(list);
    }
    exe::comp_util_sort_and_shuffle_top_clubs(list, v.len() as u32, 0x10);
    for k in 0..0x10 { // cqv3: the 16 entering the third division
        let c = read_usize(list + 4 * k);
        write_usize(c + club::DIVISION, record);
        if read_usize(c + club::RESERVE_DIVISION) != 0 && read_usize(c + club::MANAGER) == 0 {
            exe::manager_manager_add_job_vacancy(read_usize(G_MANAGER_MANAGER), c, 0usize);
        }
    }
    covered!("cqv_3 third division renewed");
    for g in -1i32..7 {
        let s = if g == -1 { t } else { read_usize(read_usize(t + league::STAGES) + 4 * g as usize) };
        let mut e = 0i32;
        while e < read_i16(s + league::CLUB_COUNT) as i32 {
            let r = exe::league_get_table_row(s, e);
            if read_u8(r + league_table_row::STATUS) == 3 {
                let c = read_usize(r + league_table_row::CLUB);
                write_usize(c + club::DIVISION, 0); // cqv3: the eliminated club leaves the third division
                bra_nat_first_club_left_division(c, read_usize(s + league::COMPETITION));
            }
            e += 1;
        }
    }
    exe::crt_free(list);
}

/// The renewal comparer: runs only the original (it draws the shuffle and calls the manager and contract code; it
/// cannot run twice), with the plan made before: the clubs without a division (up to 160) and those of the rows
/// marked 3 in the eight tables. Afterwards, with 16 or more without a division: exactly 16 of them are in the third
/// division and the others without a division; the marked ones, without a division. With fewer than 16: nothing
/// changed for them.
unsafe extern "C" fn compare_bra_nat_first_refresh_third_division_clubs() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_BRA_NAT_FIRST_REFRESH_THIRD_DIVISION_CLUBS);
    let (t, record) = bra_nat_first_third_division();
    let v = bra_nat_first_clubs_without_division();
    let mut marked = Vec::new();
    if v.len() >= 0x10 {
        for g in -1i32..7 {
            let s = if g == -1 { t } else { read_usize(read_usize(t + league::STAGES) + 4 * g as usize) };
            for e in 0..read_i16(s + league::CLUB_COUNT) as i32 {
                let r = exe::league_get_table_row(s, e);
                if read_u8(r + league_table_row::STATUS) == 3 {
                    marked.push(read_usize(r + league_table_row::CLUB));
                }
            }
        }
    }
    o();
    let entered = v.iter().filter(|&&c| read_usize(c + club::DIVISION) == record).count();
    let outside = v.iter().filter(|&&c| read_usize(c + club::DIVISION) == 0).count();
    let expected = if v.len() >= 0x10 { 0x10 } else { 0 };
    let left = marked.iter().all(|&c| read_usize(c + club::DIVISION) == 0);
    SCOREBOARD.record(entered == expected && entered + outside == v.len() && left,
                     || format!("bra_nat_first_refresh_third_division_clubs: {} without a division, {} entered (expected {}), {} stayed out, {} marked, all out: {}", v.len(), entered, expected, outside, marked.len(), left));
}

/// The J2 season turnover. `league::SEASON_CLOSED` = 0 and the table freed (`exe::league_free_table`(this, 0)); the
/// round dates (`league::ROUND_DATES`) freed (`exe::crt_free`) and zeroed; with statistics (`league::COMP_STATS`),
/// `exe::comp_stats_archive_season_and_reset`(stats, 1); the year (`league::YEAR`, word) + 1; then, each with its own
/// error box (returning 0; the original returns the box's eax, which nobody reads) if it returns 0: the rules
/// (`comp_vtable::SET_UP_RULES`, thiscall, ret 0; line 0x196), the table (JapJ2::create_table 0x665030, thiscall, ret 0;
/// 0x19d), the fixtures (`exe::league_create_fixtures`; 0x1a3) and the sort (`exe::league_sort_table`(this, 0); 0x1aa);
/// the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations
/// (`exe::league_set_club_season_expectations`) and the referee renewal (OfficialsManager::renew_officials
/// `FN_OFFICIALSMANAGER_RENEW_OFFICIALS`, thiscall([`G_OFFICIALS_MANAGER`], `league::COMPETITION`), ret 4), whose eax it
/// returns. The J2 has no stages besides the league.
/// Original: `JapJ2::start_new_season` (0x6651f0, jap_j2.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn jap_j2_start_new_season(this: usize) -> u32 {
    jap_j2_start_new_season_years(this, 1) // cqv3: the J2's next year
}

/// The body of `jap_j2_start_new_season`, moving the year by `years`.
unsafe fn jap_j2_start_new_season_years(this: usize, years: i16) -> u32 {
    write_u32(this + league::SEASON_CLOSED, 0);
    exe::league_free_table(this, 0usize);
    let l = read_usize(this + league::ROUND_DATES);
    if l != 0 {
        exe::crt_free(l);
        write_usize(this + league::ROUND_DATES, 0);
    }
    let stats = read_usize(this + league::COMP_STATS);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let a = read_i16(this + league::YEAR);
    write_i16(this + league::YEAR, a.wrapping_add(years));
    covered!("cqv_3 j2 turnover");
    let set_up_rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_UP_RULES));
    if set_up_rules(this) == 0 {
        show_error_at(0x9eaedc, 0x196);
        return 0;
    }
    let create_table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x665030usize);
    if create_table(this) == 0 {
        show_error_at(0x9eaedc, 0x19d);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x9eaedc, 0x1a3);
        return 0;
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(0x9eaedc, 0x1aa);
        return 0;
    }
    let set_predicted_positions: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
    set_predicted_positions(this);
    exe::league_set_club_season_expectations(this);
    // the full eax is returned, so the u8 wrapper `exe::officialsmanager_renew_officials` is not used
    let renew_officials: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_OFFICIALSMANAGER_RENEW_OFFICIALS);
    renew_officials(read_usize(G_OFFICIALS_MANAGER), read_usize(this + league::COMPETITION)) // cqv3: the J2 referee renewal
}

/// The turnover comparer: runs only the original (it frees the table and the round dates, allocates the new ones and
/// draws the fixtures and referees; it cannot run twice), checking what only it guarantees: the year + 1, the same
/// statistics block and, with the rules, table and fixtures built, the new table (`league::TABLE`).
unsafe extern "thiscall" fn compare_jap_j2_start_new_season(this: usize) -> u32 {
    let before = read_i16(this + league::YEAR);
    let stats = read_usize(this + league::COMP_STATS);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_J2_START_NEW_SEASON)(this);
    let after = read_i16(this + league::YEAR);
    let same = after == before.wrapping_add(1) && read_usize(this + league::COMP_STATS) == stats && read_usize(this + league::TABLE) != 0;
    SCOREBOARD.record(same, || format!("jap_j2_start_new_season({:#x}) year before={} after={} +8={:#x} +0xb1={:#x}", this, before, after, read_usize(this + league::COMP_STATS), read_usize(this + league::TABLE)));
    r
}
