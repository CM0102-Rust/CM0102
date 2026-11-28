use super::*;

/// Rule: the Irish League Cup's groups 1 to 7 (stages 0 to 6; the cup's own table is the first group): three clubs in
/// stages 0 to 4 and four in stages 5 and 6, the next entrants of the pool in order, in a league stage of their own
/// where one place goes through; then the group draw news; out of memory the box of line 0x240, without the stage that
/// of 0x259 (and 0).
///
/// `irish_league_cup_group_stage` (ItaCCup::create_group_stage's frame, read instruction by instruction: the group size
/// is a local, 3 or 4, the pool scan stops at the entrant count and a local place list replaces the cup's).
/// Original: `IreLgeCup::create_group_stage` (0x6379c0, ire_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let n = if (g as i32) >= 5 { 4 } else { 3 }; // mwa23: four clubs from stage 5
    irish_league_cup_group_stage(League(this), g, n, 1, 0x9e9f4c, (0x240, 0x259), "mwa_23 irish league cup group")
}

/// Comparer for `ire_lge_cup_create_group_stage` (original only, the plan).
pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_IRE_LGE_CUP_CREATE_GROUP_STAGE);
    let n = if (g as i32) >= 5 { 4 } else { 3 };
    compare_irish_league_cup_group_stage("ire_lge_cup_create_group_stage", League(this), g, n, || o(this, g))
}

/// Rule: in the seven Irish League Cup groups (stages below 7), a club going through gets the qualification record
/// (round 0x78, weight 40) and one going out the failed-to-qualify record (weight 40); in the knockout (stage 7) a club
/// going through gets the qualification by round and one going out the knocked-out record (weight 50 to 100 over 3
/// rounds), and the winner and the runner-up their final records.
///
/// `welsh_cup_staff_records` with 7 group stages and 3 knockout rounds (read instruction by instruction against
/// WelLgeCup::update_staff_records_for_status: only the group stage count, the knockout slot and the round count
/// change). Returns 0.
/// Original: `IreLgeCup::update_staff_records_for_status` (0x638110, ire_lge_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    covered!("mwa_23 irish league cup staff records");
    welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 7, 3) // mwa23: three knockout rounds
}

pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_IRE_LGE_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("ire_lge_cup_update_staff_records_for_status({:#x}) returned {}", this, r));
    r
}

/// Rule: the Irish League Cup's predicted positions (at creation and at the turnover): the 26 entrants by reputation;
/// the knockout (stage 7) gives 1 to the winner and 2 to the runner-up, then a shared place to the losers of each
/// earlier round; the rest in bands of 8; the board and press expectation of each club [inference].
///
/// `italian_cup_predicted_positions` with the knockout stage 7, bands of 8 and the boxes at lines 0x3ee and 0x3f6 of
/// ire_lge_cup.cpp (read instruction by instruction against ItaCCup::set_predicted_positions: only the file, the lines,
/// the stage and the band change).
/// Original: `IreLgeCup::set_predicted_positions` (0x638420, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0x9e9f4c, (0x3ee, 0x3f6), 7, 8, "mwa_23 irish league cup predicted") // mwa23: the Irish League Cup bands of 8
}

/// Comparer for `ire_lge_cup_set_predicted_positions` (`compare_italian_cup_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_IRE_LGE_CUP_SET_PREDICTED_POSITIONS);
    compare_italian_cup_predicted("ire_lge_cup_set_predicted_positions", this, ire_lge_cup_set_predicted_positions, || o(this));
}

/// Rule: the Irish League Cup's knockout (stage 7, built when the seven groups are over) takes the eight group
/// winners: the top club of the cup's own table (the first group) and of each of the seven group stages, in order; out
/// of memory the box of line 0x278.
///
/// The club vector (`exe::crt_malloc`(0x20)); slot 0 the first row of `league::TABLE`, slots 1 to 7 the first row of
/// stage i's table (League::get_table_row `exe::league_get_table_row`(stage, 0): push 0; mov ecx, stage -> thiscall,
/// ret 4); then `create_knockout_stage` (8 clubs, stage 7, no seeds, the box of line 0x290; the frame of
/// ItaCup::create_knockout_stage: the round table of stage 7, Cup_Stage::Cup_Stage, the vector and the table freed, the
/// first draw).
/// Original: `IreLgeCup::create_knockout_stage` (0x637c00, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_create_knockout_stage(this: usize) {
    let this = League(this);
    let teams = exe::crt_malloc(0x20);
    if teams == 0 {
        return show_error_at(0x9e9f4c, 0x278);
    }
    covered!("mwa_25 irish league cup knockout");
    write_usize(teams, read_usize(this.table())); // mwa25: the first group's winner
    for i in 0..7usize {
        let stage = read_usize(this.stages() + 4 * i);
        write_usize(teams + 4 * (i + 1), read_usize(exe::league_get_table_row(stage, 0)));
    }
    create_knockout_stage(this, 8, teams, 7, core::ptr::null(), 0x9e9f4c, 0x290);
}

