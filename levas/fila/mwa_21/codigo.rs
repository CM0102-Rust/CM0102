// ---------- batch %%LEVA%%: the Irish Super Cup: the constructor, the rules and the schedule (ire_super_cup.cpp) ----------

/// The Irish Super Cup's league rules: a single round robin of four clubs (one meeting), 3 points for a win and 1 for a
/// draw, match category 2, the tie-breaks 4, 1, 2 and 3 (head-to-head [inference], goal difference, goals for and the
/// row byte), no promotion or relegation places; no prize fund written.
pub(crate) const IRISH_SUPER_CUP_RULES: GroupStageRules = GroupStageRules { win: 3, draw: 1, category: 2, tiebreak: [4, 1, 2, 3], slots: 0, prize: None, bits: None }; // mwa21: the Super Cup tie-breaks

/// The body of `construct_ire_super_cup`.
///
/// Over League::League (`exe::new_league`): `league::COMPETITION` = the record, the vtable 0x96c0c8, `league::YEAR`, 7
/// substitutes on the bench and 5 substitutions, discipline rule `discipline_rule::IRELAND`. If the saved state loads
/// (`exe::league_load_from_temp_file`(this, 1); ≠ 0) it returns this. Otherwise `league::STAGE_INDEX` =
/// `stage_index::MAIN`, `league::CURRENT_STAGE` = −1, no stages; the rules written inline with the round table
/// (`set_cup_league_rules` with `IRISH_SUPER_CUP_RULES` and one meeting: the same fields the original writes, then the
/// virtual `comp_vtable::CREATE_SCHEDULE` into `league::ROUND_DATES`); `league::CLUB_COUNT` = 0, `league::TABLE` = 0 and
/// `league::ACTIVE` = 0 (the clubs come later, `ire_super_cup_update_for_date`); the statistics (`ita_cups_create_comp_stats`;
/// without them, the error box at line 0x51). Returns this.
pub(crate) unsafe fn construct_irish_super_cup(this: League, year: u32, record: usize) -> usize {
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), 0x96c0c8);
    this.set_year(year as u16 as i16);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(5);
    this.set_discipline_rule(discipline_rule::IRELAND);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("mwa_21 irish super cup new");
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    this.set_stage_count(0);
    this.set_stages(0);
    set_cup_league_rules(this, &IRISH_SUPER_CUP_RULES, 1);
    this.set_club_count(0);
    this.set_table(0);
    write_u32(this.addr() + league::ACTIVE, 0);
    if !ita_cups_create_comp_stats(this) {
        show_error_at(0x9ea0b8, 0x51);
    }
    this.addr()
}

/// Rule: the Irish Super Cup at career creation: the year and the database competition, 7 substitutes on the bench and
/// 5 substitutions, the Irish discipline rules; with nothing saved, its rules (a four-club round robin) and its three
/// dates, with no clubs yet (they come once the European places are known), and the statistics.
/// Original: `IreSuperCup::IreSuperCup` (0x63acd0, ire_super_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_super_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_super_cup(League(this), year, record) // mwa21: the Irish Super Cup
}

unsafe extern "thiscall" fn compare_construct_ire_super_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    let r = o(this, year, record);
    let fields = [(league::STAGE_INDEX, 1u8, 0xff), (league::SUBSTITUTES_NAMED, 1, 7), (league::SUBSTITUTIONS_ALLOWED, 1, 5), (league::DISCIPLINE_RULE, 1, 0x10),
        (league::CLUB_COUNT, 2, 0), (league::TABLE, 4, 0), (league::ACTIVE, 4, 0), (league::STAGE_COUNT, 4, 0)];
    let error = ita_cups_check_constructor_plan(this, r, year, record, 0x96c0c8, &fields, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_ire_super_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// Rule: the Irish Super Cup is a single round robin of four clubs: 3 points for a win and 1 for a draw, ties split by
/// the head-to-head [inference], goal difference and goals for; category 2; no promotion or relegation.
///
/// `set_cup_league_rules` with `IRISH_SUPER_CUP_RULES` and one meeting (ItaCup::set_up_rules's template: the fields, then
/// the round table). Returns 1.
/// Original: `IreSuperCup::set_up_rules` (0x63b4c0, ire_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_super_cup_set_up_rules(this: usize) -> u32 {
    covered!("mwa_21 irish super cup rules");
    set_cup_league_rules(League(this), &IRISH_SUPER_CUP_RULES, 1)
}

// simplification: the virtual +0x3c allocates the round table: it does not run twice. Only the original; the plan
// checks the rules.
unsafe extern "thiscall" fn compare_ire_super_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this);
    check_cup_league_rules("ire_super_cup_set_up_rules", League(this), &IRISH_SUPER_CUP_RULES, r);
    r
}

/// The Irish Super Cup's three dates: 27 June (weekday 2), 29 June (weekday 4) and 1 July (weekday 6) of the cup's year;
/// no TV slots.
pub(crate) const IRISH_SUPER_CUP_ROUNDS: [FraThirdLeagueRound; 3] = [
    (27, 5, 0, 2, 2, &[(-1, -1, -1)]), // mwa21: the Super Cup opens on 27 June
    (29, 5, 0, 4, 2, &[(-1, -1, -1)]),
    (1, 6, 0, 6, 1, &[(-1, -1, -1)]),
];

pub(crate) const IRISH_SUPER_CUP_SCHEDULE: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 3, file: 0x9ea0b8, row: 0x91, table: &IRISH_SUPER_CUP_ROUNDS, branch: "mwa_21 irish super cup schedule" };

/// Rule: the Irish Super Cup is played over three matchdays: 27 June, 29 June and 1 July of its year, each moved to its
/// weekday.
///
/// The single-stage league schedule (`create_league_schedule`, FraThird::create_schedule's template: only stage −1;
/// `out` = 1, three rounds, `extra` = 0; the table of 3 × 0x41 bytes, out of memory the error box at line 0x91) with
/// `IRISH_SUPER_CUP_ROUNDS`. Returns the table.
/// Original: `IreSuperCup::create_schedule` (0x63af60, ire_super_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_super_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    create_league_schedule(League(this), stage, rounds, extra, out, &IRISH_SUPER_CUP_SCHEDULE)
}

unsafe extern "thiscall" fn compare_ire_super_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    compare_cfa_schedule("ire_super_cup_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| ire_super_cup_create_schedule(this, stage, r, e, l))
}
