use super::*;

/// Champion of a half: club in row 0 of stage k's table.
pub(crate) unsafe fn half_champion(this: League, k: usize) -> usize {
    read_usize(read_usize(read_usize(this.stages() + 4 * k) + league::TABLE))
}

/// The Minas Gerais state league has two halves (stages 0 and 1, each a round-robin league) and, if needed, a final
/// (stage 2): the next stage moves the current stage on and, at the end of the second half, builds the final; past the
/// last stage it is an error (the box)
/// Original: `BraRegMinasGerais::advance_stage` (0x434ae0, bra_reg_minas_gerais.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_advance_stage(this: usize) {
    let this = League(this);
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x98aed0, 0x202);
    }
    let stage = stage + 1;
    this.set_current_stage((stage) as usize);
    if stage == 2 {
        exe::braregminasgerais_create_final_stage(this); // cqo7: the final after the second stage
    }
}

/// The Minas Gerais final: the winners of the first and second halves meet over two games [inference: home and away],
/// the first-half winner as seed 0; if the same club wins both halves, it is champion without a final.
///
/// `BraRegMinasGerais::create_final_stage()`, called from the next stage when the second half ends. The two half
/// winners (the club in the first row of each League_Stage's table, `half_champion`) go into a list of 2 (malloc
/// `exe::crt_malloc` of 8; out of memory: the error box at line 0x226 and return).
///
/// - The same club in both halves: it is champion without a final. `set_current_stage(1)`, `set_stage_count(2)` (the
/// season ends with the two stages), status 5 (`league_table_row::STATUS`, `table_status::CHAMPIONS`) in the first row
/// of each half's table and in its row of the league table (`table()`, the `club_count()` rows). The list is not freed
/// (the original leaks it; so does the copy).
///
/// - Different winners: the final, over two games [inference: home and away]. Virtual `comp_vtable::CREATE_SCHEDULE`
/// (create_round_table: thiscall(this, stage 2, &rounds, &extra, 0), ret 0x10) builds the round table; the new stage
/// (operator new `exe::crt_operator_new` of 0xb2; Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c:
/// the league, 2, the list, the rounds, `competition()`, the table, the year `year()`, 2, 1, the extra, 0x14, 0, 1, 0
/// and the seeds {0, 1}: the first-half winner is seed 0) goes into the third stage slot (`stages()` + 8; no stage: the
/// error box at line 0x250 and return, without freeing). With it: Cup::set_up_round (`exe::cup_set_up_round`,
/// thiscall(stage, 0), ret 4), the table and the list freed (free `exe::crt_free`, in that order) and
/// `set_current_stage(2)`. The original's SEH frame covers only the constructor (no destructor).
/// Original: `BraRegMinasGerais::create_final_stage` (0x434b70, bra_reg_minas_gerais.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_create_final_stage(this: usize) {
    let this = League(this);
    let seeds: [u8; 4] = [0, 1, 0, 0];
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(0x98aed0, 0x226);
    }
    let a = half_champion(this, 0);
    write_usize(list, a);
    let b = half_champion(this, 1);
    write_usize(list + 4, b);
    if a == b {
        covered!("cqo_7 same champion");
        this.set_current_stage((1) as usize);
        this.set_stage_count((2) as usize);
        for k in 0..2 {
            write_u8(read_usize(read_usize(this.stages() + 4 * k) + league::TABLE) + league_table_row::STATUS, 5); // cqo7: champion of both halves
        }
        let mut i: i16 = 0;
        while i < this.club_count() {
            let row = LeagueTableRow(this.table() + i as usize * league_table_row::SIZE);
            if read_usize(row.addr()) == read_usize(list) {
                row.set_status(table_status::CHAMPIONS);
            }
            i += 1;
        }
        // simplification: the 8-byte list is not freed, as in the original.
        return;
    }
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 2, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year in dwords with garbage in the high 16 bits (the outputs of the
    // virtual are words on the stack; the year goes in dx); the constructor only reads the words.
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 1, 0, seeds.as_ptr())
    } else {
        0
    };
    write_usize(this.stages() + 8, stage);
    let stage = read_usize(this.stages() + 8);
    if stage == 0 {
        return show_error_at(0x98aed0, 0x250);
    }
    covered!("cqo_7 final");
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(t);
    exe::crt_free(list);
    this.set_current_stage((2) as usize);
}

