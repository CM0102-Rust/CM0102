// ---------- 279a leva: o Mineiro: a fase seguinte, a final entre os campeões dos dois turnos e o fim da temporada (bra_reg_minas_gerais.cpp) ----------

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
    orig::<unsafe extern "thiscall" fn(usize)>(1899)(this);
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
    orig::<unsafe extern "thiscall" fn(usize)>(1900)(this);
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
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(1901)(this)
}

// ---------- 274a leva: o progresso do clube nas estatísticas da competição no Mineiro (Paranaense, Carioca), na MLS e na Série C (bra_reg_minas_gerais.cpp, usa_mls.cpp, bra_nat_third.cpp) ----------

/// 0x4a4850 `Comp_Stats::find_club_entry(clube)` (thiscall, ret 4): a entrada do clube nas estatísticas da competição
/// ([this+8]); ordena a tabela pelo clube quando preciso (marca +0x18) e busca. Original (não portada).
unsafe fn cqk8_entrada(this: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    f(core::ptr::read_unaligned((this + 8) as *const u32) as usize, clube)
}

unsafe fn cqk8_gravar(e: usize, atual: u8, minima: u8, maxima: u8) {
    *((e + 0x73) as *mut u8) = atual;
    *((e + 0x74) as *mut u8) = minima;
    *((e + 0x75) as *mut u8) = maxima;
}

/// { 0x4350d0 } (thiscall, ret 0x14, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::set_club_progress(clube, fase, a,
/// b, c)` (slot 27 do Mineiro, do Paranaense e do Carioca): grava os três códigos de progresso do clube na entrada dele
/// (+0x73, +0x74, +0x75); na fase de pontos corridos (-1), o segundo abaixo de 3 vira 1 e o terceiro abaixo de 3 vira 2.
/// Devolve a entrada.
unsafe extern "thiscall" fn cqk8_progresso_no_mineiro(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = cqk8_entrada(this, clube);
    let (a, mut b, c) = (a as u8, b as u8, c as u8);
    if fase as u8 == 0xff {
        if b < 3 {
            b = 1;
        }
        if c < 3 {
            crate::comparacao::ramo("cqk_8 mineiro terceiro abaixo de 3");
            cqk8_gravar(e, a, b, 2); // cqk8: o terceiro código do Mineiro
            return e;
        }
    }
    crate::comparacao::ramo("cqk_8 mineiro como vem");
    cqk8_gravar(e, a, b, c); // cqk8: os códigos do Mineiro como vêm
    e
}

/// Comparador: lado a lado pela entrada do clube (+0x73..+0x75), achada antes (a 0x4a4850 ordena na primeira vez e só
/// busca depois; a original e a cópia a chamam de novo); a devolução.
unsafe extern "thiscall" fn comparar_cqk8_progresso_no_mineiro(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = cqk8_entrada(this, clube);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(e + 0x73, 3)], || orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(1734)(this, clube, fase, a, b, c), || cqk8_progresso_no_mineiro(this, clube, fase, a, b, c));
    PLACAR.registrar(igual && r == n, || format!("cqk8_progresso_no_mineiro({:x}, {:x}, {}, {}, {}, {}) original={:x} rust={:x} {:?} {:?}", this, clube, fase as u8 as i8, a as u8, b as u8, c as u8, r, n, mo, mr));
    r
}

/// O código da MLS na temporada regular: 1..=3 viram `ate_3`, 4 vira 10, os outros ficam.
fn cqk8_mls(x: u8, ate_3: u8) -> u8 {
    match x {
        1..=3 => ate_3,
        4 => 10,
        _ => x,
    }
}

