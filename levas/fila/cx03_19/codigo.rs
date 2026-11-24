// ---------- cx03_19: the constructors of the Copa Libertadores and the Copa Mercosur, and the Mercosur rules (conmebol_liber.cpp, conmebol_merc.cpp) ----------

/// ConmebolLiber::set_up_rules (thiscall, ret 0; 0 = failure).
const FN_CONMEBOLLIBER_SET_UP_RULES: usize = 0x4c3890;

/// What differs between the two South American cups' constructors: the vtable, the stage count, whether the
/// Conmebol seeding moves to the new season first, the rules routine and the source file with the box lines of the
/// rules and the statistics.
pub(crate) struct SouthAmericanCup {
    pub(crate) vtable: usize,
    pub(crate) stages: usize,
    pub(crate) seeding: bool,
    pub(crate) rules: usize,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 2],
}

/// The Copa Libertadores: 8 stages, the seeding moved on, the boxes of lines 0x52 and 0x60.
pub(crate) const COPA_LIBERTADORES: SouthAmericanCup = SouthAmericanCup {
    vtable: 0x968bf4, stages: 8, seeding: true, rules: FN_CONMEBOLLIBER_SET_UP_RULES, file: 0x9a7de4, rows: [0x52, 0x60],
};

/// The Copa Mercosur: 5 stages, the boxes of lines 0x4a and 0x58.
pub(crate) const COPA_MERCOSUR: SouthAmericanCup = SouthAmericanCup {
    vtable: 0x968ca8, stages: 5, seeding: false, rules: 0x4c4f50, file: 0x9a7e5c, rows: [0x4a, 0x58],
};

/// The constructor of the two South American cups (`Libertadores(year, record)`, `Mercosur(year, record)`), run once, at
/// career creation; the same body in both (checked instruction by instruction), `SouthAmericanCup` sets them apart.
///
/// On the League base (`exe::new_league`, thiscall, ret 0): the competition record (`league::COMPETITION`), the vtable,
/// the year (`set_year`), `discipline_rule` 2 (the South American rules), the drawn club list (+0xee) = 0 and
/// `importance` 0xf. If the saved state loads (`exe::league_load_from_temp_file`: push 1 -> thiscall, ret 4; nonzero),
/// returns this. On a new cup, in order: `league_flags` 0x28, `stage_index` 0xff, `current_stage` −1, the stage count
/// and the stage vector (`exe::crt_malloc` of 4 per stage, unchecked); in the Libertadores the Conmebol seeding's new
/// season (ConmebolSeeding::update_for_new_season on `G_CONMEBOL_SEEDING` with 0, thiscall, ret 4); the rules (thiscall,
/// ret 0; 0: the box of `rows[0]` and this); `club_count` 0, `table` 0 and `active` 0; the statistics
/// (`create_comp_stats_at_8`; without them the box of `rows[1]`). Returns this.
pub(crate) unsafe fn build_south_american_cup(this: League, year: u32, record: usize, cup: &SouthAmericanCup, branch: &'static str) -> usize {
    exe::new_league(this);
    write_u32(this.addr() + league::COMPETITION, record as u32);
    write_u32(this.addr(), cup.vtable as u32);
    this.set_year(year as u16 as i16);
    this.set_discipline_rule(2);
    write_u32(this.addr() + conmebol_liber::DRAWN_CLUBS, 0);
    this.set_importance(0xf);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    this.set_league_flags(0x28);
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(cup.stages);
    this.set_stages(exe::crt_malloc(cup.stages as u32 * 4));
    if cup.seeding {
        let seed: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CONMEBOLSEEDING_UPDATE_FOR_NEW_SEASON); // return unused
        seed(read_usize(G_CONMEBOL_SEEDING), 0);
    }
    let rules: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(cup.rules);
    if rules(this.addr()) == 0 {
        show_error_at(cup.file, cup.rows[0]);
        return this.addr();
    }
    this.set_club_count(0);
    this.set_table(0);
    this.set_active(0);
    if !create_comp_stats_at_8(this.addr()) {
        show_error_at(cup.file, cup.rows[1]);
    }
    this.addr()
}