/// Comparer for `ire_lge_cup_create_knockout_stage` (original only: it builds the stage and draws its first round;
/// `ita_cup_compare_knockout_stage` checks the stage in slot 7).
pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_IRE_LGE_CUP_CREATE_KNOCKOUT_STAGE);
    ita_cup_compare_knockout_stage("ire_lge_cup_create_knockout_stage", League(this), 7, 0, || o(this));
}

/// Rule: the Irish League Cup takes 26 Irish clubs: those the database assigns to the cup's groups A to E (through the
/// club's reserve division: six each in A, B and C, four each in D and E) and, for the free places, the other Irish
/// clubs ranked by reputation with a shuffle among the best 20, each into the band of its regional league (Leinster
/// with A, Connacht with C, Munster with D, Ulster with E, the rest with B); the first three bands are shuffled; places
/// still free take the first Irish clubs in table order not yet in.
/// Original: `IreLgeCup::collect_entrants` (0x637dd0, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_collect_entrants(this: usize) -> i32 {
    collect_irish_league_cup_entrants(League(this)).0
}

/// Comparer for `ire_lge_cup_collect_entrants` (side by side with the random generator restored: `compare_select_entrants`).
pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_IRE_LGE_CUP_COLLECT_ENTRANTS);
    compare_select_entrants("ire_lge_cup_collect_entrants", this, || o(this), || collect_irish_league_cup_entrants(League(this)))
}

/// Rule: the Irish League Cup's groups (stages up to 6) play three matchdays in late September (20, 24 and 27; one
/// round robin of three clubs, or four in the last two groups), each group a league stage (is_league = 1, label 0x3e9
/// + stage); the knockout (stage 7) has eight clubs: quarter-finals on 8 November and semi-finals on 29 November (single
/// matches), a two-legged final from 24 January of the next year.
///
/// `create_schedule` with the knockout at stage 7, the label 0x3e9, `IRISH_LEAGUE_CUP_GROUP_ROUNDS`,
/// `IRISH_LEAGUE_CUP_KNOCKOUT_ROUNDS` and the boxes at lines 0xbc and 0xdb of ire_lge_cup.cpp (read instruction by
/// instruction against ItaCCup::create_schedule: only the stage, the label, the dates, the rounds and the lines
/// change). Above stage 7: 0.
/// Original: `IreLgeCup::create_schedule` (0x637120, ire_lge_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule(League(this), stage, rounds, extra, league, 7, 0x3e9, &IRISH_LEAGUE_CUP_GROUP_ROUNDS, &IRISH_LEAGUE_CUP_KNOCKOUT_ROUNDS, 0x9e9f4c, 0xbc, 0xdb,
        "mwa_25 irish league cup groups", "mwa_25 irish league cup knockout rounds")
}

/// Comparer for `ire_lge_cup_create_schedule` (side by side: `ita_cup_compare_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_IRE_LGE_CUP_CREATE_SCHEDULE);
    ita_cup_compare_schedule("ire_lge_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| ire_lge_cup_create_schedule(this, stage, r, e, l), 7)
}

/// Rule: the Irish League Cup at career creation: the year and the database competition, 5 substitutes on the bench and
/// 3 substitutions, the Irish discipline rules, importance 12; with nothing saved, eight stages (seven groups and the
/// knockout), the 26 entrants, the statistics, the predicted positions, the rules, the first group's table (the cup is
/// itself the league of the first group), its fixtures and order and the stages of the seven groups.
///
/// `construct_irish_league_cup` with `IRISH_LEAGUE_CUP_CONSTRUCTOR` (checked instruction by instruction against
/// ItaCCup::ItaCCup 0x63cc40: only the vtable, the bench, the discipline rule, the stage and group counts, the routines
/// and the file change).
/// Original: `IreLgeCup::IreLgeCup` (0x636dd0, ire_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_lge_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league_cup(League(this), year, record, &IRISH_LEAGUE_CUP_CONSTRUCTOR, "mwa_16 irish league cup new") // mwa16: the Irish League Cup
}

pub(crate) unsafe extern "thiscall" fn compare_construct_ire_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_IRE_LGE_CUP);
    compare_irish_league_cup_constructor("construct_ire_lge_cup", this, year, record, &IRISH_LEAGUE_CUP_CONSTRUCTOR, || o(this, year, record))
}

/// Rule: the first group of the Irish League Cup has three clubs, the first three entrants, which leave the waiting
/// list; the group draw news follows.
///
/// `create_first_group_table`(3 clubs; out of memory, the error box at line 0x172 of ire_lge_cup.cpp) (ItaCCup's
/// init_first_group template, checked instruction by instruction: only the count, the file and the line). Returns 1.
/// Original: `IreLgeCup::create_table` (0x6375c0, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_create_table(this: usize) -> u32 {
    create_first_group_table(League(this), 3, 0x9e9f4c, 0x172, "mwa_16 irish league cup first group")
}

pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_LGE_CUP_CREATE_TABLE);
    compare_first_group_table("ire_lge_cup_create_table", League(this), 3, || o(this))
}

/// Rule: the Irish League Cup groups are single round robins: 3 points for a win and 1 for a draw, ties split by goal
/// difference, goals for and the head-to-head [inference]; the group winner goes through; category 4 (national cup); no
/// promotion or relegation; a 100,000 prize fund.
///
/// `set_cup_league_rules` with `IRISH_LEAGUE_CUP_GROUP_RULES` and one meeting (ItaCup::set_up_rules's template, checked
/// instruction by instruction: only the tie-break order and the prize change), then the round table. Returns 1.
/// Original: `IreLgeCup::set_up_rules` (0x6376c0, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_set_up_rules(this: usize) -> u32 {
    covered!("mwa_16 irish league cup rules");
    set_cup_league_rules(League(this), &IRISH_LEAGUE_CUP_GROUP_RULES, 1)
}

pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_LGE_CUP_SET_UP_RULES)(this);
    check_cup_league_rules("ire_lge_cup_set_up_rules", League(this), &IRISH_LEAGUE_CUP_GROUP_RULES, r);
    r
}

/// Rule: the Irish League Cup moves stage by stage through its seven groups; when the groups are over (stage 7) the
/// knockout of the eight group winners is built; past the last stage, the error box.
///
/// `advance_cup_stage` (the Italian cups' template, checked instruction by instruction against ItaCup::advance_stage
/// 0x63f5b0: only the file, the line 0x212 and the knockout routine IreLgeCup::create_knockout_stage 0x637c00 change).
/// Original: `IreLgeCup::advance_stage` (0x637930, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_advance_stage(this: usize) {
    advance_cup_stage(League(this), 0x9e9f4c, 0x212, 7, 0x637c00) // mwa18: the Irish League Cup knockout after the seven groups
}

pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_IRE_LGE_CUP_ADVANCE_STAGE);
    ita_cup_compare_advance_stage("ire_lge_cup_advance_stage", League(this), || o(this))
}

/// Rule: at the season turn the Irish League Cup is rebuilt: the table, the round table, the entrants and the stages of
/// the last edition are freed, the statistics archived, the year moves on; then the 26 entrants, the predicted
/// positions, the rules, the first group's table, its fixtures and order and the stages of the seven groups.
///
/// `start_irish_league_cup_season` with IreLgeCup::collect_entrants (0x637dd0), create_table (0x6375c0),
/// create_group_stage (0x6379c0), seven groups and the error box lines 0x1e4, 0x1eb, 0x1f1 and 0x1f8 of
/// ire_lge_cup.cpp.
/// Original: `IreLgeCup::start_new_season` (0x637760, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_start_new_season(this: usize) -> u32 {
    start_irish_league_cup_season(League(this), 0x637dd0, 0x6375c0, 0x6379c0, 7, 0x9e9f4c, [0x1e4, 0x1eb, 0x1f1, 0x1f8], 1) // mwa18: the Irish League Cup turnover
}

pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_LGE_CUP_START_NEW_SEASON);
    compare_state_league_new_season("ire_lge_cup_start_new_season", League(this), || o(this))
}

