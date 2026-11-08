use super::*;

/// Champion of a half: club in row 0 of stage k's table.
///
/// [pt] O campeão de um turno: o clube da primeira linha da tabela (+0xb1) da fase `k` ([+0xc][k], um League_Stage).
pub(crate) unsafe fn half_champion(this: usize, k: usize) -> usize {
    read_usize(read_usize(read_usize(read_usize(this + league::STAGES) + 4 * k) + 0xb1))
}

/// The Minas Gerais state league has two halves (stages 0 and 1, each a round-robin league) and, if needed, a final
/// (stage 2): the next stage moves the current stage on and, at the end of the second half, builds the final; past the
/// last stage it is an error (the box)
/// Original: `BraRegMinasGerais::advance_stage` (0x434ae0, bra_reg_minas_gerais.cpp, thiscall, ret 0).
///
/// [pt] { 0x434ae0 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::advance_stage()`, a fase seguinte do Mineiro (dois turnos, as fases 0
/// e 1, cada um um League_Stage, e a final, a fase 2). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa de
/// erro da linha 0x202 e nada mais. Senão +0x30 sobe um e, ao chegar a 2 (o fim do segundo turno), monta a final
/// (0x434b70, call direto com o ecx da liga).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_advance_stage(this: usize) {
    let stage = read_usize(this + league::CURRENT_STAGE) as i32;
    if stage >= (read_usize(this + league::STAGE_COUNT) as i32).wrapping_sub(1) {
        return show_error_at(0x98aed0, 0x202);
    }
    let stage = stage + 1;
    core::ptr::write_unaligned((this + league::CURRENT_STAGE) as *mut i32, stage);
    if stage == 2 {
        let final_: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_BRAREGMINASGERAIS_CREATE_FINAL_STAGE);
        final_(this); // cqo7: the final after the second stage
    }
}

/// The Minas Gerais final: the winners of the first and second halves meet over two games [inference: home and away],
/// the first-half winner as seed 0; if the same club wins both halves, it is champion without a final.
///
/// `BraRegMinasGerais::create_final_stage()`, called from the next stage when the second half ends. The two half
/// winners (the club in the first row of each League_Stage's table) go into a list of 2 (malloc 0x944e46 of 8; out of
/// memory: the error box at line 0x226 and return).
///
/// - The same club in both halves: it is champion without a final. +0x30 = 1, +0x2c = 2 (the season ends with the two
/// stages), status 5 (+0x37, champion) in the first row of each half's table and in its row of the league table (+0xb1,
/// the +0x3e rows). The list is not freed (the original leaks it; so does the copy).
///
/// - Different winners: the final, over two games [inference: home and away]. Virtual +0x3c (create_round_table:
/// thiscall(this, stage 2, &rounds, &extra, 0), ret 0x10) builds the round table; the new stage (operator new 0x944cf1
/// of 0xb2; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: the league, 2, the list, the rounds, +4, the table, the
/// year +0x40, 2, 1, the extra, 0x14, 0, 1, 0 and the seeds {0, 1}: the first-half winner is seed 0) goes into the
/// third stage slot ([+0xc]+8; no stage: the error box at line 0x250 and return, without freeing). With it:
/// Cup::set_up_round (0x51c800, thiscall(stage, 0), ret 4), the table and the list freed (free 0x9452ca, in that order)
/// and +0x30 = 2. The original's SEH frame covers only the constructor (no destructor).
/// Original: `BraRegMinasGerais::create_final_stage` (0x434b70, bra_reg_minas_gerais.cpp, thiscall, ret 0).
///
/// [pt] { 0x434b70 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::create_final_stage()`, da fase seguinte quando o segundo turno
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
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_create_final_stage(this: usize) {
    let seeds: [u8; 4] = [0, 1, 0, 0];
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let list = alloc(8);
    if list == 0 {
        return show_error_at(0x98aed0, 0x226);
    }
    let a = half_champion(this, 0);
    core::ptr::write_unaligned(list as *mut usize, a);
    let b = half_champion(this, 1);
    core::ptr::write_unaligned((list + 4) as *mut usize, b);
    if a == b {
        crate::comparison::branch("cqo_7 mesmo campeao");
        core::ptr::write_unaligned((this + league::CURRENT_STAGE) as *mut u32, 1);
        core::ptr::write_unaligned((this + league::STAGE_COUNT) as *mut u32, 2);
        for k in 0..2 {
            *((read_usize(read_usize(read_usize(this + league::STAGES) + 4 * k) + 0xb1) + 0x37) as *mut u8) = 5; // cqo7: campeão dos dois turnos
        }
        let mut i: i16 = 0;
        while i < read_i16(this + league::CLUB_COUNT) {
            let row = read_usize(this + league::TABLE) + i as usize * 0x3b;
            if read_usize(row) == read_usize(list) {
                *((row + 0x37) as *mut u8) = 5;
            }
            i += 1;
        }
        // simplification: the 8-byte list is not freed, as in the original.
        return;
    }
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this, 2, &mut rodadas, &mut extra, 0);
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let p = new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year in dwords with garbage in the high 16 bits (the outputs of the
    // virtual are words on the stack; the year goes in dx); the constructor only reads the words.
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 2, list, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + league::YEAR) as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 1, 0, seeds.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + league::STAGES) + 8) as *mut usize, stage);
    let stage = read_usize(read_usize(this + league::STAGES) + 8);
    if stage == 0 {
        return show_error_at(0x98aed0, 0x250);
    }
    crate::comparison::branch("cqo_7 final");
    let build: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    build(stage, 0);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    free(t);
    free(list);
    core::ptr::write_unaligned((this + league::CURRENT_STAGE) as *mut u32, 2);
}

