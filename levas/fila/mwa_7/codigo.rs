// ---------- batch %%LEVA%%: the constructors of the Irish Challenge, Leinster and Munster cups (ire_chal_cup.cpp, ire_leinster_cup.cpp, ire_munster_cup.cpp) ----------

/// What changes between the Irish cups' constructors (the Cup template of JapEmpCup and SpaSuper, read instruction by
/// instruction): the vtable, the bench (`cup::SUBSTITUTES_NAMED`, `cup::SUBSTITUTIONS_ALLOWED`; None keeps the base's),
/// the discipline rule, the entrant routine (thiscall, ret 0), whether a step fails on a negative return (otherwise on
/// zero), the prize fund written before the round table (None keeps the base's), whether the final ranking runs, the
/// source file and the three error box lines.
pub(crate) struct IrishCupConstructor {
    pub(crate) vtable: usize,
    pub(crate) bench: Option<(u8, u8)>,
    pub(crate) discipline: u8,
    pub(crate) entrants: usize,
    pub(crate) fails_when_negative: bool,
    pub(crate) prize: Option<i32>,
    pub(crate) rank: bool,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 3],
}

/// The body of the Irish cups' constructors (thiscall, ret 8: this, year, the competition record).
///
/// Over the base Cup::Cup (`exe::new_cup`, thiscall, ret 0): Comp +4 = the record, the vtable, `cup::YEAR` = the year
/// (word), `cup::CURRENT_ROUND` = 0, `cup::STAGE_INDEX` = `stage_index::MAIN` (0xff), `cup::CURRENT_STAGE` = −1,
/// `cup::STAGE_COUNT` = 0, `cup::CLUB_SIDES` = 1, the bench, the discipline rule and `cup::RUN_BY_STAGES` = 0. If the
/// saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx -> thiscall, ret 4; ≠ 0) it returns this. Otherwise,
/// in order: the entrants (the cup's routine; when it fails, the error box at the first line); the prize fund; the
/// round table (`jap_emp_cup_create_rounds`: the cup's own virtual `comp_vtable::CREATE_SCHEDULE` into
/// `cup::ROUND_TABLE`); the tie tree (Cup::map_fixture_tree 0x518790, thiscall, ret 0; failing, the second line); the
/// statistics at Comp +8 (`jap_emp_cup_create_comp_stats`; without them, the third line); the final ranking
/// (Cup::rank_teams 0x5223a0, thiscall, ret 0, unchecked) when `rank`. Each box returns this. Returns this.
pub(crate) unsafe fn construct_irish_cup(this: Cup, year: u32, record: usize, k: &IrishCupConstructor, branch: &'static str) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), k.vtable);
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_stage_count(0);
    this.set_club_sides(1);
    if let Some((named, allowed)) = k.bench {
        this.set_substitutes_named(named);
        this.set_substitutions_allowed(allowed);
    }
    this.set_discipline_index(k.discipline);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    let failed = |r: i32| if k.fails_when_negative { r < 0 } else { r == 0 };
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(k.entrants);
    if failed(entrants(this.addr())) {
        show_error_at(k.file, k.rows[0]);
        return this.addr();
    }
    if let Some(p) = k.prize {
        this.set_prize_fund(p);
    }
    jap_emp_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    if failed(map_fixture_tree(this.addr())) {
        show_error_at(k.file, k.rows[1]);
        return this.addr();
    }
    if !jap_emp_cup_create_comp_stats(this) {
        show_error_at(k.file, k.rows[2]);
        return this.addr();
    }
    if k.rank {
        let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
        rank_teams(this.addr());
    }
    this.addr()
}

