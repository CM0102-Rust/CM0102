use super::*;

/// Rule: the Irish Super Cup at career creation: the year and the database competition, 7 substitutes on the bench and
/// 5 substitutions, the Irish discipline rules; with nothing saved, its rules (a four-club round robin) and its three
/// dates, with no clubs yet (they come once the European places are known), and the statistics.
/// Original: `IreSuperCup::IreSuperCup` (0x63acd0, ire_super_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_super_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_super_cup(League(this), year, record) // mwa21: the Irish Super Cup
}

pub(crate) unsafe extern "thiscall" fn compare_construct_ire_super_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_IRE_SUPER_CUP);
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

pub(crate) unsafe extern "thiscall" fn compare_ire_super_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_SUPER_CUP_SET_UP_RULES)(this);
    check_cup_league_rules("ire_super_cup_set_up_rules", League(this), &IRISH_SUPER_CUP_RULES, r);
    r
}

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

pub(crate) unsafe extern "thiscall" fn compare_ire_super_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_IRE_SUPER_CUP_CREATE_SCHEDULE);
    compare_cfa_schedule("ire_super_cup_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| ire_super_cup_create_schedule(this, stage, r, e, l))
}

/// Rule: the Irish Super Cup has four clubs: the Irish clubs in the Champions League, then those in the UEFA Cup, then
/// those in the Intertoto Cup, completed by the Premier Division clubs in database order.
/// Original: `IreSuperCup::create_table` (0x63b0c0, ire_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_super_cup_create_table(this: usize) -> i32 {
    create_irish_super_cup_table(League(this))
}

pub(crate) unsafe extern "thiscall" fn compare_ire_super_cup_create_table(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_IRE_SUPER_CUP_CREATE_TABLE);
    let plan = irish_super_cup_clubs(4);
    let r = o(this);
    let l = League(this);
    let ok = l.table() != 0 && l.club_count() == 4 && r == (plan.len() == 4) as i32 && plan.iter().enumerate().all(|(k, &c)| read_usize(l.table() + k * 0x3b) == c);
    SCOREBOARD.record(ok, || format!("ire_super_cup_create_table({:#x}) returned {}, plan {:x?}", this, r, plan));
    r
}

/// Rule: once the European places of the Super Cup's year are known (the UEFA seeding has moved to that year), the Irish
/// Super Cup takes its four clubs, draws its fixtures and order and its predicted positions, once; then the day goes on
/// as in any league.
///
/// The league's daily update (thiscall(date, flag), ret 8): when due (`irish_super_cup_due`), the table
/// (`ire_super_cup_create_table` 0x63b0c0; 0: the error box at line 0x101), the fixtures (`exe::league_create_fixtures`;
/// 0: the box at line 0x106) and the order (`exe::league_sort_table`(this, 0); 0: the box at line 0x10c), each box
/// going on to the next step; the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`) and `league::ACTIVE` = 1.
/// Then always League::update_for_date (`exe::league_update_for_date`(this, date, flag), thiscall, ret 8), whose return
/// it passes on.
/// Original: `IreSuperCup::update_for_date` (0x63b300, ire_super_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn ire_super_cup_update_for_date(this: usize, date: usize, flag: u32) -> u8 {
    let this = League(this);
    let l = this;
    if irish_super_cup_due(l, flag) { // mwa22: the Super Cup is set up once the seeding reaches its year
        covered!("mwa_22 irish super cup set up");
        if exe::iresupercup_create_table(this) == 0 {
            show_error_at(0x9ea0b8, 0x101);
        }
        if exe::league_create_fixtures(l) == 0 {
            show_error_at(0x9ea0b8, 0x106);
        }
        if exe::league_sort_table(l, 0usize) == 0 {
            show_error_at(0x9ea0b8, 0x10c);
        }
        bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
        this.set_active((1) as usize);
    }
    exe::league_update_for_date(l, date, flag)
}

pub(crate) unsafe extern "thiscall" fn compare_ire_super_cup_update_for_date(this: usize, date: usize, flag: u32) -> u8 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_IRE_SUPER_CUP_UPDATE_FOR_DATE);
    let l = this;
    let due = irish_super_cup_due(l, flag);
    let before = this.active() as u32;
    let r = o(this.addr(), date, flag);
    let after = this.active() as u32;
    let ok = if due { after == 1 && l.table() != 0 } else { after == before };
    SCOREBOARD.record(ok, || format!("ire_super_cup_update_for_date({:#x}) due {} active {} -> {}", this.addr(), due, before, after));
    r
}

/// Original: `IreSuperCup::start_new_season` (0x63b550, ire_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_super_cup_start_new_season(this: usize) {
    start_irish_super_cup_season(League(this), 1) // mwa22: the Irish Super Cup turnover
}

pub(crate) unsafe extern "thiscall" fn compare_ire_super_cup_start_new_season(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_IRE_SUPER_CUP_START_NEW_SEASON);
    compare_state_league_new_season("ire_super_cup_start_new_season", League(this), || {
        o(this);
        0
    });
}

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

