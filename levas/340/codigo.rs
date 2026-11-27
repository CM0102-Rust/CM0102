// ---------- rx09_1: the Spanish Second Division B at career creation, its rules and its round tables (spa_second_b.cpp) ----------

/// The source file of the Spanish Second Division B (spa_second_b.cpp), for the error boxes.
pub(crate) const SEGUNDA_B_SOURCE: usize = 0xa828f8;

/// A league split into regional groups (the Spanish Second Division B, the Swedish Second Division) as its
/// constructor builds it: the shape the constructors' comparer checks (`compare_league_with_stages_ctor`: vtable,
/// discipline, stage count; the table routine, the source file and the box lines in the order rules, table,
/// fixtures, sort, statistics), the rules written inline (the Swedish constructor does not call its set_up_rules) or
/// called, the group stage routine with the number of other groups, and the predicted positions routine.
pub(crate) struct GroupedLeagueCtor {
    pub(crate) shape: LeagueWithStages,
    pub(crate) inline_rules: Option<unsafe fn() -> Vec<(usize, u8, u32)>>,
    pub(crate) group_stage: usize,
    pub(crate) groups: u32,
    pub(crate) predicted: usize,
}

/// The body of a grouped league's constructor (thiscall(this, year, record), ret 8; SEH frame only for the operator
/// new of the statistics): League::League (`exe::new_league`), the record (`league::COMPETITION`), the vtable, the year
/// (word) and the discipline rule; the saved state (`exe::league_load_from_temp_file`, push 1) ends it. Otherwise stage
/// index 0xff, current stage −1, the stage count and its vector (`exe::crt_malloc` of 4 per stage, not checked); then,
/// each failure showing the box of its line and returning: the rules (written inline with the main round table, or
/// the set_up_rules routine), the table routine, League::sort_table (`exe::league_sort_table`(0)), the Comp_Stats
/// (operator new of 0x5ce, `exe::new_comp_stats`, into `league::COMP_STATS`) and League::create_fixtures. Last, the
/// other groups (the group stage routine with k = 0.., thiscall(k), ret 4; the original pushes a dword whose upper
/// bytes are stack leftovers and the callee reads the byte) and the predicted positions routine. Returns this.
pub(crate) unsafe fn construct_grouped_league(this: usize, year: u32, record: usize, g: &GroupedLeagueCtor, branch: &'static str) -> usize {
    let l = League(this);
    let d = &g.shape.division;
    exe::new_league(l);
    write_usize(this + league::COMPETITION, record);
    write_usize(this, d.vtabela);
    l.set_year(year as u16 as i16);
    l.set_discipline_rule(d.discipline);
    if exe::league_load_from_temp_file(l, 1) != 0 {
        return this;
    }
    covered!(branch);
    l.set_stage_index(stage_index::MAIN);
    l.set_current_stage(0xffff_ffff);
    l.set_stage_count(g.shape.stages as usize);
    l.set_stages(exe::crt_malloc(g.shape.stages.wrapping_mul(4)));
    let step = |f: usize| {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(f);
        f(this)
    };
    match g.inline_rules {
        Some(rules) => {
            eng_prm_apply_league_rules(l, &rules());
        }
        None => {
            if step(d.rules) == 0 {
                show_error_at(d.file, d.rows[0]);
                return this;
            }
        }
    }
    if step(d.table) == 0 {
        show_error_at(d.file, d.rows[1]);
        return this;
    }
    if exe::league_sort_table(l, 0) == 0 {
        show_error_at(d.file, d.rows[3]);
        return this;
    }
    let e = exe::crt_operator_new(0x5ce);
    let stats = if e != 0 { exe::new_comp_stats(e, l) } else { 0 };
    l.set_comp_stats(stats);
    if stats == 0 {
        show_error_at(d.file, d.rows[4]);
        return this;
    }
    if exe::league_create_fixtures(l) == 0 {
        show_error_at(d.file, d.rows[2]);
        return this;
    }
    let group_stage: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(g.group_stage);
    for k in 0..g.groups {
        group_stage(this, k);
    }
    call_on_league(g.predicted, l);
    this
}

/// The Spanish Second Division B: vtable 0x96fc84, the Spanish discipline rule 0x1a, nine stages after group 1 (groups
/// 2 to 4, the four promotion play-off groups, the relegation play-off and its final); SpaSecondB::set_up_rules
/// (0x856ef0, line 0x48), SpaSecondB::load_group_clubs (0x856cd0, line 0x4f), sort 0x56, statistics 0x5e, fixtures
/// 0x67; SpaSecondB::create_group_stage (0x857210) for groups 2 to 4 and SpaSecondB::set_predicted_positions
/// (0x8582f0).
pub(crate) const SPANISH_SEGUNDA_B: GroupedLeagueCtor = GroupedLeagueCtor {
    shape: LeagueWithStages {
        division: LeagueDivision { vtabela: 0x96fc84, discipline: 0x1a, rules: 0x856ef0, table: 0x856cd0, file: SEGUNDA_B_SOURCE, rows: [0x48, 0x4f, 0x67, 0x56, 0x5e], low: false },
        stages: 9, // rx09_1: the nine stages of the Second Division B
        finish: None,
    },
    inline_rules: None,
    group_stage: 0x857210,
    groups: 3,
    predicted: 0x8582f0,
};

/// Builds the Spanish Second Division B at game creation (`construct_grouped_league` with `SPANISH_SEGUNDA_B`): group 1 in the
/// league's own table, groups 2 to 4 as stages 0 to 2, then the predicted positions.
/// Original: `SpaSecondB::SpaSecondB` (0x855f40, spa_second_b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_spa_second_b(this: usize, year: u32, record: usize) -> usize {
    construct_grouped_league(this, year, record, &SPANISH_SEGUNDA_B, "rx09_1 new segunda b")
}

/// Comparer for `construct_spa_second_b` (`compare_league_with_stages_ctor` with the shape of `SPANISH_SEGUNDA_B`): only the
/// original runs (it allocates the league, the stages and the statistics).
pub(crate) unsafe extern "thiscall" fn compare_construct_spa_second_b(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SPA_SECOND_B);
    compare_league_with_stages_ctor("construct_spa_second_b", League(this), year, record, &SPANISH_SEGUNDA_B.shape, || o(this, year, record))
}

/// The rules of the Spanish Second Division B (four groups of 20): 2 legs, 3 points a win and 1 a draw, league matches,
/// national clubs; tie-breaks 4, 1, 2 (head to head, goal difference, goals for); none up directly, 4 to the promotion
/// play-off, 1 to the relegation play-off, 4 down; the Second Division above, nothing below; 3 substitutions, 5
/// substitutes; the rule bits 0x42 (a word). +0xab, +0xdb, +0xc8 and +0x52 are not written.
pub(crate) unsafe fn spanish_segunda_b_rules() -> Vec<(usize, u8, u32)> {
    vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1),
        (league::TIEBREAKS, 1, 4), (league::TIEBREAKS + 1, 1, 1), (league::TIEBREAKS + 2, 1, 2), (league::PROMOTED_COUNT, 1, 0),
        (league::PROMOTION_PLAYOFF_COUNT, 1, 4), (league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 4), // rx09_1: four go down
        (league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION) as u32), (league::SUBSTITUTIONS_ALLOWED, 1, 3),
        (league::LOWER_DIVISION_ID, 4, 0xffff_ffff), (league::LEAGUE_FLAGS, 2, 0x42), (league::SUBSTITUTES_NAMED, 1, 5)]
}

/// Rule: the Spanish Second Division B: 3 points a win and 1 a draw; tie-breaks head to head, goal difference, goals
/// for; the top four of each group to the promotion play-off, the sixteenth to the relegation play-off, the last four
/// down; the Second Division above (`spanish_segunda_b_rules`, then the main round table into `league::ROUND_DATES`).
/// Original: `SpaSecondB::set_up_rules` (0x856ef0, spa_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_second_b_set_up_rules(this: usize) -> u32 {
    covered!("rx09_1 segunda b rules");
    eng_prm_apply_league_rules(League(this), &spanish_segunda_b_rules())
}

/// Comparer for `spa_second_b_set_up_rules` (the rehearsal of `eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_SECOND_B_SET_UP_RULES);
    eng_prm_compare_league_rules("spa_second_b_set_up_rules", League(this), spanish_segunda_b_rules(), || o(this), || spa_second_b_set_up_rules(this))
}

/// A league round table (0x41-byte rounds) of a `create_schedule`: the league output 1 (when given), the rounds and
/// the extra (when there is one: the Swedish table leaves it alone below stage −1); the table (`exe::crt_malloc`; out of memory: the box at `row` of `file` and 0) with the rounds
/// (`spa_second_set_league_rounds`, the year pushed as the word `league::YEAR`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn league_round_table(this: League, rounds: *mut i16, extra: *mut i16, league: *mut i32, table: &[SpaSecondScheduleRound], code: Option<i16>, file: usize, row: u32,
    branch: &'static str) -> usize {
    if !league.is_null() {
        *league = 1;
    }
    if let Some(code) = code {
        *extra = code;
    }
    *rounds = table.len() as i16;
    let t = exe::crt_malloc((table.len() * 0x41) as u32);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    covered!(branch);
    spa_second_set_league_rounds(t, (this.year() as u16) as u32, 0, table);
    t
}

/// A cup round table (0x68-byte rounds) of a `create_schedule`: the league output 0 (when given), the rounds and the
/// extra; the table (`exe::crt_malloc` of 0x68 per round; out of memory: the box at `row` of `file` and 0) with the
/// rounds (`write_play_off_round`, the year sign-extended).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn cup_round_table(this: League, rounds: *mut i16, extra: *mut i16, league: *mut i32, table: &[EngCcCupCupRound], code: i16, file: usize, row: u32,
    branch: &'static str) -> usize {
    if !league.is_null() {
        *league = 0;
    }
    *rounds = table.len() as i16;
    *extra = code;
    let t = exe::crt_malloc((table.len() * 0x68) as u32);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    covered!(branch);
    for (i, r) in table.iter().enumerate() {
        write_play_off_round(this, t, i as u32, r);
    }
    t
}

