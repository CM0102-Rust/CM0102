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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    let cup = matches!(stage as u8 as i8, 7 | 8);
    compare_league_or_cup_schedule("spa_second_b_create_schedule", stage, cup, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| spa_second_b_create_schedule(this, stage, r, e, l))
}
