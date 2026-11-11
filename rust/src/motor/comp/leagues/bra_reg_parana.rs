use super::*;

/// The Parana state league has two halves (stages 0 and 1, each a round-robin league) and, if needed, a final (stage
/// 2): the next stage moves the current stage on and, at the end of the second half, builds the final; past the last
/// stage it is an error (the box)
/// Original: `BraRegParana::advance_stage` (0x438230, bra_reg_parana.cpp, thiscall, ret 0).
///
/// [pt] { 0x438230 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::advance_stage()`, a fase seguinte do Paranaense (dois turnos, as fases 0
/// e 1, cada um um League_Stage, e a final, a fase 2). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa de
/// erro da linha 0x202 e nada mais. Senão +0x30 sobe um e, ao chegar a 2 (o fim do segundo turno), monta a final
/// (0x4382c0, call direto com o ecx da liga).
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
/// champions (the club in the first row of the table of each League_Stage) go into a list of 2 (malloc `0x944e46` of 8;
/// out of memory: the box of line `0x226` and it returns).
///
/// Same club in both halves: it is champion without a final. `+0x30` = 1, `+0x2c` = 2 (the season ends with the two
/// stages), status 5 (`+0x37`, champion) in the first row of each half's table and in its row of the league table
/// (`+0xb1`, the `+0x3e` rows). The list is not freed (the original leaks it; the port does too).
///
/// Different champions: the final, in two games [inference: home and away]. Virtual `+0x3c` (create_round_table:
/// thiscall(this, stage 2, &rounds, &extra, 0), ret `0x10`) builds the round table; the new stage (operator new
/// `0x944cf1` of `0xb2`; Cup_Stage::Cup_Stage `0x522e00`, thiscall, ret `0x3c`: the league, 2, the list, the rounds,
/// `+4`, the table, the year `+0x40`, 2, 1, the extra, `0x14`, 0, 1, 0 and the seeds {0, 1}: the first-half champion is
/// seed 0) goes into the third stage slot (`[+0xc]+8`; no stage: the box of line `0x250` and it returns, without
/// freeing). With it: Cup::set_up_round (`0x51c800`, thiscall(stage, 0), ret 4), the table and the list freed (free
/// `0x9452ca`, in that order) and `+0x30` = 2. The original's SEH frame only covers the constructor (no destructor).
/// Original: `BraRegParana::create_final_stage` (0x4382c0, bra_reg_parana.cpp, thiscall, ret 0).
///
/// [pt] { 0x4382c0 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::create_final_stage()`, da fase seguinte quando o segundo turno
/// acaba. Os dois campeões dos turnos (o clube da primeira linha da tabela de cada League_Stage) vão para uma lista
/// de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x226 e volta).
/// - O mesmo clube nos dois turnos: é o campeão sem final. +0x30 = 1, +0x2c = 2 (a temporada acaba com as duas fases),
///   a situação 5 (+0x37, campeão) na primeira linha da tabela de cada turno e na linha dele da tabela da liga
///   (+0xb1, as +0x3e linhas). A lista não é liberada (a original a deixa vazar; a cópia também).
/// - Campeões diferentes: a final, em dois jogos [inferência: ida e volta]. A virtual +0x3c (create_round_table:
///   thiscall(this, fase 2, &rodadas, &extra, 0), ret 0x10) monta a tabela de rodadas; a fase nova (operator new
///   0x944cf1 de 0xb2; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela,
///   o ano +0x40, 2, 1, o extra, 0x14, 0, 1, 0 e as sementes {0, 1}: o campeão do primeiro turno é a semente 0) vai
///   para a terceira vaga das fases ([+0xc]+8; sem fase: a caixa da linha 0x250 e volta, sem liberar). Com ela:
///   Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela e a lista liberadas (free 0x9452ca, nessa
///   ordem) e +0x30 = 2. O quadro SEH da original só cobre o construtor (sem destrutor).
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
        crate::comparison::branch("cqo_8 same champion");
        this.set_current_stage((1) as usize);
        this.set_stage_count((2) as usize);
        for k in 0..2 {
            write_u8(read_usize(read_usize(this.stages() + 4 * k) + league::TABLE) + league_table_row::STATUS, 5); // cqo8: champion of both halves
        }
        let mut i: i16 = 0;
        while i < this.club_count() {
            let row = this.table() + i as usize * 0x3b;
            if read_usize(row) == read_usize(list) {
                write_u8(row + league_table_row::STATUS, 5);
            }
            i += 1;
        }
        // simplification: the 8-byte list is not freed, as in the original.
        return;
    }
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x3c));
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
    crate::comparison::branch("cqo_8 final");
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
/// With a final (`+0x30` = 2): the final's history (slot 13, `+0x30`, of stage `[+0xc]+8`) and, as a tail jump,
/// `0x42bba0` (`bra_nat_first_fill_champions_cup_entrants`, cdecl with no arguments: when the twelve state leagues
/// finish, it puts the champions into the Champions Cup); returns its eax.
///
/// Without a final (the same champion in both halves): the league history (League::record_season_honours `0x684640`,
/// thiscall, ret 0); the prizes, 175,000 to the champion (first row of table `+0xb1`) and 100,000 to the runner-up
/// (second row), each to the balance (`0x5a0590` on `[0xae2a38]` -> the club's finances, thiscall, ret 4; `0x5999a0`
/// Finance::add_prize_money, thiscall, ret 4) and in the prize news (`0x48e3e0` Comp::send_prize_money_news:
/// thiscall(this, club, 64-bit value, position 1 or 2, round 0, match 0, type −2), ret `0x1c`); then `0x42bba0`, as
/// above.
/// Original: `BraRegParana::record_season_honours` (0x4384e0, bra_reg_parana.cpp, thiscall, ret 0).
///
/// [pt] { 0x4384e0 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::record_season_honours()` (slot 13), o fim da temporada do Paranaense.
/// - Com final (+0x30 = 2): o histórico da final (o slot 13, +0x30, da fase [+0xc]+8) e, em salto final, a 0x42bba0
///   (`bra_nat_first_fill_champions_cup_entrants`, cdecl sem argumentos: quando os doze estaduais terminam, põe os
///   campeões na Copa dos Campeões); devolve o eax dela.
/// - Sem final (o mesmo campeão nos dois turnos): o histórico da liga (League::record_season_honours 0x684640,
///   thiscall, ret 0); os prêmios, 175.000 ao campeão (primeira linha da tabela +0xb1) e 100.000 ao vice (segunda linha),
///   cada um no saldo (0x5a0590 em [0xae2a38] -> as finanças do clube, thiscall, ret 4; 0x5999a0 Finance::add_prize_money,
///   thiscall, ret 4) e na notícia do prêmio (0x48e3e0 Comp::send_prize_money_news: thiscall(this, clube, valor em 64
///   bits, posição 1 ou 2, rodada 0, partida 0, tipo −2), ret 0x1c); depois a 0x42bba0, como acima.
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_record_season_honours(this: usize) -> u32 {
    let this = League(this);
    if this.current_stage() == 2 {
        let f = read_usize(this.stages() + 8);
        let history: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(f) + 0x30));
        history(f);
        crate::comparison::branch("cqo_8 final history");
        return exe::bra_nat_first_fill_champions_cup_entrants();
    }
    let history: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_RECORD_SEASON_HONOURS);
    history(this.addr());
    for (linha, valor, posicao) in [(0usize, 175000i32, 1i32), (0x3b, 100000, 2)] {
        let c = read_usize(this.table() + linha);
        exe::finance_add_prize_money(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c), valor);
        exe::comp_send_prize_money_news(this, read_usize(this.table() + linha), valor, 0, posicao, 0, 0, -2);
    }
    crate::comparison::branch("cqo_8 prizes");
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
///
/// [pt] As regras comuns destes estaduais (as três rotinas gravam o mesmo conjunto; mudam as vagas de cima +0xbe e os
/// rebaixados +0xc1; os três têm dois turnos): vitória 3, empate 1, categoria 2 (liga), clube nacional, desempates
/// confronto direto e saldo, sem play-offs na tabela, sem liga acima nem abaixo, palavra de regras 8, 7 reservas e
/// 3 substituições, e as palavras +0xab e +0xdb em 0.
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
///
/// [pt] O Paranaense (0x437fc0): nenhuma vaga de cima na tabela geral (os turnos têm estágios próprios), 4 rebaixados.
pub(crate) unsafe fn parana_rules(o: usize) {
    bra_reg_parana_set_state_league_rules(o, 0, 4); // cqi9: o Paranaense
}

