use super::*;



/// Champion of a half: club in row 0 of stage k's table.
///
/// [pt] O campeão de um turno: o clube da primeira linha da tabela (+0xb1) da fase `k` ([+0xc][k], um League_Stage).
pub(crate) unsafe fn half_champion(this: usize, k: usize) -> usize {
    read_usize(read_usize(read_usize(read_usize(this + 0xc) + 4 * k) + 0xb1))
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
    let fase = read_usize(this + 0x30) as i32;
    if fase >= (read_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(0x98aed0, 0x202);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 2 {
        let final_: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_BRAREGMINASGERAIS_CREATE_FINAL_STAGE);
        final_(this); // cqo7: a final depois do segundo turno
    }
}

/// The Minas Gerais final: the winners of the first and second halves meet over two games [inference: home and away],
/// the first-half winner as seed 0; if the same club wins both halves, it is champion without a final (the season ends
/// with the two stages and it gets status 5, champion, in the three tables)
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
    let sementes: [u8; 4] = [0, 1, 0, 0];
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let lista = alocar(8);
    if lista == 0 {
        return show_error_at(0x98aed0, 0x226);
    }
    let a = half_champion(this, 0);
    core::ptr::write_unaligned(lista as *mut usize, a);
    let b = half_champion(this, 1);
    core::ptr::write_unaligned((lista + 4) as *mut usize, b);
    if a == b {
        crate::comparacao::ramo("cqo_7 mesmo campeao");
        core::ptr::write_unaligned((this + 0x30) as *mut u32, 1);
        core::ptr::write_unaligned((this + 0x2c) as *mut u32, 2);
        for k in 0..2 {
            *((read_usize(read_usize(read_usize(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *mut u8) = 5; // cqo7: campeão dos dois turnos
        }
        let mut i: i16 = 0;
        while i < read_i16(this + 0x3e) {
            let linha = read_usize(this + 0xb1) + i as usize * 0x3b;
            if read_usize(linha) == read_usize(lista) {
                *((linha + 0x37) as *mut u8) = 5;
            }
            i += 1;
        }
        // ponytail: a lista de 8 bytes fica sem liberar, como na original.
        return;
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 2, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no dx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construir(p, this, 2, lista, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 1, 0, sementes.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + 0xc) + 8) as *mut usize, fase);
    let fase = read_usize(read_usize(this + 0xc) + 8);
    if fase == 0 {
        return show_error_at(0x98aed0, 0x250);
    }
    crate::comparacao::ramo("cqo_7 final");
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 2);
}

/// The end of the Minas Gerais season: with a final, the honours come from the final; without one, from the league, and
/// the champion gets 350,000 and the runner-up 100,000 in prize money (to the balance, with the prize money news).
/// Then, once the twelve state leagues are over, the champions go into the Champions Cup
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
    if read_usize(this + 0x30) == 2 {
        let f = read_usize(read_usize(this + 0xc) + 8);
        let historico: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(f) + 0x30));
        historico(f);
        crate::comparacao::ramo("cqo_7 historico da final");
        return campeoes();
    }
    let historico: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_RECORD_SEASON_HONOURS);
    historico(this);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let premio: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_ADD_PRIZE_MONEY);
    let noticia: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, i32) = core::mem::transmute(FN_COMP_SEND_PRIZE_MONEY_NEWS);
    for (linha, valor, posicao) in [(0usize, 350000i32, 1i32), (0x3b, 100000, 2)] {
        let c = read_usize(read_usize(this + 0xb1) + linha);
        premio(financas(read_usize(G_FINANCE_TABLE), c), valor);
        noticia(this, read_usize(read_usize(this + 0xb1) + linha), valor, 0, posicao, 0, 0, -2);
    }
    crate::comparacao::ramo("cqo_7 premios");
    campeoes()
}

/// Comparer for `bra_reg_minas_gerais_advance_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da fase seguinte: só a original (ponytail: ao chegar a 2 monta a final, que aloca e sorteia; não pode
/// rodar duas vezes), conferindo o plano: na última fase, +0x30 igual; senão +0x30 + 1, ou, ao chegar a 2, 2 (com
/// final) ou 1 (o mesmo campeão nos dois turnos, que a final grava).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_advance_stage(this: usize) {
    let antes = read_usize(this + 0x30) as i32;
    let ultima = antes >= (read_usize(this + 0x2c) as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_MINAS_GERAIS_ADVANCE_STAGE)(this);
    let depois = read_usize(this + 0x30) as i32;
    let igual = if ultima { depois == antes } else if antes + 1 == 2 { depois == 2 || depois == 1 } else { depois == antes + 1 };
    PLACAR.registrar(igual, || format!("bra_reg_minas_gerais_advance_stage({:#x}) antes={} depois={}", this, antes, depois));
}

