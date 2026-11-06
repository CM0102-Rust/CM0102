// ---------- %%LEVA%%a leva: o Mineiro: a fase seguinte, a final entre os campeões dos dois turnos e o fim da temporada (bra_reg_minas_gerais.cpp) ----------

unsafe fn cqo7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqo7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O campeão de um turno: o clube da primeira linha da tabela (+0xb1) da fase `k` ([+0xc][k], um League_Stage).
unsafe fn cqo7_campeao_do_turno(this: usize, k: usize) -> usize {
    cqo7_d(cqo7_d(cqo7_d(cqo7_d(this + 0xc) + 4 * k) + 0xb1))
}

/// { 0x434ae0 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::advance_stage()`, a fase seguinte do Mineiro (dois turnos, as fases 0
/// e 1, cada um um League_Stage, e a final, a fase 2). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa de
/// erro da linha 0x202 e nada mais. Senão +0x30 sobe um e, ao chegar a 2 (o fim do segundo turno), monta a final
/// (0x434b70, call direto com o ecx da liga).
unsafe extern "thiscall" fn cqo7_fase_seguinte_do_mineiro(this: usize) {
    let fase = cqo7_d(this + 0x30) as i32;
    if fase >= (cqo7_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0x98aed0, 0x202);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 2 {
        let final_: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x434b70usize);
        final_(this); // cqo7: a final depois do segundo turno
    }
}