/// The Copa Libertadores at career creation: discipline rule 2 and importance 15; a new cup has eight stages, the
/// Conmebol seeding of the new season, the rules and the statistics (`build_south_american_cup`).
/// Original: `ConmebolLiber::ConmebolLiber` (0x4c2f50, conmebol_liber.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_conmebol_liber(this: usize, year: u32, record: usize) -> usize {
    build_south_american_cup(League(this), year, record, &COPA_LIBERTADORES, "cx03_19 libertadores new")
}

/// The Copa Mercosur at career creation: as the Libertadores, with five stages and no seeding step
/// (`build_south_american_cup`).
/// Original: `ConmebolMerc::ConmebolMerc` (0x4c46c0, conmebol_merc.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_conmebol_merc(this: usize, year: u32, record: usize) -> usize {
    build_south_american_cup(League(this), year, record, &COPA_MERCOSUR, "cx03_19 mercosur new")
}

/// The Copa Mercosur rules: the Merconorte's fields with 7 substitutes named and 3 substitutions allowed, one qualifier
/// per group, the start on day 15 of month 5 [inference: 15 June] and the prize fund of 2,000,000.
pub(crate) const MERCOSUR_RULES: ContinentalGroupRules = ContinentalGroupRules {
    fields: &[(0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0x3c, 2, 2), (0xc6, 1, 2), (0x42, 1, 2), (0xc3, 1, 1), (0xc4, 1, 7), (0xc5, 1, 1),
        (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x1c, 4, 0xffff_ffff), (0x20, 4, 0xffff_ffff), (0x49, 1, 7)],
    day: 0xf, month: 5, week: 0xffff_ffff, prize: Some(0x1e8480),
};

/// rule: the Copa Mercosur groups are home and away, 3 points a win and 1 a draw, tiebreak by codes 1, 2 and 3, one
/// club going through directly from each group, clubs from several nations, category 7, 7 substitutes named and 3
/// changes; the season starts on day 15 of month 5 of the competition year [inference: 15 June]; prize fund of 2,000,000
/// (`apply_cup_rules` with `MERCOSUR_RULES`: the fields, the start day word, its year offset 0, the prize and the
/// round table from virtual +0x3c into `league::ROUND_DATES`). Returns 1.
/// Original: `ConmebolMerc::set_up_rules` (0x4c4f50, conmebol_merc.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_set_up_rules(this: usize) -> u32 {
    covered!("cx03_19 mercosur rules");
    apply_cup_rules(League(this), &MERCOSUR_RULES, MERCOSUR_RULES.day, 3)
}

/// The fixed fields the South American cups' constructors leave on a new cup, for the comparers.
pub(crate) const fn south_american_cup_ctor_fields(stages: u32) -> [(usize, u8, u32); 10] {
    [(league::STAGE_COUNT, 4, stages), (league::CURRENT_STAGE, 4, 0xffff_ffff), (league::CLUB_COUNT, 2, 0), (league::STAGE_INDEX, 1, 0xff), (league::ACTIVE, 4, 0),
        (league::DISCIPLINE_RULE, 1, 2), (league::IMPORTANCE, 1, 0xf), (league::TABLE, 4, 0), (league::LEAGUE_FLAGS, 2, 0x28), (conmebol_liber::DRAWN_CLUBS, 4, 0)]
}

/// Comparer for `construct_conmebol_liber`: only the original runs (it allocates and reads the save file), then the
/// plan is checked (`cup_constructor_plan_mismatch`, without the round table).
pub(crate) unsafe extern "thiscall" fn compare_construct_conmebol_liber(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, COPA_LIBERTADORES.vtable, &south_american_cup_ctor_fields(8), false);
    SCOREBOARD.record(error.is_none(), || format!("construct_conmebol_liber({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// Comparer for `construct_conmebol_merc` (as above).
pub(crate) unsafe extern "thiscall" fn compare_construct_conmebol_merc(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, COPA_MERCOSUR.vtable, &south_american_cup_ctor_fields(5), false);
    SCOREBOARD.record(error.is_none(), || format!("construct_conmebol_merc({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// Comparer for `conmebol_merc_set_up_rules`: only the original runs (virtual +0x3c allocates the round table); the plan
/// checks the rules (`check_cup_rules`).
pub(crate) unsafe extern "thiscall" fn compare_conmebol_merc_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    check_cup_rules("conmebol_merc_set_up_rules", League(this), &MERCOSUR_RULES, || o(this))
}
