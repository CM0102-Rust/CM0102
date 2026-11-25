// ---------- rx09_3: the Spanish Second Division B groups 2 to 4, promotion play-off groups and relegation play-off (spa_second_b.cpp) ----------

/// The places of a Second Division B group of 19 clubs (a local array of the original): none up, 4 to the promotion
/// play-off, 1 to the relegation play-off, 3 down.
pub(crate) const SEGUNDA_B_SHORT_GROUP_PLACES: [u8; 4] = [0, 4, 1, 3];

/// The group stage of a Second Division B group, 0 to 3 (0 the league's own table, the others the group stages 0 to 2).
pub(crate) unsafe fn segunda_b_group(l: League, g: u8) -> League {
    if g == 0 {
        l
    } else {
        League(read_usize(l.stages() + 4 * (g as i8 as isize - 1) as usize))
    }
}

/// Builds Second Division B group `group` + 2 (0 to 2: groups B2, B3 and, for any other value, B4) as stage `group`:
/// its round table (`stage_round_table`, the argument pushed as it came), a list of 20 (`exe::crt_malloc` of 0x50; none:
/// the box at line 0x2a1) with the group's clubs (`segunda_b_group_clubs`); each club past the twentieth shows the box
/// at line 0x2b2 and loses its division (`club::DIVISION` = 0, a defect of the original), and the scan goes on. A group
/// of exactly 19 clubs is built with 19 and the places `SEGUNDA_B_SHORT_GROUP_PLACES`; otherwise with 20 and the
/// league's own places (`league::PROMOTED_COUNT` and the three bytes after it); fewer clubs than that: the box at line
/// 0x2c7. Then the League_Stage (`category_b_league_stage`: the league's importance and rule bits) into stage slot
/// `group` (signed byte); none: the box at line 0x2d2. Otherwise the list and the round table freed and the current
/// stage = `group`. On the error paths the round table (and the list) are not freed (leaks of the original).
/// Original: `SpaSecondB::create_group_stage` (0x857210, spa_second_b.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn spa_second_b_create_group_stage(this: usize, group: u32) {
    let l = League(this);
    let (table, rounds, extra) = stage_round_table(l, group);
    let key = match group as u8 {
        0 => KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B2,
        1 => KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B3,
        _ => KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B4,
    };
    let list = exe::crt_malloc(0x50);
    if list == 0 {
        return show_error_at(SEGUNDA_B_SOURCE, 0x2a1);
    }
    let mut count = 0i16;
    for c in segunda_b_group_clubs(key) {
        if count >= 0x14 {
            show_error_at(SEGUNDA_B_SOURCE, 0x2b2);
            write_usize(c.addr() + club::DIVISION, 0);
        } else {
            write_usize(list + 4 * count as usize, c.addr());
            count += 1;
        }
    }
    let short = count == 0x13; // rx09_3: a group of 19 relegates three
    let (n, places) = if short {
        (0x13i16, &SEGUNDA_B_SHORT_GROUP_PLACES)
    } else {
        (0x14i16, &*((this + league::PROMOTED_COUNT) as *const [u8; 4]))
    };
    if count < n {
        return show_error_at(SEGUNDA_B_SOURCE, 0x2c7);
    }
    let slot = group as u8 as i8 as isize as usize;
    let stage = category_b_league_stage(l, n as u32, list, table, rounds, extra, places, group, l.importance() as u32, l.league_flags() as u16 as u32);
    write_usize(l.stages() + 4 * slot, stage);
    if read_usize(l.stages() + 4 * slot) == 0 {
        return show_error_at(SEGUNDA_B_SOURCE, 0x2d2);
    }
    covered!("rx09_3 segunda b group stage");
    exe::crt_free(list);
    exe::crt_free(table);
    l.set_current_stage(slot);
}

/// Comparer for `spa_second_b_create_group_stage`: only the original runs (it allocates the stage); with 19 or more
/// clubs in the plan, the stage's clubs (as a set) and the current stage against it.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_create_group_stage(this: usize, group: u32) {
    let l = League(this);
    let key = match group as u8 {
        0 => KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B2,
        1 => KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B3,
        _ => KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B4,
    };
    let plan: Vec<usize> = segunda_b_group_clubs(key).iter().take(20).map(|c| c.addr()).collect();
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%)(this, group);
    if plan.len() >= 19 {
        compare_category_b_stage("spa_second_b_create_group_stage", l, group as u8 as i8 as isize as usize, &plan);
    }
}