/// Builds group `g` of an Irish league cup (IreLgeCup, NirLgeCup) as a League_Stage, in the original routine's order.
///
/// The round table of stage `g` (virtual `comp_vtable::CREATE_SCHEDULE`(g, &rounds, &label, 0): push 0; push &label;
/// push &rounds; push g -> thiscall, ret 0x10); the club list (`exe::crt_malloc` of 4 × `n`; null: the box of `rows.0`
/// and 0) with the first `n` non-null entrants of `ita_cup_fields::ENTRANT_CLUBS`, scanned in order while the index is
/// below `ita_cup_fields::ENTRANT_CLUB_COUNT` (the list stays short when the pool runs out), each zeroed as it leaves; a
/// League_Stage (`exe::crt_operator_new`(0xee); League_Stage::League_Stage 0x68b100, thiscall, ret 0x58: this cup as the
/// parent, `n` clubs, the list, 1, the competition, the round table and its count, the win, draw and category bytes,
/// the tie-breaks (`league::TIEBREAKS`), a local byte list (`places`, 0, 0, 0) where the Italian cups pass
/// `league::PROMOTED_COUNT` (the places that go through [inference]), the year, `g`, the stage label, the importance,
/// 1, 0, the league flags, −1, 0, 2) into stage slot `g` (0 without memory); the list and the round table freed
/// (`exe::crt_free`); no stage: the box of `rows.1` and 0; otherwise the group draw news of the stage
/// (League::send_group_draw_news), `league::CURRENT_STAGE` = `g` and 1.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn irish_league_cup_group_stage(this: League, g: u32, n: u32, places: u8, file: usize, rows: (u32, u32), branch: &'static str) -> u32 {
    let places: [u8; 4] = [places, 0, 0, 0];
    let (mut rounds, mut label) = (0i16, 0i16);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let table = schedule(this.addr(), g, &mut rounds, &mut label, 0);
    let clubs = exe::crt_malloc(4 * n);
    if clubs == 0 {
        show_error_at(file, rows.0);
        return 0;
    }
    covered!(branch);
    let pool = this.addr() + ita_cup_fields::ENTRANT_CLUBS;
    let (mut k, mut i) = (0u32, 0i32);
    while k < n && i < read_i32(this.addr() + ita_cup_fields::ENTRANT_CLUB_COUNT) {
        let e = read_usize(pool) + 4 * i as usize;
        let c = read_usize(e);
        if c != 0 {
            write_usize(clubs + 4 * k as usize, c); // mwa23: the group clubs in pool order
            write_usize(e, 0);
            k += 1;
        }
        i += 1;
    }
    let p = exe::crt_operator_new(0xee);
    let mut s = 0usize;
    if p != 0 {
        type LeagueStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize;
        let ctor: LeagueStageCtor = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        // simplification: the original pushes the round count, the stage label, the year and the flags as stack or
        // register dwords with stale high bytes; League_Stage::League_Stage reads the words and bytes
        s = ctor(p, this.addr(), n, clubs, 1, this.competition().addr(), table, rounds as u16 as u32, this.win_points() as u32,
            this.draw_points() as u32, this.match_category() as u32, this.addr() + league::TIEBREAKS, places.as_ptr() as usize,
            this.year() as u16 as u32, g, label as u16 as u32, this.importance() as u32, 1, 0, this.league_flags() as u16 as u32, 0xffff_ffff, 0, 2);
    }
    write_usize(this.stages() + 4 * g as usize, s);
    exe::crt_free(clubs);
    exe::crt_free(table);
    let s = read_usize(this.stages() + 4 * g as usize);
    if s == 0 {
        show_error_at(file, rows.1);
        return 0;
    }
    exe::league_send_group_draw_news(s);
    this.set_current_stage(g as usize);
    1
}

/// The body of `ire_lge_cup_collect_entrants`; returns (the routine's return, the entries to compare).
///
/// `ita_cup_fields::ENTRANT_CLUB_COUNT` = 26; the pool (`exe::crt_calloc`(4, 26) into `ita_cup_fields::ENTRANT_CLUBS`;
/// none: the box of line 0x2af and 0). First pass over the clubs (`G_CLUB_COUNT`, a signed word index, in table order),
/// those of the Republic of Ireland (`club::NATION`'s first dword = [`KEY_NATION_REPUBLIC_OF_IRELAND`]): with a
/// `club::RESERVE_DIVISION` whose id is one of the Irish groups A to E (`KEY_CLUB_COMP_IRISH_GROUP_*`), the first free
/// slot of that group's band (A 0 to 5, B 6 to 11, C 12 to 17, D 18 to 21, E 22 to 25; a full band: nothing); with any
/// other reserve division, nothing; with none (or an id 0) but a `club::DIVISION`, a local list of up to 100. The list
/// is sorted and shuffled by `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 20) (cdecl); while the clubs placed
/// are fewer than 26, each listed club goes to the first free slot of the band of its division's regional league
/// (Leinster 0 to 5, Connacht 12 to 17, Munster 18 to 21, Ulster 22 to 25, any other 6 to 11). The first three bands of
/// six are shuffled (`exe::comp_util_shuffle_pointer_array`(band, 6)). If fewer than 26 were placed, each empty slot
/// takes the first club in table order with a division, of the nation record of [`KEY_NATION_REPUBLIC_OF_IRELAND`]
/// (`G_NATION_TABLE` + id × 0x122) and not yet in the pool. Returns 1.
pub(crate) unsafe fn collect_irish_league_cup_entrants(this: League) -> (i32, i32) {
    let t = this.addr();
    write_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT, 0x1a);
    let v = exe::crt_calloc(4, 0x1a);
    write_usize(t + ita_cup_fields::ENTRANT_CLUBS, v);
    if v == 0 {
        show_error_at(0x9e9f4c, 0x2af);
        return (0, 0);
    }
    let mut list = [0usize; 100];
    let (mut n, mut placed) = (0i16, 0i32);
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let c = club_at(i as isize);
        let nation = c.nation();
        if !nation.is_null() && read_usize(nation.addr()) == read_usize(KEY_NATION_REPUBLIC_OF_IRELAND) {
            let r = c.reserve_division();
            if !r.is_null() && read_usize(r.addr()) != 0 {
                let id = read_usize(r.addr());
                let band = if id == read_usize(KEY_CLUB_COMP_IRISH_GROUP_A) { Some((0, 6)) } // mwa25: the group A band
                    else if id == read_usize(KEY_CLUB_COMP_IRISH_GROUP_B) { Some((6, 0xc)) }
                    else if id == read_usize(KEY_CLUB_COMP_IRISH_GROUP_C) { Some((0xc, 0x12)) }
                    else if id == read_usize(KEY_CLUB_COMP_IRISH_GROUP_D) { Some((0x12, 0x16)) }
                    else if id == read_usize(KEY_CLUB_COMP_IRISH_GROUP_E) { Some((0x16, 0x1a)) }
                    else { None };
                if let Some((lo, hi)) = band {
                    if let Some(j) = first_free_entrant_slot(v, lo, hi) {
                        write_usize(v + 4 * j, c.addr());
                        placed += 1;
                    }
                }
            } else if !c.division().is_null() && n < 100 {
                list[n as usize] = c.addr();
                n += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    covered!("mwa_25 irish league cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(list.as_ptr() as usize, n as i32 as u32, 0x14);
    let mut k = 0i16;
    while k < n && placed < read_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT) {
        let c = Club(list[k as usize]);
        let id = read_usize(c.division().addr());
        let (lo, hi) = if id == read_usize(KEY_CLUB_COMP_IRISH_LEINSTER_SENIOR_LEAGUE) { (0, 6) }
            else if id == read_usize(KEY_CLUB_COMP_IRISH_CONNACHT_SENIOR_LEAGUE) { (0xc, 0x12) } // mwa25: the Connacht band
            else if id == read_usize(KEY_CLUB_COMP_IRISH_MUNSTER_SENIOR_LEAGUE) { (0x12, 0x16) }
            else if id == read_usize(KEY_CLUB_COMP_IRISH_ULSTER_SENIOR_LEAGUE) { (0x16, 0x1a) }
            else { (6, 0xc) };
        if let Some(j) = first_free_entrant_slot(v, lo, hi) {
            write_usize(v + 4 * j, c.addr());
            placed += 1;
        }
        k += 1;
    }
    exe::comp_util_shuffle_pointer_array(v, 6);
    exe::comp_util_shuffle_pointer_array(v + 0x18, 6);
    exe::comp_util_shuffle_pointer_array(v + 0x30, 6);
    if placed < read_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT) {
        let mut j = 0i16;
        while (j as i32) < read_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT) {
            let count = read_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT);
            if read_usize(v + 4 * j as usize) == 0 {
                let ireland = read_usize(G_NATION_TABLE).wrapping_add(read_usize(KEY_NATION_REPUBLIC_OF_IRELAND).wrapping_mul(0x122));
                let mut q = 0i32;
                while q < read_i32(G_CLUB_COUNT) {
                    let c = club_at(q as isize);
                    if !c.division().is_null() && c.nation().addr() == ireland
                        && !(0..count.max(0) as usize).any(|k| read_usize(v + 4 * k) == c.addr()) {
                        write_usize(v + 4 * j as usize, c.addr());
                        break;
                    }
                    q += 1;
                }
            }
            j = j.wrapping_add(1);
        }
    }
    (1, 0x1a)
}

