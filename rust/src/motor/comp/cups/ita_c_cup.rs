use super::*;

/// rule: the Serie C Cup starts with five-round groups in August (12, 19, 22, 26 and 29; id 0x420 + stage); the
/// knockout (stage 0xf) over two legs: 1st round drawn on 30/8 and played on 11/10 (32 teams), 2nd 26/10 and 1/11,
/// quarter-finals 9/11 and 15/11, semi-final 30/11 and 11 April, final draw 26/4 and match on 9 May; 14 days between
/// legs, 7 in the 2nd round and the final; category 4 up to the 2nd round and 6 after
/// Original: `ItaCCup::create_schedule` (0x63cf90, ita_c_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ita_c_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule(League(this), stage, rounds, extra, league, 0xf, 0x420, &SERIE_C_CUP_GROUP_ROUNDS, &SERIE_C_CUP_KNOCKOUT_ROUNDS, 0x9ea204, 0xbc, 0xe3,
        "cqo_10 grupos da copa da serie c", "cqo_10 mata-mata da copa da serie c")
}

/// rule: the Serie C Cup takes the 90 Italian clubs of C1 and C2 (the two C1 groups and the three C2 ones). Those that
/// came down from Serie B and those that came from C1 in 4th, 5th or 6th go to the last places (from the 81st); the
/// first 80 are shuffled, sorted by the city's longitude and each of their 8 tens shuffled (the last 10 places stay in
/// order): the groups are regional [inference]
/// Original: `ItaCCup::select_entrants` (0x63dfe0, ita_c_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_c_cup_select_entrants(this: usize) -> i32 {
    select_serie_c_cup_entrants(this).0
}

/// rule: the Serie C Italian Cup groups follow the Italian Cup rules (single round robin, 3/1, tiebreak 4, 1, 2, one
/// qualifier per group); prize fund of 300,000
/// Original: `ItaCCup::set_up_rules` (0x63d6e0, ita_c_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_c_cup_set_up_rules(this: usize) -> u32 {
    covered!("cqi_12 serie c cup rules");
    let rounds = 1i16; // cqi12: the single round-robin of the Serie C groups
    set_cup_league_rules(League(this), &SERIE_C_CUP_RULES, rounds)
}

/// The season turnover of the Serie C Italian Cup: the season archived, last season's stages and entrant list dropped,
/// the year plus one, the 90 Serie C entrants drawn again (ItaCCup::select_entrants), the predicted positions, the
/// rules, the first group of 5 (ItaCCup::init_first_group), its fixtures and order, and the 15 group stages
/// (ItaCCup::create_group_stage 0..14).
///
/// `ita_group_cup_start_new_season` with 0x63dfe0, 0x63d5e0, 0x63d9e0, 15 groups and the ita_c_cup.cpp boxes at lines
/// 0x212, 0x219, 0x21f and 0x226.
/// Original: `ItaCCup::start_new_season` (0x63d780, ita_c_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_c_cup_start_new_season(this: usize) -> u32 {
    ita_group_cup_start_new_season(League(this), 0x63dfe0, 0x63d5e0, 0x63d9e0, 15, 0x9ea204, [0x212, 0x219, 0x21f, 0x226], 1, "mwf_9 serie c cup rollover") // mwf9: the Serie C cup turnover
}

/// Comparer for `ita_c_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go
/// up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_c_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_C_CUP_START_NEW_SEASON);
    compare_state_league_new_season("ita_c_cup_start_new_season", League(this), || o(this))
}

/// The predicted positions of the Serie C cup (at creation): the 90 entrants by reputation; the knockout of stage 0xf
/// gives 1 and 2 to the final's pair, then a shared place to the losers of each round; the rest in bands of 16
/// (`italian_cup_predicted_positions`, cqx_7; the boxes at lines 0x439 and 0x441).
/// Original: `ItaCCup::set_predicted_positions` (0x63e580, ita_c_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_c_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0x9ea204, (0x439, 0x441), 0xf, 0x10, "cqx_8 serie c cup predicted") // cqx8: the Serie C cup bands of 16
}

/// Comparer for `ita_c_cup_set_predicted_positions` (`compare_italian_cup_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_ita_c_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_C_CUP_SET_PREDICTED_POSITIONS);
    compare_italian_cup_predicted("ita_c_cup_set_predicted_positions", this, ita_c_cup_set_predicted_positions, || o(this));
}

/// Rule: in the Serie C cup, as in the Coppa Italia, with the groups below stage 0xf and the group qualification at
/// round 0x14 (weight 0x28) (`italian_cup_staff_records`, cqx_7).
/// Original: `ItaCCup::update_staff_records_for_status` (0x63e270, ita_c_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn ita_c_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    italian_cup_staff_records(this, club, outcome, stage, game, final_, 0xf, 0x14, "cqx_8 serie c cup group qualified")
}

pub(crate) unsafe extern "thiscall" fn compare_ita_c_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_ITA_C_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("ita_c_cup_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}

/// The Serie C cup constructor: see `ita_c_cup_constructor_body`.
/// Original: `ItaCCup::ItaCCup` (0x63cc40, ita_c_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ita_c_cup(this: usize, year: u32, owner: usize) -> usize {
    ita_c_cup_constructor_body(this, year, owner) // cqv8: the Serie C cup
}

