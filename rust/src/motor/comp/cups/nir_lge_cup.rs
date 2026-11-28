use super::*;

/// Rule: the Northern Irish League Cup's groups 2 to 4 (stages 0 to 2; the cup's own table is the first group): five
/// clubs each, the next entrants of the pool in order, in a league stage of their own where two places go through;
/// then the group draw news; out of memory the box of line 0x241, without the stage that of 0x25a (and 0).
///
/// `irish_league_cup_group_stage` (mwa_23; read instruction by instruction against IreLgeCup::create_group_stage: the
/// group size is fixed at 5 and the local place list holds 2; the pool scan tests the count before the first entry,
/// the same for a group of five).
/// Original: `NirLgeCup::create_group_stage` (0x78d3c0, nir_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    irish_league_cup_group_stage(League(this), g, 5, 2, 0xa69698, (0x241, 0x25a), "mwa_24 northern irish league cup group") // mwa24: two places through
}

/// Comparer for `nir_lge_cup_create_group_stage` (original only, the plan: `compare_irish_league_cup_group_stage`).
pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_NIR_LGE_CUP_CREATE_GROUP_STAGE);
    compare_irish_league_cup_group_stage("nir_lge_cup_create_group_stage", League(this), g, 5, || o(this, g))
}

/// Rule: in the four Northern Irish League Cup groups (stages below 3), a club going through gets the qualification
/// record (round 0x78, weight 40) and one going out the failed-to-qualify record (weight 40); in the knockout (stage 3)
/// a club going through gets the qualification by round and one going out the knocked-out record (weight 50 to 100
/// over 3 rounds), and the winner and the runner-up their final records.
///
/// `welsh_cup_staff_records` with 3 group stages and 3 knockout rounds (read instruction by instruction against
/// WelLgeCup::update_staff_records_for_status: only the group stage count, the knockout slot and the round count
/// change). Returns 0.
/// Original: `NirLgeCup::update_staff_records_for_status` (0x78d9c0, nir_lge_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    covered!("mwa_24 northern irish league cup staff records");
    welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 3, 3) // mwa24: the knockout is stage 3
}

pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_NIR_LGE_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("nir_lge_cup_update_staff_records_for_status({:#x}) returned {}", this, r));
    r
}

/// Rule: the Northern Irish League Cup's predicted positions (at creation and at the turnover): the 20 entrants by
/// reputation; the knockout (stage 3) gives 1 to the winner and 2 to the runner-up, then a shared place to the losers
/// of each earlier round; the rest in bands of 4; the board and press expectation of each club [inference].
///
/// `italian_cup_predicted_positions` with the knockout stage 3, bands of 4 and the boxes at lines 0x3bc and 0x3c4 of
/// nir_lge_cup.cpp (read instruction by instruction against IreLgeCup::set_predicted_positions: the registers are
/// allocated differently; only the file, the lines, the stage and the band change).
/// Original: `NirLgeCup::set_predicted_positions` (0x78dcd0, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0xa69698, (0x3bc, 0x3c4), 3, 4, "mwa_24 northern irish league cup predicted") // mwa24: the Northern Irish League Cup bands of 4
}

/// Comparer for `nir_lge_cup_set_predicted_positions` (`compare_italian_cup_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NIR_LGE_CUP_SET_PREDICTED_POSITIONS);
    compare_italian_cup_predicted("nir_lge_cup_set_predicted_positions", this, nir_lge_cup_set_predicted_positions, || o(this));
}

/// Rule: the Northern Irish League Cup's knockout (stage 3, built when the groups are over) takes the top two of each
/// of the four groups: the first and second of the cup's own table (the first group) and of each of the three group
/// stages, paired in order (winner, runner-up); out of memory the box of line 0x278.
///
/// The club vector (`exe::crt_malloc`(0x20)); slots 0 and 1 the first two rows of `league::TABLE`; slots 2 + 2i and
/// 3 + 2i the first and second rows of stage i's table (League::get_table_row `exe::league_get_table_row`(stage, row):
/// push row; mov ecx, stage -> thiscall, ret 4; the original fills the winners first, then the runners-up); then
/// `create_knockout_stage` (8 clubs, stage 3, no seeds, the box of line 0x297).
/// Original: `NirLgeCup::create_knockout_stage` (0x78d600, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_create_knockout_stage(this: usize) {
    let this = League(this);
    let teams = exe::crt_malloc(0x20);
    if teams == 0 {
        return show_error_at(0xa69698, 0x278);
    }
    covered!("mwa_26 northern irish league cup knockout");
    write_usize(teams, read_usize(this.table()));
    for i in 0..3usize {
        let stage = read_usize(this.stages() + 4 * i);
        write_usize(teams + 8 * (i + 1), read_usize(exe::league_get_table_row(stage, 0)));
    }
    write_usize(teams + 4, read_usize(this.table() + league_table_row::SIZE)); // mwa26: the first group's runner-up
    for i in 0..3usize {
        let stage = read_usize(this.stages() + 4 * i);
        write_usize(teams + 8 * (i + 1) + 4, read_usize(exe::league_get_table_row(stage, 1)));
    }
    create_knockout_stage(this, 8, teams, 3, core::ptr::null(), 0xa69698, 0x297);
}