/// Virtual +0x44 of the two Welsh group cups: the staff records by outcome and stage; returns 0.
///
/// In the groups (`stage`, signed byte, below `groups`): outcome 6, qualification (`european_cup_add_qualified_record`:
/// stage 0, round 0x78, weight 0x28); 0xfe, failure to qualify (`european_cup_add_failed_to_qualify_record`: 0x41c,
/// weight 0x28). In the knockout (stage `groups`, the stage slot `groups`): 0xff, knocked out with the match's round
/// prefix (+0x32), round name (+0x30) and the weight `scale_by_round(0x32, 0x64, round number, scale)`; 0, qualification
/// with the round prefix, the code of the next round of the knockout's round table and the same weight; 1 and 2, the
/// winner's and the runner-up's records (0x86a800, 0x86b0b0: the staff records, club, match, a6).
pub(crate) unsafe fn welsh_cup_staff_records(this: League, club: usize, outcome: u32, stage: u32, game: usize, a6: u32, groups: i8, scale: u32) -> u32 {
    let game = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = this.competition().addr();
    if f < groups {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0, 0x78, 0x28); // mwb19: the group qualification weight
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x28);
        }
    } else if f == groups {
        match d {
            0xff => {
                let weight = scale_by_round(0x32, 0x64, game.round_number(), scale);
                // simplification: +0x32 and +0x30 go in registers with garbage in the high 16 bits; 0x86c000 only uses
                // the words
                european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
            }
            0 => {
                let r = game.round_number();
                let weight = scale_by_round(0x32, 0x64, r, scale);
                let knockout = Cup(read_usize(this.stages() + groups as usize * 4));
                let round = read_i16(knockout.round_table().wrapping_add((r as isize * (cup_round::SIZE as isize)) as usize) + cup_round::ROUND_CODE + 0x68) as u16 as u32;
                european_cup_add_qualified_record(club, comp, game.round_prefix() as u16 as u32, round, weight);
            }
            1 => african_nations_record_final_result(0x86a800, club, game.addr(), a6),
            2 => african_nations_record_final_result(0x86b0b0, club, game.addr(), a6),
            _ => {}
        }
    }
    0
}