/// The end of the Minas Gerais season: with a final, the honours come from the final; without one, from the league, and
/// the champion gets 350,000 and the runner-up 100,000 in prize money. Then, once the twelve state leagues are over,
/// the champions go into the Champions Cup.
///
/// `BraRegMinasGerais::record_season_honours()` (slot 13).
///
/// - With a final (+0x30 = 2): the history of the final (slot 13, +0x30, of stage [+0xc]+8) and then, as a tail jump,
/// 0x42bba0 (`bra_nat_first_fill_champions_cup_entrants`, cdecl with no arguments: when the twelve state leagues are
/// over, it puts the champions into the Champions Cup); returns its eax.
///
/// - Without a final (the same champion in both halves): the league history (League::record_season_honours 0x684640,
/// thiscall, ret 0); the prizes, 350,000 to the champion (first row of table +0xb1) and 100,000 to the runner-up
/// (second row), each one to the balance (0x5a0590 on [0xae2a38] -> the club's finances, thiscall, ret 4; 0x5999a0
/// Finance::add_prize_money, thiscall, ret 4) and in the prize money news (0x48e3e0 Comp::send_prize_money_news:
/// thiscall(this, club, 64-bit amount, position 1 or 2, round 0, match 0, type −2), ret 0x1c); then 0x42bba0, as above.
/// Original: `BraRegMinasGerais::record_season_honours` (0x434ed0, bra_reg_minas_gerais.cpp, thiscall, ret 0).
///
/// [pt] { 0x434ed0 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::record_season_honours()` (slot 13), o fim da temporada do Mineiro.
/// - Com final (+0x30 = 2): o histórico da final (o slot 13, +0x30, da fase [+0xc]+8) e, em salto final, a 0x42bba0
///   (`bra_nat_first_fill_champions_cup_entrants`, cdecl sem argumentos: quando os doze estaduais terminam, põe os
///   campeões na Copa dos Campeões); devolve o eax dela.
/// - Sem final (o mesmo campeão nos dois turnos): o histórico da liga (League::record_season_honours 0x684640,
///   thiscall, ret 0); os prêmios, 350.000 ao campeão (primeira linha da tabela +0xb1) e 100.000 ao vice (segunda linha),
///   cada um no saldo (0x5a0590 em [0xae2a38] -> as finanças do clube, thiscall, ret 4; 0x5999a0 Finance::add_prize_money,
///   thiscall, ret 4) e na notícia do prêmio (0x48e3e0 Comp::send_prize_money_news: thiscall(this, clube, valor em 64
///   bits, posição 1 ou 2, rodada 0, partida 0, tipo −2), ret 0x1c); depois a 0x42bba0, como acima.
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_record_season_honours(this: usize) -> u32 {
    let campeoes: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_BRA_NAT_FIRST_FILL_CHAMPIONS_CUP_ENTRANTS);
    if read_usize(this + league::CURRENT_STAGE) == 2 {
        let f = read_usize(read_usize(this + league::STAGES) + 8);
        let history: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(f) + 0x30));
        history(f);
        crate::comparison::branch("cqo_7 historico da final");
        return campeoes();
    }
    let history: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_RECORD_SEASON_HONOURS);
    history(this);
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let prize: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_ADD_PRIZE_MONEY);
    let news: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, i32) = core::mem::transmute(FN_COMP_SEND_PRIZE_MONEY_NEWS);
    for (linha, valor, posicao) in [(0usize, 350000i32, 1i32), (0x3b, 100000, 2)] {
        let c = read_usize(read_usize(this + league::TABLE) + linha);
        prize(finances(read_usize(G_FINANCE_TABLE), c), valor);
        news(this, read_usize(read_usize(this + league::TABLE) + linha), valor, 0, posicao, 0, 0, -2);
    }
    crate::comparison::branch("cqo_7 premios");
    campeoes()
}