/// The end of the Minas Gerais season: with a final, the honours come from the final; without one, from the league, and
/// the champion gets 350,000 and the runner-up 100,000 in prize money. Then, once the twelve state leagues are over,
/// the champions go into the Champions Cup.
///
/// `BraRegMinasGerais::record_season_honours()` (slot 13).
///
/// - With a final (`current_stage()` = 2): the history of the final (slot 13, `comp_vtable::RECORD_SEASON_HONOURS`, of
/// stage `stages()` + 8) and then, as a tail jump, `exe::bra_nat_first_fill_champions_cup_entrants` (cdecl with no
/// arguments: when the twelve state leagues are over, it puts the champions into the Champions Cup); returns its eax.
///
/// - Without a final (the same champion in both halves): the league history (League::record_season_honours
/// `FN_LEAGUE_RECORD_SEASON_HONOURS`, thiscall, ret 0); the prizes, 350,000 to the champion (first row of table
/// `table()`) and 100,000 to the runner-up (second row), each one to the balance (`exe::financetable_get_club_finance`
/// on [`G_FINANCE_TABLE`] -> the club's finances, thiscall, ret 4; `exe::finance_add_prize_money`
/// Finance::add_prize_money, thiscall, ret 4) and in the prize money news (`exe::comp_send_prize_money_news`
/// Comp::send_prize_money_news: thiscall(this, club, 64-bit amount, position 1 or 2, round 0, match 0, type −2), ret
/// 0x1c); then `exe::bra_nat_first_fill_champions_cup_entrants`, as above.
/// Original: `BraRegMinasGerais::record_season_honours` (0x434ed0, bra_reg_minas_gerais.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_record_season_honours(this: usize) -> u32 {
    let this = League(this);
    if this.current_stage() == 2 {
        let f = read_usize(this.stages() + 8);
        let history: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(f) + comp_vtable::RECORD_SEASON_HONOURS));
        history(f);
        covered!("cqo_7 final history");
        return exe::bra_nat_first_fill_champions_cup_entrants();
    }
    let history: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_RECORD_SEASON_HONOURS);
    history(this.addr());
    for (linha, valor, posicao) in [(0usize, 350000i32, 1i32), (0x3b, 100000, 2)] {
        let c = read_usize(this.table() + linha);
        exe::finance_add_prize_money(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c), valor);
        exe::comp_send_prize_money_news(this, read_usize(this.table() + linha), valor, 0, posicao, 0, 0, -2);
    }
    covered!("cqo_7 prizes");
    exe::bra_nat_first_fill_champions_cup_entrants()
}

/// Comp_Stats::find_club_entry(club) on [this+8] (original routine).
pub(crate) unsafe fn bra_reg_minas_gerais_comp_stats_club_entry(this: usize, club: usize) -> usize {
    exe::comp_stats_find_club_entry(read_u32(this + 8) as usize, club)
}

/// Writes the club's current, lowest and highest progress codes (+0x73, +0x74, +0x75).
pub(crate) unsafe fn write_progress_codes(e: usize, current: u8, minimum: u8, maxima: u8) {
    let e = CompStatsClub(e);
    e.set_current_position(current);
    e.set_best_position(minimum);
    e.set_worst_position(maxima);
}

/// Minas Gerais, Parana and Rio: writes the club progress codes (current, lowest and highest position); in the round
/// robin, the lowest under 3 becomes 1 and the highest under 3 becomes 2 [inference: the two halves have tables of
/// their own and the overall one gives no title]
/// Original: `BraRegMinasGerais::set_club_progress` (0x4350d0, bra_reg_minas_gerais.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_set_club_progress(this: usize, club: usize, stage: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, club);
    let (a, mut b, c) = (a as u8, b as u8, c as u8);
    if stage as u8 == 0xff {
        if b < 3 {
            b = 1;
        }
        if c < 3 {
            covered!("cqk_8 mineiro third below 3");
            write_progress_codes(e, a, b, 2); // cqk8: the third Mineiro code
            return e;
        }
    }
    covered!("cqk_8 mineiro as it comes");
    write_progress_codes(e, a, b, c); // cqk8: the Mineiro codes as they come
    e
}

/// MLS regular-season progress code: 1..3 -> ate_3, 4 -> 10, others unchanged.
pub(crate) fn mls_progress_code(x: u8, ate_3: u8) -> u8 {
    match x {
        1..=3 => ate_3,
        4 => 10,
        _ => x,
    }
}

/// Serie C group progress code: 1,2 -> um_dois, 3..7 -> 0x11..0x31 in steps of 8, others unchanged.
pub(crate) fn serie_c_progress_code(x: u8, one_two: u8) -> u8 {
    match x {
        1 | 2 => one_two,
        3 => 0x11,
        4 => 0x19,
        5 => 0x21,
        6 => 0x29,
        7 => 0x31,
        _ => x,
    }
}