/// The body of the Irish league cups' constructors (thiscall, ret 8: this, year, the competition record).
///
/// Over League::League (`exe::new_league`, thiscall, ret 0): `league::COMPETITION` = the record, the vtable,
/// `league::YEAR`, 3 substitutions allowed, 5 substitutes named, the discipline rule and importance 0xc. If the saved
/// state loads (`exe::league_load_from_temp_file`(this, 1); ≠ 0) it returns this. Otherwise, in order:
/// `league::LEAGUE_FLAGS` = 0x28, `league::STAGE_INDEX` = `stage_index::MAIN`, `league::CURRENT_STAGE` = −1, the stage
/// count and its array (`exe::crt_malloc` of 4 × stages, unchecked); the entrants (thiscall, ret 0, unchecked); the
/// statistics (`ita_cups_create_comp_stats`; without them, the error box at line 0x4f); the predicted positions
/// (unchecked); the rules (failing on 0: line 0x59); the first group's table (failing on 0: line 0x60); the fixtures
/// (`exe::league_create_fixtures`; line 0x66); the order (`exe::league_sort_table`(this, 0); line 0x6d); the stage of
/// each group (push g; mov ecx, this -> thiscall, ret 4), g from 0. Each box returns this. Returns this.
pub(crate) unsafe fn construct_irish_league_cup(this: League, year: u32, record: usize, k: &IrishLeagueCupConstructor, branch: &'static str) -> usize {
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), k.vtable);
    this.set_year(year as u16 as i16);
    this.set_substitutions_allowed(3);
    this.set_substitutes_named(5);
    this.set_discipline_rule(k.discipline);
    this.set_importance(0xc);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    this.set_league_flags(0x28);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    this.set_stage_count(k.stages as usize);
    this.set_stages(exe::crt_malloc(k.stages * 4));
    let call = |f: usize| {
        let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(f);
        f(this.addr())
    };
    call(k.entrants);
    if !ita_cups_create_comp_stats(this) {
        show_error_at(k.file, 0x4f);
        return this.addr();
    }
    call(k.predicted);
    if call(k.rules) == 0 {
        show_error_at(k.file, 0x59);
        return this.addr();
    }
    if call(k.first_group) == 0 {
        show_error_at(k.file, 0x60);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(k.file, 0x66);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(k.file, 0x6d);
        return this.addr();
    }
    let group: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(k.group_stage);
    for g in 0..k.groups {
        group(this.addr(), g);
    }
    this.addr()
}

