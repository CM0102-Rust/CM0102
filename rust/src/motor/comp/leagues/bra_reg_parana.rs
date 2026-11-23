use super::*;

/// The Parana state league has two halves (stages 0 and 1, each a round-robin league) and, if needed, a final (stage
/// 2): the next stage moves the current stage on and, at the end of the second half, builds the final; past the last
/// stage it is an error (the box)
/// Original: `BraRegParana::advance_stage` (0x438230, bra_reg_parana.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_advance_stage(this: usize) {
    let this = League(this);
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x98af98, 0x202);
    }
    let stage = stage + 1;
    this.set_current_stage((stage) as usize);
    if stage == 2 {
        exe::braregparana_create_final_stage(this); // cqo8: the final after the second stage
    }
}

/// The Paraná final stage: the winners of the first and second halves of the season meet over two games [inference:
/// home and away], the first-half winner as seed 0; if the same club wins both halves, it is champion without a final
/// (the season ends with the two stages and it gets status 5, champion, in the three tables).
///
/// `BraRegParana::create_final_stage()`, called from the next-stage step when the second half ends. The two half
/// champions (the club in the first row of the table of each League_Stage) go into a list of 2 (malloc
/// `exe::crt_malloc` of 8; out of memory: the box of line `0x226` and it returns).
///
/// Same club in both halves: it is champion without a final. `.set_current_stage(1)`, `.set_stage_count(2)` (the season
/// ends with the two stages), status 5 (`league_table_row::STATUS`, `table_status::CHAMPIONS`) in the first row of each
/// half's table (`league::TABLE`) and in its row of the league table (`.table()`, the `.club_count()` rows). The list
/// is not freed (the original leaks it; the port does too).
///
/// Different champions: the final, in two games [inference: home and away]. Virtual `comp_vtable::CREATE_SCHEDULE`
/// (create_round_table: thiscall(this, stage 2, &rounds, &extra, 0), ret `0x10`) builds the round table; the new stage
/// (operator new `exe::crt_operator_new` of `0xb2`; Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret
/// `0x3c`: the league, 2, the list, the rounds, `.competition()`, the table, the year `.year()`, 2, 1, the extra,
/// `0x14`, 0, 1, 0 and the seeds {0, 1}: the first-half champion is seed 0) goes into the third stage slot (`.stages()`
/// + 8; no stage: the box of line `0x250` and it returns, without freeing). With it: Cup::set_up_round
/// (`exe::cup_set_up_round`, thiscall(stage, 0), ret 4), the table and the list freed (free `exe::crt_free`, in that
/// order) and `.set_current_stage(2)`. The original's SEH frame only covers the constructor (no destructor).
/// Original: `BraRegParana::create_final_stage` (0x4382c0, bra_reg_parana.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_create_final_stage(this: usize) {
    let this = League(this);
    let seeds: [u8; 4] = [0, 1, 0, 0];
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(0x98af98, 0x226);
    }
    let a = half_champion(this, 0);
    write_usize(list, a);
    let b = half_champion(this, 1);
    write_usize(list + 4, b);
    if a == b {
        covered!("cqo_8 same champion");
        this.set_current_stage((1) as usize);
        this.set_stage_count((2) as usize);
        for k in 0..2 {
            write_u8(read_usize(read_usize(this.stages() + 4 * k) + league::TABLE) + league_table_row::STATUS, 5); // cqo8: champion of both halves
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
        return show_error_at(0x98af98, 0x250);
    }
    covered!("cqo_8 final");
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(t);
    exe::crt_free(list);
    this.set_current_stage((2) as usize);
}

/// The end of the Paraná season: with a final, the honours come from the final; without one, from the league, and the
/// champion gets 175,000 and the runner-up 100,000 in prize money (to the balance, with the prize money news). Then,
/// once the twelve state leagues are over, the champions go into the Champions Cup.
///
/// `BraRegParana::record_season_honours()` (slot 13).
///
/// With a final (`.current_stage()` = 2): the final's history (slot 13, `comp_vtable::RECORD_SEASON_HONOURS`, of stage
/// `.stages()` + 8) and, as a tail jump, `exe::bra_nat_first_fill_champions_cup_entrants` (cdecl with no arguments:
/// when the twelve state leagues finish, it puts the champions into the Champions Cup); returns its eax.
///
/// Without a final (the same champion in both halves): the league history (League::record_season_honours
/// `FN_LEAGUE_RECORD_SEASON_HONOURS`, thiscall, ret 0); the prizes, 175,000 to the champion (first row of table
/// `.table()`) and 100,000 to the runner-up (second row), each to the balance (`exe::financetable_get_club_finance` on
/// `G_FINANCE_TABLE` -> the club's finances, thiscall, ret 4; `exe::finance_add_prize_money` Finance::add_prize_money,
/// thiscall, ret 4) and in the prize news (`exe::comp_send_prize_money_news` Comp::send_prize_money_news:
/// thiscall(this, club, 64-bit value, position 1 or 2, round 0, match 0, type −2), ret `0x1c`); then
/// `exe::bra_nat_first_fill_champions_cup_entrants`, as above.
/// Original: `BraRegParana::record_season_honours` (0x4384e0, bra_reg_parana.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_record_season_honours(this: usize) -> u32 {
    let this = League(this);
    if this.current_stage() == 2 {
        let f = read_usize(this.stages() + 8);
        let history: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(f) + comp_vtable::RECORD_SEASON_HONOURS));
        history(f);
        covered!("cqo_8 final history");
        return exe::bra_nat_first_fill_champions_cup_entrants();
    }
    let history: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_RECORD_SEASON_HONOURS);
    history(this.addr());
    for (linha, valor, posicao) in [(0usize, 175000i32, 1i32), (0x3b, 100000, 2)] {
        let c = read_usize(this.table() + linha);
        exe::finance_add_prize_money(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c), valor);
        exe::comp_send_prize_money_news(this, read_usize(this.table() + linha), valor, 0, posicao, 0, 0, -2);
    }
    covered!("cqo_8 prizes");
    exe::bra_nat_first_fill_champions_cup_entrants()
}