/// Minas Gerais, Parana and Rio: the round robin as in the Centre-West; the two halves (stages 0 and 1) record nothing
/// and return -1; the final knockout (stage 2, a single round: weight 40 for the loser) gives the champion (fate 5) and
/// the runner-up (fate -2)
/// Original: `BraRegMinasGerais::update_staff_records_for_status` (0x434d90, bra_reg_minas_gerais.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    mineiro_end_of_stage(this, club, status, stage, game, true).0
}

/// The Minas Gerais state league at career creation: the year and the database competition, disciplinary rule 7 and,
/// with no saved state, importance 15, two halves and the final (three stages besides the league), the rules, the table
/// with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegMinasGerais::BraRegMinasGerais` (0x433d50, bra_reg_minas_gerais.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_minas_gerais(this: usize, year: u32, record: usize) -> usize {
    bra_reg_goias_construct_state_league(League(this), year, record, &MINEIRO_STATE_LEAGUE, 7, true, "cqt_2 mineiro novo") // cqt2: the discipline and expected positions of the Mineiro
}

/// the Minas Gerais league has 12 clubs: those with it as their reserve division in the database (the field that holds
/// the state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegMinasGerais::create_table` (0x434810, bra_reg_minas_gerais.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_create_table(this: usize) -> u32 {
    fill_state_league_table(League(this), 0xc, 0x98aed0, 0x169, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, 0, "cql_9 mineiro cheio")
}

/// One half of the Minas Gerais state league: when the first game of a half is processed, it builds that half's
/// round-robin stage (a league with all the clubs of the table, the same points for a win and a draw and the same
/// tie-breakers as the league, and one top place, the half winner's); the first half goes to stage 0 and the second
/// (from round 12, by the match +0x34) to stage 1; each half winner (the top of the stage table) goes to the final
/// Original: `BraRegMinasGerais::create_half_season_stage` (0x4351b0, bra_reg_minas_gerais.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_create_half_season_stage(this: usize, k: u32) {
    create_half_season_stage(League(this), k, 0x98aed0, 0x3d5, 0xb, 0xc2, "cqr_1 turno do mineiro") // cqr1: the points per win in the Mineiro
}

/// The Minas Gerais state league turnover: archives the season in the statistics, clears the table, the schedule and
/// the stages (the two halves and the final) and prepares three new stage slots; moves to the next year and rebuilds
/// the rules, the table with the twelve state clubs, the fixtures and the order; then the predicted positions and the
/// referee renewal. The halves are only built at each one's first game (cqr_1)
/// Original: `BraRegMinasGerais::start_new_season` (0x434910, bra_reg_minas_gerais.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_start_new_season(this: usize) -> u32 {
    start_new_season_two_halves(League(this), 0x434810, 0x98aed0, 1, "cqr_2 virada do mineiro") // cqr2: the next year of the Mineiro
}

/// The predicted positions of the Minas Gerais state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, and the others their place in the order (`state_league_predicted_positions`
/// with `MINAS_GERAIS_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x33f (no statistics) and 0x347 (no memory).
/// Original: `BraRegMinasGerais::set_predicted_positions` (0x434f80, bra_reg_minas_gerais.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &MINAS_GERAIS_PREDICTED)
}

/// Comparer for `bra_reg_minas_gerais_set_predicted_positions` (`compare_state_league_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_minas_gerais_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_MINAS_GERAIS_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_minas_gerais_set_predicted_positions", this, &MINAS_GERAIS_PREDICTED, || o(this));
}

/// The schedule of the Minas Gerais state league: stage −1, the round-robin league stage: 22 rounds from 11 Jan to 3 May; stage 2, the knockout or final stage: 1 round from 4 May to 10 May; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`minas_gerais_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraRegMinasGerais::create_schedule` (0x434050, bra_reg_minas_gerais.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), minas_gerais_schedule_stage(stage), rounds, extra, league, 0x98aed0)
}

/// Comparer for `bra_reg_minas_gerais_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_minas_gerais_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_MINAS_GERAIS_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_minas_gerais_create_schedule", stage, minas_gerais_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_minas_gerais_create_schedule(this, stage, r, e, l))
}