/// The draws of the promotion play-off: for each of three rotations (picked at random), the group (0 to 3, as
/// `segunda_b_group`) whose k-th placed club goes into play-off group g, for g and k = 0 to 3 (a local table of the
/// original).
pub(crate) const SEGUNDA_B_PLAY_OFF_DRAWS: [[[u8; 4]; 4]; 3] = [
    [[1, 0, 3, 2], [3, 2, 1, 0], [0, 1, 2, 3], [2, 3, 0, 1]],
    [[2, 1, 0, 3], [1, 2, 3, 0], [3, 0, 2, 1], [0, 3, 1, 2]],
    [[3, 2, 0, 1], [0, 1, 3, 2], [1, 3, 2, 0], [2, 0, 1, 3]],
];

/// The four clubs of promotion play-off group g with rotation r: the k-th row of the group the draw names.
pub(crate) unsafe fn segunda_b_play_off_group_clubs(l: League, r: usize, g: usize) -> [usize; 4] {
    let mut list = [0usize; 4];
    for (k, slot) in list.iter_mut().enumerate() {
        *slot = aus_nsl_table_row_club(segunda_b_group(l, SEGUNDA_B_PLAY_OFF_DRAWS[r][g][k]), k);
    }
    list
}

/// Builds the four promotion play-off groups of the Second Division B, stages 3 to 6: a rotation r drawn at random
/// (`exe::utils_random`(3), the signed byte), then for each play-off group g the four clubs of
/// `segunda_b_play_off_group_clubs` in a local list, its round table (`stage_round_table`(g + 3)) and the League_Stage
/// (`category_b_league_stage`: four clubs, places 1 up and none else, importance 0x14 and rule bits 0x22) into slot g + 3.
/// None: the box at line 0x315 and the remaining groups are not built (the round table not freed). Otherwise the round
/// table freed and the current stage = g + 3.
/// Original: `SpaSecondB::create_promotion_playoff_groups` (0x857570, spa_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_second_b_create_promotion_playoff_groups(this: usize) {
    let l = League(this);
    let r = exe::utils_random(3) as u8 as i8 as isize as usize;
    for g in 0..4usize {
        let list = segunda_b_play_off_group_clubs(l, r, g);
        let stage = g + 3;
        let (table, rounds, extra) = stage_round_table(l, stage as u32);
        let p = category_b_league_stage(l, 4, list.as_ptr() as usize, table, rounds, extra, &[1, 0, 0, 0], stage as u32, 0x14, 0x22); // rx09_3: one goes up from each play-off group
        write_usize(l.stages() + 4 * stage, p);
        if read_usize(l.stages() + 4 * stage) == 0 {
            return show_error_at(SEGUNDA_B_SOURCE, 0x315);
        }
        covered!("rx09_3 segunda b promotion play-off group");
        exe::crt_free(table);
        l.set_current_stage(stage);
    }
}

/// Comparer for `spa_second_b_create_promotion_playoff_groups`: only the original runs (it draws and allocates the
/// stages); the four play-off groups built must hold, as sets, the clubs of one of the three rotations, and the
/// current stage must be 6.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_create_promotion_playoff_groups(this: usize) {
    let l = League(this);
    let plans: Vec<Vec<Vec<usize>>> = (0..3).map(|r| (0..4).map(|g| {
        let mut v = segunda_b_play_off_group_clubs(l, r, g).to_vec();
        v.sort_unstable();
        v
    }).collect()).collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let found: Vec<Vec<usize>> = (3..7).map(|s| {
        let stage = League(read_usize(l.stages() + 4 * s));
        let mut v = if stage.is_null() { Vec::new() } else { category_b_stage_clubs(stage) };
        v.sort_unstable();
        v
    }).collect();
    let same = l.current_stage() == 6 && plans.iter().any(|p| *p == found);
    SCOREBOARD.record(same, || format!("spa_second_b_create_promotion_playoff_groups({:#x}) current {} found {:x?} plans {:x?}", this, l.current_stage() as i32, found, plans));
}