/// Writes the rule set common to these Brazilian state leagues, with the top places (+0xbe) and the relegated (+0xc1)
/// as parameters.
///
/// The three routines write the same set; only the top places +0xbe and the relegated +0xc1 change; all three have two
/// rounds.
///
/// Win 3, draw 1, category 2 (league), national club, tie-breakers head-to-head and goal difference, no play-offs in
/// the table, no league above or below, rules word 8, 7 substitutes and 3 substitutions, and the words +0xab and +0xdb
/// at 0.
pub(crate) unsafe fn bra_reg_parana_set_state_league_rules(o: usize, cima: u8, relegated: u8) {
    write_u16_at(o, 0x3c, 2);
    write_u8_at(o, 0xc4, 2);
    write_u16_at(o, 0xab, 0);
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbe, cima);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc5, 3);
    write_u8_at(o, 0x4a, 3);
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc6, 1);
    write_u8_at(o, 0xc1, relegated);
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u8_at(o, 0x49, 7);
    write_u16_at(o, 0xd9, 8);
}

/// State league rules for Paranaense (`bra_reg_parana_set_up_rules`): no top places, 4 relegated.
pub(crate) unsafe fn parana_rules(o: usize) {
    bra_reg_parana_set_state_league_rules(o, 0, 4); // cqi9: o Paranaense
}

/// State league rules for Pernambucano (`bra_reg_pern_set_up_rules`, also Norte): 4 top places, 2 relegated.
pub(crate) unsafe fn pernambucano_rules(o: usize) {
    bra_reg_parana_set_state_league_rules(o, 4, 2); // cqi9: o Pernambucano
}

/// State league rules for Carioca (`bra_reg_rio_set_up_rules`, also Mineiro): no top places, 2 relegated.
pub(crate) unsafe fn carioca_rules(o: usize) {
    bra_reg_parana_set_state_league_rules(o, 0, 2); // cqi9: o Carioca
}

/// Parana: no top places in the overall table (each half is a league stage of its own, created by add_result 0x4386e0,
/// and decides the title [inference]); 4 relegated; double round robin; win 3, draw 1; tie-break by head-to-head, then
/// goal difference; no play-offs in the table; no league above or below (the relegated go to the non-playable pool
/// [inference]); rule word 8 (the top places lead to the next stage; no champion band); 7 bench substitutes and 3
/// substitutions; category 2 (league) matches. Then the main stage schedule
/// Original: `BraRegParana::set_up_rules` (0x437fc0, bra_reg_parana.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_set_up_rules(this: usize) -> u32 {
    parana_rules(this);
    covered!("cqi_9 paranaense rules");
    create_main_schedule(League(this));
    1
}

/// The Parana state league at career creation: the year and the database competition, disciplinary rule 7 and, with no
/// saved state, importance 15, two halves and the final (three stages besides the league), the rules, the table with
/// the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegParana::BraRegParana` (0x437400, bra_reg_parana.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_parana(this: usize, year: u32, record: usize) -> usize {
    bra_reg_northeast_construct_state_league(League(this), year, record, &PARANA_CHAMPIONSHIP, 7, true, "cqt_3 paranaense novo") // cqt3: the discipline and expected positions of the Paranaense
}

/// the Parana league has 12 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegParana::create_table` (0x437ec0, bra_reg_parana.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_create_table(this: usize) -> u32 {
    fill_state_league_table(League(this), 0xc, 0x98af98, 0x169, KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, 0, "cqm_7 paranaense cheio")
}

