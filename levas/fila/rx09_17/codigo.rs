// ---------- rx09_17: the Asian World Cup qualifying first and second round groups and next stage (wc_asia_league.cpp) ----------

/// A League_Stage of the Asian qualifying (`exe::crt_operator_new` of 0xee; League_Stage::League_Stage
/// `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58; 0 when out of memory): this league, `n` teams from `list`, two
/// legs, the competition, the round table with its rounds, the win, draw and category bytes and the tie-breaks of this
/// league, the place counts at `places`, the year, the stage, the extra as its label, `importance`, 4, 0, the rule bits
/// `flags`, −1, 0, 2 (as `category_b_league_stage`, with 4 where the leagues pass 1).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn qualifying_league_stage(l: League, n: u32, list: usize, table: usize, rounds: i16, extra: i16, places: usize, stage: u32, importance: u32, flags: u32) -> usize {
    let p = exe::crt_operator_new(0xee);
    if p == 0 {
        return 0;
    }
    let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    ctor(p, l.addr(), n, list, 2, l.competition().addr(), table, rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32, l.match_category() as u32,
        l.addr() + league::TIEBREAKS, places, l.year() as u16 as u32, stage, extra as u16 as u32, importance, 4, 0, flags, -1, 0, 2)
}

/// Builds Asian first round group k (0 to 8: groups 1 to 9, after the first in the league's own table) as stage k: its
/// size (`wc_asia_group_size`(k)), a list of that many (`exe::crt_malloc`; none: the box at line 0x49a and 0) with the
/// group's drawn teams (slots of group k + 1 in `WC_ASIA_DRAW`), its round table (`stage_round_table`(k)) and the
/// League_Stage (`qualifying_league_stage`: the league's own places from `league::PROMOTED_COUNT`, importance and rule
/// bits) into slot k; the list and the round table freed. No stage: the box at line 0x4b0 and 0. Otherwise the current
/// stage = k, the group's draw news (League::send_group_draw_news on the stage) and 1.
/// Original: `WcAsiaLeague::create_first_round_group` (0x913540, wc_asia_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_create_first_round_group(this: usize, k: u32) -> u32 {
    let l = League(this);
    let size = wc_asia_group_size(this, k as u8 as i8);
    let n = size as i32;
    let list = exe::crt_malloc((n * 4) as u32);
    if list == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x49a);
        return 0;
    }
    let draw = read_usize(this + WC_ASIA_DRAW);
    for i in 0..n.max(0) as usize {
        write_usize(list + 4 * i, read_usize(draw + 16 * (k as usize + 1) + 4 * i)); // rx09_17: the group's drawn teams
    }
    let (table, rounds, extra) = stage_round_table(l, k);
    let stage = qualifying_league_stage(l, size as u16 as u32, list, table, rounds, extra, this + league::PROMOTED_COUNT, k, l.importance() as u32, l.league_flags() as u16 as u32);
    let slot = l.stages() + 4 * k as usize;
    write_usize(slot, stage);
    exe::crt_free(list);
    exe::crt_free(table);
    if read_usize(slot) == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x4b0);
        return 0;
    }
    covered!("rx09_17 asian first round group");
    l.set_current_stage(k as usize);
    exe::league_send_group_draw_news(read_usize(slot));
    1
}

/// Comparer for `wc_asia_league_create_first_round_group`: only the original runs (it allocates the stage and sends the
/// news); with a stage built, its teams (as a set) and the current stage against the group's draw.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_create_first_round_group(this: usize, k: u32) -> u32 {
    let l = League(this);
    let n = wc_asia_group_size(this, k as u8 as i8).max(0) as usize;
    let draw = read_usize(this + WC_ASIA_DRAW);
    let plan: Vec<usize> = (0..n).map(|i| read_usize(draw + 16 * (k as usize + 1) + 4 * i)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%)(this, k);
    if r == 1 {
        compare_category_b_stage("wc_asia_league_create_first_round_group", l, k as usize, &plan);
    }
    r
}

/// Rule: the next stage of the Asian qualifying: past the last stage, the error box; otherwise the current stage goes
/// up by one; at 9 (the first round is over) the two second round groups are built, at 11 the third round tie, at 12
/// the play-off.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box at line 0x471 and nothing
/// else. Otherwise the current stage + 1; at 9, WcAsiaLeague::create_second_round_groups (0x913740); at 11,
/// WcAsiaLeague::create_third_round_tie (0x913b60); at 12, WcAsiaLeague::create_playoff_tie (0x913d50) (thiscall,
/// ret 0).
/// Original: `WcAsiaLeague::advance_stage` (0x913490, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x471);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("rx09_17 asian next stage");
    match stage {
        9 => call_on_league(0x913740, l),
        11 => call_on_league(0x913b60, l),
        12 => call_on_league(0x913d50, l),
        _ => {}
    }
}