/// { 0x909dd0 } (thiscall, ret 0x14, usa_mls.cpp): `UsaMls::set_club_progress(clube, fase, a, b, c)`: nas fases 0 e 1
/// e na temporada (-1), a: 1 → 1, 2 → 3, 3 → 7, 4 → 10; b: 1..3 → 1, 4 → 10; c: 1..3 → 7, 4 → 10; na fase 2, cada
/// código vira 2 × código − 1; depois, os códigos como vêm. Devolve a entrada.
unsafe extern "thiscall" fn cqk8_progresso_na_mls(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = cqk8_entrada(this, clube);
    let (fa, a, b, c) = (fase as u8 as i8, a as u8, b as u8, c as u8);
    if fa < 2 {
        let (b, c) = (cqk8_mls(b, 1), cqk8_mls(c, 7));
        let a = match a {
            1 => 1,
            2 => {
                crate::comparacao::ramo("cqk_8 mls segundo");
                3 // cqk8: o código 2 da MLS
            }
            3 => 7,
            4 => 10,
            _ => a,
        };
        cqk8_gravar(e, a, b, c);
    } else if fa == 2 {
        cqk8_gravar(e, a.wrapping_mul(2).wrapping_sub(1), b.wrapping_mul(2).wrapping_sub(1), c.wrapping_mul(2).wrapping_sub(1));
    } else {
        cqk8_gravar(e, a, b, c);
    }
    e
}

/// Comparador: como o do Mineiro.
unsafe extern "thiscall" fn comparar_cqk8_progresso_na_mls(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = cqk8_entrada(this, clube);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(e + 0x73, 3)], || orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(1735)(this, clube, fase, a, b, c), || cqk8_progresso_na_mls(this, clube, fase, a, b, c));
    PLACAR.registrar(igual && r == n, || format!("cqk8_progresso_na_mls({:x}, {:x}, {}, {}, {}, {}) original={:x} rust={:x} {:?} {:?}", this, clube, fase as u8 as i8, a as u8, b as u8, c as u8, r, n, mo, mr));
    r
}

/// O código da Série C nos grupos (fases até 6): 1 e 2 viram `um_dois`, 3 → 0x11, 4 → 0x19, 5 → 0x21, 6 → 0x29,
/// 7 → 0x31 (a posição no grupo levada para a escala de 49 lugares [inferência]); os outros ficam.
fn cqk8_serie_c(x: u8, um_dois: u8) -> u8 {
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

/// { 0x42e100 } (thiscall, ret 0x14, bra_nat_third.cpp): `BraNatThird::set_club_progress(clube, fase, a, b, c)`: nos
/// grupos (fase < 7), a e b: 1, 2 → 1, 3 → 0x11 ... 7 → 0x31; c: 1, 2 → 9, depois igual; nas fases 7 em diante, os
/// códigos como vêm (o trecho das fases 1 a 3 que a original tem depois do teste `fase >= 7` nunca roda). Devolve a
/// entrada.
unsafe extern "thiscall" fn cqk8_progresso_na_serie_c(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = cqk8_entrada(this, clube);
    let (fa, a, b, c) = (fase as u8 as i8, a as u8, b as u8, c as u8);
    if fa < 7 {
        let (a, c) = (cqk8_serie_c(a, 1), cqk8_serie_c(c, 9));
        let b2 = cqk8_serie_c(b, 1);
        if b == 3 {
            crate::comparacao::ramo("cqk_8 serie c segundo 3");
        }
        cqk8_gravar(e, a, b2, c); // cqk8: os códigos dos grupos da Série C
    } else {
        cqk8_gravar(e, a, b, c);
    }
    e
}

/// Comparador: como o do Mineiro.
unsafe extern "thiscall" fn comparar_cqk8_progresso_na_serie_c(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = cqk8_entrada(this, clube);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(e + 0x73, 3)], || orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(1736)(this, clube, fase, a, b, c), || cqk8_progresso_na_serie_c(this, clube, fase, a, b, c));
    PLACAR.registrar(igual && r == n, || format!("cqk8_progresso_na_serie_c({:x}, {:x}, {}, {}, {}, {}) original={:x} rust={:x} {:?} {:?}", this, clube, fase as u8 as i8, a as u8, b as u8, c as u8, r, n, mo, mr));
    r
}