/// State league rules for Pernambucano (`bra_reg_pern_set_up_rules`, also Norte): 4 top places, 2 relegated.
///
/// [pt] O Pernambucano (0x439430, também o Norte): 4 vagas de cima, 2 rebaixados.
pub(crate) unsafe fn pernambucano_rules(o: usize) {
    bra_reg_parana_set_state_league_rules(o, 4, 2); // cqi9: o Pernambucano
}

/// State league rules for Carioca (`bra_reg_rio_set_up_rules`, also Mineiro): no top places, 2 relegated.
///
/// [pt] O Carioca (0x43a600, também o Mineiro): nenhuma vaga de cima na tabela geral, 2 rebaixados.
pub(crate) unsafe fn carioca_rules(o: usize) {
    bra_reg_parana_set_state_league_rules(o, 0, 2); // cqi9: o Carioca
}

/// Parana: no top places in the overall table (each half is a league stage of its own, created by add_result 0x4386e0,
/// and decides the title [inference]); 4 relegated; double round robin; win 3, draw 1; tie-break by head-to-head, then
/// goal difference; no play-offs in the table; no league above or below (the relegated go to the non-playable pool
/// [inference]); rule word 8 (the top places lead to the next stage; no champion band); 7 bench substitutes and 3
/// substitutions; category 2 (league) matches. Then the main stage schedule
/// Original: `BraRegParana::set_up_rules` (0x437fc0, bra_reg_parana.cpp, thiscall).
///
/// [pt] { 0x437fc0 } (thiscall, bra_reg_parana.cpp): `BraRegParana::set_up_rules` (slot 36), as regras e o calendário;
/// devolve 1.
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_set_up_rules(this: usize) -> u32 {
    parana_rules(this);
    crate::comparison::branch("cqi_9 paranaense rules");
    create_main_schedule(League(this));
    1
}