/// Comparer for `bra_reg_minas_gerais_create_final_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), conferindo o plano feito
/// antes: os campeões dos turnos. O mesmo nos dois: +0x30 = 1, +0x2c = 2 e a situação 5 na primeira linha de cada turno
/// e na linha dele da tabela da liga. Diferentes: a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os
/// dois campeões na lista dela (+0xa7, 6 bytes; como conjunto: o sorteio da rodada pode reordenar), e +0x30 = 2.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_create_final_stage(this: usize) {
    let (a, b) = (half_champion(this, 0), half_champion(this, 1));
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_MINAS_GERAIS_CREATE_FINAL_STAGE)(this);
    let igual = if a == b {
        let marca = |k: usize| *((read_usize(read_usize(read_usize(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *const u8) == 5;
        let na_liga = (0..read_i16(this + 0x3e).max(0) as usize).all(|i| {
            let linha = read_usize(this + 0xb1) + i * 0x3b;
            read_usize(linha) != a || *((linha + 0x37) as *const u8) == 5
        });
        read_usize(this + 0x30) == 1 && read_usize(this + 0x2c) == 2 && marca(0) && marca(1) && na_liga
    } else {
        let fase = read_usize(read_usize(this + 0xc) + 8);
        if fase == 0 {
            false
        } else {
            let l = read_usize(fase + 0xa7);
            let (x, y) = (read_usize(l), read_usize(l + 6));
            read_i16(fase + 0x3e) == 2 && read_usize(fase + 4) == read_usize(this + 4) && ((x, y) == (a, b) || (x, y) == (b, a)) && read_usize(this + 0x30) == 2
        }
    };
    PLACAR.registrar(igual, || format!("bra_reg_minas_gerais_create_final_stage({:#x}) campeoes={:#x},{:#x} +0x30={} +0x2c={}", this, a, b, read_usize(this + 0x30), read_usize(this + 0x2c)));
}

/// Comparer for `bra_reg_minas_gerais_record_season_honours` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador do fim da temporada: só a original (ponytail: grava o histórico, que aloca o registro, paga os prêmios e
/// manda a notícia, e pode montar a Copa dos Campeões; não pode rodar duas vezes). A cópia roda no modo Rust (contadores
/// "cqo_7 historico da final" e "cqo_7 premios") e é provada pelas mutações e pelas duas carreiras Rust.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_MINAS_GERAIS_RECORD_SEASON_HONOURS)(this)
}

/// Comp_Stats::find_club_entry(club) on [this+8] (original routine).
///
/// [pt] 0x4a4850 `Comp_Stats::find_club_entry(clube)` (thiscall, ret 4): a entrada do clube nas estatísticas da competição
/// ([this+8]); ordena a tabela pelo clube quando preciso (marca +0x18) e busca. Original (não portada).
pub(crate) unsafe fn bra_reg_minas_gerais_comp_stats_club_entry(this: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    f(core::ptr::read_unaligned((this + 8) as *const u32) as usize, clube)
}

/// Writes the club's current, lowest and highest progress codes (+0x73, +0x74, +0x75).
pub(crate) unsafe fn write_progress_codes(e: usize, atual: u8, minima: u8, maxima: u8) {
    *((e + 0x73) as *mut u8) = atual;
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
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_set_club_progress(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, clube);
    let (a, mut b, c) = (a as u8, b as u8, c as u8);
    if fase as u8 == 0xff {
        if b < 3 {
            b = 1;
        }
        if c < 3 {
            crate::comparacao::ramo("cqk_8 mineiro terceiro abaixo de 3");
            write_progress_codes(e, a, b, 2); // cqk8: o terceiro código do Mineiro
            return e;
        }
    }
    crate::comparacao::ramo("cqk_8 mineiro como vem");
    write_progress_codes(e, a, b, c); // cqk8: os códigos do Mineiro como vêm
    e
}

/// Comparer for `bra_reg_minas_gerais_set_club_progress` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela entrada do clube (+0x73..+0x75), achada antes (a 0x4a4850 ordena na primeira vez e só
/// busca depois; a original e a cópia a chamam de novo); a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_set_club_progress(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, clube);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(e + 0x73, 3)], || original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(IDX_BRA_REG_MINAS_GERAIS_SET_CLUB_PROGRESS)(this, clube, fase, a, b, c), || bra_reg_minas_gerais_set_club_progress(this, clube, fase, a, b, c));
    PLACAR.registrar(igual && r == n, || format!("bra_reg_minas_gerais_set_club_progress({:x}, {:x}, {}, {}, {}, {}) original={:x} rust={:x} {:?} {:?}", this, clube, fase as u8 as i8, a as u8, b as u8, c as u8, r, n, mo, mr));
    r
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
pub(crate) fn serie_c_progress_code(x: u8, um_dois: u8) -> u8 {
    match x {
        1 | 2 => um_dois,
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
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    mineiro_end_of_stage(this, clube, situacao, fase, jogo, true).0
}

/// Comparador: como o do Centro-Oeste.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let (n, l, dest) = mineiro_end_of_stage(this, clube, situacao, fase, jogo, false);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_BRA_REG_MINAS_GERAIS_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, clube, situacao, fase, a4, jogo, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("bra_reg_minas_gerais_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

/// The Minas Gerais state league at career creation: the year and the database competition, disciplinary rule 7 and,
/// with no saved state, importance 15, two halves and the final (three stages besides the league), the rules, the table
/// with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegMinasGerais::BraRegMinasGerais` (0x433d50, bra_reg_minas_gerais.cpp, thiscall, ret 8).
///
/// [pt] { 0x433d50 } (thiscall, ret 8, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::BraRegMinasGerais(ano, registro)`: o Mineiro (três fases além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_minas_gerais(this: usize, ano: u32, registro: usize) -> usize {
    bra_reg_goias_construct_state_league(this, ano, registro, &CQT2_MINEIRO, 7, true, "cqt_2 mineiro novo") // cqt2: a disciplina e as posições previstas do Mineiro
}

/// Comparer for `construct_bra_reg_minas_gerais` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_minas_gerais(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_MINAS_GERAIS);
    bra_reg_goias_compare_state_league_ctor("construct_bra_reg_minas_gerais", this, ano, registro, &CQT2_MINEIRO, || o(this, ano, registro))
}

/// the Minas Gerais league has 12 clubs: those with it as their reserve division in the database (the field that holds
/// the state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegMinasGerais::create_table` (0x434810, bra_reg_minas_gerais.cpp, thiscall, ret 0).
///
/// [pt] { 0x434810 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::create_table()`: o Mineiro tem 12 clubes (divisão reserva +0x60 = [0x9cf9d4]).
/// Fonte 0x98aed0, linha 0x169.
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_create_table(this: usize) -> u32 {
    bra_reg_goias_create_state_league_table(this, 0xc, 0x98aed0, 0x169, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, 0, "cql_9 mineiro cheio")
}

/// Comparer for `bra_reg_minas_gerais_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_MINAS_GERAIS_CREATE_TABLE);
    bra_reg_goias_compare_state_league_table("bra_reg_minas_gerais_create_table", this, 0xc, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, || o(this))
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
    create_half_season_stage(this, k, 0x98aed0, 0x3d5, 0xb, 0xc2, "cqr_1 turno do mineiro") // cqr1: os pontos por vitória no Mineiro
}

