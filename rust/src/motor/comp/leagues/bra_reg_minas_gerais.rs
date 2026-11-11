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