/// The Second Division B groups: 38 rounds on Sundays (weekday 6) from 22 August to 14 May, the winter break from 19
/// December (that round on its own date, weekday −1, code 2) to 9 January, a Wednesday round on 5 April; no TV slots.
pub(crate) const SEGUNDA_B_GROUP_ROUNDS: [SpaSecondScheduleRound; 38] = [
    (22, 7, 0, 6, 1, &[(-1, -1, -1)]), // 0: 22/ago (rx09_1: the first group round)
    (29, 7, 0, 6, 1, &[(-1, -1, -1)]), // 1: 29/ago
    (5, 8, 0, 6, 1, &[(-1, -1, -1)]), // 2: 5/set
    (12, 8, 0, 6, 1, &[(-1, -1, -1)]), // 3: 12/set
    (19, 8, 0, 6, 1, &[(-1, -1, -1)]), // 4: 19/set
    (26, 8, 0, 6, 1, &[(-1, -1, -1)]), // 5: 26/set
    (3, 9, 0, 6, 1, &[(-1, -1, -1)]), // 6: 3/out
    (10, 9, 0, 6, 1, &[(-1, -1, -1)]), // 7: 10/out
    (17, 9, 0, 6, 1, &[(-1, -1, -1)]), // 8: 17/out
    (24, 9, 0, 6, 1, &[(-1, -1, -1)]), // 9: 24/out
    (31, 9, 0, 6, 1, &[(-1, -1, -1)]), // 10: 31/out
    (7, 10, 0, 6, 1, &[(-1, -1, -1)]), // 11: 7/nov
    (14, 10, 0, 6, 1, &[(-1, -1, -1)]), // 12: 14/nov
    (21, 10, 0, 6, 1, &[(-1, -1, -1)]), // 13: 21/nov
    (28, 10, 0, 6, 1, &[(-1, -1, -1)]), // 14: 28/nov
    (5, 11, 0, 6, 1, &[(-1, -1, -1)]), // 15: 5/dez
    (12, 11, 0, 6, 1, &[(-1, -1, -1)]), // 16: 12/dez
    (19, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 19/dez
    (9, 0, 1, 6, 1, &[(-1, -1, -1)]), // 18: 9/jan +1
    (16, 0, 1, 6, 1, &[(-1, -1, -1)]), // 19: 16/jan +1
    (23, 0, 1, 6, 1, &[(-1, -1, -1)]), // 20: 23/jan +1
    (30, 0, 1, 6, 1, &[(-1, -1, -1)]), // 21: 30/jan +1
    (6, 1, 1, 6, 1, &[(-1, -1, -1)]), // 22: 6/fev +1
    (13, 1, 1, 6, 1, &[(-1, -1, -1)]), // 23: 13/fev +1
    (20, 1, 1, 6, 1, &[(-1, -1, -1)]), // 24: 20/fev +1
    (27, 1, 1, 6, 1, &[(-1, -1, -1)]), // 25: 27/fev +1
    (5, 2, 1, 6, 1, &[(-1, -1, -1)]), // 26: 5/mar +1
    (12, 2, 1, 6, 1, &[(-1, -1, -1)]), // 27: 12/mar +1
    (19, 2, 1, 6, 1, &[(-1, -1, -1)]), // 28: 19/mar +1
    (26, 2, 1, 6, 1, &[(-1, -1, -1)]), // 29: 26/mar +1
    (2, 3, 1, 6, 1, &[(-1, -1, -1)]), // 30: 2/abr +1
    (5, 3, 1, 2, 2, &[(-1, -1, -1)]), // 31: 5/abr +1
    (9, 3, 1, 6, 1, &[(-1, -1, -1)]), // 32: 9/abr +1
    (16, 3, 1, 6, 1, &[(-1, -1, -1)]), // 33: 16/abr +1
    (23, 3, 1, 6, 1, &[(-1, -1, -1)]), // 34: 23/abr +1
    (30, 3, 1, 6, 1, &[(-1, -1, -1)]), // 35: 30/abr +1
    (7, 4, 1, 6, 1, &[(-1, -1, -1)]), // 36: 7/mai +1
    (14, 4, 1, 6, 1, &[(-1, -1, -1)]), // 37: 14/mai +1
];

/// The four promotion play-off groups of the Second Division B: six rounds from 21 May to 11 June, on Sundays with two
/// Wednesday rounds (weekday 2, code 2).
pub(crate) const SEGUNDA_B_PLAY_OFF_GROUP_ROUNDS: [SpaSecondScheduleRound; 6] = [
    (21, 4, 1, 6, 1, &[(-1, -1, -1)]), // 0: 21/mai +1 (rx09_1: the first play-off group round)
    (24, 4, 1, 2, 2, &[(-1, -1, -1)]), // 1: 24/mai +1
    (28, 4, 1, 6, 1, &[(-1, -1, -1)]), // 2: 28/mai +1
    (31, 4, 1, 2, 2, &[(-1, -1, -1)]), // 3: 31/mai +1
    (4, 5, 1, 6, 1, &[(-1, -1, -1)]), // 4: 4/jun +1
    (11, 5, 1, 6, 1, &[(-1, -1, -1)]), // 5: 11/jun +1
];

/// The relegation play-off of the Second Division B (stage 7): drawn on 15 May (weekday 0), two ties of two legs 7 days
/// apart from 21 May (weekday 6), four clubs, category 5, two-leg tie-break 3, no prizes.
pub(crate) const SEGUNDA_B_RELEGATION_PLAY_OFF: EngCcCupCupRound = cql10r((0xf, 4, 1, 0), (0x15, 4, 1, 6, 1, 0), 0, (0, 0, 3), 5, (4, 2, 4, 0), (0, 2, 7), (0, 0, 0));

/// The relegation play-off final of the Second Division B (stage 8): drawn on 29 May (weekday 0), one tie of two legs 7
/// days apart from 4 June (weekday 6), two clubs, category 5, two-leg tie-break 3, no prizes.
pub(crate) const SEGUNDA_B_RELEGATION_FINAL: EngCcCupCupRound = cql10r((0x1d, 4, 1, 0), (4, 5, 1, 6, 1, 0), 0, (0, 0, 3), 5, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)); // rx09_1: the final on 4 June

/// The round tables of the Spanish Second Division B by stage (the signed byte of the argument): stages up to 2 (group 1
/// at −1, groups 2 to 4 at 0 to 2) the 38 group rounds, extra = stage + 0x3e9 (line 0xb0); stages 3 to 6 (the promotion
/// play-off groups) the six play-off rounds, extra = stage + 0x44a (line 0x137); stage 7 the relegation play-off, extra
/// 0x118 (line 0x15d); stage 8 its final, extra 0x122 (line 0x185); later stages 0 with nothing written.
/// Original: `SpaSecondB::create_schedule` (0x856260, spa_second_b.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn spa_second_b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let l = League(this);
    let s = stage as u8 as i8;
    if s < 3 {
        league_round_table(l, rounds, extra, league, &SEGUNDA_B_GROUP_ROUNDS, Some((s as i16).wrapping_add(0x3e9)), SEGUNDA_B_SOURCE, 0xb0, "rx09_1 segunda b group rounds")
    } else if s < 7 {
        league_round_table(l, rounds, extra, league, &SEGUNDA_B_PLAY_OFF_GROUP_ROUNDS, Some((s as i16).wrapping_add(0x44a)), SEGUNDA_B_SOURCE, 0x137, "rx09_1 segunda b play-off group rounds")
    } else if s == 7 {
        cup_round_table(l, rounds, extra, league, &[SEGUNDA_B_RELEGATION_PLAY_OFF], 0x118, SEGUNDA_B_SOURCE, 0x15d, "rx09_1 segunda b relegation play-off round")
    } else if s == 8 {
        cup_round_table(l, rounds, extra, league, &[SEGUNDA_B_RELEGATION_FINAL], 0x122, SEGUNDA_B_SOURCE, 0x185, "rx09_1 segunda b relegation final round")
    } else {
        0
    }
}

/// Side-by-side comparer of a `create_schedule` whose stages are league round tables or cup rounds (`cup` says which,
/// for the stage asked): the original with the caller's outputs, the copy with its own, from the same [0xb67a34]; the
/// outputs and the bytes each writes in its table (`written_league_round_bytes` or `eng_cc_cup_written_round_bytes`)
/// must match; the copy's table is freed.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn compare_league_or_cup_schedule(name: &str, stage: u32, cup: bool, rounds: *mut i16, extra: *mut i16, league: *mut i32,
    original: impl FnOnce() -> usize, copy: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let pl2 = if league.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, same, _, _) = crate::comparison::side_by_side(&[(0xb67a34usize, 4usize)], original, || copy(&mut r2, &mut e2, pl2));
    let outs = (a == 0 && b == 0) || (*rounds == r2 && *extra == e2 && (league.is_null() || *league == l2));
    let table = if a != 0 && b != 0 {
        if cup {
            eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, *rounds)
        } else {
            written_league_round_bytes(a, a, *rounds as usize) == written_league_round_bytes(b, a, *rounds as usize)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(same && outs && table, || format!("{}(stage {}): outs {} table {}", name, stage as u8 as i8, outs, table));
    a
}

/// Comparer for `spa_second_b_create_schedule` (`compare_league_or_cup_schedule`; cup rounds at stages 7 and 8).
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SPA_SECOND_B_CREATE_SCHEDULE);
    let cup = matches!(stage as u8 as i8, 7 | 8);
    compare_league_or_cup_schedule("spa_second_b_create_schedule", stage, cup, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| spa_second_b_create_schedule(this, stage, r, e, l))
}

// ---------- rx09_2: the Spanish Second Division B group 1 table, turnover and next stage (spa_second_b.cpp) ----------

/// The ids of the four Second Division B groups, in group order.
pub(crate) const SEGUNDA_B_GROUPS: [usize; 4] = [KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B2, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B3,
    KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B4];

/// The first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] clubs (the count read again at each step, as the original
/// does), in database order.
pub(crate) unsafe fn domestic_clubs() -> Vec<Club> {
    let mut v = Vec::new();
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2)) {
        v.push(Club(club));
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    v
}

/// The clubs of a Second Division B group, in database order: those with a division (`club::DIVISION`) and a reserve
/// division (`club::RESERVE_DIVISION`, here the group) whose ids are the Second Division B's
/// ([`KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B`]) and the group's (`group`, a key).
pub(crate) unsafe fn segunda_b_group_clubs(group: usize) -> Vec<Club> {
    domestic_clubs().into_iter().filter(|c| {
        let division = read_usize(c.addr() + club::DIVISION);
        let reserve = read_usize(c.addr() + club::RESERVE_DIVISION);
        division != 0 && reserve != 0 && read_usize(reserve) == read_usize(group) && read_usize(division) == read_usize(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B)
    }).collect()
}

/// In 2001 the database puts the Second Division B clubs in their group's competition: each club whose division's id
/// is one of the four groups (`SEGUNDA_B_GROUPS`) gets that group as its reserve division (`club::RESERVE_DIVISION`)
/// and this league's record as its division.
pub(crate) unsafe fn move_segunda_b_groups(l: League) {
    for c in domestic_clubs() {
        let division = read_usize(c.addr() + club::DIVISION);
        if division != 0 && SEGUNDA_B_GROUPS.iter().any(|&g| read_usize(division) == read_usize(g)) {
            write_usize(c.addr() + club::RESERVE_DIVISION, division);
            write_usize(c.addr() + club::DIVISION, l.competition().addr()); // rx09_2: the group club joins the Second Division B
        }
    }
}

