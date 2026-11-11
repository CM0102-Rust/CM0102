// ---------- batch %%LEVA%%: the constructors of the Brazilian third division, J2 and MLS (the rules, the table, the fixtures, the statistics, the groups and the predicted positions at creation) (bra_nat_third.cpp, jap_j2.cpp, usa_mls.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    bra_nat_third_compare_league_ctor("construct_bra_nat_third", League(this), year, record, &BRA_NAT_THIRD_LEAGUE, discipline_rule::BRAZIL_REGIONAL, || o(this, year, record))
}

/// Comparer for `construct_jap_j2`.
pub(crate) unsafe extern "thiscall" fn compare_construct_jap_j2(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    bra_nat_third_compare_league_ctor("construct_jap_j2", League(this), year, record, &JAP_J2_LEAGUE, discipline_rule::JAPAN_LEAGUE, || o(this, year, record))
}

/// Comparer for `construct_usa_mls`.
pub(crate) unsafe extern "thiscall" fn compare_construct_usa_mls(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    bra_nat_third_compare_league_ctor("construct_usa_mls", League(this), year, record, &USA_MLS_LEAGUE, discipline_rule::USA, || o(this, year, record))
}