/// Comparer for `bra_reg_minas_gerais_create_half_season_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_create_half_season_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_BRA_REG_MINAS_GERAIS_CREATE_HALF_SEASON_STAGE);
    compare_half_season_stage("bra_reg_minas_gerais_create_half_season_stage", this, k, || o(this, k))
}

/// The Minas Gerais state league turnover: archives the season in the statistics, clears the table, the schedule and
/// the stages (the two halves and the final) and prepares three new stage slots; moves to the next year and rebuilds
/// the rules, the table with the twelve state clubs, the fixtures and the order; then the predicted positions and the
/// referee renewal. The halves are only built at each one's first game (cqr_1)
/// Original: `BraRegMinasGerais::start_new_season` (0x434910, bra_reg_minas_gerais.cpp, thiscall, ret 0).
///
/// [pt] { 0x434910 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::start_new_season()`, a virada do Mineiro (a tabela: 0x434810).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_start_new_season(this: usize) -> u32 {
    start_new_season_two_halves(this, 0x434810, 0x98aed0, 1, "cqr_2 virada do mineiro") // cqr2: o ano seguinte do Mineiro
}

/// Comparer for `bra_reg_minas_gerais_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_MINAS_GERAIS_START_NEW_SEASON);
    compare_new_season("bra_reg_minas_gerais_start_new_season", this, || o(this))
}