/// Builds the table of group 1 of the Spanish Second Division B: `club_count` = 20; in 2001 (the word
/// `league::YEAR`) the clubs are first moved into the league (`move_segunda_b_groups`); the table (`exe::crt_malloc` of
/// 0x3b per club; out of memory: the box at line 0x1cf and 0) and a row (League::init_table_row
/// `exe::league_init_table_row`(this, k, club, 0, 0)) for each club of group 1 (`segunda_b_group_clubs`). A 21st club
/// shows the box at line 0x1e0, loses its division (`club::DIVISION` = 0) and ends the scan. Returns 1 when the table
/// is full, else 0.
/// Original: `SpaSecondB::load_group_clubs` (0x856cd0, spa_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_second_b_load_group_clubs(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(0x14);
    if l.year() == 0x7d1 {
        move_segunda_b_groups(l);
    }
    let t = exe::crt_malloc((l.club_count() as i32 * league_table_row::SIZE as i32) as u32);
    l.set_table(t);
    if t == 0 {
        show_error_at(SEGUNDA_B_SOURCE, 0x1cf);
        return 0;
    }
    covered!("rx09_2 segunda b group 1 table");
    let mut k: u8 = 0;
    for c in segunda_b_group_clubs(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1) {
        if (k as i8 as i16) >= l.club_count() {
            show_error_at(SEGUNDA_B_SOURCE, 0x1e0);
            write_usize(c.addr() + club::DIVISION, 0);
            break;
        }
        exe::league_init_table_row(l, k as u32, c, 0, 0); // rx09_2: the row of the k-th club
        k = k.wrapping_add(1);
    }
    ((k as i8 as i16) == l.club_count()) as u32
}

/// Comparer for `spa_second_b_load_group_clubs`: only the original runs (it allocates the table and moves the clubs in
/// 2001); afterwards no club is left in a group's competition, the club count is 20, and the rows and the result
/// match group 1's clubs as they then stand.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_load_group_clubs(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_SECOND_B_LOAD_GROUP_CLUBS)(this);
    let l = League(this);
    let plan: Vec<usize> = segunda_b_group_clubs(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1).iter().take(20).map(|c| c.addr()).collect();
    let left = domestic_clubs().iter().filter(|c| {
        let d = read_usize(c.addr() + club::DIVISION);
        d != 0 && SEGUNDA_B_GROUPS.iter().any(|&g| read_usize(d) == read_usize(g))
    }).count();
    let found: Vec<usize> = if l.table() != 0 { (0..plan.len()).map(|k| aus_nsl_table_row_club(l, k)).collect() } else { Vec::new() };
    let same = r == (plan.len() == 20) as u32 && l.club_count() == 20 && found == plan && (l.year() != 0x7d1 || left == 0);
    SCOREBOARD.record(same, || format!("spa_second_b_load_group_clubs({:#x}) original={} plan={:x?} table={:x?} left in groups {}", this, r, plan, found, left));
    r
}

/// The turnover of a league split into regional groups (the Turkish category B mould): the season reopened, League::
/// free_table (`exe::league_free_table`(0)), the round dates freed and cleared; with statistics,
/// `exe::comp_stats_archive_season_and_reset`(1); with a current stage ≥ 0, the stages 0..=current (re-read on each
/// step) destroyed by their virtual 0 with 1; the year + 1 and the current stage −1. Then, each with the box of `file`
/// at its line (`lines`) and 0 returned when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table
/// routine (thiscall, ret 0), League::sort_table (`exe::league_sort_table`(0)) and League::create_fixtures. Then the
/// group stage routine (thiscall(k), ret 4) for k = 0..`groups`, the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`) and OfficialsManager::renew_officials on `G_OFFICIALS_MANAGER` with the
/// record, whose eax is returned.
pub(crate) unsafe fn grouped_league_new_season(this: League, table: usize, group_stage: usize, groups: u32, file: usize, lines: [u32; 4], branch: &'static str) -> u32 {
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    let dates = this.round_dates();
    if dates != 0 {
        exe::crt_free(dates);
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
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    this.set_year(this.year().wrapping_add(1)); // rx09_2: the next season
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, lines[0]);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, lines[1]);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, lines[2]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, lines[3]);
        return 0;
    }
    covered!(branch);
    let group_stage: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(group_stage);
    for k in 0..groups {
        group_stage(this.addr(), k);
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32
}

/// Rule: the Spanish Second Division B turnover: last season's stages destroyed, the next year, the rules, group 1's
/// table, the order and the fixtures, then groups 2 to 4, the predicted positions and the officials
/// (`grouped_league_new_season` with SpaSecondB::load_group_clubs 0x856cd0 and SpaSecondB::create_group_stage 0x857210,
/// boxes at lines 0x23b, 0x242, 0x249 and 0x24f).
/// Original: `SpaSecondB::start_new_season` (0x856f90, spa_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_second_b_start_new_season(this: usize) -> u32 {
    grouped_league_new_season(League(this), 0x856cd0, 0x857210, 3, SEGUNDA_B_SOURCE, [0x23b, 0x242, 0x249, 0x24f], "rx09_2 segunda b turnover")
}

/// Comparer for `spa_second_b_start_new_season`: only the original runs (it frees and rebuilds the league); the year
/// must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_SECOND_B_START_NEW_SEASON);
    compare_state_league_new_season("spa_second_b_start_new_season", League(this), || o(this))
}

/// Rule: the next stage of the Spanish Second Division B: past the last stage, the error box; otherwise the current
/// stage goes up by one; on reaching stage 3 (the groups are over) the four promotion play-off groups and the
/// relegation play-off are built, on reaching stage 8 the relegation play-off final.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box at line 0x26d and nothing
/// else. Otherwise the current stage + 1; at 3, SpaSecondB::create_promotion_playoff_groups (0x857570) and
/// SpaSecondB::create_relegation_playoff (0x857860); at 8, SpaSecondB::create_relegation_playoff_final (0x857a00) (all
/// thiscall, ret 0).
/// Original: `SpaSecondB::advance_stage` (0x857150, spa_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_second_b_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(SEGUNDA_B_SOURCE, 0x26d);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("rx09_2 segunda b next stage");
    if stage == 3 {
        call_on_league(0x857570, l);
        call_on_league(0x857860, l);
    } else if stage == 8 {
        call_on_league(0x857a00, l);
    }
}

/// Comparer for `spa_second_b_advance_stage` (`compare_advance_stage_building`: stages 3 and 8 build).
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPA_SECOND_B_ADVANCE_STAGE);
    compare_advance_stage_building("spa_second_b_advance_stage", League(this), |s| s == 3 || s == 8, || o(this), || spa_second_b_advance_stage(this))
}

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
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SPA_SECOND_B_CREATE_GROUP_STAGE)(this, group);
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
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPA_SECOND_B_CREATE_PROMOTION_PLAYOFF_GROUPS)(this);
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
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPA_SECOND_B_CREATE_RELEGATION_PLAYOFF)(this);
    compare_league_cup_stage("spa_second_b_create_relegation_playoff", l, 7, &plan);
}

// ---------- rx09_4: the Spanish Second Division B relegation final, daily update and staff records (spa_second_b.cpp) ----------

/// The two clubs of the relegation play-off final: the first two entries of stage 7's team table (`cup::TEAM_TABLE`)
/// whose status (`cup_team_entry::STATUS`) is 2, scanning while the team count (re-read) allows.
pub(crate) unsafe fn segunda_b_relegation_final_clubs(l: League) -> Vec<usize> {
    let s7 = Cup(read_usize(l.stages() + 4 * 7));
    let mut v = Vec::new();
    let mut i = 0i16;
    while i < s7.team_count() && v.len() < 2 {
        let e = s7.team_table() + 6 * i as usize;
        if read_u8(e + cup_team_entry::STATUS) == 2 { // rx09_4: the losers of the relegation play-off
            v.push(read_usize(e + cup_team_entry::CLUB));
        }
        i += 1;
    }
    v
}

/// Builds the relegation play-off final of the Second Division B as stage 8: the two clubs of
/// `segunda_b_relegation_final_clubs` in a local list of two, the round table of stage 8 (`stage_round_table`), the
/// Cup_Stage (`league_cup_stage`: two clubs, stage 8, 1) into slot 8; none: the box at line 0x371 (the round table not
/// freed). Otherwise the round table freed, its round set up (Cup::set_up_round `exe::cup_set_up_round`(stage, 0)) and
/// the current stage = 8. With fewer than two losers the original passes stack leftovers in the empty slots and the
/// copy zeros (never seen: the two ties have two losers).
/// Original: `SpaSecondB::create_relegation_playoff_final` (0x857a00, spa_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_second_b_create_relegation_playoff_final(this: usize) {
    let l = League(this);
    let mut list = [0usize; 2];
    for (k, c) in segunda_b_relegation_final_clubs(l).into_iter().enumerate() {
        list[k] = c;
    }
    let (table, rounds, extra) = stage_round_table(l, 8);
    let p = league_cup_stage(l, 2, list.as_ptr() as usize, table, rounds, extra, 8, 1, 0);
    write_usize(l.stages() + 4 * 8, p);
    if read_usize(l.stages() + 4 * 8) == 0 {
        return show_error_at(SEGUNDA_B_SOURCE, 0x371);
    }
    covered!("rx09_4 segunda b relegation final");
    exe::crt_free(table);
    exe::cup_set_up_round(read_usize(l.stages() + 4 * 8), 0);
    l.set_current_stage(8);
}

/// Comparer for `spa_second_b_create_relegation_playoff_final`: only the original runs (it allocates the stage); with
/// two losers, the stage's clubs and the current stage against the plan.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_create_relegation_playoff_final(this: usize) {
    let l = League(this);
    let plan = segunda_b_relegation_final_clubs(l);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPA_SECOND_B_CREATE_RELEGATION_PLAYOFF_FINAL)(this);
    if plan.len() == 2 {
        compare_league_cup_stage("spa_second_b_create_relegation_playoff_final", l, 8, &plan);
    }
}

/// True when the Second Division B is at stage 7 (`league::CURRENT_STAGE`) and its relegation play-off is complete
/// (the stage's virtual `comp_vtable::CHECK_COMPLETE`(0, 0), the byte of its answer).
pub(crate) unsafe fn segunda_b_relegation_play_off_done(l: League) -> bool {
    if l.current_stage() as i32 != 7 {
        return false;
    }
    let s7 = read_usize(l.stages() + 4 * 7);
    let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(s7) + comp_vtable::CHECK_COMPLETE));
    complete(s7, 0, 0) as u8 != 0
}

/// The day of the Spanish Second Division B: when the relegation play-off is over (`segunda_b_relegation_play_off_done`)
/// the league moves to its next stage (its virtual `comp_vtable::ADVANCE_STAGE`: the final); then the league day
/// (League::update_for_date `exe::league_update_for_date`, the arguments as they came), whose answer it returns.
/// Original: `SpaSecondB::update_for_date` (0x858160, spa_second_b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn spa_second_b_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    if segunda_b_relegation_play_off_done(l) {
        covered!("rx09_4 segunda b relegation play-off over");
        bra_nat_first_call_virtual_no_args(this, comp_vtable::ADVANCE_STAGE);
    }
    exe::league_update_for_date(this, today, full) as u32
}