/// Comp_Stats::find_club_entry(club) on [this+8] (original routine).
///
/// [pt] 0x4a4850 `Comp_Stats::find_club_entry(clube)` (thiscall, ret 4): a entrada do clube nas estatísticas da competição
/// ([this+8]); ordena a tabela pelo clube quando preciso (marca +0x18) e busca. Original (não portada).
pub(crate) unsafe fn bra_reg_minas_gerais_comp_stats_club_entry(this: usize, club: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    f(core::ptr::read_unaligned((this + 8) as *const u32) as usize, club)
}

/// Writes the club's current, lowest and highest progress codes (+0x73, +0x74, +0x75).
pub(crate) unsafe fn write_progress_codes(e: usize, current: u8, minima: u8, maxima: u8) {
    *((e + 0x73) as *mut u8) = current;
    *((e + 0x74) as *mut u8) = minima;
    *((e + 0x75) as *mut u8) = maxima;
}

/// Minas Gerais, Parana and Rio: writes the club progress codes (current, lowest and highest position); in the round
/// robin, the lowest under 3 becomes 1 and the highest under 3 becomes 2 [inference: the two halves have tables of
/// their own and the overall one gives no title]
/// Original: `BraRegMinasGerais::set_club_progress` (0x4350d0, bra_reg_minas_gerais.cpp, thiscall).
///
/// [pt] { 0x4350d0 } (thiscall, ret 0x14, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::set_club_progress(clube, fase, a,
/// b, c)` (slot 27 do Mineiro, do Paranaense e do Carioca): grava os três códigos de progresso do clube na entrada dele
/// (+0x73, +0x74, +0x75); na fase de pontos corridos (-1), o segundo abaixo de 3 vira 1 e o terceiro abaixo de 3 vira 2.
/// Devolve a entrada.
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_set_club_progress(this: usize, club: usize, stage: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, club);
    let (a, mut b, c) = (a as u8, b as u8, c as u8);
    if stage as u8 == 0xff {
        if b < 3 {
            b = 1;
        }
        if c < 3 {
            crate::comparison::branch("cqk_8 mineiro terceiro abaixo de 3");
            write_progress_codes(e, a, b, 2); // cqk8: the third Mineiro code
            return e;
        }
    }
    crate::comparison::branch("cqk_8 mineiro como vem");
    write_progress_codes(e, a, b, c); // cqk8: the Mineiro codes as they come
    e
}

/// MLS regular-season progress code: 1..3 -> ate_3, 4 -> 10, others unchanged.
///
/// [pt] O código da MLS na temporada regular: 1..=3 viram `ate_3`, 4 vira 10, os outros ficam.
pub(crate) fn mls_progress_code(x: u8, ate_3: u8) -> u8 {
    match x {
        1..=3 => ate_3,
        4 => 10,
        _ => x,
    }
}