/// The clubs of the Irish Super Cup table, in the original's order (the plan of the comparer too): three passes over
/// all `G_CLUB_COUNT` clubs taking the Irish clubs (`club::NATION` = `KEY_NATION_REPUBLIC_OF_IRELAND`) entered in the
/// Champions League, then the UEFA Cup, then the Intertoto Cup (`club::CONTINENTAL_COMP_ID`); then, while places
/// remain, the Premier Division clubs (`irish_cup_club_count` clubs, by division) in table order, without checking
/// whether they are already in (a defect of the original: with fewer than four clubs in Europe, a Premier club in
/// Europe enters twice). Up to `n` clubs.
pub(crate) unsafe fn irish_super_cup_clubs(n: i16) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::new();
    for key in [KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_INTER_TOTO_CUP] { // mwa22: the European entrants first
        let mut i = 0i16;
        while (i as i32) < read_i32(G_CLUB_COUNT) {
            let c = club_at(i as isize);
            let nation = c.nation().addr();
            if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_REPUBLIC_OF_IRELAND) && c.continental_comp_id() == read_u32(key) && (v.len() as i16) < n {
                v.push(c.addr());
            }
            if v.len() as i16 == n {
                break;
            }
            i = i.wrapping_add(1);
        }
    }
    let mut i = 0i16;
    while (i as i32) < irish_cup_club_count() && (v.len() as i16) < n {
        let c = club_at(i as isize);
        if irish_club_division_id(c) == Some(read_usize(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION)) {
            v.push(c.addr());
        }
        i = i.wrapping_add(1);
    }
    v
}

/// The body of `ire_super_cup_create_table`: `league::CLUB_COUNT` = 4; the table (`exe::crt_malloc` of 4 × 0x3b; out of
/// memory, the error box at line 0xb5 and −1); the clubs of `irish_super_cup_clubs` in rows 0 on (League::init_table_row
/// `exe::league_init_table_row`(this, row, club, 0, 0), thiscall, ret 0x10). Returns whether the four rows were filled.
pub(crate) unsafe fn create_irish_super_cup_table(this: League) -> i32 {
    this.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    this.set_table(t);
    if t == 0 {
        show_error_at(0x9ea0b8, 0xb5);
        return -1;
    }
    covered!("mwa_22 irish super cup table");
    let clubs = irish_super_cup_clubs(this.club_count());
    for (k, &c) in clubs.iter().enumerate() {
        exe::league_init_table_row(this, k as u8 as u32, c, 0, 0);
    }
    (clubs.len() as i16 == this.club_count()) as i32
}

/// Whether the Irish Super Cup is due today: not yet active (`league::ACTIVE` = 0), the UEFA seeding already in the
/// cup's year (the word `uefa_seeding::SEASON_YEAR` of [`G_UEFA_SEEDING`], zero-extended, equal to `league::YEAR`) and
/// the day's flag set.
pub(crate) unsafe fn irish_super_cup_due(this: League, flag: u32) -> bool {
    (this.active() as u32) == 0 && read_u16(read_usize(G_UEFA_SEEDING) + uefa_seeding::SEASON_YEAR) as i32 == this.year() as i32 && flag != 0
}

/// Rule: at the season turn the Irish Super Cup is cleared: the table and the round table freed, the statistics
/// archived, the stages destroyed, the year moved on and the rules (with the new dates) made again; it waits inactive
/// for its new clubs.
///
/// `league::SEASON_CLOSED` = 0; the table freed (`exe::league_free_table`(this, 0)); the round table freed and zeroed;
/// the season archived (when there are statistics); with the current stage ≥ 0, the stages 0 to the current one
/// destroyed (their virtual 0 with 1); the year up by `year`; `league::CURRENT_STAGE` = −1; the rules
/// (`comp_vtable::SET_UP_RULES`; 0: the error box at line 0x15d and return); `league::ACTIVE` = 0.
pub(crate) unsafe fn start_irish_super_cup_season(this: League, year: i16) {
    this.set_season_closed(0);
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
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    this.set_year(this.year().wrapping_add(year));
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        return show_error_at(0x9ea0b8, 0x15d);
    }
    covered!("mwa_22 irish super cup turnover");
    write_u32(this.addr() + league::ACTIVE, 0);
}

/// The Irish Super Cup's league rules: a single round robin of four clubs (one meeting), 3 points for a win and 1 for a
/// draw, match category 2, the tie-breaks 4, 1, 2 and 3 (head-to-head [inference], goal difference, goals for and the
/// row byte), no promotion or relegation places; no prize fund written.
pub(crate) const IRISH_SUPER_CUP_RULES: GroupStageRules = GroupStageRules { win: 3, draw: 1, category: 2, tiebreak: [4, 1, 2, 3], slots: 0, prize: None, bits: None }; // mwa21: the Super Cup tie-breaks

pub(crate) const IRISH_SUPER_CUP_SCHEDULE: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 3, file: 0x9ea0b8, row: 0x91, table: &IRISH_SUPER_CUP_ROUNDS, branch: "mwa_21 irish super cup schedule" };

/// The Irish Super Cup's three dates: 27 June (weekday 2), 29 June (weekday 4) and 1 July (weekday 6) of the cup's year;
/// no TV slots.
pub(crate) const IRISH_SUPER_CUP_ROUNDS: [FraThirdLeagueRound; 3] = [
    (27, 5, 0, 2, 2, &[(-1, -1, -1)]), // mwa21: the Super Cup opens on 27 June
    (29, 5, 0, 4, 2, &[(-1, -1, -1)]),
    (1, 6, 0, 6, 1, &[(-1, -1, -1)]),
];