/// Comparer for `spa_second_b_update_for_date`: only the original runs (it builds the final and plays the day); when
/// the relegation play-off was over the league must be at stage 8 afterwards.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    let due = segunda_b_relegation_play_off_done(l);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_SPA_SECOND_B_UPDATE_FOR_DATE)(this, today, full);
    SCOREBOARD.record(!due || l.current_stage() as i32 == 8, || format!("spa_second_b_update_for_date({:#x}) final due, stage {}", this, l.current_stage() as i32));
    r
}

/// The staff record a Second Division B status adds (`spa_second_b_update_staff_records_for_status`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SegundaBRecord {
    /// Staff_Records::add_qualified_for_round_record with 0x438, 0x41c, 0x28 (through to the promotion play-off).
    Qualified,
    /// Staff_Records::add_promotion_record with 0x64.
    Promotion,
    /// Staff_Records::add_relegation_record.
    Relegation,
}

/// What `spa_second_b_update_staff_records_for_status` does for a club, worked out without side effects: the row of
/// its group and the status written there, and the record added (the result is always 0).
#[derive(Debug)]
pub(crate) struct SegundaBRecordPlan {
    pub(crate) row: Option<(usize, u8)>,
    pub(crate) record: Option<SegundaBRecord>,
}

/// The group of a Second Division B club by the id of its reserve division (the group): B1 the league's own table,
/// B2 to B4 the group stages 0 to 2. Defect of the original: for any other id the status argument itself is taken as
/// the group (its fields are then read from that address, an access violation); the copy does the same.
pub(crate) unsafe fn segunda_b_group_of(l: League, reserve: usize, status: u32) -> League {
    match SEGUNDA_B_GROUPS.iter().position(|&g| read_usize(reserve) == read_usize(g)) {
        Some(k) => segunda_b_group(l, k as u8),
        None => League(status as usize),
    }
}

/// The address of the club's row in a group's table, if it has one (rows 0 to `league::CLUB_COUNT` − 1).
pub(crate) unsafe fn league_row_of(group: League, club: Club) -> Option<usize> {
    let mut i = 0i16;
    while i < group.club_count() {
        let row = group.table() + i as usize * league_table_row::SIZE;
        if read_usize(row) == club.addr() {
            return Some(row);
        }
        i += 1;
    }
    None
}

/// The plan of `spa_second_b_update_staff_records_for_status` for a club with `status` (its byte) at `stage` (signed):
/// - the groups (below 3): status 1 adds the qualified record;
/// - the promotion play-off groups (3 to 6): with a reserve division, the club's row in its group
///   (`segunda_b_group_of`, `league_row_of`; none: nothing); status 0 writes 0 in the row with the promotion record,
///   any other status writes 0xfe;
/// - the relegation play-off (7): status 1 writes 0xfe in the club's row;
/// - its final (8): with a reserve division, status 1 writes 0xfe in the row; any other status writes 3 in the row
///   (when found) and, for status 2, adds the relegation record (found or not);
/// - later stages: nothing.
pub(crate) unsafe fn segunda_b_record_plan(l: League, club: Club, status: u32, stage: i8) -> SegundaBRecordPlan {
    let mut plan = SegundaBRecordPlan { row: None, record: None };
    let s = status as u8;
    if stage < 3 {
        if s == 1 {
            plan.record = Some(SegundaBRecord::Qualified);
        }
        return plan;
    }
    if stage > 8 || (stage == 7 && s != 1) {
        return plan;
    }
    let reserve = club.reserve_division().addr();
    if reserve == 0 {
        return plan;
    }
    let row = league_row_of(segunda_b_group_of(l, reserve, status), club);
    match stage {
        3..=6 => {
            if let Some(row) = row {
                if s != 0 {
                    plan.row = Some((row, 0xfe));
                } else {
                    plan.row = Some((row, 0)); // rx09_4: the play-off group winner's row
                    plan.record = Some(SegundaBRecord::Promotion);
                }
            }
        }
        7 => plan.row = row.map(|r| (r, 0xfe)),
        _ => {
            if s == 1 {
                plan.row = row.map(|r| (r, 0xfe));
            } else {
                plan.row = row.map(|r| (r, 3));
                if s == 2 {
                    plan.record = Some(SegundaBRecord::Relegation);
                }
            }
        }
    }
    plan
}

/// Updates the staff records by a club's status in the Spanish Second Division B and marks its row in its group
/// (`segunda_b_record_plan`: the row status, then the record; the competition is this league's,
/// `league::COMPETITION`). The last three arguments are not read; the result is 0.
/// Original: `SpaSecondB::update_staff_records_for_status` (0x857b70, spa_second_b.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn spa_second_b_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    let l = League(this);
    let plan = segunda_b_record_plan(l, Club(club), status, stage as u8 as i8);
    if let Some((row, s)) = plan.row {
        covered!("rx09_4 segunda b row status");
        write_u8(row + league_table_row::STATUS, s);
    }
    let records = read_usize(G_STAFF_RECORDS);
    let comp = l.competition().addr();
    match plan.record {
        Some(SegundaBRecord::Qualified) => exe::staff_records_add_qualified_for_round_record(records, club, comp, 0x438, 0x41c, 0x28),
        Some(SegundaBRecord::Promotion) => exe::staff_records_add_promotion_record(records, club, comp, 0x64),
        Some(SegundaBRecord::Relegation) => exe::staff_records_add_relegation_record(records, club, comp),
        None => {}
    }
    0
}

/// Comparer for `spa_second_b_update_staff_records_for_status`: only the original runs (it adds records); its result
/// (0) and the status of the planned row against the plan.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let plan = segunda_b_record_plan(League(this), Club(club), status, stage as u8 as i8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(IDX_SPA_SECOND_B_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, status, stage, a4, a5, a6);
    let row_ok = plan.row.map_or(true, |(row, s)| read_u8(row + league_table_row::STATUS) == s);
    SCOREBOARD.record(r == 0 && row_ok, || format!("spa_second_b_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {}) original={:#x} plan={:?}", this, club,
        status as u8, stage as u8 as i8, r, plan));
    r
}

// ---------- rx09_5: the Spanish Second Division B fate news text, club list and predicted positions (spa_second_b.cpp) ----------

/// The text of a club that missed promotion from a play-off group (with a news item): the stage's name
/// (`exe::comp_util_get_stage_name` of `stage_code` into a local of 0x64 bytes) and its gender byte
/// (`exe::comp_util_get_stage_name_gender`), then 0xa829ac "{club} have failed to win promotion from {group} in the
/// {competition}." with the club's short gender byte twice, the stage's twice and the competition's (the long name's,
/// `competition::GENDER_NAME`) twice, the club's short name, the stage name and the competition's name
/// (`competition::NAME`) into the shared buffer (`PRAISE_TEXT_BUFFER`); the buffer set in `text` with the club's link
/// (0x7d5) and the competition's (0x7d0).
pub(crate) unsafe fn failed_promotion_from_group_text(club_id: u32, comp: usize, stage_code: u32, text: usize) {
    let mut name = [0u8; 0x64];
    exe::comp_util_get_stage_name(name.as_mut_ptr(), 0x64, stage_code, 0);
    let sg = exe::comp_util_get_stage_name_gender(stage_code, 0) as i8 as i32;
    let club = club_at(club_id as isize);
    let g = club.short_gender_name() as i8 as i32;
    let cg = read_u8(comp + competition::GENDER_NAME) as i8 as i32;
    let team = club.addr() + club::SHORT_NAME;
    story_lang!(PRAISE_TEXT_BUFFER, 0xa829ac, g, g, sg, sg, cg, cg, team, name.as_ptr(), comp + competition::NAME);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, team as *const u8, 0x7d5, club.addr());
    exe::text_add_link(text, (comp + competition::NAME) as *const u8, 0x7d0, comp);
}

/// True when `spa_second_b_build_fate_news_text` writes the text itself (not the common text nor the cup tie news).
pub(crate) fn segunda_b_writes_own_text(stage: i8, outcome: u8, news: u32) -> bool {
    match stage {
        3..=6 => outcome == 0xfe,
        7 | 8 => news == 0 && (outcome == 1 || outcome == 2),
        _ => false,
    }
}

/// The fate news text of a Spanish Second Division B club (thiscall, ret 0x28; same arguments as
/// Comp::build_fate_news_text):
/// - the groups (stage below 3): the common text (`exe::comp_build_fate_news_text`, a7 = 0, the rest as they came);
/// - the promotion play-off groups (3 to 6): outcome 0 the common text with outcome 0; outcome 0xfe, with a news item,
///   `failed_promotion_from_group_text` (the stage code a4) and, without one, 0xa82970 "{club} fail to win promotion"
///   (`play_off_club_text`); any other outcome nothing;
/// - the relegation play-off (7) and its final (8): with a news item the cup tie news
///   (`exe::comp_util_write_cup_tie_result_news`(club, a5, a8, outcome, a6, text)); without one, outcome 1 gives
///   0x987594 "{club} avoid relegation", outcome 2 0xa82934 "{club} slip nearer relegation" at 7 and 0x987564 "{club}
///   relegated" at 8; any other nothing;
/// - later stages: nothing.
/// Original: `SpaSecondB::build_fate_news_text` (0x857e20, spa_second_b.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn spa_second_b_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let s = stage as u8 as i8;
    let common = |outcome: u32| exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, news, text);
    if s < 3 {
        return common(outcome);
    }
    if s < 7 {
        match outcome as u8 {
            0 => common(0),
            0xfe if news != 0 => {
                covered!("rx09_5 segunda b failed promotion news");
                failed_promotion_from_group_text(club_id, League(this).competition().addr(), a4, text);
            }
            0xfe => {
                covered!("rx09_5 segunda b failed promotion text");
                play_off_club_text(club_id, 0xa82970, text); // rx09_5: "{club} fail to win promotion"
            }
            _ => {}
        }
        return;
    }
    if s != 7 && s != 8 {
        return;
    }
    if news != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text);
    }
    match outcome as u8 {
        1 => play_off_club_text(club_id, 0x987594, text), // "{club} avoid relegation"
        2 => {
            covered!("rx09_5 segunda b relegation text");
            play_off_club_text(club_id, if s == 7 { 0xa82934 } else { 0x987564 }, text); // rx09_5: slip nearer relegation / relegated
        }
        _ => {}
    }
}

/// Comparer for `spa_second_b_build_fate_news_text`: the texts the routine writes itself side by side with
/// `compare_text_writer`; the common text and the cup tie news only the original (it writes the Text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_SPA_SECOND_B_BUILD_FATE_NEWS_TEXT);
    let s = stage as u8 as i8;
    if !segunda_b_writes_own_text(s, outcome as u8, news) || text == 0 {
        return o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| spa_second_b_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
        || format!("spa_second_b_build_fate_news_text(club {}, outcome {:#x}, stage {}, news {})", club_id, outcome as u8, s, news));
}