/// A Cup_Stage of a league (`exe::crt_operator_new` of 0xb2 and Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`,
/// thiscall, ret 0x3c; 0 when out of memory): this league, `n` clubs from `list`, the rounds, the competition, the
/// round table, the year, the stage, `legs`, the extra as its label, 0x14, 0, 0, 0 and the seed levels of the clubs
/// (`seeds`, a byte each; 0: none). (The original pushes the rounds and the year as dwords with stack leftovers above
/// the word; the constructor reads the words.)
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn league_cup_stage(l: League, n: u32, list: usize, table: usize, rounds: i16, extra: i16, stage: u32, legs: u32, seeds: usize) -> usize {
    let p = exe::crt_operator_new(0xb2);
    if p == 0 {
        return 0;
    }
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    construct(p, l.addr(), n, list, rounds as u16 as u32, l.competition().addr(), table, l.year() as u16 as u32, stage, legs, extra as u16 as u32, 0x14, 0, 0, 0, seeds)
}

/// The clubs of a cup stage's team table (`cup::TEAM_TABLE`, `cup::TEAM_COUNT` entries of 6 bytes), in order.
pub(crate) unsafe fn league_cup_stage_clubs(stage: usize) -> Vec<usize> {
    let c = Cup(stage);
    (0..c.team_count().max(0) as usize).map(|i| read_usize(c.team_table() + i * 6 + cup_team_entry::CLUB)).collect()
}

/// The four clubs of the relegation play-off: the sixteenth-placed club (row 15) of group 1 and of the group stages 0
/// to 2, in that order.
pub(crate) unsafe fn segunda_b_relegation_play_off_clubs(l: League) -> [usize; 4] {
    let mut list = [0usize; 4];
    for (g, slot) in list.iter_mut().enumerate() {
        *slot = aus_nsl_table_row_club(segunda_b_group(l, g as u8), 15); // rx09_3: the sixteenth of each group
    }
    list
}

/// Builds the relegation play-off of the Second Division B as stage 7: the four clubs of
/// `segunda_b_relegation_play_off_clubs` in a local list, the round table of stage 7 (`stage_round_table`), the
/// Cup_Stage (`league_cup_stage`: four clubs, stage 7, 1) into slot 7; none: the box at line 0x33d (the round table not
/// freed). Otherwise the round table freed, its round drawn (Cup::draw_round `exe::cup_draw_round`(stage, 0)) and the
/// current stage = 7.
/// Original: `SpaSecondB::create_relegation_playoff` (0x857860, spa_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_second_b_create_relegation_playoff(this: usize) {
    let l = League(this);
    let list = segunda_b_relegation_play_off_clubs(l);
    let (table, rounds, extra) = stage_round_table(l, 7);
    let p = league_cup_stage(l, 4, list.as_ptr() as usize, table, rounds, extra, 7, 1, 0);
    write_usize(l.stages() + 4 * 7, p);
    if read_usize(l.stages() + 4 * 7) == 0 {
        return show_error_at(SEGUNDA_B_SOURCE, 0x33d);
    }
    covered!("rx09_3 segunda b relegation play-off");
    exe::crt_free(table);
    exe::cup_draw_round(read_usize(l.stages() + 4 * 7), 0);
    l.set_current_stage(7);
}

/// Checks a built cup stage of a league against the planned clubs (as a set: the draw orders them) and the current
/// stage.
pub(crate) unsafe fn compare_league_cup_stage(name: &str, l: League, slot: usize, plan: &[usize]) {
    let stage = read_usize(l.stages() + 4 * slot);
    let mut found = if stage == 0 { Vec::new() } else { league_cup_stage_clubs(stage) };
    let mut want = plan.to_vec();
    found.sort_unstable();
    want.sort_unstable();
    SCOREBOARD.record(found == want && l.current_stage() == slot, || format!("{}({:#x}) stage {} current {}: plan {:x?} found {:x?}", name, l.addr(), slot, l.current_stage() as i32, want, found));
}

/// Comparer for `spa_second_b_create_relegation_playoff`: only the original runs (it allocates and draws); the stage's
/// clubs and the current stage against the plan.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_create_relegation_playoff(this: usize) {
    let l = League(this);
    let plan = segunda_b_relegation_play_off_clubs(l);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    compare_league_cup_stage("spa_second_b_create_relegation_playoff", l, 7, &plan);
}