/// { 0x434b70 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::create_final_stage()`, da fase seguinte quando o segundo turno
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
unsafe extern "thiscall" fn cqo7_final_do_mineiro(this: usize) {
    let sementes: [u8; 4] = [0, 1, 0, 0];
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let lista = alocar(8);
    if lista == 0 {
        return aviso_em(0x98aed0, 0x226);
    }
    let a = cqo7_campeao_do_turno(this, 0);
    core::ptr::write_unaligned(lista as *mut usize, a);
    let b = cqo7_campeao_do_turno(this, 1);
    core::ptr::write_unaligned((lista + 4) as *mut usize, b);
    if a == b {
        crate::comparacao::ramo("cqo_7 mesmo campeao");
        core::ptr::write_unaligned((this + 0x30) as *mut u32, 1);
        core::ptr::write_unaligned((this + 0x2c) as *mut u32, 2);
        for k in 0..2 {
            *((cqo7_d(cqo7_d(cqo7_d(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *mut u8) = 5; // cqo7: campeão dos dois turnos
        }
        let mut i: i16 = 0;
        while i < cqo7_w(this + 0x3e) {
            let linha = cqo7_d(this + 0xb1) + i as usize * 0x3b;
            if cqo7_d(linha) == cqo7_d(lista) {
                *((linha + 0x37) as *mut u8) = 5;
            }
            i += 1;
        }
        // ponytail: a lista de 8 bytes fica sem liberar, como na original.
        return;
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqo7_d(cqo7_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 2, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no dx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 2, lista, rodadas as u16 as u32, cqo7_d(this + 4), t, cqo7_w(this + 0x40) as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 1, 0, sementes.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned((cqo7_d(this + 0xc) + 8) as *mut usize, fase);
    let fase = cqo7_d(cqo7_d(this + 0xc) + 8);
    if fase == 0 {
        return aviso_em(0x98aed0, 0x250);
    }
    crate::comparacao::ramo("cqo_7 final");
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 2);
}

/// { 0x434ed0 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::record_season_honours()` (slot 13), o fim da temporada do Mineiro.
/// - Com final (+0x30 = 2): o histórico da final (o slot 13, +0x30, da fase [+0xc]+8) e, em salto final, a 0x42bba0
///   (`bra_nat_first_fill_champions_cup_entrants`, cdecl sem argumentos: quando os doze estaduais terminam, põe os
///   campeões na Copa dos Campeões); devolve o eax dela.
/// - Sem final (o mesmo campeão nos dois turnos): o histórico da liga (League::record_season_honours 0x684640,
///   thiscall, ret 0); os prêmios, 350.000 ao campeão (primeira linha da tabela +0xb1) e 100.000 ao vice (segunda linha),
///   cada um no saldo (0x5a0590 em [0xae2a38] -> as finanças do clube, thiscall, ret 4; 0x5999a0 Finance::add_prize_money,
///   thiscall, ret 4) e na notícia do prêmio (0x48e3e0 Comp::send_prize_money_news: thiscall(this, clube, valor em 64
///   bits, posição 1 ou 2, rodada 0, partida 0, tipo −2), ret 0x1c); depois a 0x42bba0, como acima.
unsafe extern "thiscall" fn cqo7_fim_da_temporada_do_mineiro(this: usize) -> u32 {
    let campeoes: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x42bba0usize);
    if cqo7_d(this + 0x30) == 2 {
        let f = cqo7_d(cqo7_d(this + 0xc) + 8);
        let historico: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqo7_d(cqo7_d(f) + 0x30));
        historico(f);
        crate::comparacao::ramo("cqo_7 historico da final");
        return campeoes();
    }
    let historico: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684640usize);
    historico(this);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let premio: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5999a0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, i32) = core::mem::transmute(0x48e3e0usize);
    for (linha, valor, posicao) in [(0usize, 350000i32, 1i32), (0x3b, 100000, 2)] {
        let c = cqo7_d(cqo7_d(this + 0xb1) + linha);
        premio(financas(cqo7_d(0xae2a38), c), valor);
        noticia(this, cqo7_d(cqo7_d(this + 0xb1) + linha), valor, 0, posicao, 0, 0, -2);
    }
    crate::comparacao::ramo("cqo_7 premios");
    campeoes()
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 2 monta a final, que aloca e sorteia; não pode
/// rodar duas vezes), conferindo o plano: na última fase, +0x30 igual; senão +0x30 + 1, ou, ao chegar a 2, 2 (com
/// final) ou 1 (o mesmo campeão nos dois turnos, que a final grava).
unsafe extern "thiscall" fn comparar_cqo7_fase_seguinte_do_mineiro(this: usize) {
    let antes = cqo7_d(this + 0x30) as i32;
    let ultima = antes >= (cqo7_d(this + 0x2c) as i32).wrapping_sub(1);
    orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let depois = cqo7_d(this + 0x30) as i32;
    let igual = if ultima { depois == antes } else if antes + 1 == 2 { depois == 2 || depois == 1 } else { depois == antes + 1 };
    PLACAR.registrar(igual, || format!("cqo7_fase_seguinte_do_mineiro({:#x}) antes={} depois={}", this, antes, depois));
}

/// O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), conferindo o plano feito
/// antes: os campeões dos turnos. O mesmo nos dois: +0x30 = 1, +0x2c = 2 e a situação 5 na primeira linha de cada turno
/// e na linha dele da tabela da liga. Diferentes: a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os
/// dois campeões na lista dela (+0xa7, 6 bytes; como conjunto: o sorteio da rodada pode reordenar), e +0x30 = 2.
unsafe extern "thiscall" fn comparar_cqo7_final_do_mineiro(this: usize) {
    let (a, b) = (cqo7_campeao_do_turno(this, 0), cqo7_campeao_do_turno(this, 1));
    orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let igual = if a == b {
        let marca = |k: usize| *((cqo7_d(cqo7_d(cqo7_d(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *const u8) == 5;
        let na_liga = (0..cqo7_w(this + 0x3e).max(0) as usize).all(|i| {
            let linha = cqo7_d(this + 0xb1) + i * 0x3b;
            cqo7_d(linha) != a || *((linha + 0x37) as *const u8) == 5
        });
        cqo7_d(this + 0x30) == 1 && cqo7_d(this + 0x2c) == 2 && marca(0) && marca(1) && na_liga
    } else {
        let fase = cqo7_d(cqo7_d(this + 0xc) + 8);
        if fase == 0 {
            false
        } else {
            let l = cqo7_d(fase + 0xa7);
            let (x, y) = (cqo7_d(l), cqo7_d(l + 6));
            cqo7_w(fase + 0x3e) == 2 && cqo7_d(fase + 4) == cqo7_d(this + 4) && ((x, y) == (a, b) || (x, y) == (b, a)) && cqo7_d(this + 0x30) == 2
        }
    };
    PLACAR.registrar(igual, || format!("cqo7_final_do_mineiro({:#x}) campeoes={:#x},{:#x} +0x30={} +0x2c={}", this, a, b, cqo7_d(this + 0x30), cqo7_d(this + 0x2c)));
}

/// Comparador do fim da temporada: só a original (ponytail: grava o histórico, que aloca o registro, paga os prêmios e
/// manda a notícia, e pode montar a Copa dos Campeões; não pode rodar duas vezes). A cópia roda no modo Rust (contadores
/// "cqo_7 historico da final" e "cqo_7 premios") e é provada pelas mutações e pelas duas carreiras Rust.
unsafe extern "thiscall" fn comparar_cqo7_fim_da_temporada_do_mineiro(this: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this)
}