/// The clubs of the Spanish Second Division B: 80 slots (the four groups of 20), filled by `grouped_league_club_array`
/// with the group stages 0..2, each copied for the league's own club count (only group 1 and the last group stage end
/// up in the list: the defect described there); out of memory: the box at line 0x4f2.
/// Original: `SpaSecondB::get_club_array` (0x8581a0, spa_second_b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn spa_second_b_get_club_array(this: usize, list: *mut usize, n: *mut i16) {
    grouped_league_club_array(League(this), list, n, 0x50, 3, true, SEGUNDA_B_SOURCE, 0x4f2);
}

/// Comparer for `spa_second_b_get_club_array` (`compare_grouped_league_club_array`).
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_get_club_array(this: usize, list: *mut usize, n: *mut i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(IDX_SPA_SECOND_B_GET_CLUB_ARRAY);
    compare_grouped_league_club_array("spa_second_b_get_club_array", League(this), list, n, 0x50, 3, true, || o(this, list, n));
}

/// The predicted positions of the Spanish Second Division B: each of the four groups (group 1 in the league's table, the
/// others in the group stages 0..2) sorted by strength, and the club in place k gets 4k + 1 whatever its group
/// (`grouped_league_predicted_positions`; a list of 4 × `league::CLUB_COUNT` bytes, boxes at lines 0x519 and 0x521).
/// Original: `SpaSecondB::set_predicted_positions` (0x8582f0, spa_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_second_b_set_predicted_positions(this: usize) {
    let l = League(this);
    let size = (l.club_count() as i32 * 4) as u32;
    grouped_league_predicted_positions(l, 3, size, SEGUNDA_B_SOURCE, (0x519, 0x521), |_, k| k.wrapping_mul(4).wrapping_add(1)); // rx09_5: 4k + 1
}

/// Comparer for `spa_second_b_set_predicted_positions` (`compare_grouped_predicted_positions`).
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPA_SECOND_B_SET_PREDICTED_POSITIONS);
    compare_grouped_predicted_positions("spa_second_b_set_predicted_positions", League(this), 3, || o(this), || spa_second_b_set_predicted_positions(this));
}

// ---------- rx09_6: the Spanish Second Division B progress codes (spa_second_b.cpp) ----------

/// The three progress codes (current, best, worst) of a Second Division B club at a stage, as the original turns them
/// into the season codes (byte arithmetic):
/// - the groups (stage below 3): best 1 to 4 becomes 1, worst 1 to 4 becomes 4, then each of the three × 4 − 3;
/// - the promotion play-off groups (3 to 6): each code 1 to 4 becomes 4v − 3 (1, 5, 9, 13), any other stays;
/// - the relegation play-off (7) and its final (8): the play-off code (1 the winner, 3 at 7 or 2 at 8 the loser) becomes
///   0x3b or 0x3f, but only in the best code: a best code is mapped, then a worst code or a current code of 1 or 3 (2)
///   overwrites the best code with its mapping, and the current and worst codes are kept as they came (defect of the
///   original: the register of the best code is reused for the other two);
/// - later stages: the codes as they came.
pub(crate) fn segunda_b_progress(stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    let scale = |v: u8| v.wrapping_mul(4).wrapping_sub(3);
    match stage {
        i8::MIN..=2 => {
            let b = if (1..=4).contains(&best) { 1 } else { best };
            let w = if (1..=4).contains(&worst) { 4 } else { worst };
            (scale(current), scale(b), scale(w))
        }
        3..=6 => {
            let group = |v: u8| if (1..=4).contains(&v) { scale(v) } else { v };
            (group(current), group(best), group(worst))
        }
        7 | 8 => {
            let lost = if stage == 7 { 3 } else { 2 };
            let code = |v: u8| if v == 1 { Some(0x3b) } else if v == lost { Some(0x3f) } else { None }; // rx09_6: the relegation play-off codes
            let mut b = code(best).unwrap_or(best);
            if let Some(c) = code(worst) {
                b = c;
            }
            if let Some(c) = code(current) {
                b = c;
            }
            (current, b, worst)
        }
        _ => (current, best, worst),
    }
}

/// Stores a Second Division B club's current, best and worst position codes in its statistics entry
/// (`exe::comp_stats_find_club_entry`, not checked), turned into the season codes by `segunda_b_progress`.
/// Original: `SpaSecondB::set_club_progress` (0x858460, spa_second_b.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn spa_second_b_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("rx09_6 segunda b progress");
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let (c, b, w) = segunda_b_progress(stage as u8 as i8, current as u8, best as u8, worst as u8);
    entry.set_best_position(b);
    entry.set_worst_position(w);
    entry.set_current_position(c);
}

/// Comparer for `spa_second_b_set_club_progress`: the three position bytes of the club's entry side by side.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_SPA_SECOND_B_SET_CLUB_PROGRESS);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst),
        || spa_second_b_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("spa_second_b_set_club_progress({:#x}, club {:#x}, stage {}, {}, {}, {}): original {:x?} rust {:x?}", this, club, stage as u8 as i8,
        current as u8, best as u8, worst as u8, mo, mr));
}

// ---------- rx09_7: the Swedish Second Division at career creation, its rules and its round tables (swe_second.cpp) ----------

/// The source file of the Swedish Second Division (swe_second.cpp), for the error boxes.
pub(crate) const SWEDISH_SECOND_SOURCE: usize = 0xa8b5b0;

/// The rules of the Swedish Second Division (six regional sections of 12): 2 legs, 3 points a win and 1 a draw, league
/// matches, national clubs; tie-breaks 1, 2 (goal difference, goals for; +0xc7 = 0 ends); none up directly, 1 to the
/// promotion play-off, 1 to the relegation play-off, 2 down; the First Division above, nothing below; the rule bits
/// 0x42 (a word). The substitutes, the substitutions, +0xab, +0xdb and +0x52 are not written.
pub(crate) unsafe fn swedish_second_rules() -> Vec<(usize, u8, u32)> {
    vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1),
        (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 0), (league::PROMOTED_COUNT, 1, 0),
        (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 2), // rx09_7: two go down
        (league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION) as u32), (league::LOWER_DIVISION_ID, 4, 0xffff_ffff),
        (league::LEAGUE_FLAGS, 2, 0x42)]
}

/// The Swedish Second Division: vtable 0x9702a0, the Swedish discipline rule 0x1c, seven stages after the North
/// section (the five other sections, the promotion play-off and the relegation play-off); the rules written inline
/// (no box; SweSecond::set_up_rules 0x892380 is not called), SweSecond::create_table (0x8921b0, line 0x50), sort
/// 0x57, statistics 0x5f, fixtures 0x68; SweSecond::create_section_stage (0x892670) for the five other sections and
/// SweSecond::set_predicted_positions (0x893470).
pub(crate) const SWEDISH_SECOND: GroupedLeagueCtor = GroupedLeagueCtor {
    shape: LeagueWithStages {
        division: LeagueDivision { vtabela: 0x9702a0, discipline: 0x1c, rules: 0x892380, table: 0x8921b0, file: SWEDISH_SECOND_SOURCE, rows: [0, 0x50, 0x68, 0x57, 0x5f], low: false },
        stages: 7, // rx09_7: the seven stages of the Swedish Second Division
        finish: None,
    },
    inline_rules: Some(swedish_second_rules),
    group_stage: 0x892670,
    groups: 5,
    predicted: 0x893470,
};

/// Builds the Swedish Second Division at game creation (`construct_grouped_league` with `SWEDISH_SECOND`): the North
/// section in the league's own table, the other five as stages 0 to 4, then the predicted positions.
/// Original: `SweSecond::SweSecond` (0x8916f0, swe_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_swe_second(this: usize, year: u32, record: usize) -> usize {
    construct_grouped_league(this, year, record, &SWEDISH_SECOND, "rx09_7 new swedish second division")
}

/// Comparer for `construct_swe_second` (`compare_league_with_stages_ctor` with the shape of `SWEDISH_SECOND`): only the
/// original runs (it allocates the league, the stages and the statistics).
pub(crate) unsafe extern "thiscall" fn compare_construct_swe_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SWE_SECOND);
    compare_league_with_stages_ctor("construct_swe_second", League(this), year, record, &SWEDISH_SECOND.shape, || o(this, year, record))
}

/// Rule: the Swedish Second Division: 3 points a win and 1 a draw; tie-breaks goal difference, goals for; each section
/// winner to the promotion play-off, each runner-up to the relegation play-off, the last two down; the First Division
/// above (`swedish_second_rules`, then the main round table into `league::ROUND_DATES`).
/// Original: `SweSecond::set_up_rules` (0x892380, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_set_up_rules(this: usize) -> u32 {
    covered!("rx09_7 swedish second division rules");
    eng_prm_apply_league_rules(League(this), &swedish_second_rules())
}

/// Comparer for `swe_second_set_up_rules` (the rehearsal of `eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_swe_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_SECOND_SET_UP_RULES);
    eng_prm_compare_league_rules("swe_second_set_up_rules", League(this), swedish_second_rules(), || o(this), || swe_second_set_up_rules(this))
}

/// The Swedish Second Division sections: 22 rounds on Sundays (weekday 6) from 6 April to 8 November, the summer break
/// from 7 June to 19 July and a Saturday round on 31 October; no TV slots.
pub(crate) const SWEDISH_SECOND_SECTION_ROUNDS: [SpaSecondScheduleRound; 22] = [
    (6, 3, 0, 6, 1, &[(-1, -1, -1)]), // 0: 6/abr (rx09_7: the first section round)
    (13, 3, 0, 6, 1, &[(-1, -1, -1)]), // 1: 13/abr
    (20, 3, 0, 6, 1, &[(-1, -1, -1)]), // 2: 20/abr
    (27, 3, 0, 6, 1, &[(-1, -1, -1)]), // 3: 27/abr
    (4, 4, 0, 6, 1, &[(-1, -1, -1)]), // 4: 4/mai
    (11, 4, 0, 6, 1, &[(-1, -1, -1)]), // 5: 11/mai
    (18, 4, 0, 6, 1, &[(-1, -1, -1)]), // 6: 18/mai
    (7, 5, 0, 6, 1, &[(-1, -1, -1)]), // 7: 7/jun
    (19, 6, 0, 6, 1, &[(-1, -1, -1)]), // 8: 19/jul
    (2, 7, 0, 6, 1, &[(-1, -1, -1)]), // 9: 2/ago
    (9, 7, 0, 6, 1, &[(-1, -1, -1)]), // 10: 9/ago
    (16, 7, 0, 6, 1, &[(-1, -1, -1)]), // 11: 16/ago
    (23, 7, 0, 6, 1, &[(-1, -1, -1)]), // 12: 23/ago
    (30, 7, 0, 6, 1, &[(-1, -1, -1)]), // 13: 30/ago
    (6, 8, 0, 6, 1, &[(-1, -1, -1)]), // 14: 6/set
    (13, 8, 0, 6, 1, &[(-1, -1, -1)]), // 15: 13/set
    (20, 8, 0, 6, 1, &[(-1, -1, -1)]), // 16: 20/set
    (4, 9, 0, 6, 1, &[(-1, -1, -1)]), // 17: 4/out
    (18, 9, 0, 6, 1, &[(-1, -1, -1)]), // 18: 18/out
    (25, 9, 0, 6, 1, &[(-1, -1, -1)]), // 19: 25/out
    (31, 9, 0, 5, 1, &[(-1, -1, -1)]), // 20: 31/out
    (8, 10, 0, 6, 1, &[(-1, -1, -1)]), // 21: 8/nov
];