/// Each game result of the Minas Gerais state league: the game goes into the overall table and, if it has no stage
/// yet, gets its half's (the first up to round 10, the second from round 11); the half is built when its first game
/// arrives (`bra_reg_minas_gerais_create_half_season_stage`, 0x4351b0), and the game also goes into the half table, without
/// counting again in the league statistics; each half winner goes to the final (`bra_reg_minas_gerais_add_two_half_result`).
/// Original: `BraRegMinasGerais::add_result` (0x435130, bra_reg_minas_gerais.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_add_result(this: usize, game: usize, stats: u32) -> u32 {
    bra_reg_minas_gerais_add_two_half_result(League(this), game, stats, 0xb, 0x4351b0, "cqu_3 minas gerais half") // cqu3: the second-half round of the Minas Gerais league
}

/// Comparer for `bra_reg_minas_gerais_add_result`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_minas_gerais_add_result(this: usize, game: usize, stats: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_BRA_REG_MINAS_GERAIS_ADD_RESULT);
    bra_reg_minas_gerais_compare_two_half_result("bra_reg_minas_gerais_add_result", League(this), game, 0xb, || o(this, game, stats))
}

/// The stages of the round table of the Minas Gerais state league (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 22 rounds from 11 Jan to 3 May; stage 2, the knockout or final stage: 1 round from 4 May to 10 May; any other stage has no table (None).
pub(crate) fn minas_gerais_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 22, extra: 0, extra_check: false, size: 0x596, row: 0xa6,
            ops: &MINAS_GERAIS_SCHEDULE_LEAGUE_OPS, name: "cqw_2 minas gerais league" }),
        2 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x13a,
            ops: &MINAS_GERAIS_SCHEDULE_STAGE_2_OPS, name: "cqw_2 minas gerais stage 2" }),
        _ => None,
    }
}

/// The result of a game in a two-half league (the same 128-byte template in all four, checked instruction by
/// instruction: only the round where the second half starts, 11 in the state leagues and 15 in J1, and the half routine
/// change).
///
/// 1) The game goes into the overall table (League::add_result `exe::league_add_result`, thiscall(this, game, stats),
///    ret 8).
/// 2) If the game has no stage yet (`fixture::STAGE_INDEX` = 0xff): its stage is the half, 0 up to round `second` − 1
///    and 1 from it on (the round is the signed word `fixture::ROUND_NUMBER`), written to `fixture::STAGE_INDEX`; while
///    the league's current stage (`league::CURRENT_STAGE`) is below it, the next half is built (the half routine `half`,
///    e.g. `jap_j1_create_half_season_stage`: thiscall(this, current stage + 1), ret 4; the original pushes the dword
///    with the byte in the low part and register garbage above; the half routine only uses the byte: the copy passes the
///    clean byte); then the game also goes into the half table (the `comp_vtable::ADD_RESULT` slot of stage
///    [`league::STAGES`][stage]: thiscall(stage, game, stats), ret 8) with the league's statistics block
///    (`league::COMP_STATS`) saved and zeroed (so the game is not counted twice) and restored; the game's
///    `fixture::STAGE_INDEX` goes back to 0xff.
///
/// Returns 1 on both paths.
pub(crate) unsafe fn bra_reg_minas_gerais_add_two_half_result(this: League, game: usize, stats: u32, second: i16, half: usize, branch: &'static str) -> u32 {
    let game = Fixture(game);
    let build: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(half);
    exe::league_add_result(this, game, stats);
    if game.stage_index() != 0xff {
        return 1;
    }
    let round = game.round_number();
    let stage: i8 = (round >= second) as i8; // cqu3: the half of the game
    game.set_stage_index((stage) as u8);
    while (this.current_stage() as i32) < stage as i32 {
        build(this.addr(), (this.current_stage() as u8).wrapping_add(1) as u32);
    }
    covered!(branch);
    let stored = this.comp_stats();
    this.set_comp_stats(0);
    let f = read_usize(this.stages() + 4 * (game.stage_index() as i8) as i32 as usize);
    let add: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(f) + comp_vtable::ADD_RESULT));
    add(f, game.addr(), stats); // cqu3: the game in the half table
    this.set_comp_stats(stored);
    game.set_stage_index(0xff); // cqu3: the game mark back
    1
}