/// The comparer of the Irish cups' constructors: only the original runs (it allocates, draws and reads the save file),
/// and the plan is checked (`jap_emp_cup_check_constructor_plan`): this, the vtable, Comp +4, the year and, on a new
/// cup, the fixed fields the constructor writes, the round table and the statistics.
pub(crate) unsafe fn compare_irish_cup_constructor(name: &str, this: usize, year: u32, record: usize, k: &IrishCupConstructor, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut fields = vec![(cup::STAGE_COUNT, 4u8, 0u32), (cup::CURRENT_STAGE, 4, 0xffff_ffff), (cup::CLUB_SIDES, 1, 1), (cup::STAGE_INDEX, 1, 0xff),
        (cup::DISCIPLINE_INDEX, 1, k.discipline as u32), (cup::CURRENT_ROUND, 1, 0), (cup::RUN_BY_STAGES, 1, 0)];
    if let Some((named, allowed)) = k.bench {
        fields.push((cup::SUBSTITUTES_NAMED, 1, named as u32));
        fields.push((cup::SUBSTITUTIONS_ALLOWED, 1, allowed as u32));
    }
    if let Some(p) = k.prize {
        fields.push((cup::PRIZE_FUND, 4, p as u32));
    }
    let error = jap_emp_cup_check_constructor_plan(Cup(this), r, year, record, k.vtable, &fields, true);
    SCOREBOARD.record(error.is_none(), || format!("{}({:#x}, {}, {:#x}): {}", name, this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The FAI Senior Challenge Cup's constructor: the vtable 0x96bc2c, 5 substitutes on the bench and 3 substitutions,
/// discipline rule `discipline_rule::IRELAND`, the entrants IreChalCup::collect_entrants (0x6347d0; fails on 0), the
/// 100,000 prize fund, the final ranking; the error box lines 0x43, 0x51 and 0x58 of ire_chal_cup.cpp.
pub(crate) const IRISH_CHALLENGE_CUP_CONSTRUCTOR: IrishCupConstructor = IrishCupConstructor { vtable: 0x96bc2c, bench: Some((5, 3)), discipline: discipline_rule::IRELAND,
    entrants: 0x6347d0, fails_when_negative: false, prize: Some(0x186a0), rank: true, file: 0x9e9e94, rows: [0x43, 0x51, 0x58] };

/// Rule: the FAI Senior Challenge Cup is a cup of one nation's clubs of the game's year, with a 100,000 prize fund, 5
/// substitutes on the bench and 3 substitutions, under the Irish discipline rules; with nothing saved, its 42 entrants,
/// the rounds, the tie tree, the statistics and the final ranking are made.
///
/// `construct_irish_cup` with `IRISH_CHALLENGE_CUP_CONSTRUCTOR` (checked instruction by instruction against
/// JapEmpCup::JapEmpCup 0x661d30: only the vtable, the discipline rule, the entrant routine, the prize, the file and the
/// lines change).
/// Original: `IreChalCup::IreChalCup` (0x6340a0, ire_chal_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_chal_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_cup(Cup(this), year, record, &IRISH_CHALLENGE_CUP_CONSTRUCTOR, "mwa_7 irish challenge cup new") // mwa7: the Irish Challenge Cup
}

unsafe extern "thiscall" fn compare_construct_ire_chal_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_irish_cup_constructor("construct_ire_chal_cup", this, year, record, &IRISH_CHALLENGE_CUP_CONSTRUCTOR, || o(this, year, record))
}

/// The Leinster Senior Cup's constructor: the vtable 0x96bd80, the same as the Challenge Cup's otherwise (5 and 3,
/// Irish rules, 100,000, the final ranking), with IreLeinsterCup::select_clubs (0x636a80) and the lines 0x44, 0x52 and
/// 0x59 of ire_leinster_cup.cpp.
pub(crate) const LEINSTER_SENIOR_CUP_CONSTRUCTOR: IrishCupConstructor = IrishCupConstructor { vtable: 0x96bd80, bench: Some((5, 3)), discipline: discipline_rule::IRELAND,
    entrants: 0x636a80, fails_when_negative: false, prize: Some(0x186a0), rank: true, file: 0x9e9f0c, rows: [0x44, 0x52, 0x59] };

/// Rule: the Leinster Senior Cup is a cup of one nation's clubs of the game's year, with a 100,000 prize fund, 5
/// substitutes on the bench and 3 substitutions, under the Irish discipline rules; with nothing saved, its 16 clubs, the
/// rounds, the tie tree, the statistics and the final ranking are made.
///
/// `construct_irish_cup` with `LEINSTER_SENIOR_CUP_CONSTRUCTOR` (checked instruction by instruction against
/// JapEmpCup::JapEmpCup).
/// Original: `IreLeinsterCup::IreLeinsterCup` (0x6364a0, ire_leinster_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_leinster_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_cup(Cup(this), year, record, &LEINSTER_SENIOR_CUP_CONSTRUCTOR, "mwa_7 leinster cup new") // mwa7: the Leinster cup
}

unsafe extern "thiscall" fn compare_construct_ire_leinster_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_irish_cup_constructor("construct_ire_leinster_cup", this, year, record, &LEINSTER_SENIOR_CUP_CONSTRUCTOR, || o(this, year, record))
}

/// The Munster Senior Cup's constructor: the vtable 0x96bed4, the same as the Challenge Cup's otherwise, with
/// IreMunsterCup::collect_entrants (0x638e30) and the lines 0x43, 0x51 and 0x58 of ire_munster_cup.cpp.
pub(crate) const MUNSTER_SENIOR_CUP_CONSTRUCTOR: IrishCupConstructor = IrishCupConstructor { vtable: 0x96bed4, bench: Some((5, 3)), discipline: discipline_rule::IRELAND,
    entrants: 0x638e30, fails_when_negative: false, prize: Some(0x186a0), rank: true, file: 0x9e9f84, rows: [0x43, 0x51, 0x58] };

/// Rule: the Munster Senior Cup is a cup of one nation's clubs of the game's year, with a 100,000 prize fund, 5
/// substitutes on the bench and 3 substitutions, under the Irish discipline rules; with nothing saved, its 20 entrants,
/// the rounds, the tie tree, the statistics and the final ranking are made.
///
/// `construct_irish_cup` with `MUNSTER_SENIOR_CUP_CONSTRUCTOR` (checked instruction by instruction against
/// JapEmpCup::JapEmpCup).
/// Original: `IreMunsterCup::IreMunsterCup` (0x6387a0, ire_munster_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_munster_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_cup(Cup(this), year, record, &MUNSTER_SENIOR_CUP_CONSTRUCTOR, "mwa_7 munster cup new") // mwa7: the Munster cup
}

unsafe extern "thiscall" fn compare_construct_ire_munster_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_irish_cup_constructor("construct_ire_munster_cup", this, year, record, &MUNSTER_SENIOR_CUP_CONSTRUCTOR, || o(this, year, record))
}