/// The promotion play-off of the Swedish Second Division (stage 5): drawn on 9 November (weekday 0), three ties of two
/// legs 7 days apart from 15 November (weekday 6) between the six section winners, code 0xa0, category 5, two-leg
/// tie-break 3, no prizes.
pub(crate) const SWEDISH_SECOND_PROMOTION_PLAY_OFF: EngCcCupCupRound = cql10r((9, 10, 0, 0), (0xf, 10, 0, 6, 1, 0), 0xa0, (0, 0, 3), 5, (6, 3, 6, 0), (0, 2, 7), (0, 0, 0));

/// The relegation play-off of the Swedish Second Division (stage 6), two rounds: drawn on 9 November, six ties of two
/// legs from 15 November between twelve lower division clubs (code 0xa); then drawn on 23 November, six ties of two
/// legs from 29 November where the six section runners-up come in (code 0x14, tie-break 0x400); category 5, two-leg
/// tie-break 3, 7 days between the legs, no prizes.
pub(crate) const SWEDISH_SECOND_RELEGATION_PLAY_OFF: [EngCcCupCupRound; 2] = [
    cql10r((9, 10, 0, 0), (0xf, 10, 0, 6, 1, 0), 0xa, (0, 0, 3), 5, (0xc, 6, 0xc, 0), (0, 2, 7), (0, 0, 0)),
    cql10r((0x17, 10, 0, 0), (0x1d, 10, 0, 6, 1, 0), 0x14, (0, 0x400, 3), 5, (0xc, 6, 6, 0xc), (0, 2, 7), (0, 0, 0)), // rx09_7: the runners-up come in
];

/// The round tables of the Swedish Second Division by stage (the signed byte of the argument): stages up to 4 (the
/// North section at −1, the other five at 0 to 4) the 22 section rounds, extra = stage + 0x46d written only for stages
/// −1 to 4 (line 0xb9); stage 5 the promotion play-off, extra 0x438 (line 0x12b); stage 6 the relegation play-off,
/// extra 0x452 (line 0x153); later stages 0 with nothing written.
/// Original: `SweSecond::create_schedule` (0x891a60, swe_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn swe_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let l = League(this);
    let s = stage as u8 as i8;
    if s < 5 {
        let code = if s >= -1 { Some((s as i16).wrapping_add(0x46d)) } else { None };
        league_round_table(l, rounds, extra, league, &SWEDISH_SECOND_SECTION_ROUNDS, code, SWEDISH_SECOND_SOURCE, 0xb9, "rx09_7 swedish second section rounds")
    } else if s == 5 {
        cup_round_table(l, rounds, extra, league, &[SWEDISH_SECOND_PROMOTION_PLAY_OFF], 0x438, SWEDISH_SECOND_SOURCE, 0x12b, "rx09_7 swedish second promotion play-off round")
    } else if s == 6 {
        cup_round_table(l, rounds, extra, league, &SWEDISH_SECOND_RELEGATION_PLAY_OFF, 0x452, SWEDISH_SECOND_SOURCE, 0x153, "rx09_7 swedish second relegation play-off rounds")
    } else {
        0
    }
}

/// Comparer for `swe_second_create_schedule` (`compare_league_or_cup_schedule`; cup rounds at stages 5 and 6).
pub(crate) unsafe extern "thiscall" fn compare_swe_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SWE_SECOND_CREATE_SCHEDULE);
    let cup = matches!(stage as u8 as i8, 5 | 6);
    compare_league_or_cup_schedule("swe_second_create_schedule", stage, cup, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| swe_second_create_schedule(this, stage, r, e, l))
}

// ---------- rx09_8: the Swedish Second Division North table, turnover and next stage (swe_second.cpp) ----------

/// The ids of the six Swedish Second Division sections, in stage order: the North section (the league's own table),
/// then East Svealand, West Svealand, East Götaland, West Götaland and South Götaland (stages 0 to 4).
pub(crate) const SWEDISH_SECOND_SECTIONS: [usize; 6] = [KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_SVEALAND,
    KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_SVEALAND, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_GOTALAND, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_GOTALAND,
    KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_SOUTH_GOTALAND];

/// The clubs of a Swedish Second Division section, in database order: those with a division (`club::DIVISION`) that is
/// this league's record and a reserve division (`club::RESERVE_DIVISION`, here the section) whose id is the section's
/// (`section`, a key).
pub(crate) unsafe fn swedish_second_section_clubs(l: League, section: usize) -> Vec<Club> {
    domestic_clubs().into_iter().filter(|c| {
        let division = read_usize(c.addr() + club::DIVISION);
        let reserve = read_usize(c.addr() + club::RESERVE_DIVISION);
        division != 0 && reserve != 0 && read_usize(reserve) == read_usize(section) && division == l.competition().addr()
    }).collect()
}

/// Marks a B team in row k of a section's table: a club linked to a senior club (`exe::club_get_linked_b_team`(club,
/// &senior, 1) non-zero with the senior flag 0) gets the row status 0xfc (barred from promotion).
pub(crate) unsafe fn mark_b_team_row(t: League, k: usize) {
    let row = t.table() + k * league_table_row::SIZE;
    let mut senior = 0u32;
    if exe::club_get_linked_b_team(read_usize(row), &mut senior, 1) != 0 && senior == 0 {
        write_u8(row + league_table_row::STATUS, 0xfc); // rx09_8: a B team cannot go up
    }
}

/// Builds the table of the North section of the Swedish Second Division: `club_count` = 12, the table
/// (`exe::crt_malloc` of 0x2c4; out of memory: the box at line 0x198 and 0) and, for each club of
/// `swedish_second_section_clubs` (the North), a row (League::init_table_row `exe::league_init_table_row`(this, k, club,
/// 0, 0)) and the B team mark (`mark_b_team_row`). A 13th club shows the box at line 0x1a9, loses its division
/// (`club::DIVISION` = 0) and ends the scan. Returns 1 when the table is full, else 0.
/// Original: `SweSecond::create_table` (0x8921b0, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_create_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(0xc);
    let t = exe::crt_malloc(0x2c4);
    l.set_table(t);
    if t == 0 {
        show_error_at(SWEDISH_SECOND_SOURCE, 0x198);
        return 0;
    }
    covered!("rx09_8 swedish second north table");
    let mut k: u8 = 0;
    for c in swedish_second_section_clubs(l, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH) {
        if (k as i8 as i16) >= l.club_count() {
            show_error_at(SWEDISH_SECOND_SOURCE, 0x1a9);
            write_usize(c.addr() + club::DIVISION, 0);
            break;
        }
        exe::league_init_table_row(l, k as u32, c, 0, 0); // rx09_8: the row of the k-th North club
        mark_b_team_row(l, k as i8 as isize as usize);
        k = k.wrapping_add(1);
    }
    ((k as i8 as i16) == l.club_count()) as u32
}

/// Comparer for `swe_second_create_table`: only the original runs (it allocates the table); the club count, the result,
/// the clubs of the rows and the B team marks against the North clubs, planned beforehand.
pub(crate) unsafe extern "thiscall" fn compare_swe_second_create_table(this: usize) -> u32 {
    let l = League(this);
    let plan: Vec<usize> = swedish_second_section_clubs(l, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH).iter().take(12).map(|c| c.addr()).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_SECOND_CREATE_TABLE)(this);
    let found: Vec<usize> = if l.table() != 0 { (0..plan.len()).map(|k| aus_nsl_table_row_club(l, k)).collect() } else { Vec::new() };
    let same = r == (plan.len() == 12) as u32 && l.club_count() == 12 && found == plan;
    SCOREBOARD.record(same, || format!("swe_second_create_table({:#x}) original={} plan={:x?} table={:x?}", this, r, plan, found));
    r
}

/// Rule: the Swedish Second Division turnover: last season's stages destroyed, the next year, the rules, the North
/// table, the order and the fixtures, then the other five sections, the predicted positions and the officials
/// (`grouped_league_new_season` with SweSecond::create_table 0x8921b0 and SweSecond::create_section_stage 0x892670,
/// boxes at lines 0x20c, 0x213, 0x21a and 0x220).
/// Original: `SweSecond::start_new_season` (0x892410, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_start_new_season(this: usize) -> u32 {
    grouped_league_new_season(League(this), 0x8921b0, 0x892670, 5, SWEDISH_SECOND_SOURCE, [0x20c, 0x213, 0x21a, 0x220], "rx09_8 swedish second turnover")
}

/// Comparer for `swe_second_start_new_season`: only the original runs (it frees and rebuilds the league); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_swe_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("swe_second_start_new_season", League(this), || o(this))
}

/// Rule: the next stage of the Swedish Second Division: past the last stage, the error box; otherwise the current stage
/// goes up by one; on reaching stage 5 (the sections are over) the promotion play-off and the relegation play-off are
/// built.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box at line 0x23e and nothing
/// else. Otherwise the current stage + 1; at 5, SweSecond::create_promotion_play_off_stage (0x892a10) and
/// SweSecond::create_relegation_play_off_stage (0x892bb0) (thiscall, ret 0).
/// Original: `SweSecond::advance_stage` (0x8925d0, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(SWEDISH_SECOND_SOURCE, 0x23e);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("rx09_8 swedish second next stage");
    if stage == 5 {
        call_on_league(0x892a10, l);
        call_on_league(0x892bb0, l);
    }
}

/// Comparer for `swe_second_advance_stage` (`compare_advance_stage_building`: stage 5 builds).
pub(crate) unsafe extern "thiscall" fn compare_swe_second_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SWE_SECOND_ADVANCE_STAGE);
    compare_advance_stage_building("swe_second_advance_stage", League(this), |s| s == 5, || o(this), || swe_second_advance_stage(this))
}

// ---------- rx09_10: the Swedish Second Division staff records, fate news text and club list (swe_second.cpp) ----------

/// The section of a Swedish Second Division club by the id of its reserve division (`SWEDISH_SECOND_SECTIONS`): the
/// North section is the league's own table, the others the stages 0 to 4. None when the id is none of the six.
pub(crate) unsafe fn swedish_second_section_of(l: League, reserve: usize) -> Option<League> {
    SWEDISH_SECOND_SECTIONS.iter().position(|&s| read_usize(reserve) == read_usize(s)).map(|k| if k == 0 { l } else { League(read_usize(l.stages() + 4 * (k - 1))) })
}