/// One half of the Parana state league: when the first game of a half is processed, it builds that half's round-robin
/// stage (a league with all the clubs of the table, the same points for a win and a draw and the same tie-breakers as
/// the league, and one top place, the half winner's); the first half goes to stage 0 and the second (from round 12, by
/// the match +0x34) to stage 1; each half winner (the top of the stage table) goes to the final
/// Original: `BraRegParana::create_half_stage` (0x438760, bra_reg_parana.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn create_half_stage(this: usize, k: u32) {
    create_half_season_stage(League(this), k, 0x98af98, 0x3d5, 0xb, 0xc2, "cqr_1 turno do paranaense") // cqr1: the points per win in the Paranaense
}

/// The Parana state league turnover: archives the season in the statistics, clears the table, the schedule and the
/// stages (the two halves and the final) and prepares three new stage slots; moves to the next year and rebuilds the
/// rules, the table with the twelve state clubs, the fixtures and the order; then the predicted positions and the
/// referee renewal. The halves are only built at each one's first game (cqr_1)
/// Original: `BraRegParana::start_new_season` (0x438060, bra_reg_parana.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_start_new_season(this: usize) -> u32 {
    start_new_season_two_halves(League(this), 0x437ec0, 0x98af98, 1, "cqr_2 virada do paranaense") // cqr2: the next year of the Paranaense
}

/// The predicted positions of the Paraná state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, and the others their place in the order (`state_league_predicted_positions`
/// with `PARANA_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x33f (no statistics) and 0x347 (no memory).
/// Original: `BraRegParana::set_predicted_positions` (0x438590, bra_reg_parana.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &PARANA_PREDICTED)
}

/// Comparer for `bra_reg_parana_set_predicted_positions` (`compare_state_league_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_parana_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_PARANA_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_parana_set_predicted_positions", this, &PARANA_PREDICTED, || o(this));
}

/// The schedule of the Paraná state league: stage −1, the round-robin league stage: 22 rounds from 11 Jan to 3 May; stage 2, the knockout or final stage: 1 round from 4 May to 10 May; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`parana_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraRegParana::create_schedule` (0x437700, bra_reg_parana.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), parana_schedule_stage(stage), rounds, extra, league, 0x98af98)
}

/// Comparer for `bra_reg_parana_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_parana_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_PARANA_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_parana_create_schedule", stage, parana_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_parana_create_schedule(this, stage, r, e, l))
}

/// Each game result of the Paraná state league: the game goes into the overall table and, if it has no stage yet, gets
/// its half's (the first up to round 10, the second from round 11); the half is built when its first game arrives
/// (BraRegParana::create_half_stage, `create_half_stage`, 0x438760), and the game also goes into the half table, without counting
/// again in the league statistics; each half winner goes to the final (`bra_reg_minas_gerais_add_two_half_result`).
/// Original: `BraRegParana::add_result` (0x4386e0, bra_reg_parana.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_add_result(this: usize, game: usize, stats: u32) -> u32 {
    bra_reg_minas_gerais_add_two_half_result(League(this), game, stats, 0xb, 0x438760, "cqu_3 parana half") // cqu3: the Paraná league
}

/// Comparer for `bra_reg_parana_add_result`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_parana_add_result(this: usize, game: usize, stats: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_BRA_REG_PARANA_ADD_RESULT);
    bra_reg_minas_gerais_compare_two_half_result("bra_reg_parana_add_result", League(this), game, 0xb, || o(this, game, stats))
}

/// The stages of the round table of the Paraná state league (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 22 rounds from 11 Jan to 3 May; stage 2, the knockout or final stage: 1 round from 4 May to 10 May; any other stage has no table (None).
pub(crate) fn parana_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 22, extra: 0, extra_check: false, size: 0x596, row: 0xa6,
            ops: &PARANA_SCHEDULE_LEAGUE_OPS, name: "cqw_2 parana league" }),
        2 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x13a,
            ops: &PARANA_SCHEDULE_STAGE_2_OPS, name: "cqw_2 parana stage 2" }),
        _ => None,
    }
}

/// The Paraná state league: the fixed places 1, 2 (the final between the top two).
pub(crate) const PARANA_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98af98, lines: [0x33f, 0x347], places: &[1, 2], label: "cqx_5 paraná predicted" }; // cqx5: the paraná places

/// The Paraná state league, the round-robin stage (−1): 22 rounds (11 Jan to 3 May), 1430 bytes, 78 operations.
pub(crate) const PARANA_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 78] = [
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

/// The Paraná state league, stage 2: 1 round (4 May to 10 May), 104 bytes, 18 operations.
pub(crate) const PARANA_SCHEDULE_STAGE_2_OPS: [AsiaClubChampScheduleOp; 18] = [
    Cqp11S(0, 4, 5, 0, 0), Cqp11J(0, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x18, 2), Cqp11W(0x1c, 2), Cqp11B(0x21, 2),
    Cqp11W(0x7, 0x96), Cqp11W(0x9, 0), Cqp11W(0xb, 0), Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x1a, 1),
    Cqp11W(0xf, 0x804), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0x2ab98),
    Cqp11X(0x64, 0x186a0),
];