/// Comparer for `wc_asia_league_advance_stage` (`compare_advance_stage_building`: stages 9, 11 and 12 build).
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_advance_stage_building("wc_asia_league_advance_stage", League(this), |s| s == 9 || s == 11 || s == 12, || o(this), || wc_asia_league_advance_stage(this))
}

/// The winner (table row 0, League::get_table_row) of Asian first round group k (−1 the league's own table, 0 to 8 the
/// stages).
pub(crate) unsafe fn wc_asia_first_round_winner(l: League, k: i32) -> usize {
    let group = if k > -1 { read_usize(l.stages() + 4 * k as usize) } else { l.addr() };
    read_usize(exe::league_get_table_row(group, 0))
}

/// The places of the Asian second round groups (a local array of the original): with no exempt nation, 2 qualify and 1
/// goes to the third round; otherwise 1 and 1.
pub(crate) unsafe fn wc_asia_second_round_places(this: usize) -> [u8; 4] {
    let first = if read_u8(this + WC_ASIA_EXEMPT) == 0 { 2 } else { 1 }; // rx09_17: two qualify with no exempt nation
    [first, 1, 0, 0]
}

/// Builds the two Asian second round groups (stages 9 and 10, five teams each). A list of five (`exe::crt_malloc` of
/// 0x14; none: the box at line 0x4d9 and 0) takes five first round winners drawn at random (`exe::utils_random`(10):
/// the group r − 1, drawn again when taken; `wc_asia_first_round_winner`); its round table (`stage_round_table`(9)) and
/// the League_Stage (`qualifying_league_stage`: five teams, `wc_asia_second_round_places`, stage 9, importance 0xf,
/// rule bits 4) into slot 9; the list and the round table freed; none: the box at line 0x502 and 0. Otherwise the
/// group's draw news, the current stage = 9, and the other five winners in first round order into a second list (none:
/// the box at line 0x510 and 0) for stage 10 the same way (the box at line 0x534); then the current stage = 10, its draw
/// news and 1.
/// Original: `WcAsiaLeague::create_second_round_groups` (0x913740, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_create_second_round_groups(this: usize) -> u32 {
    let l = League(this);
    let places = wc_asia_second_round_places(this);
    let mut taken = [0u8; 10];
    let first = exe::crt_malloc(0x14);
    if first == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x4d9);
        return 0;
    }
    let mut placed = 0usize;
    while placed < 5 {
        let r = exe::utils_random(0xa) as usize;
        if taken[r] == 0 {
            write_usize(first + 4 * placed, wc_asia_first_round_winner(l, r as i32 - 1));
            placed += 1;
            taken[r] = 1;
        }
    }
    let build = |list: usize, stage: u32| {
        let (table, rounds, extra) = stage_round_table(l, stage);
        let p = qualifying_league_stage(l, 5, list, table, rounds, extra, places.as_ptr() as usize, stage, 0xf, 4);
        write_usize(l.stages() + 4 * stage as usize, p);
        exe::crt_free(list);
        exe::crt_free(table);
        read_usize(l.stages() + 4 * stage as usize)
    };
    let stage = build(first, 9);
    if stage == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x502);
        return 0;
    }
    covered!("rx09_17 asian second round groups");
    exe::league_send_group_draw_news(stage);
    l.set_current_stage(9);
    let second = exe::crt_malloc(0x14);
    if second == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x510);
        return 0;
    }
    let mut placed = 0usize;
    for k in -1i32..9 {
        if taken[(k + 1) as usize] == 0 {
            write_usize(second + 4 * placed, wc_asia_first_round_winner(l, k)); // rx09_17: the winners not drawn into the first group
            placed += 1;
        }
    }
    let stage = build(second, 10);
    if stage == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x534);
        return 0;
    }
    l.set_current_stage(10);
    exe::league_send_group_draw_news(stage);
    1
}

/// Comparer for `wc_asia_league_create_second_round_groups`: only the original runs (it draws and allocates); the two
/// groups together hold the ten first round winners, five each, and the current stage is 10.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_create_second_round_groups(this: usize) -> u32 {
    let l = League(this);
    let mut winners: Vec<usize> = (-1i32..9).map(|k| wc_asia_first_round_winner(l, k)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    if r == 1 {
        let (a, b) = (League(read_usize(l.stages() + 4 * 9)), League(read_usize(l.stages() + 4 * 10)));
        let (ca, cb) = (category_b_stage_clubs(a), category_b_stage_clubs(b));
        let mut found: Vec<usize> = ca.iter().chain(cb.iter()).copied().collect();
        found.sort_unstable();
        winners.sort_unstable();
        SCOREBOARD.record(ca.len() == 5 && cb.len() == 5 && found == winners && l.current_stage() == 10, || format!("wc_asia_league_create_second_round_groups({:#x}) winners {:x?} found {:x?} current {}", this,
            winners, found, l.current_stage() as i32));
    }
    r
}