/// The plan of `swe_second_update_staff_records_for_status` for a club with `status` (its byte) at `stage` (signed):
/// - the sections (below 5): status 1 adds the qualified record; the result is 0;
/// - the promotion play-off (5): no reserve division, nothing and 0; the section by `swedish_second_section_of` (none:
///   the box at line 0x38b and −1), the club's row in it (League::find_club_row `exe::league_find_club_row`(section,
///   club, 0); none: −1); status 1 writes 0 in the row with the promotion record, any other 0xfe; the result 0;
/// - the relegation play-off (6): no reserve division or a division other than this league's record: −1; the section
///   (none: the box at line 0x3bb and −1), the row (none: −1); status 1 writes 0xfe, any other 3 with the relegation
///   record; the result 0;
/// - later stages: nothing and 0.
pub(crate) unsafe fn swedish_second_record_plan(l: League, club: Club, status: u8, stage: i8) -> CategoryBRecordPlan {
    let mut plan = CategoryBRecordPlan { error_line: None, row: None, record: None, result: 0 };
    if stage < 5 {
        if status == 1 {
            plan.record = Some(CategoryBRecord::Qualified);
        }
        return plan;
    }
    if stage > 6 {
        return plan;
    }
    let reserve = club.reserve_division().addr();
    if reserve == 0 {
        if stage == 6 {
            plan.result = u32::MAX;
        }
        return plan;
    }
    if stage == 6 && read_usize(club.addr() + club::DIVISION) != l.competition().addr() {
        plan.result = u32::MAX;
        return plan;
    }
    let Some(section) = swedish_second_section_of(l, reserve) else {
        plan.error_line = Some(if stage == 5 { 0x38b } else { 0x3bb });
        plan.result = u32::MAX;
        return plan;
    };
    let row = exe::league_find_club_row(section, club, 0);
    if row == 0 {
        plan.result = u32::MAX;
        return plan;
    }
    match (stage, status) {
        (5, 1) => {
            plan.row = Some((row, 0)); // rx09_10: the promotion play-off winner's row
            plan.record = Some(CategoryBRecord::Promotion);
        }
        (5, _) => plan.row = Some((row, 0xfe)),
        (_, 1) => plan.row = Some((row, 0xfe)),
        _ => {
            plan.row = Some((row, 3));
            plan.record = Some(CategoryBRecord::Relegation);
        }
    }
    plan
}

/// Updates the staff records by a club's status in the Swedish Second Division and marks its row in its section
/// (`swedish_second_record_plan`: the error box, the row status, then the record, with this league's competition).
/// The last three arguments are not read.
/// Original: `SweSecond::update_staff_records_for_status` (0x892ee0, swe_second.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn swe_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    let l = League(this);
    let plan = swedish_second_record_plan(l, Club(club), status as u8, stage as u8 as i8);
    if let Some(line) = plan.error_line {
        show_error_at(SWEDISH_SECOND_SOURCE, line);
    }
    if let Some((row, s)) = plan.row {
        covered!("rx09_10 swedish second row status");
        write_u8(row + league_table_row::STATUS, s);
    }
    let records = read_usize(G_STAFF_RECORDS);
    let comp = l.competition().addr();
    match plan.record {
        Some(CategoryBRecord::Qualified) => exe::staff_records_add_qualified_for_round_record(records, club, comp, 0x458, 0, 0x28),
        Some(CategoryBRecord::Promotion) => exe::staff_records_add_promotion_record(records, club, comp, 0x64),
        Some(CategoryBRecord::Relegation) => exe::staff_records_add_relegation_record(records, club, comp),
        None => {}
    }
    plan.result
}

/// Comparer for `swe_second_update_staff_records_for_status`: only the original runs (it adds records); its result and
/// the status of the planned row against the plan.
pub(crate) unsafe extern "thiscall" fn compare_swe_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let plan = swedish_second_record_plan(League(this), Club(club), status as u8, stage as u8 as i8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(IDX_SWE_SECOND_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, status, stage, a4, a5, a6);
    let row_ok = plan.row.map_or(true, |(row, s)| read_u8(row + league_table_row::STATUS) == s);
    SCOREBOARD.record(r == plan.result && row_ok, || format!("swe_second_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {}) original={:#x} plan={:?}", this, club,
        status as u8, stage as u8 as i8, r, plan));
    r
}

/// True when `swe_second_build_fate_news_text` writes the text itself (not the common text nor the cup news).
pub(crate) fn swedish_second_writes_own_text(stage: i8, outcome: u8, news: u32) -> bool {
    match stage {
        5 => news == 0,
        6 => news == 0 && matches!(outcome, 1 | 2 | 0xff),
        _ => false,
    }
}

/// The fate news text of a Swedish Second Division club (thiscall, ret 0x28; same arguments as
/// Comp::build_fate_news_text):
/// - the sections (stage below 5): the common text (`exe::comp_build_fate_news_text`, a7 = 0, the rest as they came);
/// - the promotion play-off (5): with a news item the cup tie news (`exe::comp_util_write_cup_tie_result_news`(club, a5,
///   a8, outcome, a6, text)); without one, outcome 1 gives 0x989c18 "{club} win promotion" and any other 0x987528
///   "{club} miss out on promotion" (`play_off_club_text`);
/// - the relegation play-off (6): with a news item the cup tie news; without one, outcome 1 gives 0x987594 "{club}
///   avoid relegation", outcomes 2 and 0xff 0x987564 "{club} relegated", any other the cup win or defeat news
///   (`exe::comp_util_write_cup_win_or_defeat_news`(club, a5, outcome, text));
/// - later stages: nothing.
/// Original: `SweSecond::build_fate_news_text` (0x893160, swe_second.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn swe_second_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let s = stage as u8 as i8;
    if s < 5 {
        return exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, news, text);
    }
    if s != 5 && s != 6 {
        return;
    }
    if news != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text);
    }
    if s == 5 {
        covered!("rx09_10 swedish second promotion text");
        return play_off_club_text(club_id, if outcome as u8 == 1 { 0x989c18 } else { 0x987528 }, text); // rx09_10: win / miss out on promotion
    }
    match outcome as u8 {
        1 => play_off_club_text(club_id, 0x987594, text), // "{club} avoid relegation"
        2 | 0xff => {
            covered!("rx09_10 swedish second relegated text");
            play_off_club_text(club_id, 0x987564, text); // "{club} relegated"
        }
        _ => exe::comp_util_write_cup_win_or_defeat_news(club_id, a5, outcome, text),
    }
}

/// Comparer for `swe_second_build_fate_news_text`: the texts the routine writes itself side by side with
/// `compare_text_writer`; the common text and the cup news only the original (it writes the Text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_swe_second_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_SWE_SECOND_BUILD_FATE_NEWS_TEXT);
    let s = stage as u8 as i8;
    if !swedish_second_writes_own_text(s, outcome as u8, news) || text == 0 {
        return o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| swe_second_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
        || format!("swe_second_build_fate_news_text(club {}, outcome {:#x}, stage {}, news {})", club_id, outcome as u8, s, news));
}

/// The clubs of the Swedish Second Division: 72 slots (the six sections of 12), filled by `grouped_league_club_array`
/// with the section stages 0..4, each copied for its own club count (only the North and the last section end up in the
/// list: the defect described there); out of memory: the box at line 0x464.
/// Original: `SweSecond::get_club_array` (0x893320, swe_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn swe_second_get_club_array(this: usize, list: *mut usize, n: *mut i16) {
    grouped_league_club_array(League(this), list, n, 0x48, 5, false, SWEDISH_SECOND_SOURCE, 0x464);
}

/// Comparer for `swe_second_get_club_array` (`compare_grouped_league_club_array`).
pub(crate) unsafe extern "thiscall" fn compare_swe_second_get_club_array(this: usize, list: *mut usize, n: *mut i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(IDX_SWE_SECOND_GET_CLUB_ARRAY);
    compare_grouped_league_club_array("swe_second_get_club_array", League(this), list, n, 0x48, 5, false, || o(this, list, n));
}

// ---------- rx09_11: the Swedish Second Division predicted positions and progress codes (swe_second.cpp) ----------

/// The predicted positions of the Swedish Second Division: each of the six sections (the North in the league's table,
/// the others in the section stages 0..4) sorted by strength, and the club in place k gets 6k + 1 whatever its section
/// (`grouped_league_predicted_positions`; a list of 0x30 bytes, boxes at lines 0x48b and 0x493).
/// Original: `SweSecond::set_predicted_positions` (0x893470, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_set_predicted_positions(this: usize) {
    grouped_league_predicted_positions(League(this), 5, 0x30, SWEDISH_SECOND_SOURCE, (0x48b, 0x493), |_, k| k.wrapping_mul(6).wrapping_add(1)); // rx09_11: 6k + 1
}

/// Comparer for `swe_second_set_predicted_positions` (`compare_grouped_predicted_positions`).
pub(crate) unsafe extern "thiscall" fn compare_swe_second_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SWE_SECOND_SET_PREDICTED_POSITIONS);
    compare_grouped_predicted_positions("swe_second_set_predicted_positions", League(this), 5, || o(this), || swe_second_set_predicted_positions(this));
}

/// The three progress codes (current, best, worst) of a Swedish Second Division club at a stage (byte arithmetic):
/// - the sections (stage below 5): each code × 5 − 4;
/// - the relegation play-off (6): each code 1, 7 or 13 becomes 0x37, 0x3d or 0x43, any other stays;
/// - the promotion play-off (5) and later stages: the codes as they came.
pub(crate) fn swedish_second_progress(stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    if stage < 5 {
        let scale = |v: u8| v.wrapping_mul(5).wrapping_sub(4);
        return (scale(current), scale(best), scale(worst));
    }
    if stage != 6 {
        return (current, best, worst);
    }
    let play_off = |v: u8| match v {
        1 => 0x37,
        7 => 0x3d, // rx09_11: the relegation play-off codes
        13 => 0x43,
        v => v,
    };
    (play_off(current), play_off(best), play_off(worst))
}

/// Stores a Swedish Second Division club's current, best and worst position codes in its statistics entry
/// (`exe::comp_stats_find_club_entry`, not checked), turned into the season codes by `swedish_second_progress`.
/// Original: `SweSecond::set_club_progress` (0x8935e0, swe_second.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn swe_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("rx09_11 swedish second progress");
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let (c, b, w) = swedish_second_progress(stage as u8 as i8, current as u8, best as u8, worst as u8);
    entry.set_best_position(b);
    entry.set_current_position(c);
    entry.set_worst_position(w);
}

/// Comparer for `swe_second_set_club_progress`: the three position bytes of the club's entry side by side.
pub(crate) unsafe extern "thiscall" fn compare_swe_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_SWE_SECOND_SET_CLUB_PROGRESS);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst),
        || swe_second_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("swe_second_set_club_progress({:#x}, club {:#x}, stage {}, {}, {}, {}): original {:x?} rust {:x?}", this, club, stage as u8 as i8,
        current as u8, best as u8, worst as u8, mo, mr));
}

// ---------- rx13_16: the Spanish Segunda B's exchange with the lower division and its four groups; the Swedish Second Division's moves (spa_first.cpp, swe_prm.cpp) ----------