/// Comparer for `nir_lge_cup_create_knockout_stage` (original only: it builds the stage and draws its first round;
/// `ita_cup_compare_knockout_stage` checks the stage in slot 3).
pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NIR_LGE_CUP_CREATE_KNOCKOUT_STAGE);
    ita_cup_compare_knockout_stage("nir_lge_cup_create_knockout_stage", League(this), 3, 0, || o(this));
}

/// Rule: the Northern Irish League Cup takes 20 Northern Irish clubs: those the database assigns to the cup's groups A
/// to D (through the club's reserve division), five a group, and, for the free places, the other Northern Irish clubs
/// with no group, in table order.
/// Original: `NirLgeCup::collect_entrants` (0x78d800, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_collect_entrants(this: usize) -> i32 {
    collect_northern_irish_league_cup_entrants(League(this)).0
}

/// Comparer for `nir_lge_cup_collect_entrants` (side by side: `compare_select_entrants`).
pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_NIR_LGE_CUP_COLLECT_ENTRANTS);
    compare_select_entrants("nir_lge_cup_collect_entrants", this, || o(this), || collect_northern_irish_league_cup_entrants(League(this)))
}

/// Rule: the Northern Irish League Cup's groups (stages up to 2) play five matchdays, every other week from 29 August
/// to 24 October (a double round robin of five clubs [inference]), each group a league stage (is_league = 1, label
/// 0x3e9 + stage); the knockout (stage 3) has eight clubs: quarter-finals on 31 October, two-legged semi-finals from 14
/// November and the final on 28 November.
///
/// `create_schedule` with the knockout at stage 3, the label 0x3e9, `NORTHERN_IRISH_LEAGUE_CUP_GROUP_ROUNDS`,
/// `NORTHERN_IRISH_LEAGUE_CUP_KNOCKOUT_ROUNDS` and the boxes at lines 0xbc and 0xe3 of nir_lge_cup.cpp (read
/// instruction by instruction against IreLgeCup::create_schedule: only the stage, the dates, the rounds and the lines
/// change). Above stage 3: 0.
/// Original: `NirLgeCup::create_schedule` (0x78cad0, nir_lge_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule(League(this), stage, rounds, extra, league, 3, 0x3e9, &NORTHERN_IRISH_LEAGUE_CUP_GROUP_ROUNDS, &NORTHERN_IRISH_LEAGUE_CUP_KNOCKOUT_ROUNDS, 0xa69698, 0xbc, 0xe3,
        "mwa_26 northern irish league cup groups", "mwa_26 northern irish league cup knockout rounds")
}

/// Comparer for `nir_lge_cup_create_schedule` (side by side: `ita_cup_compare_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_NIR_LGE_CUP_CREATE_SCHEDULE);
    ita_cup_compare_schedule("nir_lge_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| nir_lge_cup_create_schedule(this, stage, r, e, l), 3)
}

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

pub(crate) unsafe extern "thiscall" fn compare_construct_nir_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_NIR_LGE_CUP);
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

pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_LGE_CUP_CREATE_TABLE);
    compare_first_group_table("nir_lge_cup_create_table", League(this), 5, || o(this))
}

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

pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_LGE_CUP_SET_UP_RULES)(this);
    check_cup_league_rules("nir_lge_cup_set_up_rules", League(this), &NORTHERN_IRISH_LEAGUE_CUP_GROUP_RULES, r);
    r
}

/// Rule: the Northern Irish League Cup moves stage by stage through its three groups; when the groups are over (stage
/// 3) the eight-club knockout is built; past the last stage, the error box.
///
/// `advance_cup_stage` (checked instruction by instruction against ItaCup::advance_stage: only the file, the line
/// 0x21a, the end of the groups 3 and the knockout routine NirLgeCup::create_knockout_stage 0x78d600 change).
/// Original: `NirLgeCup::advance_stage` (0x78d330, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_advance_stage(this: usize) {
    advance_cup_stage(League(this), 0xa69698, 0x21a, 3, 0x78d600) // mwa19: the Northern Irish League Cup knockout after the three groups
}

pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NIR_LGE_CUP_ADVANCE_STAGE);
    ita_cup_compare_advance_stage("nir_lge_cup_advance_stage", League(this), || o(this))
}

/// Rule: at the season turn the Northern Irish League Cup is rebuilt: the table, the round table, the entrants and the
/// stages of the last edition are freed, the statistics archived, the year moves on; then the 20 entrants, the
/// predicted positions, the rules, the first group's table, its fixtures and order and the stages of the three groups.
///
/// `start_irish_league_cup_season` with NirLgeCup::collect_entrants (0x78d800), create_table (0x78cfd0),
/// create_group_stage (0x78d3c0), three groups and the error box lines 0x1ec, 0x1f3, 0x1f9 and 0x200 of
/// nir_lge_cup.cpp (checked instruction by instruction against IreLgeCup::start_new_season).
/// Original: `NirLgeCup::start_new_season` (0x78d160, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_start_new_season(this: usize) -> u32 {
    start_irish_league_cup_season(League(this), 0x78d800, 0x78cfd0, 0x78d3c0, 3, 0xa69698, [0x1ec, 0x1f3, 0x1f9, 0x200], 1) // mwa19: the Northern Irish League Cup turnover
}

pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_LGE_CUP_START_NEW_SEASON);
    compare_state_league_new_season("nir_lge_cup_start_new_season", League(this), || o(this))
}

/// Comparer for the Irish league cups' group stages: the entrants of the group planned beforehand (the first `n`
/// non-null pool entries below the count); only the original runs (it builds a stage with its fixtures and sends the
/// news); then the return 1, the stage in its slot with `n` clubs and the cup's year, the current stage, the planned
/// pool entries zeroed and, in the stage's table (rows of `league_table_row::SIZE`), the planned clubs (in any order).
pub(crate) unsafe fn compare_irish_league_cup_group_stage(name: &str, this: League, g: u32, n: u32, original: impl FnOnce() -> u32) -> u32 {
    let pool = read_usize(this.addr() + ita_cup_fields::ENTRANT_CLUBS);
    let mut plan = Vec::new();
    let mut slots = Vec::new();
    let mut i = 0i32;
    while (plan.len() as u32) < n && i < read_i32(this.addr() + ita_cup_fields::ENTRANT_CLUB_COUNT) {
        let c = read_usize(pool + 4 * i as usize);
        if c != 0 {
            plan.push(c);
            slots.push(i as usize);
        }
        i += 1;
    }
    let r = original();
    let f = League(read_usize(this.stages() + 4 * g as usize));
    let mut ok = r == 1 && !f.is_null() && this.current_stage() as u32 == g && f.club_count() as u32 == n && f.year() == this.year()
        && slots.iter().all(|&k| read_usize(pool + 4 * k) == 0);
    if ok && f.table() != 0 {
        let mut a: Vec<usize> = (0..plan.len()).map(|k| read_usize(f.table() + k * league_table_row::SIZE)).collect();
        let mut b = plan.clone();
        a.sort_unstable();
        b.sort_unstable();
        ok = a == b;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}, group {}) returned {}, stage {:#x}, current {}, plan {:x?}", name, this.addr(), g, r, f.addr(), this.current_stage() as i32, plan));
    r
}

/// The body of `nir_lge_cup_collect_entrants`; returns (the routine's return, the entries to compare).
///
/// `ita_cup_fields::ENTRANT_CLUB_COUNT` = 20; the pool (`exe::crt_calloc`(4, 20) into `ita_cup_fields::ENTRANT_CLUBS`;
/// none: the box of line 0x2b3 and 0). First pass over the clubs (`G_CLUB_COUNT`, a signed word index, in table order),
/// those of Northern Ireland (`club::NATION`'s first dword = [`KEY_NATION_NORTHERN_IRELAND`]) with a
/// `club::RESERVE_DIVISION` of id one of the Northern Irish groups A to D (`KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_*`): the
/// first free slot of that group's band of five (A 0 to 4, B 5 to 9, C 10 to 14, D 15 to 19; a full band: nothing).
/// Second pass, the Northern Irish clubs with no reserve division: the first free slot of the 20 (none: nothing).
/// Returns 1.
pub(crate) unsafe fn collect_northern_irish_league_cup_entrants(this: League) -> (i32, i32) {
    let t = this.addr();
    write_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT, 0x14);
    let v = exe::crt_calloc(4, 0x14);
    write_usize(t + ita_cup_fields::ENTRANT_CLUBS, v);
    if v == 0 {
        show_error_at(0xa69698, 0x2b3);
        return (0, 0);
    }
    let northern = |c: Club| !c.nation().is_null() && read_usize(c.nation().addr()) == read_usize(KEY_NATION_NORTHERN_IRELAND);
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let c = club_at(i as isize);
        let r = c.reserve_division();
        if northern(c) && !r.is_null() && read_usize(r.addr()) != 0 {
            let id = read_usize(r.addr());
            let band = if id == read_usize(KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_A) { Some((0, 5)) } // mwa26: the group A band
                else if id == read_usize(KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_B) { Some((5, 0xa)) }
                else if id == read_usize(KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_C) { Some((0xa, 0xf)) }
                else if id == read_usize(KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_D) { Some((0xf, 0x14)) }
                else { None };
            if let Some((lo, hi)) = band {
                if let Some(j) = first_free_entrant_slot(v, lo, hi) {
                    write_usize(v + 4 * j, c.addr());
                }
            }
        }
        i = i.wrapping_add(1);
    }
    covered!("mwa_26 northern irish league cup entrants");
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let c = club_at(i as isize);
        if northern(c) && c.reserve_division().is_null() {
            if let Some(j) = first_free_entrant_slot(v, 0, 0x14) {
                write_usize(v + 4 * j, c.addr()); // mwa26: a club with no group in the first free place
            }
        }
        i = i.wrapping_add(1);
    }
    (1, 0x14)
}