/// The Parana state league at career creation: the year and the database competition, disciplinary rule 7 and, with no
/// saved state, importance 15, two halves and the final (three stages besides the league), the rules, the table with
/// the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegParana::BraRegParana` (0x437400, bra_reg_parana.cpp, thiscall, ret 8).
///
/// [pt] { 0x437400 } (thiscall, ret 8, bra_reg_parana.cpp): `BraRegParana::BraRegParana(ano, registro)`: o Paranaense (três fases além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_parana(this: usize, year: u32, record: usize) -> usize {
    bra_reg_northeast_construct_state_league(League(this), year, record, &PARANA_CHAMPIONSHIP, 7, true, "cqt_3 paranaense novo") // cqt3: the discipline and expected positions of the Paranaense
}

/// the Parana league has 12 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegParana::create_table` (0x437ec0, bra_reg_parana.cpp, thiscall, ret 0).
///
/// [pt] { 0x437ec0 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::create_table()`: o Paranaense tem 12 clubes (divisão reserva +0x60 = [0x9cf9d0]).
/// Fonte 0x98af98, linha 0x169.
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_create_table(this: usize) -> u32 {
    fill_state_league_table(League(this), 0xc, 0x98af98, 0x169, KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, 0, "cqm_7 paranaense cheio")
}

/// One half of the Parana state league: when the first game of a half is processed, it builds that half's round-robin
/// stage (a league with all the clubs of the table, the same points for a win and a draw and the same tie-breakers as
/// the league, and one top place, the half winner's); the first half goes to stage 0 and the second (from round 12, by
/// the match +0x34) to stage 1; each half winner (the top of the stage table) goes to the final
/// Original: `BraRegParana::create_half_stage` (0x438760, bra_reg_parana.cpp, thiscall, ret 4).
///
/// [pt] { 0x438760 } (thiscall, ret 4, bra_reg_parana.cpp): `BraRegParana::create_half_stage(k)`, um turno do Paranaense (a
/// caixa da linha 0x3d5).
pub(crate) unsafe extern "thiscall" fn create_half_stage(this: usize, k: u32) {
    create_half_season_stage(League(this), k, 0x98af98, 0x3d5, 0xb, 0xc2, "cqr_1 turno do paranaense") // cqr1: the points per win in the Paranaense
}

/// The Parana state league turnover: archives the season in the statistics, clears the table, the schedule and the
/// stages (the two halves and the final) and prepares three new stage slots; moves to the next year and rebuilds the
/// rules, the table with the twelve state clubs, the fixtures and the order; then the predicted positions and the
/// referee renewal. The halves are only built at each one's first game (cqr_1)
/// Original: `BraRegParana::start_new_season` (0x438060, bra_reg_parana.cpp, thiscall, ret 0).
///
/// [pt] { 0x438060 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::start_new_season()`, a virada do Paranaense (a tabela: 0x437ec0).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_start_new_season(this: usize) -> u32 {
    start_new_season_two_halves(League(this), 0x437ec0, 0x98af98, 1, "cqr_2 virada do paranaense") // cqr2: the next year of the Paranaense
}