/// The `qsort` comparers of two club pointers by their stadium city's latitude and longitude.
pub(crate) const CLUB_LATITUDE_COMPARE: usize = 0x53fd40;
pub(crate) const CLUB_LONGITUDE_COMPARE: usize = 0x53fe50;
/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\leagues\spa_first.cpp" (the error boxes).
const SPA_FIRST_CPP: usize = 0xa82844;

/// The club table's clubs (the national teams at its end, two per nation, left out), as the Spanish turnover reads it.
unsafe fn club_sides_count() -> i32 {
    read_i32(G_CLUB_COUNT) - 2 * read_i32(G_NATION_COUNT)
}

/// Group `k` of a league split in groups: the league itself for −1, its stage `k` otherwise (`league::STAGES`).
unsafe fn league_group(main: League, k: i8) -> League {
    if k < 0 { main } else { League(read_usize(main.stages() + k as usize * 4)) }
}

/// rule: at the Spanish turnover with a played Segunda B, the clubs relegated from its four groups go down to the
/// generic lower division, and as many Spanish clubs of the lower division (one more when a group had 19 clubs), the
/// strongest first, go up into the Segunda B; a reserve side whose first team is already in the Segunda B is skipped
///
/// Up to 200 candidates in club table order (`club_sides_count`): division [`KEY_CLUB_COMP_A_LOWER_DIVISION`] and
/// nation [`KEY_NATION_SPAIN`]. For the Segunda B ([`KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B`]) and its stages 0 to 2
/// (`league_group`): a group of 19 clubs sets the extra place; every row with `league_table_row::STATUS` 3 sends its club
/// down (`exe::league_relegate_club_to_division`(this, club, the lower division's record, 1)) and clears its
/// `club::RESERVE_DIVISION` [the dword at +0x60: here the Segunda B group]. The places are the relegated plus the extra
/// one. Fewer candidates: the box of spa_first.cpp line 0x3b7 (the relegations stay). Otherwise the candidates are
/// sorted (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, places)) and, in that order while places are left,
/// each goes up (`exe::league_move_club_to_division`(this, club, the Segunda B record, 1)), unless it is a B team
/// (`exe::club_get_linked_b_team`(club, &is_a, 1) gives a partner and is_a is 0) whose partner's division is the Segunda
/// B.
/// Original: `spa_first_exchange_second_division_b_clubs` (0x850300, spa_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_first_exchange_second_division_b_clubs(this: usize) {
    let this = League(this);
    let mut list = [0usize; 200];
    let mut n = 0usize;
    for i in 0..club_sides_count().max(0) as usize {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        let nation = read_usize(c + club::NATION);
        if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_A_LOWER_DIVISION) && nation != 0 && read_u32(nation) == read_u32(KEY_NATION_SPAIN) {
            list[n] = c;
            n += 1;
            if n == 200 {
                break;
            }
        }
    }
    let second_b = League(comp_by_global(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B));
    let mut places = 0i16;
    let mut extra = false;
    for k in -1i8..3 {
        let g = league_group(second_b, k);
        if g.club_count() == 0x13 {
            extra = true; // rx13_16: a group of 19
        }
        for r in 0..g.club_count().max(0) as usize {
            let row = g.table() + r * 0x3b;
            if read_u8(row + league_table_row::STATUS) == 3 {
                exe::league_relegate_club_to_division(this, read_usize(row), comp_record_of_key(KEY_CLUB_COMP_A_LOWER_DIVISION), 1);
                places += 1;
                write_u32(read_usize(row) + club::RESERVE_DIVISION, 0);
            }
        }
    }
    if extra {
        places += 1;
    }
    if (n as i16) < places {
        show_error_at(SPA_FIRST_CPP, 0x3b7);
        return;
    }
    covered!("rx13_16 segunda b exchange");
    exe::comp_util_sort_and_shuffle_top_clubs(list.as_mut_ptr() as usize, n as u32, places as u16 as u32);
    for &c in &list[..n] {
        if places == 0 {
            break;
        }
        let mut is_a = 0u32;
        let partner = exe::club_get_linked_b_team(c, &mut is_a, 1);
        if partner != 0 && is_a == 0 {
            let d = read_usize(partner + club::DIVISION);
            if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B) {
                covered!("rx13_16 b team kept down");
                continue;
            }
        }
        exe::league_move_club_to_division(this, c, comp_record_of_key(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B), 1);
        places -= 1;
    }
}

/// rule: the 80 Segunda B clubs are dealt into its four groups of 20 by geography: sorted from south to north by their
/// stadium city, the 20 southernmost go to group 4; the other 60, sorted from west to east, 20 each to groups 1, 2 and
/// 3; with a number other than 80 nothing is dealt (an error box)
///
/// In club table order (`club_sides_count`, stopping at 80), the clubs whose division is the Segunda B. Not 80: the
/// box of spa_first.cpp line 0x3f7. Otherwise `exe::crt_qsort`(list, 80, 4, `CLUB_LATITUDE_COMPARE`), then the last 60
/// with `CLUB_LONGITUDE_COMPARE`; entry k's `club::RESERVE_DIVISION` [the dword at +0x60: the Segunda B group] becomes
/// the record of group 4 (k < 20), 1, 2 or 3.
/// Original: `spa_first_assign_second_division_b_groups` (0x850530, spa_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_first_assign_second_division_b_groups(_this: usize) {
    let groups = [KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B4, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1,
        KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B2, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B3].map(|k| comp_record_of_key(k));
    let mut list = [0usize; 80];
    let mut n = 0usize;
    for i in 0..club_sides_count().max(0) as usize {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B) {
            list[n] = c;
            n += 1;
            if n == 80 {
                break;
            }
        }
    }
    if n != 80 {
        show_error_at(SPA_FIRST_CPP, 0x3f7);
        return;
    }
    covered!("rx13_16 segunda b groups");
    exe::crt_qsort(list.as_mut_ptr() as usize, 80, 4, CLUB_LATITUDE_COMPARE);
    exe::crt_qsort(list[20..].as_mut_ptr() as usize, 60, 4, CLUB_LONGITUDE_COMPARE); // rx13_16: the northern 60 by longitude
    for (k, &c) in list.iter().enumerate() {
        write_usize(c + club::RESERVE_DIVISION, groups[k / 20]);
    }
}

/// rule: at the Swedish turnover with a playable Second Division, the winners of its promotion play-off go up into it,
/// the clubs relegated from its four sections go down to the lower division, and the twelve clubs it kept for promotion
/// go up
///
/// The Second Division is the object of [`KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION`]. Its stage 6 (the play-off, a cup):
/// every entrant of `cup::TEAM_TABLE` (6-byte entries, `cup::TEAM_COUNT` of them) with the outcome byte (+5) 1 whose
/// club's division is not the Second Division's record goes up (`exe::league_move_club_to_division`(this, club, record,
/// 1)). Then the Second Division and its stages 0 to 3 (`league_group`): every row with `league_table_row::STATUS` 3
/// goes down (`exe::league_relegate_club_to_division`(this, club, the [`KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION`] record,
/// 1)). Then the twelve club ids at +0xee (the clubs promoted into the Second Division) each go up to its record.
/// Original: `swe_prm_apply_second_division_moves` (0x890b40, swe_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_prm_apply_second_division_moves(this: usize) {
    let this = League(this);
    let second = League(comp_by_global(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION));
    let record = second.competition().addr();
    let play_off = read_usize(second.stages() + 0x18); // stage 6: the promotion play-off
    for k in 0..read_i16(play_off + cup::TEAM_COUNT).max(0) as usize {
        let e = read_usize(play_off + cup::TEAM_TABLE) + k * 6;
        let c = read_usize(e);
        if read_u8(e + 5) == 1 && read_usize(c + club::DIVISION) != record {
            covered!("rx13_16 swedish play-off winner up");
            exe::league_move_club_to_division(this, c, record, 1);
        }
    }
    for k in -1i8..4 {
        let g = league_group(second, k);
        for r in 0..g.club_count().max(0) as usize {
            let row = g.table() + r * 0x3b;
            if read_u8(row + league_table_row::STATUS) == 3 {
                exe::league_relegate_club_to_division(this, read_usize(row), comp_record_of_key(KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION), 1);
            }
        }
    }
    covered!("rx13_16 swedish second division moves");
    for k in 0..0xc {
        let id = read_u32(second.addr() + 0xee + k * 4); // the twelve promoted club ids
        exe::league_move_club_to_division(this, club_at(id as isize), comp_record_of_key(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION), 1);
    }
}

/// Comparer for `spa_first_exchange_second_division_b_clubs`: only the original runs (it draws in the sort and moves
/// clubs); afterwards no relegated row of the four groups may still hold a club in the Segunda B.
pub(crate) unsafe extern "thiscall" fn compare_spa_first_exchange_second_division_b_clubs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPA_FIRST_EXCHANGE_SECOND_DIVISION_B_CLUBS)(this);
    let second_b = League(comp_by_global(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B));
    let record = comp_record_of_key(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B);
    let wrong: Vec<usize> = (-1i8..3).flat_map(|k| {
        let g = league_group(second_b, k);
        (0..g.club_count().max(0) as usize).map(move |r| g.table() + r * 0x3b)
    }).filter(|&row| read_u8(row + league_table_row::STATUS) == 3 && read_usize(read_usize(row) + club::DIVISION) == record)
        .map(|row| read_usize(row)).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("spa_first_exchange_second_division_b_clubs({:#x}) relegated clubs still in the Segunda B: {:x?}", this, wrong));
}

/// Comparer for `spa_first_assign_second_division_b_groups`: side by side over the +0x60 dword of the 80 clubs (the
/// copy's sorts on a copy of the list give the same order).
pub(crate) unsafe extern "thiscall" fn compare_spa_first_assign_second_division_b_groups(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPA_FIRST_ASSIGN_SECOND_DIVISION_B_GROUPS);
    let regions: Vec<(usize, usize)> = (0..club_sides_count().max(0) as usize)
        .map(|i| read_usize(G_CLUB_TABLE) + i * club::SIZE)
        .filter(|&c| { let d = read_usize(c + club::DIVISION); d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B) })
        .take(80)
        .map(|c| (c + club::RESERVE_DIVISION, 4))
        .collect();
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || spa_first_assign_second_division_b_groups(this));
    SCOREBOARD.record(equal, || format!("spa_first_assign_second_division_b_groups {:?}", first_differences(&mo, &mr)));
}

/// Comparer for `swe_prm_apply_second_division_moves`: only the original runs (it moves clubs); afterwards the twelve
/// stored clubs must be in the Second Division.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_apply_second_division_moves(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SWE_PRM_APPLY_SECOND_DIVISION_MOVES)(this);
    let second = comp_by_global(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION);
    let record = comp_record_of_key(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION);
    let wrong: Vec<u32> = (0..0xc).map(|k| read_u32(second + 0xee + k * 4))
        .filter(|&id| read_usize(club_at(id as isize).addr() + club::DIVISION) != record).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("swe_prm_apply_second_division_moves({:#x}) stored clubs outside the Second Division: {:?}", this, wrong));
}