/// Only the original (it allocates, draws and reads the save file), checking the plan (`ita_cups_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_ita_c_cup(this: usize, year: u32, owner: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ITA_C_CUP)(this, year, owner);
    let error = ita_cups_check_constructor_plan(this, r, year, owner, 0x96c264, &SERIE_C_CUP_FIELDS, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_ita_c_cup({:#x}, {}, {:#x}): {}", this, year as u16, owner, error.clone().unwrap_or_default()));
    r
}

/// The constructor of the Serie C cup, on the League base (`exe::new_league`, thiscall, ret 0): the fixed fields (the
/// vtable 0x96c264, the competition record in `league::COMPETITION`, the year in `league::YEAR`, 3 substitutions
/// allowed, 7 substitutes named, discipline rule `ITALY_CUP` (18), importance 0xc); if the saved state loads
/// (`exe::league_load_from_temp_file`(this, 1), thiscall, ret 4; returns != 0), returns this. On a new cup, in order:
/// `league::LEAGUE_FLAGS` = 0x28, `league::STAGE_INDEX` = `stage_index::MAIN`, `league::CURRENT_STAGE` = -1,
/// `league::STAGE_COUNT` = 0x10 and the stage array in `league::STAGES` (`exe::crt_malloc` of 0x40, cdecl,
/// unchecked); the 90 entrants (`ita_c_cup_select_entrants`, 0x63dfe0, thiscall, ret 0, unchecked); the statistics in
/// `league::COMP_STATS` (the box for line 0x4f without them); the predicted positions (ItaCCup::set_predicted_positions,
/// 0x63e580, thiscall, ret 0, unchecked); the rules (`ita_c_cup_set_up_rules`, 0x63d6e0, thiscall, ret 0; the box for
/// line 0x59 if it returns 0); the first group table (`init_first_group`, 0x63d5e0, thiscall, ret 0; the box for line
/// 0x60 if it returns 0); the fixtures (`exe::league_create_fixtures`, thiscall, ret 0; the box for line 0x66 if it
/// returns 0); the table order (`exe::league_sort_table`(this, 0), thiscall, ret 4; the box for line 0x6d if it returns
/// 0); the stage of each group (ItaCCup::create_group_stage, 0x63d9e0, push i; mov ecx, this -> thiscall, ret 4) for
/// groups 0 to 14. Every failing step shows its box (`show_error_at`) and returns this. Returns this. Rule: no prize
/// fund (`league::PRIZE_FUND` stays as the base left it); 7 substitutes on the bench and 3 substitutions; discipline
/// rule 18 [from the competitions reference reading, section 2.1].
/// Original: `ItaCCup::ItaCCup` (0x63cc40, ita_c_cup.cpp, thiscall, ret 8).
pub(crate) unsafe fn ita_c_cup_constructor_body(this: usize, year: u32, owner: usize) -> usize {
    let this = League(this);
    let league = this;
    exe::new_league(league);
    write_usize(this.addr() + league::COMPETITION, owner);
    write_usize(this.addr(), 0x96c264);
    this.set_year((year as u16) as i16);
    league.set_substitutions_allowed(0x3);
    league.set_substitutes_named(0x7);
    league.set_discipline_rule(discipline_rule::ITALY_CUP);
    league.set_importance(0xc);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this.addr();
    }
    covered!("cqv_8 serie c cup new");
    league.set_league_flags(0x28);
    league.set_stage_index(stage_index::MAIN);
    this.set_current_stage((0xffffffff) as usize);
    this.set_stage_count((0x10) as usize);
    league.set_stages(exe::crt_malloc(0x40));
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_ITACCUP_SELECT_ENTRANTS);
    f(this.addr());
    if !ita_cups_create_comp_stats(league) {
        show_error_at(0x9ea204, 0x4f);
        return this.addr();
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_ITACCUP_SET_PREDICTED_POSITIONS);
    f(this.addr());
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_ITACCUP_SET_UP_RULES);
    let r = f(this.addr());
    if r == 0 {
        show_error_at(0x9ea204, 0x59);
        return this.addr();
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_ITACCUP_INIT_FIRST_GROUP);
    let r = f(this.addr());
    if r == 0 {
        show_error_at(0x9ea204, 0x60);
        return this.addr();
    }
    if exe::league_create_fixtures(league) == 0 {
        show_error_at(0x9ea204, 0x66);
        return this.addr();
    }
    if exe::league_sort_table(league, 0) == 0 {
        show_error_at(0x9ea204, 0x6d);
        return this.addr();
    }
    let f: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(FN_ITACCUP_CREATE_GROUP_STAGE);
    for i in 0..15 {
        f(this.addr(), i);
    }
    this.addr()
}

/// The fixed fields the Serie C cup constructor writes on a new cup (offset, width, value).
pub(crate) const SERIE_C_CUP_FIELDS: [(usize, u8, u32); 7] = [(league::STAGE_COUNT, 4, 0x10), (league::STAGE_INDEX, 1, 0xff), (league::SUBSTITUTES_NAMED, 1, 0x7), (league::SUBSTITUTIONS_ALLOWED, 1, 0x3), (league::DISCIPLINE_RULE, 1, 0x12), (league::IMPORTANCE, 1, 0xc), (league::LEAGUE_FLAGS, 2, 0x28)];