/// Serie C group progress code: 1,2 -> um_dois, 3..7 -> 0x11..0x31 in steps of 8, others unchanged.
///
/// [pt] O código da Série C nos grupos (fases até 6): 1 e 2 viram `um_dois`, 3 → 0x11, 4 → 0x19, 5 → 0x21, 6 → 0x29,
/// 7 → 0x31 (a posição no grupo levada para a escala de 49 lugares [inferência]); os outros ficam.
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
///
/// [pt] { 0x434d90 } (thiscall, ret 0x18, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::update_staff_records_for_status`
/// (slot 18 do Mineiro, do Paranaense e do Carioca): os pontos corridos como no Centro-Oeste; os turnos (fases 0 e 1)
/// devolvem -1; o mata-mata na fase 2 (peso pela escala de uma rodada só: 40). Devolve 0 ou -1.
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    mineiro_end_of_stage(this, club, status, stage, game, true).0
}

/// The Minas Gerais state league at career creation: the year and the database competition, disciplinary rule 7 and,
/// with no saved state, importance 15, two halves and the final (three stages besides the league), the rules, the table
/// with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegMinasGerais::BraRegMinasGerais` (0x433d50, bra_reg_minas_gerais.cpp, thiscall, ret 8).
///
/// [pt] { 0x433d50 } (thiscall, ret 8, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::BraRegMinasGerais(ano, registro)`: o Mineiro (três fases além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_minas_gerais(this: usize, year: u32, record: usize) -> usize {
    bra_reg_goias_construct_state_league(this, year, record, &MINEIRO_STATE_LEAGUE, 7, true, "cqt_2 mineiro novo") // cqt2: the discipline and expected positions of the Mineiro
}

/// the Minas Gerais league has 12 clubs: those with it as their reserve division in the database (the field that holds
/// the state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegMinasGerais::create_table` (0x434810, bra_reg_minas_gerais.cpp, thiscall, ret 0).
///
/// [pt] { 0x434810 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::create_table()`: o Mineiro tem 12 clubes (divisão reserva +0x60 = [0x9cf9d4]).
/// Fonte 0x98aed0, linha 0x169.
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_create_table(this: usize) -> u32 {
    fill_state_league_table(this, 0xc, 0x98aed0, 0x169, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, 0, "cql_9 mineiro cheio")
}

/// One half of the Minas Gerais state league: when the first game of a half is processed, it builds that half's
/// round-robin stage (a league with all the clubs of the table, the same points for a win and a draw and the same
/// tie-breakers as the league, and one top place, the half winner's); the first half goes to stage 0 and the second
/// (from round 12, by the match +0x34) to stage 1; each half winner (the top of the stage table) goes to the final
/// Original: `BraRegMinasGerais::create_half_season_stage` (0x4351b0, bra_reg_minas_gerais.cpp, thiscall, ret 4).
///
/// [pt] { 0x4351b0 } (thiscall, ret 4, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::create_half_season_stage(k)`, um turno
/// do Mineiro (a caixa da linha 0x3d5).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_create_half_season_stage(this: usize, k: u32) {
    create_half_season_stage(this, k, 0x98aed0, 0x3d5, 0xb, 0xc2, "cqr_1 turno do mineiro") // cqr1: the points per win in the Mineiro
}

/// The Minas Gerais state league turnover: archives the season in the statistics, clears the table, the schedule and
/// the stages (the two halves and the final) and prepares three new stage slots; moves to the next year and rebuilds
/// the rules, the table with the twelve state clubs, the fixtures and the order; then the predicted positions and the
/// referee renewal. The halves are only built at each one's first game (cqr_1)
/// Original: `BraRegMinasGerais::start_new_season` (0x434910, bra_reg_minas_gerais.cpp, thiscall, ret 0).
///
/// [pt] { 0x434910 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::start_new_season()`, a virada do Mineiro (a tabela: 0x434810).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_start_new_season(this: usize) -> u32 {
    start_new_season_two_halves(this, 0x434810, 0x98aed0, 1, "cqr_2 virada do mineiro") // cqr2: the next year of the Mineiro
}