/// The Northern Irish League Cup's five group matchdays, every other week from 29 August to 24 October (weekday 2,
/// code 2: midweek [inference]); (day, month from 0, weekday, code).
pub(crate) const NORTHERN_IRISH_LEAGUE_CUP_GROUP_ROUNDS: [ItaCupGroupRound; 5] = [(0x1d, 7, 2, 2), (0xc, 8, 2, 2), (0x1a, 8, 2, 2), (0xa, 9, 2, 2), (0x18, 9, 2, 2)]; // mwa26: the group matchdays

/// The Northern Irish League Cup's knockout: quarter-finals drawn on 25 October and played on 31 October (single
/// matches, tie rules 0x203); semi-finals over two legs seven days apart, drawn on 1 November and first played on 14
/// November (tie rule 4, two-leg rule 7); the final drawn on 22 November and played on 28 November (match code 4);
/// category 6; the tie winner's prize a share of the fund (−1).
pub(crate) const NORTHERN_IRISH_LEAGUE_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((0x19, 9, 0, 3), (0x1f, 9, 0, 2, 2, 0), 0x78, (0, 0x203, 0), 6, (8, 4, 8, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((1, 0xa, 0, 3), (0xe, 0xa, 0, 2, 2, 0), 0x82, (0, 4, 7), 6, (4, 2, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0x16, 0xa, 0, 3), (0x1c, 0xa, 0, 2, 2, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// The Northern Irish League Cup's constructor: the vtable 0x96dc18, discipline rule
/// `discipline_rule::NORTHERN_IRELAND`, four stages (three groups and the knockout), NirLgeCup's collect_entrants
/// (0x78d800), set_predicted_positions (0x78dcd0), set_up_rules (0x78d0d0), create_table (0x78cfd0) and
/// create_group_stage (0x78d3c0), nir_lge_cup.cpp.
pub(crate) const NORTHERN_IRISH_LEAGUE_CUP_CONSTRUCTOR: IrishLeagueCupConstructor = IrishLeagueCupConstructor { vtable: 0x96dc18, discipline: discipline_rule::NORTHERN_IRELAND, stages: 4, groups: 3,
    entrants: 0x78d800, predicted: 0x78dcd0, rules: 0x78d0d0, first_group: 0x78cfd0, group_stage: 0x78d3c0, file: 0xa69698 };

/// The Northern Irish League Cup's group rules: 3 points for a win and 1 for a draw, the tie-breaks 1, 2 and 4, the top
/// two go through, category 4, a 100,000 prize fund.
pub(crate) const NORTHERN_IRISH_LEAGUE_CUP_GROUP_RULES: GroupStageRules = GroupStageRules { win: 3, draw: 1, category: 4, tiebreak: [1, 2, 4, 0], slots: 2, prize: Some(0x186a0), bits: None }; // mwa17: two through per group

/// The Northern Irish League Cup entrant list (`load_group_cup_entrants`, nir_lge_cup.cpp lines 0x34f, 0x357 and
/// 0x35f).
/// Original: `NirLgeCup::load_extra_data` (0x78db10, nir_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0xa69698, (0x34f, 0x357, 0x35f), false)
}

pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_NIR_LGE_CUP_LOAD_EXTRA_DATA);
    compare_group_cup_entrants_load("nir_lge_cup_load_extra_data", this, file, || o(this, file), || nir_lge_cup_load_extra_data(this, file))
}