/// The comparer of the Irish league cups' constructors: only the original runs, and the plan is checked
/// (`ita_cups_check_constructor_plan`: this, the vtable, the record, the year and, on a new cup, the fixed fields and
/// the statistics).
pub(crate) unsafe fn compare_irish_league_cup_constructor(name: &str, this: usize, year: u32, record: usize, k: &IrishLeagueCupConstructor, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let fields = [(league::STAGE_COUNT, 4u8, k.stages), (league::STAGE_INDEX, 1, 0xff), (league::SUBSTITUTES_NAMED, 1, 5), (league::SUBSTITUTIONS_ALLOWED, 1, 3),
        (league::DISCIPLINE_RULE, 1, k.discipline as u32), (league::IMPORTANCE, 1, 0xc), (league::LEAGUE_FLAGS, 2, 0x28)];
    let error = ita_cups_check_constructor_plan(this, r, year, record, k.vtable, &fields, false);
    SCOREBOARD.record(error.is_none(), || format!("{}({:#x}, {}, {:#x}): {}", name, this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The turnover of an Irish league cup (the cup is the league of its first group).
///
/// `league::SEASON_CLOSED` = 0; the table freed (`exe::league_free_table`(this, 0)); the round table freed and zeroed;
/// the season archived in the statistics (when there); the entrant vector (`ita_cup_fields::ENTRANT_CLUBS`) freed (not
/// zeroed: the entrant routine replaces it); with the current stage ≥ 0, the stages 0 to the current one destroyed
/// (their virtual 0 with 1; the slots are not zeroed); the year up by `year`; `league::CURRENT_STAGE` = −1. Then the
/// entrants (thiscall, ret 0, unchecked), the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`) and, each
/// with its own error box (returning 0) when it returns 0: the rules (`comp_vtable::SET_UP_RULES`), the first group's
/// table (`table`), the fixtures and the order; finally the stage of each group (push g -> thiscall, ret 4), g from 0.
pub(crate) unsafe fn start_irish_league_cup_season(this: League, entrants: usize, table: usize, group_stage: usize, groups: u32, file: usize, rows: [u32; 4], year: i16) -> u32 {
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
    let v = read_usize(this.addr() + ita_cup_fields::ENTRANT_CLUBS);
    if v != 0 {
        exe::crt_free(v);
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
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(entrants);
    entrants(this.addr());
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    // simplification: in the boxes the original returns the box's eax; the caller of the turnover does not read it
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(file, rows[3]);
        return 0;
    }
    covered!("mwa_18 irish league cup turnover");
    let group: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(group_stage);
    let mut r = 0;
    for g in 0..groups {
        r = group(this.addr(), g) as u32;
    }
    r
}

/// The predicted positions of the Italian cups (0x640210 and 0x63e580, the same body; only the source file, the box
/// lines, the knockout stage and the band width change). Rule: the entrants by reputation; from the final backwards,
/// the clubs of each knockout round share the place of the losers of that round (the winner 1 and the runner-up 2,
/// then 3 for the semi-finalists, 5 for the quarter-finalists...), and the rest go in bands of `band`.
///
/// Without statistics (+8): the box `lines.0` and return. A copy of the entrant list (+0xee, +0xf2 clubs, a dword)
/// (`exe::crt_malloc`(4 × n); out of memory: the box `lines.1` and return), sorted by reputation (`exe::crt_qsort`(v, n,
/// 4, 0x4b07d0): 4 pushes; add esp -> cdecl); the round table of the knockout stage (virtual
/// `comp_vtable::CREATE_SCHEDULE`(this, `stage`, &rounds, &extra, 0): push 0; push &extra; push &rounds; push stage;
/// mov ecx -> thiscall, ret 0x10). Two byte counters start at 1: `next` (the next place) and `place` (the one being
/// given). From the last round down to round 0, with the word +0x1a of the round record (`cup_round::SIZE` bytes: the
/// round's ties): in the last round first, that many clubs get `place` (still 1: the winner) and `next` goes up by one
/// each; then `place` = `next` and that many clubs get it (`next` up by one each). The clubs taken in order from the
/// sorted copy, with a word index and no bound check (as the original). Then `place` = `next` and each remaining club
/// (index below the dword +0xf2) gets `place`; `next` goes up by one, and when `next` equals `place` + `band` (in 32
/// bits: a band past 255 never closes), `place` = `next`. Each place goes through Comp_Stats::set_club_final_position
/// (`exe::comp_stats_set_club_final_position`(stats, club, place): push place; push club; mov ecx, [+8] -> thiscall, ret
/// 8; the original pushes the place in a dword with stack garbage above the byte, which the routine does not read: the
/// copy passes the clean byte). The round table and the copy are freed (`exe::crt_free`).
pub(crate) unsafe fn italian_cup_predicted_positions(this: usize, file: usize, lines: (u32, u32), stage: u32, band: u32, label: &'static str) {
    let stats = read_usize(this + 8);
    if stats == 0 {
        return show_error_at(file, lines.0);
    }
    let v = exe::crt_malloc(read_u32(this + 0xf2) << 2);
    if v == 0 {
        return show_error_at(file, lines.1);
    }
    if read_i32(this + 0xf2) > 0 {
        let mut i = 0i16;
        loop {
            write_usize(v + 4 * i as usize, read_usize(read_usize(this + 0xee) + 4 * i as usize));
            i = i.wrapping_add(1);
            if i as i32 >= read_i32(this + 0xf2) {
                break;
            }
        }
    }
    exe::crt_qsort(v, read_u32(this + 0xf2), 4, 0x4b07d0);
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = create_schedule(this, stage, &mut rounds, &mut extra, 0);
    let (mut next, mut place) = (1u8, 1u8);
    let mut idx = 0i16;
    let give = |place: u8, idx: &mut i16, next: &mut u8| {
        exe::comp_stats_set_club_final_position(stats, read_usize(v + 4 * *idx as usize), place as u32);
        *idx = idx.wrapping_add(1);
        *next = next.wrapping_add(1);
    };
    let last = rounds.wrapping_sub(1);
    if last >= 0 {
        covered!(label);
        let mut r = last as i32;
        while r >= 0 {
            let ties = table.wrapping_add((r as usize).wrapping_mul(cup_round::SIZE)) + 0x1a;
            if r == rounds as i32 - 1 {
                let mut k = 0i16;
                while k < read_i16(ties) {
                    give(place, &mut idx, &mut next); // cqx7: the final's winner
                    k += 1;
                }
            }
            place = next;
            let mut k = 0i16;
            while k < read_i16(ties) {
                give(place, &mut idx, &mut next);
                k += 1;
            }
            r -= 1;
        }
    }
    place = next;
    while (idx as i32) < read_i32(this + 0xf2) {
        exe::comp_stats_set_club_final_position(stats, read_usize(v + 4 * idx as usize), place as u32);
        next = next.wrapping_add(1);
        if next as u32 == place as u32 + band {
            place = next;
        }
        idx = idx.wrapping_add(1);
    }
    exe::crt_free(table);
    exe::crt_free(v);
}

/// The comparer of the Italian cups' predicted positions, side by side over the same entries: the original first
/// (Comp_Stats::set_club_final_position adds the entry of a club that has none), then the copy, which finds the entries
/// already there and only rewrites `comp_stats_club::FINAL_POSITION` (the same list, the same qsort: the same order;
/// the round table made again and freed); the position of each entrant after the copy must equal the original's.
/// Without statistics or list: only the original.
pub(crate) unsafe fn compare_italian_cup_predicted(name: &str, this: usize, copy: unsafe extern "thiscall" fn(usize), o: impl FnOnce()) {
    if read_usize(this + 8) == 0 || read_usize(this + 0xee) == 0 || read_i32(this + 0xf2) <= 0 {
        return o();
    }
    o();
    let original = italian_cup_positions(this);
    copy(this);
    let after = italian_cup_positions(this);
    SCOREBOARD.record(after == original, || format!("{}({:#x}): copy {:?} original {:?}", name, this, after, original));
}

/// The first free slot of `v` in `lo..hi` (signed words, as the original counts), or None.
pub(crate) unsafe fn first_free_entrant_slot(v: usize, lo: i16, hi: i16) -> Option<usize> {
    let mut j = lo;
    while j < hi {
        if read_usize(v + 4 * j as usize) == 0 {
            return Some(j as usize);
        }
        j += 1;
    }
    None
}

/// The predicted position (`comp_stats_club::FINAL_POSITION`) of each entrant (+0xee, +0xf2) in the cup's statistics
/// (+8), 0 without entry.
pub(crate) unsafe fn italian_cup_positions(this: usize) -> Vec<u8> {
    let n = read_i32(this + 0xf2).clamp(0, 0x200) as usize;
    (0..n).map(|i| {
        let c = read_usize(read_usize(this + 0xee) + 4 * i);
        let e = if c != 0 { exe::comp_stats_find_club_entry(read_usize(this + 8), c) } else { 0 };
        if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 }
    }).collect()
}

/// The Irish League Cup's three group matchdays: 20 September (weekday 2, code 2: midweek [inference]), 24 September
/// (weekday 6, code 1: the weekend) and 27 September (weekday 2, code 2); (day, month from 0, weekday, code).
pub(crate) const IRISH_LEAGUE_CUP_GROUP_ROUNDS: [ItaCupGroupRound; 3] = [(0x14, 8, 2, 2), (0x18, 8, 6, 1), (0x1b, 8, 2, 2)]; // mwa25: the first group matchday

/// The Irish League Cup's knockout: quarter-finals drawn on 28 September and played on 8 November, semi-finals drawn on
/// 9 November and played on 29 November, single matches with tie rule 3; the final over two legs 21 days apart, drawn
/// on 30 November and first played on 24 January of the next year (two-leg tie rule 7); category 6; the tie winner's
/// prize a share of the fund (−1).
pub(crate) const IRISH_LEAGUE_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((0x1c, 8, 0, 3), (8, 0xa, 0, 2, 2, 0), 0x78, (0, 3, 0), 6, (8, 4, 8, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((9, 0xa, 0, 3), (0x1d, 0xa, 0, 2, 2, 0), 0x82, (0, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x1e, 0xa, 0, 3), (0x18, 0, 1, 2, 2, 0), 0x96, (0, 0, 7), 6, (2, 1, 0, 0), (0, 2, 0x15), (0, 0xffff_ffff, 0)),
];

/// The Irish League Cup's constructor: the vtable 0x96be20, discipline rule `discipline_rule::IRELAND`, eight stages
/// (seven groups and the knockout), IreLgeCup's collect_entrants (0x637dd0), set_predicted_positions (0x638420),
/// set_up_rules (0x6376c0), create_table (0x6375c0) and create_group_stage (0x6379c0), ire_lge_cup.cpp.
pub(crate) const IRISH_LEAGUE_CUP_CONSTRUCTOR: IrishLeagueCupConstructor = IrishLeagueCupConstructor { vtable: 0x96be20, discipline: discipline_rule::IRELAND, stages: 8, groups: 7,
    entrants: 0x637dd0, predicted: 0x638420, rules: 0x6376c0, first_group: 0x6375c0, group_stage: 0x6379c0, file: 0x9e9f4c };

/// The Irish League Cup's group rules: 3 points for a win and 1 for a draw, the tie-breaks 1, 2 and 4 (goal difference,
/// goals for, head-to-head [inference]), the group winner goes through, category 4, a 100,000 prize fund.
pub(crate) const IRISH_LEAGUE_CUP_GROUP_RULES: GroupStageRules = GroupStageRules { win: 3, draw: 1, category: 4, tiebreak: [1, 2, 4, 0], slots: 1, prize: Some(0x186a0), bits: None }; // mwa16: one through per group

/// The Irish League Cup entrant list (`load_group_cup_entrants`, ire_lge_cup.cpp lines 0x381, 0x389 and 0x391).
/// Original: `IreLgeCup::load_extra_data` (0x638260, ire_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0x9e9f4c, (0x381, 0x389, 0x391), false)
}

pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_IRE_LGE_CUP_LOAD_EXTRA_DATA);
    compare_group_cup_entrants_load("ire_lge_cup_load_extra_data", this, file, || o(this, file), || ire_lge_cup_load_extra_data(this, file))
}