/// Comparer of the four: only the original runs (the game goes into the tables and the half may be built; it cannot run
/// twice), with the plan made before: the return value 1, the game's `fixture::STAGE_INDEX` as it was (0xff back, if it
/// was), the league's `league::COMP_STATS` unchanged and, when the game had no stage, the league's current stage
/// (`league::CURRENT_STAGE`) already at the game's half, with that stage built.
pub(crate) unsafe fn bra_reg_minas_gerais_compare_two_half_result(name: &str, this: League, game: usize, second: i16, original: impl FnOnce() -> u32) -> u32 {
    let game = Fixture(game);
    let mark = game.stage_index();
    let block = this.comp_stats();
    let stage = (game.round_number() >= second) as i32;
    let r = original();
    let mut ok = r == 1 && game.stage_index() == mark && this.comp_stats() == block;
    if mark == 0xff {
        ok = ok && this.current_stage() as i32 >= stage && read_usize(this.stages() + 4 * stage as usize) != 0;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}, {:#x}): returned {}, mark {:#x} -> {:#x}, +0x30 = {}, game stage {}", name, this.addr(), game.addr(), r, mark, game.stage_index(), this.current_stage() as i32, stage));
    r
}

/// The Minas Gerais state league: the fixed places 1, 2 (the final between the top two).
pub(crate) const MINAS_GERAIS_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98aed0, lines: [0x33f, 0x347], places: &[1, 2], label: "cqx_4 minas gerais predicted" }; // cqx4: the minas gerais places

/// The Minas Gerais state league, the round-robin stage (−1): 22 rounds (11 Jan to 3 May), 1430 bytes, 78 operations.
pub(crate) const MINAS_GERAIS_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 78] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x15, 1, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0), Cqp11D(3, 0x19, 1, 0, 6, 1, 0),
    Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0), Cqp11D(4, 4, 2, 0, 6, 1, 0),
    Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0), Cqp11D(5, 0xb, 2, 0, 6, 1, 0),
    Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0), Cqp11D(6, 0xe, 2, 0, 2, 2, 0),
    Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x12, 2, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0),
    Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 0x19, 2, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0),
    Cqp11L(8, 2, -1, -1, -1, 0), Cqp11D(9, 1, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0),
    Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 4, 3, 0, 2, 2, 0), Cqp11L(0xa, 0, 5, 1, 1, 0),
    Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 8, 3, 0, 6, 1, 0),
    Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0),
    Cqp11D(0xc, 0xf, 3, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0),
    Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0),
    Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 0x19, 3, 0, 2, 2, 0),
    Cqp11L(0xe, 0, -1, -1, -1, 0), Cqp11D(0xf, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xf, 0, 5, 1, 1, 0),
    Cqp11L(0xf, 1, 5, 1, 1, 0), Cqp11L(0xf, 2, -1, -1, -1, 0), Cqp11D(0x10, 6, 4, 0, 6, 1, 0),
    Cqp11L(0x10, 0, 5, 1, 1, 0), Cqp11L(0x10, 1, 5, 1, 1, 0), Cqp11L(0x10, 2, -1, -1, -1, 0),
    Cqp11D(0x11, 0xd, 4, 0, 6, 1, 0), Cqp11L(0x11, 0, 5, 1, 1, 0), Cqp11L(0x11, 1, 5, 1, 1, 0),
    Cqp11L(0x11, 2, -1, -1, -1, 0), Cqp11D(0x12, 0x10, 4, 0, 2, 2, 0), Cqp11L(0x12, 0, -1, -1, -1, 0),
    Cqp11D(0x13, 0x14, 4, 0, 6, 1, 0), Cqp11L(0x13, 0, 5, 1, 1, 0), Cqp11L(0x13, 1, 5, 1, 1, 0),
    Cqp11L(0x13, 2, -1, -1, -1, 0), Cqp11D(0x14, 0x1b, 4, 0, 6, 1, 0), Cqp11L(0x14, 0, 5, 1, 1, 0),
    Cqp11L(0x14, 1, 5, 1, 1, 0), Cqp11L(0x14, 2, -1, -1, -1, 0), Cqp11D(0x15, 3, 5, 0, 6, 1, 0),
    Cqp11L(0x15, 0, -1, -1, -1, 0),
];

/// The Minas Gerais state league, stage 2: 1 round (4 May to 10 May), 104 bytes, 18 operations.
pub(crate) const MINAS_GERAIS_SCHEDULE_STAGE_2_OPS: [AsiaClubChampScheduleOp; 18] = [
    Cqp11S(0, 4, 5, 0, 0), Cqp11J(0, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x18, 2), Cqp11W(0x1c, 2), Cqp11B(0x21, 2),
    Cqp11W(0x7, 0x96), Cqp11W(0x9, 0), Cqp11W(0xb, 0), Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x1a, 1),
    Cqp11W(0xf, 0x804), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0x55730),
    Cqp11X(0x64, 0x186a0),
];
