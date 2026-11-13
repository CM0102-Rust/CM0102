// ---------- batch %%LEVA%%: the Northern Irish League Cup: the constructor, the first group's table and the rules (nir_lge_cup.cpp) ----------

/// The Northern Irish League Cup's constructor: the vtable 0x96dc18, discipline rule
/// `discipline_rule::NORTHERN_IRELAND`, four stages (three groups and the knockout), NirLgeCup's collect_entrants
/// (0x78d800), set_predicted_positions (0x78dcd0), set_up_rules (0x78d0d0), create_table (0x78cfd0) and
/// create_group_stage (0x78d3c0), nir_lge_cup.cpp.
pub(crate) const NORTHERN_IRISH_LEAGUE_CUP_CONSTRUCTOR: IrishLeagueCupConstructor = IrishLeagueCupConstructor { vtable: 0x96dc18, discipline: discipline_rule::NORTHERN_IRELAND, stages: 4, groups: 3,
    entrants: 0x78d800, predicted: 0x78dcd0, rules: 0x78d0d0, first_group: 0x78cfd0, group_stage: 0x78d3c0, file: 0xa69698 };

/// Rule: the Northern Irish League Cup at career creation: the year and the database competition, 5 substitutes on the
/// bench and 3 substitutions, the Northern Irish discipline rules, importance 12; with nothing saved, four stages
/// (three groups and the knockout), the 20 entrants, the statistics, the predicted positions, the rules, the first
/// group's table (the cup is itself the league of the first group), its fixtures and order and the stages of the three
/// groups.
///
/// `construct_irish_league_cup` with `NORTHERN_IRISH_LEAGUE_CUP_CONSTRUCTOR` (checked instruction by instruction against
/// IreLgeCup::IreLgeCup: only the vtable, the discipline rule, the stage and group counts, the routines and the file).
/// Original: `NirLgeCup::NirLgeCup` (0x78c780, nir_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nir_lge_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league_cup(League(this), year, record, &NORTHERN_IRISH_LEAGUE_CUP_CONSTRUCTOR, "mwa_17 northern irish league cup new") // mwa17: the Northern Irish League Cup
}

unsafe extern "thiscall" fn compare_construct_nir_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_irish_league_cup_constructor("construct_nir_lge_cup", this, year, record, &NORTHERN_IRISH_LEAGUE_CUP_CONSTRUCTOR, || o(this, year, record))
}

/// Rule: the first group of the Northern Irish League Cup has five clubs, the first five entrants, which leave the
/// waiting list; the group draw news follows.
///
/// `create_first_group_table`(5 clubs; out of memory, the error box at line 0x17a of nir_lge_cup.cpp) (ItaCCup's
/// init_first_group template, checked instruction by instruction: only the file and the line). Returns 1.
/// Original: `NirLgeCup::create_table` (0x78cfd0, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_create_table(this: usize) -> u32 {
    create_first_group_table(League(this), 5, 0xa69698, 0x17a, "mwa_17 northern irish league cup first group")
}

unsafe extern "thiscall" fn compare_nir_lge_cup_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_first_group_table("nir_lge_cup_create_table", League(this), 5, || o(this))
}

/// The Northern Irish League Cup's group rules: 3 points for a win and 1 for a draw, the tie-breaks 1, 2 and 4, the top
/// two go through, category 4, a 100,000 prize fund.
pub(crate) const NORTHERN_IRISH_LEAGUE_CUP_GROUP_RULES: GroupStageRules = GroupStageRules { win: 3, draw: 1, category: 4, tiebreak: [1, 2, 4, 0], slots: 2, prize: Some(0x186a0), bits: None }; // mwa17: two through per group

/// Rule: the Northern Irish League Cup groups are single round robins: 3 points for a win and 1 for a draw, ties split
/// by goal difference, goals for and the head-to-head [inference]; the top two go through; category 4 (national cup);
/// no promotion or relegation; a 100,000 prize fund.
///
/// `set_cup_league_rules` with `NORTHERN_IRISH_LEAGUE_CUP_GROUP_RULES` and one meeting (ItaCup::set_up_rules's
/// template), then the round table. Returns 1.
/// Original: `NirLgeCup::set_up_rules` (0x78d0d0, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_set_up_rules(this: usize) -> u32 {
    covered!("mwa_17 northern irish league cup rules");
    set_cup_league_rules(League(this), &NORTHERN_IRISH_LEAGUE_CUP_GROUP_RULES, 1)
}

// simplification: the virtual +0x3c allocates the round table: it does not run twice. Only the original; the plan
// checks the rules.
unsafe extern "thiscall" fn compare_nir_lge_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    check_cup_league_rules("nir_lge_cup_set_up_rules", League(this), &NORTHERN_IRISH_LEAGUE_CUP_GROUP_RULES, r);
    r
}
