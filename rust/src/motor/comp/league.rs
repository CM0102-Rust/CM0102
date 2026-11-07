// ---------- octogesima nona leva: decisao pela classificacao do grupo, narracao da classificacao ----------

/// { 0x6854a0 } (ret 8, league.cpp): a fase (this) com a partida (`partida`, +0x42 = 0xff) ja decide
/// o clube? So nas competicoes que contam (0x4b0700), com o bit 1 de +0xd9 e a rodada +0xa9 >= 24:
/// a linha do clube (+0xb1, 0x3b bytes, +0x3e linhas) sem situacao +0x37 de 5, 2, 3, -2, -4 ou -5;
/// numa copia da tabela (0x687710) preenchida (0x685f60) e ordenada (0x6827d0), o clube tem de estar
/// na frente (+4 zero) e nenhum outro alcanca-lo com os jogos que faltam (+0xa7 - +0x11 vezes os
/// pontos por vitoria +0xc2, contra os pontos +0x19). A copia volta (0x687970).
unsafe extern "thiscall" fn grupo_decidido(this: usize, clube: usize, partida: usize) -> u32 {
    if *((partida + 0x42) as *const u8) != 0xff {
        return 0;
    }
    let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    if conta(*(*((this + 4) as *const usize) as *const i32)) == 0 {
        return 0;
    }
    if *((this + 0xd9) as *const u8) & 1 == 0 || *((this + 0xa9) as *const i16) < 24 {
        return 0;
    }
    let linhas = *((this + 0x3e) as *const i16);
    let tabela = *((this + 0xb1) as *const usize);
    let linha = |base: usize, k: i16| base.wrapping_add(k as usize * 0x3b);
    let Some(k) = (0..linhas).find(|&k| *(linha(tabela, k) as *const usize) == clube) else {
        return 0;
    };
    let r = linha(tabela, k);
    if r == 0 || matches!(*((r + 0x37) as *const i8), 5 | 2 | 3 | -2 | -4 | -5) {
        return 0;
    }
    let copia: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x687710usize);
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, u32) = core::mem::transmute(0x685f60usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x6827d0usize);
    let soltar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x687970usize);
    let t = copia(this);
    if t == 0 {
        return 0;
    }
    preencher(this, partida, t, *((this + 0x3e) as *const u16) as i32, 0, 0, 0);
    ordenar(this, t);
    let n = *((this + 0x3e) as *const i16);
    // a original le +4 do endereco 0 se o clube nao estiver na copia (nunca acontece no jogo)
    let achada = (0..n).find(|&k| *(linha(t, k) as *const usize) == clube).map_or(0, |k| linha(t, k));
    if *((achada + 4) as *const u8) != 0 {
        soltar(this, t);
        return 0;
    }
    let rodadas = *((this + 0xa7) as *const i16);
    let pontos = *((this + 0xc2) as *const i8) as i16;
    for k in 1..*((this + 0x3e) as *const i16) {
        let o = linha(t, k);
        let jogos = *((o + 0x11) as *const i8) as i16;
        let alcance = rodadas.wrapping_sub(jogos).wrapping_mul(pontos).wrapping_add(*((o + 0x19) as *const i16));
        if alcance >= *((t + 0x19) as *const i16) && (*((t + 0x11) as *const i8) as i16 != rodadas || jogos != rodadas) {
            soltar(this, t);
            return 0;
        }
    }
    soltar(this, t);
    1
}

/// { 0x6be120 } (ret 4, match_eng.cpp) com o comentario injetado: narra a situacao da
/// classificacao do lado. Grupo decidido (0x6854a0) ou mata-mata ja resolvido (0x6ce310 na rodada
/// 0x82 ou 0x96): 0x2171/0x2170. Com +0x8ebb: 0x2120 se o lado esta na frente do agregado, senao
/// 0x2121 quando a media do +0x1b dos titulares e baixa e a forca (+0x80) do outro clube pesa. No fim,
/// para o lado 1 sem +0x1a8: 0x2001 se o jogo ainda nao estiver decidido.
unsafe fn narrar_classificacao_com(this: usize, lado: u32, chamar: ChamarComentario) {
    let t = || *((this + 0x4792) as *const usize);
    let w = |o: usize| *((t() + o) as *const i16);
    let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    let conta_t = || conta(*(t() as *const i32));
    let objeto = || *((*(0xadadfc as *const usize)).wrapping_add((*(t() as *const i32) as usize).wrapping_mul(4)) as *const usize);
    let decidido: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x6854a0usize);
    let agregado: unsafe extern "thiscall" fn(usize, i8, *mut i8, *mut i8, *mut u8, *mut u8, *mut u8, u8) -> i8 = core::mem::transmute(0x6ce310usize);
    let narrador = || *((this + 0x8f7a) as *const usize);
    let fala = |c: &mut dyn FnMut(usize, [u32; 7]), cod: u32, l: u32| c(narrador(), [cod, this as u32, u32::MAX, l, u32::MAX, u32::MAX, 0]);
    // o placar agregado com a 5a saida (fase3): na original ela cai no proprio argumento
    let resolvido = |l: i8| -> bool {
        let (mut a, mut b, mut c, mut d, mut e) = (0i8, 0i8, 0u8, 0u8, 0u8);
        agregado(this, l, &mut a, &mut b, &mut c, &mut d, &mut e, 0) > 0 && e == 0
    };
    let bl = lado as u8 as i8;
    if w(0x32) == 0 && conta_t() == 0 && w(0x30) == 0x82 && resolvido(bl) {
        fala(chamar, 0x2171, lado);
        return;
    }
    if conta_t() != 0 {
        let clube = *((t() + if bl != 0 { 0x20 } else { 0x1c }) as *const usize);
        if decidido(objeto(), clube, t()) != 0 {
            fala(chamar, 0x2170, lado);
            return;
        }
    } else if w(0x30) == 0x96 && resolvido(bl) {
        fala(chamar, 0x2170, lado);
        return;
    }
    if byte(this, 0x8ebb) != 0 {
        let (mut casa, mut fora, mut classificado, mut x, mut y) = (0i8, 0i8, 0u8, 0u8, 0u8);
        agregado(this, bl, &mut casa, &mut fora, &mut classificado, &mut x, &mut y, 0);
        if casa > fora || (casa == fora && classificado != 0) {
            fala(chamar, 0x2120, lado);
        } else {
            let mut soma = 0.0f64;
            let mut n = 0i8;
            for k in 0..20usize {
                let j = jogador_do_bloco(this, bl, k);
                if k <= 10 || *((j + 0x1f) as *const u8) != 0xff {
                    soma += byte(j, 0x1b) as f64;
                    n = n.wrapping_add(1);
                }
            }
            // sem nenhum titular a media seria o que estivesse no argumento (nao acontece)
            let media = if n != 0 { (soma / n as f64) as f32 as f64 } else { 0.0 };
            let forca = |o: usize| *((*((t() + o) as *const usize) + 0x80) as *const i16) as f64;
            let x = media * f64_em(0x968968) - f64_em(0x968950);
            if bl == 0 {
                if media < f64_em(0x968988) && x * forca(0x20) < forca(0x1c) {
                    fala(chamar, 0x2121, 0);
                }
                return;
            }
            if media < f64_em(0x96d2e8) && x * forca(0x1c) < forca(0x20) * f64_em(0x9689b0) {
                fala(chamar, 0x2121, lado);
            }
        }
    }
    if bl != 1 || byte(this, 0x1a8) != 0 {
        return;
    }
    if w(0x32) == 0 && conta_t() == 0 && w(0x30) == 0x82 && resolvido(0) {
        return;
    }
    if conta_t() != 0 {
        if decidido(objeto(), *((t() + 0x1c) as *const usize), t()) != 0 {
            return;
        }
    } else if w(0x30) == 0x96 && resolvido(0) {
        return;
    }
    fala(chamar, 0x2001, 0);
}

/// { 0x6be120 } (ret 4, match_eng.cpp): ver `narrar_classificacao_com`.
unsafe extern "thiscall" fn narrar_classificacao(this: usize, lado: u32) {
    narrar_classificacao_com(this, lado, &mut |t, a| comentar(t, a))
}

// ---------- 272a leva: a Liga dos Campeões: a tabela das rodadas das fases de grupos; a liga: as expectativas da temporada e o rebaixamento de um clube (european_cup.cpp, league.cpp) ----------

/// As datas das 6 rodadas das fases de grupos (0x68a160: rodada, dia, mês 0..11, anos a somar, dia da semana; código 2,
/// valor 0), pela metade da fase: [0] primeira fase, grupos 0..3; [1] primeira fase, 4..7; [2] segunda fase, 8..9; [3]
/// segunda fase, 10..11.
const CQH4_DATAS: [[(i32, i32, i32, i32, i32); 6]; 4] = [
    [(0, 0xc, 8, 0, 1), (1, 0x14, 8, 0, 2), (2, 0x1b, 8, 0, 2), (3, 0x11, 9, 0, 1), (4, 0x19, 9, 0, 2), (5, 7, 0xa, 0, 1)], // cqh4: a primeira fase, grupos A a D
    [(0, 0xd, 8, 0, 2), (1, 0x13, 8, 0, 1), (2, 0x1a, 8, 0, 1), (3, 0x12, 9, 0, 2), (4, 0x18, 9, 0, 1), (5, 8, 0xa, 0, 2)], // cqh4: a primeira fase, grupos E a H
    [(0, 0x15, 0xa, 0, 1), (1, 6, 0xb, 0, 2), (2, 0xe, 1, 1, 2), (3, 0x14, 1, 1, 1), (4, 7, 2, 1, 2), (5, 0xd, 2, 1, 1)],
    [(0, 0x16, 0xa, 0, 2), (1, 5, 0xb, 0, 1), (2, 0xd, 1, 1, 1), (3, 0x15, 1, 1, 2), (4, 6, 2, 1, 1), (5, 0xe, 2, 1, 2)], // cqh4: a segunda fase, grupos C e D
];

/// { 0x583c50 } (thiscall, ret 0x10, european_cup.cpp, método +0x3c da Liga dos Campeões, quadro SEH pelas datas locais
/// sem destrutor no caminho normal): `EuropeanCup::create_round_table(fase, &rodadas, &id, &é_liga)`, dos sorteios dos
/// grupos (0x585290, 0x585ab0) e das posições previstas (0x586900). Nas fases de grupos (char 0..11, e as negativas
/// abaixo de −1, que caem no mesmo ramo): é_liga (se pedido) = 1, rodadas = 6, id = 0x43d + fase; a tabela de 6 rodadas
/// de 0x41 bytes (0x944e46(0x186); sem memória, a caixa da linha 0x17a ou 0x1aa e devolve 0); na segunda fase, antes, a
/// data de 1º de março do ano seguinte levada à terça (0x549f70), um dia antes (0x54c8e0, que grava a data estática dela)
/// e a cópia (0x417c10), sem uso depois (a cópia as chama do mesmo jeito); as datas de `CQH4_DATAS` (0x68a160 com o ano
/// +0x40 da copa) e o jogo 0 de cada rodada vazio (0x68a1c0 com −1). As preliminares (−1) e o mata-mata (12 em diante):
/// a original pelo trampolim. Devolve a tabela.
unsafe extern "thiscall" fn campeoes_rodadas_da_fase(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let f = fase as u8 as i8;
    if f == -1 || f >= 0xc {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1652)(this, fase, rodadas, id, liga);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(0x68a160usize);
    let jogo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = 6;
    *id = (f as i16).wrapping_add(0x43d);
    let t = alocar(0x186);
    if t == 0 {
        aviso_em(0x9c4900, if f < 8 { 0x17a } else { 0x1aa });
        return 0;
    }
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as u16 as u32;
    let metade = if f < 8 {
        if f < 4 { 0 } else { 1 }
    } else {
        let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
        let menos: unsafe extern "thiscall" fn(*mut [u16; 4], *mut [u16; 4], u32) -> usize = core::mem::transmute(0x54c8e0usize);
        let copia: unsafe extern "thiscall" fn(*mut [u16; 4], *mut [u16; 4], usize) -> usize = core::mem::transmute(0x417c10usize);
        let (mut d, mut t1, mut t2) = ([0u16; 4], [0u16; 4], [0u16; 4]);
        // ponytail: o ano + 1 vai no ax com o resto do eax nos bytes altos; a 0x549f70 lê o ano como word
        construir(&mut d, 1, 2, (ano as u16).wrapping_add(1) as u32, 2);
        let r = menos(&mut d, &mut t1, 1);
        copia(&mut d, &mut t2, r);
        if f < 10 { 2 } else { 3 }
    };
    crate::comparacao::ramo("cqh_4 tabela das rodadas");
    for &(i, dia, mes, anos, semana) in CQH4_DATAS[metade].iter() {
        data(t, i, dia, mes, anos, semana, 2, ano, 0);
    }
    for r in 0..6 {
        jogo(t, r, 0, -1, -1, -1, 0);
    }
    t
}

/// Os bytes que a tabela tem gravados (o resto da memória alocada não é escrito): de cada rodada, +0..+4, +0x3d..+0x40
/// e o jogo 0 (+5..+0xb).
unsafe fn cqh4_gravados(t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..6usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xc));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
    }
    v
}

/// Lado a lado nas fases de grupos: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada). As
/// preliminares e o mata-mata: só a original.
unsafe extern "thiscall" fn comparar_campeoes_rodadas_da_fase(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1652);
    let f = fase as u8 as i8;
    if f == -1 || f >= 0xc {
        return o(this, fase, rodadas, id, liga);
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let (mut r2, mut i2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(
        &reg,
        || o(this, fase, rodadas, id, liga),
        || campeoes_rodadas_da_fase(this, fase, &mut r2, &mut i2, if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }),
    );
    let saidas = *rodadas == r2 && *id == i2 && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { cqh4_gravados(a) == cqh4_gravados(b) } else { a == 0 && b == 0 };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("campeoes_rodadas_da_fase({}): saídas {} tabela {}", f, saidas, tabela));
    a
}



/// { 0x68aa80 } (thiscall, ret 0, league.cpp, método da liga): `League::set_club_season_expectations()`: com o
/// gerenciador dos técnicos ([0xb67a44]), as expectativas da temporada de cada clube da tabela (+0xb1, linhas de 0x3b
/// bytes, +0x3e clubes) pela 0x6a3770 (push 1; push clube; mov ecx, [0xb67a44] -> thiscall, ret 8: com a notícia).
unsafe extern "thiscall" fn liga_expectativas_da_temporada(this: usize) {
    if read_u32_as_usize(0xb67a44) == 0 {
        return;
    }
    let expectativas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a3770usize);
    let mut i = 0i16;
    while i < read_i16(this + 0x3e) {
        crate::comparacao::ramo("cqh_4 expectativa do clube");
        let clube = read_u32_as_usize(read_u32_as_usize(this + 0xb1) + i as usize * 0x3b);
        expectativas(read_u32_as_usize(0xb67a44), clube, 1); // cqh4: com a notícia
        i += 1; // cqh4: o próximo clube
    }
}

/// Só a original (as expectativas mudam a diretoria e mandam notícias; não rodam duas vezes): a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_liga_expectativas_da_temporada(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1653)(this)
}

/// { 0x6831a0 } (thiscall, ret 0xc, league.cpp; this não é lido): `League::relegate_club_to_division(clube, divisão,
/// _)`: a competição do clube (+0x57) vira a divisão e a anterior vai para +0x5b; as cláusulas de rebaixamento (0x4cd030
/// push antiga; push clube; mov ecx, [0xae1050] -> thiscall, ret 8); se a divisão não é jogada (sem a tabela das
/// competições [0xadadfc], sem divisão ou sem a competição do id dela) e o clube tem técnico humano que decide
/// (0x600a50 push 0; push 0; push 1; push 1; push clube; add esp, 0x14 -> cdecl), o técnico sai (0x69b790 push 0; push
/// 7; push 1; push clube; mov ecx, [0xb67a44] -> thiscall, ret 0x10).
unsafe extern "thiscall" fn liga_rebaixar_clube(_this: usize, clube: usize, divisao: usize, _x: u32) {
    let clausulas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cd030usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x69b790usize);
    let antiga = read_u32_as_usize(clube + 0x57);
    core::ptr::write_unaligned((clube + 0x57) as *mut u32, divisao as u32);
    core::ptr::write_unaligned((clube + 0x5b) as *mut u32, antiga as u32);
    clausulas(read_u32_as_usize(0xae1050), clube, antiga);
    let comps = read_u32_as_usize(0xadadfc);
    if comps != 0 && divisao != 0 && read_u32_as_usize(comps + read_u32_as_usize(divisao) * 4) != 0 {
        return;
    }
    if humano(clube, 1, 1, 0, 0) != 0 {
        crate::comparacao::ramo("cqh_4 técnico sai");
        tirar(read_u32_as_usize(0xb67a44), clube, 1, 7, 0);
    }
}

/// Só a original (cláusulas de contrato e o técnico que sai não rodam duas vezes), conferindo o que ela grava: +0x57 =
/// a divisão e +0x5b = a competição de antes.
unsafe extern "thiscall" fn comparar_liga_rebaixar_clube(this: usize, clube: usize, divisao: usize, x: u32) {
    let antiga = read_u32_as_usize(clube + 0x57);
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1654)(this, clube, divisao, x);
    PLACAR.registrar(read_u32_as_usize(clube + 0x57) == divisao && read_u32_as_usize(clube + 0x5b) == antiga, || format!("liga_rebaixar_clube: +0x57 {:#x} +0x5b {:#x}", read_u32_as_usize(clube + 0x57), read_u32_as_usize(clube + 0x5b)));
}

// ---------- 269a leva: a liga: os jogos levados para a TV, a notícia dos grupos e a notícia da tabela (league.cpp) ----------







/// O clube é dirigido por um técnico humano (0x7875e0: push clube; ecx = [0xb7431c] -> thiscall(notícias, clube),
/// ret 4).
unsafe fn cqc6_humano(clube: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    f(read_u32_as_usize(0xb7431c), clube) != 0
}

/// A entrada da tabela das datas da TV da liga (0x41 bytes em [+0xba], conta +0xa9) para a data, como a 0x685640: sem
/// entradas, data depois da última (ano +2 + o da liga +0x40, sem sinal; dia +0 com sinal) ou sem a entrada pela busca
/// (0x946464: push 0x685cf0; push 0x41; push conta; push tabela; push &chave; add esp, 0x14 -> cdecl bsearch; a chave
/// é o dia e o ano menos o da liga) ou com a primeira vaga vazia (+0xb = 0xff): nenhuma.
unsafe fn cqc6_entrada(this: usize, data: usize) -> Option<usize> {
    let n = read_i16(this + 0xa9);
    if n == 0 {
        return None;
    }
    let tab = read_u32_as_usize(this + 0xba);
    let base = read_i16(this + 0x40);
    let ult = (tab as isize + n as isize * 0x41 - 0x41) as usize;
    let ult_ano = read_i16(ult + 2) as u16 as i32 + base as i32;
    let ano = read_i16(data + 2) as u16 as i32;
    if ano > ult_ano || (ano == ult_ano && read_i16(data) > read_i16(ult)) {
        return None;
    }
    let chave = [read_i16(data), read_i16(data + 2).wrapping_sub(base)];
    let busca: unsafe extern "C" fn(*const i16, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let e = busca(chave.as_ptr(), tab, n as i32, 0x41, 0x685cf0);
    if e == 0 || read_u8(e + 0xb) == 0xff {
        return None;
    }
    Some(e)
}


/// { 0x685640 } (thiscall, ret 4, league.cpp, quadro SEH sem destrutor no caminho normal: as datas e a lista locais):
/// `League::move_fixtures_for_tv(data)`, do dia da liga (0x6847c0) e da montagem do calendário (0x683e30). Pela
/// entrada da data (cqc6_entrada), os jogos da liga no dia e fatia da entrada (+4) vão a uma lista local (0x68d020;
/// 0x5abd90: push lista; push liga +0x44; push -1; push id; push fatia; push dia; push ano; mov ecx, [0xae2a58] ->
/// thiscall, ret 0x1c). Com jogos: as vagas da entrada (7 bytes a partir de +5, até a primeira com +0xb = 0xff) são
/// ordenadas (0x945333 com 0x8b4e00) se alguma tem bits 0x7fc0 no dword +5. Para cada vaga com fatia (+0xa) 0..2: a data
/// levada ao dia da semana da vaga (+9; 0x54a2d0); a janela de conflito: 2..5 dias depois da data pedida se o dia da TV
/// cai depois dela, senão 5..2 dias antes (0x54c5b0/0x54c8e0 + 0x417c10). Entre os jogos da lista (do tipo da liga,
/// +0x42), o de maior nota (palavra com sinal; o primeiro no empate): vaga do tipo 3, 100 para jogo de seleção (0x53b900
/// no mandante) no estádio (+0x18 → +0x38 → +0x1f) da nação de um dos dois, senão 50; senão o sorteio (0x90cfc0) da
/// reputação (+0x80) do visitante + 1500 mais o da do mandante, zerada se um dos dois tem jogo na janela (0x5acbe0). O
/// escolhido: fora do tipo 3, a notícia 3 aos dois clubes; sai do calendário (0x5abc30), ganha a data da TV, a fatia e
/// o tipo da vaga, o estádio (0x85c260) e volta ao calendário (0x5aa680); a vaga fica 0xff e o jogo sai da lista
/// (0x68d1f0).
unsafe extern "thiscall" fn liga_jogos_para_a_tv(this: usize, data: usize) {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let (mut a, mut b, mut c) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    // as datas pelos dwords que a original lê da pilha (dia|ano, ano|bissexto)
    let (pa, pb, pc) = (a.as_mut_ptr() as usize, b.as_mut_ptr() as usize, c.as_mut_ptr() as usize);
    padrao(pa);
    padrao(pb);
    padrao(pc);
    let e = match cqc6_entrada(this, data) {
        Some(e) => e,
        None => return,
    };
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32 = core::mem::transmute(0x5abd90usize);
    // ponytail: a original empurra o ano, o dia, a fatia e o tipo em registradores com sobra nos bytes altos; a
    // 0x5abd90 só lê as palavras e os bytes
    let n = jogos(read_u32_as_usize(0xae2a58), read_i16(data + 2) as u16 as u32, read_i16(data) as u16 as u32, read_u8(e + 4) as u32,
        read_u32_as_usize(read_u32_as_usize(this + 4)) as u32, u32::MAX, read_u8(this + 0x44) as u32, l) & 0xffff;
    if n != 0 {
        let mut k = 0usize;
        let mut ordenar = false;
        while k < 8 {
            if read_u8(e + k * 7 + 0xb) == 0xff {
                break;
            }
            if read_u32_as_usize(e + k * 7 + 5) & 0x7fc0 != 0 {
                ordenar = true;
            }
            k += 1;
        }
        if ordenar {
            let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
            qsort(e + 5, k as u32, 7, 0x8b4e00);
        }
        let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
        let ajustar: unsafe extern "thiscall" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x54a2d0usize);
        let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
        let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
        let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
            core::mem::transmute(0x5acbe0usize);
        for k in 0..8usize {
            let v = e + k * 7;
            if read_u8(v + 0xb) == 0xff {
                break;
            }
            let fatia = read_u8(v + 0xa) as i8;
            if !(0..=2).contains(&fatia) {
                continue;
            }
            let mut x = [0u8; 8];
            let px = x.as_mut_ptr() as usize;
            nova(px, read_i16(data) as u16 as u32, read_i16(data + 2) as u16 as u32);
            ajustar(px, read_u8(v + 9) as i8 as i16 as u16 as u32, pa);
            let (xa, aa) = (read_i16(px + 2) as u16, read_i16(pa + 2) as u16);
            let (mut t1, mut t2, mut t3, mut t4) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
            if xa < aa || (xa == aa && read_i16(px) < read_i16(pa)) {
                let r = mais(px, t1.as_mut_ptr() as usize, 2);
                copia(pb, t2.as_mut_ptr() as usize, r);
                let r = mais(px, t3.as_mut_ptr() as usize, 5);
                copia(pc, t4.as_mut_ptr() as usize, r);
            } else {
                let r = menos(px, t1.as_mut_ptr() as usize, 5);
                copia(pb, t2.as_mut_ptr() as usize, r);
                let r = menos(px, t3.as_mut_ptr() as usize, 2);
                copia(pc, t4.as_mut_ptr() as usize, r);
            }
            write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
            let mut melhor = 0usize;
            let mut nota: i16 = -1;
            let mut f = proximo(l);
            while f != 0 {
                if read_u8(this + 0x44) == read_u8(f + 0x42) {
                    let mut s: i32;
                    if read_u8(v + 0xb) == 3 {
                        let est = read_u32_as_usize(f + 0x18);
                        s = 50;
                        if selecao(read_u32_as_usize(f + 0x1c)) != 0 && est != 0 && read_u32_as_usize(est + 0x38) != 0 {
                            let g = read_u32_as_usize(read_u32_as_usize(est + 0x38) + 0x1f);
                            if read_u32_as_usize(read_u32_as_usize(f + 0x1c) + 0x53) == g || read_u32_as_usize(read_u32_as_usize(f + 0x20) + 0x53) == g {
                                s = 100;
                            }
                        }
                    } else {
                        s = sorteio(read_i16(read_u32_as_usize(f + 0x20) + 0x80) as i32 + 0x5dc); // cqc6: o visitante com o bônus
                        s = s.wrapping_add(sorteio(read_i16(read_u32_as_usize(f + 0x1c) + 0x80) as i32));
                    }
                    if read_u8(v + 0xb) != 3 {
                        let janela = |clube: usize| {
                            achar(read_u32_as_usize(0xae2a58), read_u32_as_usize(pb) as u32, read_u32_as_usize(pb + 2) as u32, read_u32_as_usize(pc) as u32, read_u32_as_usize(pc + 2) as u32,
                                read_u32_as_usize(clube) as u32, 0, u32::MAX, 0xffff_fffe, u32::MAX) as u16 != 0
                        };
                        if janela(read_u32_as_usize(f + 0x1c)) || janela(read_u32_as_usize(f + 0x20)) {
                            crate::comparacao::ramo("cqc_6 conflito na janela");
                            s = 0; // cqc6: conflito na janela
                        }
                    }
                    if s as i16 > nota {
                        melhor = f;
                        nota = s as i16;
                    }
                }
                f = proximo(l);
            }
            if melhor == 0 {
                continue;
            }
            let mut buf = [0u8; 0x50];
            let r = buf.as_mut_ptr() as usize;
            core::ptr::copy_nonoverlapping(melhor as *const u8, r as *mut u8, 0x4f);
            if read_u8(v + 0xb) != 3 {
                let mut noticia = [0u32; 0x38];
                let n = noticia.as_mut_ptr() as usize;
                let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
                let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
                let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
                construir(n, 3, 0);
                gravar(n, 0, read_u32_as_usize(r + 0xc) as u32);
                gravar(n, 1, read_u32_as_usize(r + 0x10) as u32);
                gravar(n, 2, read_u32_as_usize(r) as u32);
                gravar(n, 3, read_i16(r + 0x32) as i32 as u32);
                gravar(n, 4, read_i16(r + 0x30) as i32 as u32);
                gravar(n, 5, read_i16(r + 0x2a) as u16 as u32);
                gravar(n, 6, read_i16(r + 0x28) as u16 as u32);
                gravar(n, 7, read_u8(r + 0x3f) as u32);
                gravar(n, 8, read_i16(pa) as i32 as u32);
                gravar(n, 9, read_i16(pa + 2) as u16 as u32);
                gravar(n, 10, read_u8(v + 0xa) as i8 as i32 as u32);
                gravar(n, 11, read_u8(v + 0xb) as i8 as i32 as u32);
                entregar(read_u32_as_usize(0xb7431c), n, read_u32_as_usize(r + 0x1c));
                entregar(read_u32_as_usize(0xb7431c), n, read_u32_as_usize(r + 0x20));
            }
            let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5abc30usize);
            tirar(read_u32_as_usize(0xae2a58), r);
            crate::comparacao::ramo("cqc_6 jogo para a TV");
            write_u16(r + 0x28, read_i16(pa + 2) as u16);
            write_u16(r + 0x2a, read_i16(pa) as u16); // cqc6: o dia da TV
            write_u8(r + 0x3f, read_u8(v + 0xa));
            write_u8(r + 0x3b, read_u8(v + 0xb));
            let estadio: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x85c260usize);
            estadio(read_u32_as_usize(0xdd7ef4), read_u32_as_usize(v + 5), r, read_u32_as_usize(this + 0x24));
            let juntar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
            juntar(read_u32_as_usize(0xae2a58), r, 1);
            write_u8(v + 0xb, 0xff);
            let sair: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
            sair(l, melhor);
        }
    }
    destruir(l);
}

/// Comparador da 0x685640: só a original (os jogos saem do calendário e voltam pela 0x5aa680, que aloca, as notas
/// sorteiam e as notícias vão aos técnicos). O plano, feito antes: pela entrada da data, os jogos da liga no dia e fatia
/// dela (0x5abd90 numa lista provisória) e as vagas válidas (fatia 0..2) antes da primeira vazia; cada vaga válida leva
/// um jogo enquanto a lista tiver (toda nota passa do −1 inicial), e a vaga fica 0xff: as vagas 0xff do bloco depois da
/// original são min(válidas, jogos).
unsafe extern "thiscall" fn comparar_liga_jogos_para_a_tv(this: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1595);
    let e = match cqc6_entrada(this, data) {
        Some(e) => e,
        None => return o(this, data),
    };
    let mut bloco = 0usize;
    let mut validas = 0usize;
    while bloco < 8 && read_u8(e + bloco * 7 + 0xb) != 0xff {
        if (0..=2).contains(&(read_u8(e + bloco * 7 + 0xa) as i8)) {
            validas += 1;
        }
        bloco += 1;
    }
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32 = core::mem::transmute(0x5abd90usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    let n = jogos(read_u32_as_usize(0xae2a58), read_i16(data + 2) as u16 as u32, read_i16(data) as u16 as u32, read_u8(e + 4) as u32,
        read_u32_as_usize(read_u32_as_usize(this + 4)) as u32, u32::MAX, read_u8(this + 0x44) as u32, l) as usize & 0xffff;
    destruir(l);
    o(this, data);
    let esperado = if n == 0 { 0 } else { validas.min(n) };
    let movidas = (0..bloco).filter(|k| read_u8(e + k * 7 + 0xb) == 0xff).count();
    PLACAR.registrar(movidas == esperado, || format!("liga_jogos_para_a_tv(data {:x}) vagas {} válidas {} jogos {}: movidas {} (esperado {})",
        data, bloco, validas, n, movidas, esperado));
}

/// { 0x684230 } (thiscall, ret 0, league.cpp, quadro SEH sem destrutor no caminho normal): `League::send_group_draw_news()`,
/// do sorteio dos grupos das competições. Grupo de até 8 times (+0x3e; os times de 0x3b bytes em [+0xb1], +0 o clube):
/// para cada clube de técnico humano, a notícia 0x15 (alocada uma vez, 0x944cf1(0xdf), montada por 0x77bbe0, liberada
/// no fim por 0x944c94 sem destrutor) com o id da competição, +0x3a, a primeira data (a da primeira entrada da tabela
/// [+0xba] levada ao dia da semana de cada vaga, 0x54a2d0, a menor), o número de adversários; por clube, o id dele (2)
/// e os dos outros (6 em diante), entregue a ele (0x7861e0).
unsafe extern "thiscall" fn liga_noticia_dos_grupos(this: usize) {
    let n = read_i16(this + 0x3e);
    if n > 8 || n <= 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let time = |i: u8| read_u32_as_usize(read_u32_as_usize(this + 0xb1) + (i as i8 as isize * 0x3b) as usize);
    let mut noticia = 0usize;
    let mut i: u8 = 0;
    loop {
        if cqc6_humano(time(i)) {
            if noticia == 0 {
                let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
                let p = alocar(0xdf);
                noticia = if p != 0 { construir(p, 0x15, 0) } else { 0 };
                let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
                let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
                let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
                let ajustar: unsafe extern "thiscall" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x54a2d0usize);
                let (mut d1, mut d2, mut d3, mut t) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
                let (p1, p2, p3) = (d1.as_mut_ptr() as usize, d2.as_mut_ptr() as usize, d3.as_mut_ptr() as usize);
                let tab = read_u32_as_usize(this + 0xba);
                // ponytail: o dia e o ano vão em registradores com sobra nos 16 bits altos; a 0x549ef0 só lê as palavras
                nova(p1, read_i16(tab) as u16 as u32, read_i16(tab + 2).wrapping_add(read_i16(this + 0x40)) as u16 as u32);
                padrao(p2);
                padrao(p3);
                copia(p2, t.as_mut_ptr() as usize, p1);
                for j in 0..8usize {
                    let v = read_u32_as_usize(this + 0xba) + j * 7;
                    if read_u8(v + 0xb) == 0xff {
                        break;
                    }
                    ajustar(p1, read_u8(v + 9) as i8 as i16 as u16 as u32, p3);
                    let (y2, y3) = (read_i16(p2 + 2) as u16, read_i16(p3 + 2) as u16);
                    if y2 > y3 || (y2 == y3 && read_i16(p2) > read_i16(p3)) {
                        let mut t2 = [0u8; 8];
                        copia(p2, t2.as_mut_ptr() as usize, p3);
                    }
                }
                gravar(noticia, 0, read_u32_as_usize(read_u32_as_usize(this + 4)) as u32);
                gravar(noticia, 1, read_i16(this + 0x3a) as i32 as u32);
                gravar(noticia, 3, read_i16(p2) as i32 as u32);
                gravar(noticia, 4, read_i16(p2 + 2) as u16 as u32);
                gravar(noticia, 5, (read_i16(this + 0x3e) as i32 - 1) as u32);
            }
            gravar(noticia, 2, read_u32_as_usize(time(i)) as u32);
            let mut k: u8 = 6;
            let mut j: u8 = 0;
            while (j as i8 as i16) < read_i16(this + 0x3e) {
                if j != i {
                    gravar(noticia, k as i8, read_u32_as_usize(time(j)) as u32);
                    k = k.wrapping_add(1);
                }
                j = j.wrapping_add(1);
            }
            crate::comparacao::ramo("cqc_6 noticia do grupo");
            entregar(read_u32_as_usize(0xb7431c), noticia, time(i));
        }
        i = i.wrapping_add(1);
        if (i as i8 as i16) >= read_i16(this + 0x3e) {
            break;
        }
    }
    if noticia != 0 {
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        liberar(noticia);
    }
}

/// Comparador da 0x684230: só a original (aloca a notícia e a entrega aos técnicos).
unsafe extern "thiscall" fn comparar_liga_noticia_dos_grupos(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1596)(this) // ponytail: só a notícia
}

/// { 0x683ee0 } (thiscall, ret 4, league.cpp, quadro SEH sem destrutor no caminho normal: as listas locais):
/// `League::send_fixture_list_news(lista)`, da criação do calendário (0x5cd1e0) e do dia da liga no ano dela. Usa a
/// lista local (0x68d020) se a liga é de primeira fase (+0x44 = 0xff) ou sem lista. Com a competição contando
/// (0x4b0700: push id; add esp, 4 -> cdecl) e de primeira fase: a data da primeira entrada da tabela [+0xba] menos 5 dias
/// (0x54c8e0, 0x417c10); para cada clube de técnico humano (times de 0x3b bytes em [+0xb1]) que não esteja na lista (ids
/// de 4 bytes): os primeiros 3 jogos dele a partir da data (0x5acee0 numa lista local, 18 argumentos: push 1; push -1;
/// push +0x44; push 0; push lista; push 3; push -1 ×4; push id do clube; push -1; push -1; push id da competição; push 1;
/// push &fatia; push &dia; push &ano; mov ecx, [0xae2a58] -> thiscall, ret 0x48); achando 3: a notícia 0xd (id da
/// competição, do clube, o ano da liga e, por jogo, o dia, o ano, o adversário e 1 em casa ou 0 fora) ao clube, que
/// entra na lista (0x68d0d0: push 4; push clube; mov ecx, lista -> thiscall, ret 8). Depois, nas fases ([+0xc], até
/// +0x30) com +0x43 = 2, a mesma rotina com a lista.
unsafe extern "thiscall" fn liga_noticia_da_tabela(this: usize, lista: usize) {
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut l0 = [0u8; 0x10];
    let p0 = l0.as_mut_ptr() as usize;
    criar(p0);
    let lista = if read_u8(this + 0x44) == 0xff || lista == 0 { p0 } else { lista };
    let conta: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x4b0700usize);
    if conta(read_u32_as_usize(read_u32_as_usize(this + 4)) as u32) == 0 || read_u8(this + 0x44) != 0xff {
        destruir(p0);
        return;
    }
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let (mut d1, mut t, mut t2) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let p1 = d1.as_mut_ptr() as usize;
    let tab = read_u32_as_usize(this + 0xba);
    // ponytail: o dia e o ano vão em registradores com sobra nos 16 bits altos; a 0x549ef0 só lê as palavras
    nova(p1, read_i16(tab) as u16 as u32, read_i16(tab + 2).wrapping_add(read_i16(this + 0x40)) as u16 as u32);
    let r = menos(p1, t.as_mut_ptr() as usize, 5);
    copia(p1, t2.as_mut_ptr() as usize, r);
    let time = |i: u32| read_u32_as_usize(read_u32_as_usize(this + 0xb1) + (i as i16 as isize * 0x3b) as usize);
    let mut i: u32 = 0;
    if read_i16(this + 0x3e) > 0 {
        loop {
            if cqc6_humano(time(i)) {
                write_u32_from_usize(lista + 0xa, read_u32_as_usize(lista + 2));
                let mut ja = false;
                let mut it = proximo(lista);
                while it != 0 {
                    if read_u32_as_usize(it) == read_u32_as_usize(time(i)) {
                        ja = true;
                        break;
                    }
                    it = proximo(lista);
                }
                if !ja {
                    let mut v_ano = read_u32_as_usize(p1 + 2) as u32;
                    let mut v_dia = read_u32_as_usize(p1) as u32;
                    let mut fatia: u8 = 0;
                    let mut l2 = [0u8; 0x10];
                    let p2 = l2.as_mut_ptr() as usize;
                    criar(p2);
                    let buscar: unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, u32, i32, i32, u32, i32, i32, i32,
                                                            i32, u32, usize, u32, u32, i32, u32) -> u32 =
                        core::mem::transmute(0x5acee0usize);
                    let achou = buscar(read_u32_as_usize(0xae2a58), &mut v_ano, &mut v_dia, &mut fatia, 1, read_u32_as_usize(read_u32_as_usize(this + 4)) as u32, -1, -1,
                        read_u32_as_usize(time(i)) as u32, -1, -1, -1, -1, 3, p2, 0, read_u8(this + 0x44) as u32, -1, 1) as u16;
                    if achou == 3 {
                        crate::comparacao::ramo("cqc_6 noticia da tabela");
                        let mut noticia = [0u32; 0x38];
                        let n = noticia.as_mut_ptr() as usize;
                        let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
                        let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
                        let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
                        let guardar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
                        construir(n, 0xd, 0);
                        let mut k: u8 = 3;
                        gravar(n, 0, read_u32_as_usize(read_u32_as_usize(this + 4)) as u32);
                        gravar(n, 1, read_u32_as_usize(time(i)) as u32);
                        gravar(n, 2, read_i16(this + 0x40) as i32 as u32);
                        write_u32_from_usize(p2 + 0xa, read_u32_as_usize(p2 + 2));
                        let mut f = proximo(p2);
                        while f != 0 {
                            gravar(n, k as i8, read_i16(f + 0x2a) as u16 as u32);
                            k = k.wrapping_add(1);
                            gravar(n, k as i8, read_i16(f + 0x28) as u16 as u32);
                            k = k.wrapping_add(1);
                            if time(i) == read_u32_as_usize(f + 0x1c) {
                                gravar(n, k as i8, read_u32_as_usize(read_u32_as_usize(f + 0x20)) as u32);
                                k = k.wrapping_add(1);
                                gravar(n, k as i8, 1);
                            } else {
                                gravar(n, k as i8, read_u32_as_usize(read_u32_as_usize(f + 0x1c)) as u32);
                                k = k.wrapping_add(1);
                                gravar(n, k as i8, 0);
                            }
                            k = k.wrapping_add(1);
                            f = proximo(p2);
                        }
                        entregar(read_u32_as_usize(0xb7431c), n, time(i));
                        guardar(lista, time(i), 4);
                    }
                    destruir(p2);
                }
            }
            i = i.wrapping_add(1);
            if (i as i16) >= read_i16(this + 0x3e) {
                break;
            }
        }
    }
    if read_u8(this + 0x44) == 0xff && (read_u32_as_usize(this + 0x30) as i32) >= 0 {
        let mut j: u8 = 0;
        loop {
            let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + (j as i8 as isize * 4) as usize);
            if read_u8(fase + 0x43) == 2 {
                let mesma: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x683ee0usize);
                mesma(fase, lista);
            }
            j = j.wrapping_add(1);
            if j as i8 as i32 > read_u32_as_usize(this + 0x30) as i32 {
                break;
            }
        }
    }
    destruir(p0);
}

/// Comparador da 0x683ee0: só a original (as notícias vão aos técnicos e a lista dos avisados cresce, alocando).
unsafe extern "thiscall" fn comparar_liga_noticia_da_tabela(this: usize, lista: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1597)(this, lista) // ponytail: só as notícias
}

// ---------- 267a leva: a liga: o dia da liga, a criação dos jogos da temporada e a promoção e o rebaixamento (league.cpp) ----------







/// O método `k` (deslocamento na tabela virtual) do objeto `o`.
unsafe fn cqd5_metodo(o: usize, k: usize) -> usize {
    read_u32_as_usize(read_u32_as_usize(o) + k)
}

/// As duas datas (dia, ano) são a mesma (as palavras, como a original compara).
unsafe fn cqd5_mesma_data(a: usize, b: usize) -> bool {
    read_i16(a + 2) == read_i16(b + 2) && read_i16(a) == read_i16(b)
}

/// { 0x6847c0 } (thiscall, ret 8, league.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `League::update_for_date(data, completo)`, o dia de cada liga (pelas competições, 46522 chamadas até 2002-06-01).
/// Liga parada (+0xb5) ou sem clubes (+0x45 = 0): 0. Senão (devolve 1, no byte):
/// - +0xa7 = -1 com times (+0x3e) > 0: +0xa7 = (times − 1) · turnos (+0x3c), na palavra;
/// - na primeira fase (+0x44 = 0xff) com tabela (+0xb1, +0x11 = 0), completo e a data nova (≠ +0xd1): no dia seguinte ao
///   do dia da Copa da nação-chave (0x669470; [0xb63d75 + 72·k] + 1, 0 depois de 364) e no ano da liga (+0x40), a
///   notícia da tabela (0x683ee0: push 0; mov ecx, this -> thiscall, ret 4);
/// - na primeira fase, o dia de cada fase (+0xc, até +0x30; método 0x14 com a data e o completo);
/// - na data +0xc9 (sem +0xdd): +0xad = 0 e o método 0x94;
/// - na primeira fase com tabela e a data nova: o método 0x10 (0, 0); fim da temporada (+0xdd = 1) com todas as fases
///   acabadas (método 0x10 com (0, 1) ≠ 0): a fase seguinte (método 0x28) se +0x30 < +0x2c − 1, senão (uma vez, +0x4c)
///   o histórico (0x442e80: push liga; mov ecx, [0xad9da8] -> thiscall, ret 4), os recordes com 1 (0x447430: push 1;
///   push liga; mov ecx, [0xad9dac] -> thiscall, ret 8) se a competição conta (0x4b0700: push id; add esp, 4 -> cdecl),
///   a caixa da linha 0x74c se +0xe6, o método 0x30 e o fim do anexo +8 (0x4a19f0: mov ecx, [+8] -> thiscall, ret 0);
///   depois os recordes com 0 e o anexo (0x4a2450: mov ecx, [+8] -> thiscall, ret 0);
/// - com tabela e completo, sem +0xdd: os jogos para a TV quatro semanas adiante (0x54c770 + 0x685640: push &data; mov
///   ecx, this -> thiscall, ret 4);
/// - completo: na primeira fase com inscrições (+0x52 bit 2), a data nova, sem +0xdd e tabela em andamento: se a primeira
///   data da tabela da TV (+0xba; dia, ano − o da liga) cai daqui a 5 dias (Argentina [0x9cf218]), 20 (Itália
///   [0x9cf360]) ou 14 dias, o prazo das inscrições (0x859e20: push 0; push 0; push liga; mov ecx, [0xdd7ef0] ->
///   thiscall, ret 0xc); e +0xd1 fica com a data (0x417c10).
unsafe extern "thiscall" fn liga_dia(this: usize, data: usize, completo: u32) -> u32 {
    let semanas: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c770usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let chave: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669470usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x683ee0usize);
    let historico: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x442e80usize);
    let conta: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x4b0700usize);
    let recordes: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x447430usize);
    let fim_anexo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4a19f0usize);
    let anexo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4a2450usize);
    let tv: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x685640usize);
    let prazo: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x859e20usize);
    // a data pedida copiada no começo (a original guarda o dword do dia|ano e o do bissexto)
    let d0 = [read_u32_as_usize(data) as u32, read_u32_as_usize(data + 4) as u32];
    let pd0 = d0.as_ptr() as usize;
    let (mut t, mut x, mut w) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (pt, px, pw) = (t.as_mut_ptr() as usize, x.as_mut_ptr() as usize, w.as_mut_ptr() as usize);
    if read_u8(this + 0xb5) != 0 || read_u32_as_usize(this + 0x45) == 0 {
        return 0;
    }
    if read_i16(this + 0xa7) == -1 {
        let n = read_i16(this + 0x3e);
        if n > 0 {
            write_u16(this + 0xa7, n.wrapping_sub(1).wrapping_mul(read_i16(this + 0x3c)) as u16);
        }
    }
    let tab = read_u32_as_usize(this + 0xb1);
    if tab != 0 && read_u8(tab + 0x11) == 0 && read_u8(this + 0x44) == 0xff && completo != 0 && !cqd5_mesma_data(this + 0xd1, data) {
        let c = read_u32_as_usize(this + 4);
        if read_u32_as_usize(c + 0x5d) != 0 {
            let k = chave(c) as i8 as isize;
            let mut dia = (read_i16((0xb63d75 + k * 72) as usize) as u16).wrapping_add(1);
            if dia > 0x16c {
                dia = 0;
            }
            if dia as i32 == read_i16(0xae2c90) as i32 && read_i16(this + 0x40) as i32 == read_i16(0xae2c92) as u16 as i32 {
                noticia(this, 0);
            }
        }
    }
    if read_u8(this + 0x44) == 0xff {
        let mut i = 0i32;
        while i <= read_u32_as_usize(this + 0x30) as i32 {
            let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + i as usize * 4);
            let f: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(cqd5_metodo(fase, 0x14));
            f(fase, data, completo);
            i += 1;
        }
    }
    if read_i16(this + 0xc9 + 2) == read_i16(0xae2c92) && read_i16(this + 0xc9) == read_i16(0xae2c90) && read_u32_as_usize(this + 0xdd) == 0 {
        write_u32_from_usize(this + 0xad, 0);
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqd5_metodo(this, 0x94));
        f(this);
    }
    let mut seguir_tv = true;
    if read_u8(this + 0x44) == 0xff {
        if read_u32_as_usize(this + 0xb1) == 0 {
            seguir_tv = false;
        } else if !cqd5_mesma_data(this + 0xd1, data) {
            let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(cqd5_metodo(this, 0x10));
            f(this, 0, 0);
            if read_u32_as_usize(this + 0xdd) == 1 {
                let mut acabadas = true;
                let mut i = 0i32;
                while i <= read_u32_as_usize(this + 0x30) as i32 {
                    let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + i as usize * 4);
                    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(cqd5_metodo(fase, 0x10));
                    if g(fase, 0, 1) as u8 == 0 {
                        acabadas = false;
                        break;
                    }
                    i += 1;
                }
                if acabadas {
                    if (read_u32_as_usize(this + 0x30) as i32) < (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
                        let h: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqd5_metodo(this, 0x28));
                        h(this);
                    } else if read_u32_as_usize(this + 0x4c) == 0 {
                        historico(read_u32_as_usize(0xad9da8), this);
                        if conta(read_u32_as_usize(read_u32_as_usize(this + 4)) as u32) != 0 {
                            recordes(read_u32_as_usize(0xad9dac), this, 1);
                        }
                        if read_u32_as_usize(this + 0xe6) != 0 {
                            aviso_em(0x9f15f4, 0x74c);
                        }
                        let h: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqd5_metodo(this, 0x30));
                        h(this);
                        if read_u32_as_usize(this + 8) != 0 {
                            fim_anexo(read_u32_as_usize(this + 8));
                        }
                        write_u32_from_usize(this + 0x4c, 1);
                    }
                }
            }
            if conta(read_u32_as_usize(read_u32_as_usize(this + 4)) as u32) != 0 {
                recordes(read_u32_as_usize(0xad9dac), this, 0);
            }
            if read_u32_as_usize(this + 8) != 0 {
                anexo(read_u32_as_usize(this + 8));
            }
        }
    }
    if seguir_tv && read_u32_as_usize(this + 0xb1) != 0 {
        if completo == 0 {
            return 1;
        }
        if read_u32_as_usize(this + 0xdd) == 0 {
            semanas(pd0, px, 4); // cqd5: a TV quatro semanas adiante
            crate::comparacao::ramo("cqd_5 jogos para a tv");
            tv(this, px);
        }
    }
    if completo == 0 {
        return 1;
    }
    if read_u8(this + 0x52) & 2 != 0 && read_u8(this + 0x44) == 0xff && !cqd5_mesma_data(this + 0xd1, data)
        && read_u32_as_usize(this + 0xdd) == 0 && read_u8(read_u32_as_usize(this + 0xb1) + 0x11) == 0 {
        let e = read_u32_as_usize(this + 0xba);
        // ponytail: a original empurra o dia e o ano em registradores com sobra nos bytes altos; a 0x549ef0 só lê as palavras
        nova(pt, read_i16(e) as u16 as u32, read_i16(e + 2).wrapping_add(read_i16(this + 0x40)) as u16 as u32);
        let c = read_u32_as_usize(this + 4);
        let nacao = if c != 0 && read_u32_as_usize(c + 0x5d) != 0 { read_u32_as_usize(read_u32_as_usize(c + 0x5d)) } else { 0 };
        let tem = c != 0 && read_u32_as_usize(c + 0x5d) != 0;
        let dias = if tem && nacao == read_u32_as_usize(0x9cf218) {
            5
        } else if tem && nacao == read_u32_as_usize(0x9cf360) {
            0x14
        } else {
            0xe
        };
        let r = mais(pd0, pw, dias);
        if cqd5_mesma_data(pt, r) {
            prazo(read_u32_as_usize(0xdd7ef0), this, 0, 0);
        }
    }
    copia(this + 0xd1, pw, data);
    1
}

/// O comparador do dia da liga: só a original (ponytail: chama os métodos de cada fase e da liga, as notícias, os
/// recordes e os jogos para a TV, que mexem no calendário: não roda duas vezes). Confere o que a rotina garante: 0 só
/// com a liga parada ou sem clubes e, completo, +0xd1 com a data.
unsafe extern "thiscall" fn comparar_liga_dia(this: usize, data: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1552);
    let parada = read_u8(this + 0xb5) != 0 || read_u32_as_usize(this + 0x45) == 0;
    let r = o(this, data, completo);
    let ok = if parada { r as u8 == 0 } else { r as u8 == 1 && (completo == 0 || cqd5_mesma_data(this + 0xd1, data)) };
    PLACAR.registrar(ok, || format!("liga_dia(completo {}): parada {} devolveu {} +0xd1 {:x} data {:x}",
        completo, parada, r as u8, read_u32_as_usize(this + 0xd1), read_u32_as_usize(data)));
    r
}

/// Um jogo da tabela da liga no registro `r` (0x4f bytes) e no calendário: os clubes das entradas `ea` (mandante) e
/// `eb` (0x3b bytes na tabela +0xb1: +0 clube, +0x38 marca), as marcas +0x40/+0x41 (marca 0), os ids +0xc/+0x10 (-1 sem
/// clube), o estádio (0x85c260: push +0x24; push &jogo; push [+0x3d da data]; mov ecx, [0xdd7ef4] -> thiscall, ret
/// 0xc), a data (+0x28 ano + o da liga, +0x2a dia) e a fatia (+0x3f) da entrada `off` da tabela das datas (+0xba), os
/// bits 0..1 de +0x4e zerados e o bit 3 na última rodada (+0x34 = +0xa9 − 1; ou com `tipo4`, data do tipo 4), e o jogo
/// vai ao calendário (0x5aa680: push 1; push &jogo; mov ecx, [0xae2a58] -> thiscall, ret 8).
unsafe fn cqd5_jogo(this: usize, r: usize, ea: usize, eb: usize, off: usize, tipo4: bool) {
    let estadio: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(0x85c260usize);
    let calendario: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
    let (ea, eb) = (ea, eb); // cqd5: o mandante e o visitante
    let (ca, cb) = (read_u32_as_usize(ea), read_u32_as_usize(eb));
    write_u32_from_usize(r + 0x1c, ca);
    write_u32_from_usize(r + 0x20, cb);
    write_u8(r + 0x40, (read_u8(ea + 0x38) == 0) as u8);
    write_u8(r + 0x41, (read_u8(eb + 0x38) == 0) as u8);
    write_u32_from_usize(r + 0xc, if ca != 0 { read_u32_as_usize(ca) } else { u32::MAX as usize });
    write_u32_from_usize(r + 0x10, if cb != 0 { read_u32_as_usize(cb) } else { u32::MAX as usize });
    estadio(read_u32_as_usize(0xdd7ef4), read_u32_as_usize(read_u32_as_usize(this + 0xba) + off + 0x3d) as u32, r, read_u32_as_usize(this + 0x24) as u32);
    let e = read_u32_as_usize(this + 0xba) + off;
    write_u16(r + 0x28, read_i16(e + 2).wrapping_add(read_i16(this + 0x40)) as u16);
    write_u16(r + 0x2a, read_i16(e) as u16);
    write_u8(r + 0x3f, read_u8(e + 4));
    write_u16(r + 0x4d, read_i16(r + 0x4d) as u16 & 0xfcff);
    if read_i16(r + 0x34) as i32 == read_i16(this + 0xa9) as i32 - 1 || (tipo4 && read_u8(e + 0xb) == 4) {
        write_u8(r + 0x4e, read_u8(r + 0x4e) | 8);
    } else {
        write_u16(r + 0x4d, read_i16(r + 0x4d) as u16 & 0xf7ff);
    }
    crate::comparacao::ramo("cqd_5 jogo da tabela");
    calendario(read_u32_as_usize(0xae2a58), r, 1);
}

/// { 0x6835c0 } (thiscall, ret 0, league.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `League::create_fixtures()`, da montagem das competições (44 chamadas até 2002-06-01). Com n = times (+0x3e) +
/// times % 2: +0xdd = 0; a grade de n linhas de n dwords (0x944cf1; sem memória, a caixa da linha 0x3fc ou 0x405 e 0),
/// montada pela 0x6844b0 (push n; push grade -> stdcall, ret 8: o mov ecx, this antes ela não lê) e, fora das
/// competições [0x9cf79c] e [0x9cf7a4], a ordem dos clubes sorteada (0x686a70: push n; mov ecx, this -> thiscall, ret
/// 4). O registro do jogo (0x4f bytes na pilha): +0 id da competição, +8 = -1, +0x14 a competição, +0x24/+0x26 a última
/// data da tabela (+0xba, entrada +0xa9 − 1), +0x2c ano (+0x40), +0x30 = 0, +0x32 (+0x3a), +0x36 (+0xdb), +0x38
/// (+0xab), +0x3a (+0xc4), +0x3b = 0xff, +0x3d/+0x3e = 0, +0x42 fase (+0x44), +0x43..+0x4c = 0xff, e o método 0x7c da
/// liga com ele. Com a lista própria (+0xea, entradas de 0x10 bytes: +8/+0xc os ids dos clubes) do ano da liga: em cada
/// rodada k (+0xa9) os times/2 jogos da lista (os índices pela tabela +0xb1), e a lista é liberada (0x9462bb +
/// 0x944c94). Senão o rodízio: para j = 1..turnos (+0x3c)·(n − 1), a rodada pela 0x68a030 (push +0xd9; push +0xa9;
/// push +0x3c; push +0x3e; push id; push &passo; push rodada; add esp, 0x1c -> cdecl) e a coluna j % (n − 1) (n − 1 no
/// zero): cada time i com adversário v ≠ 0 na grade apaga a vaga do adversário; o mando pela paridade de v > 0, do turno
/// ((j − 1)/(n − 1)) e do ano + fase; com a nação +0x24, o clube dessa nação (+0x69) fica como mandante. Cada jogo vai
/// ao calendário (cqd5_jogo). No fim, a grade é liberada e as datas do tipo 3 da tabela levam os jogos para a TV
/// (0x685640). Devolve 1.
unsafe extern "thiscall" fn liga_criar_tabela(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let montar: unsafe extern "stdcall" fn(usize, i32) -> u32 = core::mem::transmute(0x6844b0usize);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x686a70usize);
    let girar: unsafe extern "C" fn(i32, *mut i8, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x68a030usize);
    let destruir_lista: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(0x9462bbusize);
    let tv: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x685640usize);
    let mut rodada: i32 = -1;
    let times = read_i16(this + 0x3e) as i32;
    let n = times + times % 2;
    let mut passo: i8 = 0;
    let (mut x, mut y, mut z) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (px, py, pz) = (x.as_mut_ptr() as usize, y.as_mut_ptr() as usize, z.as_mut_ptr() as usize);
    padrao(px);
    write_u32_from_usize(this + 0xdd, 0);
    let grade = novo((n * 4 + 4) as u32);
    if grade == 0 {
        aviso_em(0x9f15f4, 0x3fc);
        return 0;
    }
    let mut i = 1i32;
    while i <= n {
        let l = novo((n * 4) as u32);
        if l == 0 {
            aviso_em(0x9f15f4, 0x405);
            liberar(grade);
            return 0;
        }
        write_u32_from_usize(grade + i as usize * 4, l);
        i += 1;
    }
    montar(grade, n);
    let c = read_u32_as_usize(this + 4);
    if c == 0 || (read_u32_as_usize(c) != read_u32_as_usize(0x9cf79c) && read_u32_as_usize(c) != read_u32_as_usize(0x9cf7a4)) {
        crate::comparacao::ramo("cqd_5 ordem sorteada");
        sortear(this, n as u32); // cqd5: a ordem dos clubes
    }
    // o registro do jogo; os bytes que a original não grava (+4..+7, +0x18, +0x2e, +0x39, +0x4d e os bits altos de
    // +0x4e: sobra da pilha dela, que o método 0x7c e a 0x85c260 podem preencher) ficam 0, um valor que o jogo produz
    let mut reg = [0u8; 0x60];
    let r = reg.as_mut_ptr() as usize;
    let c = read_u32_as_usize(this + 4);
    write_u32_from_usize(r + 0x14, c);
    write_u16(r + 0x32, read_i16(this + 0x3a) as u16);
    write_u32_from_usize(r, read_u32_as_usize(c));
    write_u16(r + 0x2c, read_i16(this + 0x40) as u16);
    write_u16(r + 0x38, read_i16(this + 0xab) as u16);
    write_u16(r + 0x4d, read_i16(r + 0x4d) as u16 & 0xfbff);
    write_u8(r + 0x3a, read_u8(this + 0xc4));
    write_u16(r + 0x36, read_i16(this + 0xdb) as u16);
    write_u16(r + 0x30, 0);
    for k in 0x43..=0x4cusize {
        write_u8(r + k, 0xff);
    }
    write_u32_from_usize(r + 8, u32::MAX as usize);
    write_u8(r + 0x42, read_u8(this + 0x44));
    write_u8(r + 0x3d, 0);
    write_u8(r + 0x3e, 0);
    write_u8(r + 0x3b, 0xff);
    let preparar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(cqd5_metodo(this, 0x7c));
    preparar(this, r);
    let ult = read_u32_as_usize(this + 0xba).wrapping_add((read_i16(this + 0xa9) as i32 * 0x41) as usize);
    // ponytail: a original empurra o dia e o ano com sobra nos bytes altos; a 0x549ef0 só lê as palavras
    let yy = nova(py, read_i16(ult - 0x41) as u16 as u32, read_i16(ult - 0x3f).wrapping_add(read_i16(this + 0x40)) as u16 as u32);
    copia(px, pz, yy);
    write_u16(r + 0x26, read_i16(px) as u16);
    write_u16(r + 0x24, read_i16(px + 2) as u16);
    let lista = read_u32_as_usize(this + 0xea);
    if lista != 0 && read_i16(this + 0x40) as i32 == read_i16(lista + 2) as u16 as i32 {
        let mut k = 0i32;
        let mut off = 0usize;
        while k < read_i16(this + 0xa9) as i32 {
            let mut i = 0i32;
            loop {
                let times = read_i16(this + 0x3e) as i32;
                let metade = times / 2;
                if i >= metade {
                    break;
                }
                write_u16(r + 0x34, k as u16);
                let e = read_u32_as_usize(this + 0xea) + ((metade * k + i) as usize) * 0x10;
                let (mut ia, mut ib) = (-1i32, -1i32);
                if times > 0 {
                    let (a, b) = (read_u32_as_usize(e + 8), read_u32_as_usize(e + 0xc));
                    let tb = read_u32_as_usize(this + 0xb1);
                    for p in 0..times {
                        let id = read_u32_as_usize(read_u32_as_usize(tb + p as usize * 0x3b));
                        if a == id {
                            ia = p;
                        }
                        if b == id {
                            ib = p;
                        }
                    }
                }
                let tb = read_u32_as_usize(this + 0xb1);
                cqd5_jogo(this, r, tb.wrapping_add((ia as isize * 0x3b) as usize), tb.wrapping_add((ib as isize * 0x3b) as usize), off, true);
                i += 1;
            }
            k += 1;
            off += 0x41;
        }
        let lista = read_u32_as_usize(this + 0xea);
        if lista != 0 {
            destruir_lista(lista, 0x10, read_u32_as_usize(lista - 4) as u32, 0x5a53c0);
            liberar(lista - 4);
        }
        write_u32_from_usize(this + 0xea, 0);
    } else {
        let mut j = 1i32;
        while j <= read_i16(this + 0x3c) as i32 * (n - 1) {
            // ponytail: a original empurra as palavras +0x3e, +0x3c, +0xa9 e +0xd9 com sobra nos bytes altos; a 0x68a030
            // só lê as palavras e o byte
            rodada = girar(rodada, &mut passo, read_u32_as_usize(read_u32_as_usize(this + 4)) as u32, read_i16(this + 0x3e) as u16 as u32,
                read_i16(this + 0x3c) as u16 as u32, read_i16(this + 0xa9) as u16 as u32, read_i16(this + 0xd9) as u16 as u32);
            let mut col = j % (n - 1);
            if col == 0 {
                col = n - 1;
            }
            write_u16(r + 0x34, rodada as u16);
            if read_i16(this + 0x3e) >= 1 {
                let mut i = 0i32;
                loop {
                    let v = read_u32_as_usize(read_u32_as_usize(grade + 4 + i as usize * 4) + col as usize * 4) as i32;
                    if v != 0 {
                        let turno = ((j - 1) / (n - 1)) % 2 != 0;
                        let paridade = (read_i16(this + 0x40) as i32 + read_u8(this + 0x44) as i8 as i32) % 2 != 0;
                        let idx = if v > 0 { v - 1 } else { -1 - v };
                        let linha = read_u32_as_usize(grade + 4 + (idx as isize * 4) as usize);
                        let (a, b) = if (v > 0) ^ turno ^ paridade { (idx, i) } else { (i, idx) };
                        write_u32_from_usize(linha + col as usize * 4, 0);
                        let ult = read_i16(this + 0x3e) as i32 - 1;
                        if a >= 0 && a <= ult && b >= 0 && b <= ult {
                            let tb = read_u32_as_usize(this + 0xb1);
                            let (ea, eb) = (tb + a as usize * 0x3b, tb + b as usize * 0x3b);
                            let sel = read_u32_as_usize(this + 0x24) as u32;
                            let (m, f) = if sel != u32::MAX {
                                let nac = read_u32_as_usize(read_u32_as_usize(eb) + 0x69);
                                if nac != 0 && read_u32_as_usize(nac) as u32 == sel { (eb, ea) } else { (ea, eb) }
                            } else {
                                (ea, eb)
                            };
                            cqd5_jogo(this, r, m, f, (rodada as isize * 0x41) as usize, false);
                        }
                    }
                    i += 1;
                    if i + 1 > read_i16(this + 0x3e) as i32 {
                        break;
                    }
                }
            }
            j += 1;
        }
    }
    let mut i = 0i32;
    while i < n {
        liberar(read_u32_as_usize(grade + 4 + i as usize * 4));
        i += 1;
    }
    liberar(grade);
    let mut k = 0i32;
    while k < read_i16(this + 0xa9) as i32 {
        let e = read_u32_as_usize(this + 0xba) + k as usize * 0x41;
        if read_u8(e + 0xb) == 3 {
            nova(py, read_i16(e) as u16 as u32, read_i16(e + 2).wrapping_add(read_i16(this + 0x40)) as u16 as u32);
            tv(this, py);
        }
        k += 1;
    }
    1
}

/// O comparador da criação dos jogos: só a original (ponytail: aloca a grade, sorteia a ordem dos clubes e põe os
/// jogos no calendário, que aloca: não roda duas vezes). Confere o que a rotina garante: com a lista própria do ano, ela
/// é liberada (+0xea = 0) quando devolve 1.
unsafe extern "thiscall" fn comparar_liga_criar_tabela(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1553);
    let lista = read_u32_as_usize(this + 0xea);
    let propria = lista != 0 && read_i16(this + 0x40) as i32 == read_i16(lista + 2) as u16 as i32;
    let r = o(this);
    let ok = !(propria && r == 1) || read_u32_as_usize(this + 0xea) == 0;
    PLACAR.registrar(ok, || format!("liga_criar_tabela: lista própria {} devolveu {} +0xea {:x}", propria, r, read_u32_as_usize(this + 0xea)));
    r
}

/// { 0x689c80 } (ret 0x18, league.cpp): `league_promote_and_relegate(acima, abaixo, contratos, aos_pares, sobem,
/// descem)`, do fim das ligas (depois de 2002-06-01). Recebe o ecx de quem chama e o repassa à 0x6830b0, que não o lê:
/// thiscall aqui para o guardar. Com `contratos`, os clubes da liga de baixo que não sobem (+0x37 ≠ 0 e ≠ 5) acionam
/// as cláusulas de não promoção (0x4cced0: push liga; push clube; mov ecx, [0xae1050] -> thiscall, ret 8). Aos pares:
/// cada rebaixado de cima (+0x37 = 3) troca com o primeiro de baixo que sobe (+0x37 = 0 ou 5): esse sobe (0x6830b0:
/// push 0; push competição de cima; push clube; ecx de quem chama -> thiscall, ret 0xc), o rebaixado guarda a
/// competição antiga (+0x5b) e fica com a de baixo (+0x57), aciona as cláusulas de rebaixamento (0x4cd030: push
/// antiga; push clube; mov ecx, [0xae1050] -> thiscall, ret 8) e, numa competição sem registro ativo e com técnico
/// humano (0x600a50), perde o técnico (0x69b790: push 0; push 7; push 1; push clube; mov ecx, [0xb67a44] -> thiscall,
/// ret 0x10); os dois ficam com +0x37 = 0xff. Senão, todos os rebaixados descem assim e depois todos os de baixo que
/// sobem sobem. Os contados que diferem dos esperados (0xff = qualquer) dão a caixa da linha 0x15dc (sobem) ou 0x15e2
/// (descem).
unsafe extern "thiscall" fn liga_promover_e_rebaixar(ecx: usize, acima: usize, abaixo: usize, contratos: u32, aos_pares: u32, sobem: u32, descem: u32) {
    let clausulas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cced0usize);
    let subir: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6830b0usize);
    let rebaixamento: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cd030usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let sem_tecnico: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x69b790usize);
    let (mut desceram, mut subiram) = (0u8, 0u8);
    if contratos != 0 {
        let mut i = 0i32;
        while i < read_i16(abaixo + 0x3e) as i32 {
            let e = read_u32_as_usize(abaixo + 0xb1) + i as usize * 0x3b;
            let s = read_u8(e + 0x37);
            if s != 5 && s != 0 {
                clausulas(read_u32_as_usize(0xae1050), read_u32_as_usize(e), read_u32_as_usize(abaixo + 4));
            }
            i += 1;
        }
    }
    // o rebaixado desce para a competição de baixo e perde o técnico humano numa competição sem registro ativo
    let descer = |clube: usize| {
        let comp = read_u32_as_usize(abaixo + 4);
        let antiga = read_u32_as_usize(clube + 0x57);
        write_u32_from_usize(clube + 0x5b, antiga);
        write_u32_from_usize(clube + 0x57, comp);
        rebaixamento(read_u32_as_usize(0xae1050), clube, antiga);
        let tab = read_u32_as_usize(0xadadfc);
        if tab == 0 || comp == 0 || read_u32_as_usize(tab + read_u32_as_usize(comp) * 4) == 0 {
            if humano(clube, 1, 1, 0, 0) != 0 {
                sem_tecnico(read_u32_as_usize(0xb67a44), clube, 1, 7, 0);
            }
        }
    };
    if aos_pares != 0 {
        let mut i = 0i32;
        while i < read_i16(acima + 0x3e) as i32 {
            let ue = read_u32_as_usize(acima + 0xb1) + i as usize * 0x3b;
            if read_u8(ue + 0x37) == 3 {
                let n = read_i16(abaixo + 0x3e) as i32;
                let mut j = 0i32;
                while j < n {
                    let s = read_u8(read_u32_as_usize(abaixo + 0xb1) + j as usize * 0x3b + 0x37);
                    if s == 0 || s == 5 {
                        break;
                    }
                    j += 1;
                }
                if j < n {
                    let le = read_u32_as_usize(abaixo + 0xb1) + j as usize * 0x3b;
                    subir(ecx, read_u32_as_usize(le), read_u32_as_usize(acima + 4), 0);
                    descer(read_u32_as_usize(ue));
                    desceram = desceram.wrapping_add(1);
                    subiram = subiram.wrapping_add(1);
                    write_u8(le + 0x37, 0xff);
                    write_u8(ue + 0x37, 0xff);
                }
            }
            i += 1;
        }
    } else {
        let mut i = 0i32;
        while i < read_i16(acima + 0x3e) as i32 {
            let ue = read_u32_as_usize(acima + 0xb1) + i as usize * 0x3b;
            if read_u8(ue + 0x37) == 3 {
                descer(read_u32_as_usize(ue));
                desceram = desceram.wrapping_add(1);
                write_u8(ue + 0x37, 0xff);
            }
            i += 1;
        }
        let mut i = 0i32;
        while i < read_i16(abaixo + 0x3e) as i32 {
            let le = read_u32_as_usize(abaixo + 0xb1) + i as usize * 0x3b;
            let s = read_u8(le + 0x37);
            if s == 0 || s == 5 {
                subir(ecx, read_u32_as_usize(le), read_u32_as_usize(acima + 4), 0);
                subiram = subiram.wrapping_add(1);
                write_u8(le + 0x37, 0xff);
            }
            i += 1;
        }
    }
    if sobem as u8 != 0xff && subiram != sobem as u8 {
        aviso_em(0x9f15f4, 0x15dc);
    }
    if descem as u8 != 0xff && desceram != descem as u8 {
        aviso_em(0x9f15f4, 0x15e2);
    }
}

/// O comparador da promoção e do rebaixamento: só a original (ponytail: os contratos, o técnico demitido e as notícias
/// não rodam duas vezes); a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_liga_promover_e_rebaixar(ecx: usize, acima: usize, abaixo: usize, contratos: u32, aos_pares: u32, sobem: u32, descem: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32)>(1554)(ecx, acima, abaixo, contratos, aos_pares, sobem, descem)
}

// ---------- 262a leva: a tabela da liga: as posições possíveis e a situação de cada clube (o título, o acesso, o mata-mata, o rebaixamento, a vaga europeia), e a fase seguinte da First e da Second (league.cpp, eng_first.cpp, eng_second.cpp) ----------


unsafe fn cpz3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}



unsafe fn cpz3_s(a: usize) -> i8 {
    *(a as *const i8)
}

/// No ensaio (o comparador da 0x684cd0) a cópia não chama as virtuais +0x40 (o anúncio da situação, que manda notícia
/// e grava os registros) e +0x68 (a situação nas estatísticas) nem as cláusulas de rebaixamento (0x4cd030).
static mut CPZ3_ENSAIO: bool = false;

/// A fita do comparador da 0x684cd0: a original roda primeiro e cada anúncio dela (+0x40 = 0x686670, pelo comparador
/// dela) guarda o clube, a situação e os bytes que a virtual +0x44 da competição gravou nas regiões (a BraNatFirst
/// 0x42abc0, a BraNatSecond 0x42d040, a BelThird 0x4230d0 e a FinFirst 0x594810 põem 5 ou 0xfe no +0x37 da linha).
/// No ensaio, o anúncio da cópia que bater com o próximo da fita repõe esses bytes.
static mut CPZ3_GRAVAR: Option<Vec<(usize, usize)>> = None;
static mut CPZ3_FITA: Vec<(usize, u8, Vec<(usize, u8)>)> = Vec::new();
static mut CPZ3_LIDO: usize = 0;

/// A linha `i` da tabela (+0xb1, 0x3b bytes).
unsafe fn cpz3_linha(liga: usize, i: i32) -> usize {
    read_usize(liga + 0xb1).wrapping_add((i as usize).wrapping_mul(0x3b))
}

/// A virtual +0x40 da liga (League::announce_team_status: thiscall(liga, clube, situação, fase, 0, 0), ret 0x14), com
/// o clube da linha `i`. `alto` = os bits de cima do registrador que a original empurra com a fase (+0x44) no byte de
/// baixo (o ponteiro da linha ou a devolução das cláusulas: reproduzidos).
unsafe fn cpz3_anunciar(liga: usize, i: i32, situacao: u32, alto: u32) {
    if CPZ3_ENSAIO {
        let fita = &*core::ptr::addr_of!(CPZ3_FITA);
        if let Some((clube, s, bytes)) = fita.get(CPZ3_LIDO) {
            CPZ3_LIDO += 1;
            if *clube == read_usize(cpz3_linha(liga, i)) && *s == situacao as u8 {
                for &(a, v) in bytes {
                    *(a as *mut u8) = v;
                }
            }
        }
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(liga) + 0x40));
    f(liga, read_usize(cpz3_linha(liga, i)), situacao, alto | read_u8(liga + 0x44) as u32, 0, 0);
}

/// Grava a situação `v` no +0x37 da linha `i` e a anuncia.
unsafe fn cpz3_situacao(liga: usize, i: i32, v: u8, alto: u32) {
    *((cpz3_linha(liga, i) + 0x37) as *mut u8) = v;
    cpz3_anunciar(liga, i, v as i8 as i32 as u32, alto);
}

/// O máximo de pontos da linha: (jogos da temporada +0xa7 − jogos feitos +0x11) × pontos da vitória (+0xc2) + pontos
/// (+0x19).
unsafe fn cpz3_maximo(liga: usize, l: usize) -> i32 {
    ((read_i16(liga + 0xa7) as i32) - (cpz3_s(l + 0x11) as i32)).wrapping_mul(cpz3_s(liga + 0xc2) as i32).wrapping_add(read_i16(l + 0x19) as i32)
}

/// { 0x684cd0 } (thiscall, ret 0, league.cpp): `League::update_positions_and_status()`, a virtual +0x94 da liga (do
/// passe do dia 0x684946). +0xa7 (os jogos da temporada) = (+0x3e − 1) × +0x3c (16 bits) se ainda −1, com linhas e
/// +0x45. Com +0xdd nada mais. Senão ordena a tabela (League::sort_table 0x6827d0, thiscall(liga, 0), ret 4) e monta
/// o mapa das faixas (malloc 0x944e46 de +0x3e bytes em 0xff; sem memória: a caixa da linha 0x7c9): de cima, as
/// contas +0xbe (faixa 0, acesso) e +0xbf (faixa 1, mata-mata) nas linhas que não estão em 0xfe, 0xfc ou 3; de baixo,
/// +0xc1 (faixa 3, rebaixamento) e +0xc0 (faixa 2) nas que não estão em 0, 1 ou 5. Com +0xd9 bit 0, o primeiro da
/// tabela fora de 0xfe, 0xfc e 3. A melhor posição (+0x39) e a pior (+0x3a) de cada linha que ainda não estão
/// decididas, pelos máximos de pontos e pelos jogos feitos (com +0xe6 no empate dos que terminaram). Depois, linha a
/// linha: a pior = o primeiro + 1, o título (5); faixa 0 na pior, o acesso (0 com +0xd9 bit 1; 7 com bit 2; 6 com bit
/// 3) em quem está em 0xff ou 1; faixa 1, o mata-mata (1) em quem está em 0xff; faixa 3 na melhor, as cláusulas de
/// rebaixamento (CONTRACT_MANAGER::trigger_relegation_clauses 0x4cd030, thiscall([0xae1050], clube, +4), ret 8) e o
/// rebaixamento (3) em quem está em 0xff, 2 ou 0xfc; faixa 2 na melhor, 2 em quem está em 0xff ou 0xfe. Cada situação
/// nova é anunciada (+0x40). Com +0xd9 bit 0x20, quem fica entre as faixas sem faixa no meio e está em 0xff vira 0xfe
/// (anunciado com −2). Melhor = pior abaixo de 9, liga de cima (+0x44 = 0xff) que conta (0x4b0700, cdecl(id)), sem
/// +0x1c e clube sem vaga europeia ([0xae2d14] + 6 × id = −1) de nação europeia ([0x9cfa10]) fora de [0x9cf45c]: as
/// vagas da nação (UefaSeeding::get_nation_places 0x905c60, thiscall([0xde1f58], nação, &cl, &uefa), ret 0xc) dão a
/// Liga dos Campeões ([0x9cf6e8]) ou a Copa da UEFA ([0x9cf6f0]). Por fim a virtual +0x68 (a situação nas
/// estatísticas: clube, +0x44, posição, melhor, pior) e o free (0x9452ca) do mapa.
unsafe extern "thiscall" fn posicoes_e_situacao_na_liga(this: usize) {
    let n0 = read_i16(this + 0x3e);
    if read_i16(this + 0xa7) == -1 && n0 > 0 && read_usize(this + 0x45) != 0 {
        *((this + 0xa7) as *mut i16) = (n0 - 1).wrapping_mul(read_i16(this + 0x3c));
    }
    if read_usize(this + 0xdd) != 0 {
        return;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    ordenar(this, 0);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let mapa = alocar(read_i16(this + 0x3e) as i32 as u32);
    if mapa == 0 {
        return aviso_em(0x9f15f4, 0x7c9);
    }
    let n = read_i16(this + 0x3e);
    core::ptr::write_bytes(mapa as *mut u8, 0xff, n as i32 as usize);
    let m = |i: i32| (mapa as isize + i as isize) as *mut u8;
    let mut contas = [read_u8(this + 0xbe), read_u8(this + 0xbf), read_u8(this + 0xc0), read_u8(this + 0xc1)];
    let mut primeiro: i8 = -1;
    if read_u8(this + 0xd9) & 1 != 0 {
        let mut i: i16 = 0;
        while i < n {
            let s = read_u8(cpz3_linha(this, i as i32) + 0x37);
            if s != 0xfe && s != 0xfc && s != 3 {
                primeiro = i as i8;
                break;
            }
            i += 1;
        }
    }
    // de cima: as faixas 0 e 1
    let mut i: i32 = 0;
    if n > 0 {
        let mut k: i8 = 0;
        while k <= 1 {
            if contas[k as usize] != 0 {
                while (i as i16) < n {
                    let s = read_u8(cpz3_linha(this, i as i16 as i32) + 0x37);
                    if s != 0xfe && s != 0xfc && s != 3 {
                        *m(i as i16 as i32) = k as u8;
                        contas[k as usize] = contas[k as usize].wrapping_sub(1);
                    }
                    i += 1;
                    if contas[k as usize] == 0 {
                        break;
                    }
                }
            }
            k += 1;
            if (i as i16) >= n {
                break;
            }
        }
    }
    // de baixo: as faixas 3 e 2
    let mut i: i32 = (n as i32) - 1;
    let mut k: i8 = 3;
    while (i as i16) >= 0 && k >= 2 {
        if contas[k as usize] != 0 {
            while (i as i16) >= 0 {
                let s = read_u8(cpz3_linha(this, i as i16 as i32) + 0x37);
                if s != 0 && s != 1 && s != 5 {
                    *m(i as i16 as i32) = k as u8;
                    contas[k as usize] = contas[k as usize].wrapping_sub(1);
                }
                i -= 1;
                if contas[k as usize] == 0 {
                    break;
                }
            }
        }
        k -= 1;
    }
    // a melhor (+0x39) e a pior (+0x3a) posição possível
    let mut i: i32 = 0;
    while (i as i16) < read_i16(this + 0x3e) {
        let li = cpz3_linha(this, i as i16 as i32);
        if read_u8(li + 0x39) != read_u8(li + 0x3a) {
            let pos = (i as u8).wrapping_add(1);
            *((li + 0x39) as *mut u8) = pos;
            // os de cima (j < i): a melhor cai um para cada um que o máximo de i alcança
            let max_i = cpz3_maximo(this, li);
            let mut j: i32 = 0;
            while (j as i16) < (i as i16) {
                let lj = cpz3_linha(this, j);
                let jogos = read_i16(this + 0xa7);
                let fim_i = (cpz3_s(li + 0x11) as i16) >= jogos;
                let fim_j = (cpz3_s(lj + 0x11) as i16) >= jogos;
                let pj = read_i16(lj + 0x19) as i32;
                let cai = if !fim_i && !fim_j {
                    max_i >= pj
                } else if !fim_j {
                    max_i == pj
                } else if !fim_i {
                    max_i >= pj
                } else {
                    max_i == pj && read_usize(this + 0xe6) != 0
                };
                if cai {
                    *((li + 0x39) as *mut u8) = read_u8(li + 0x39).wrapping_sub(1);
                }
                j += 1;
            }
            // os de baixo (j > i): a pior sobe um para cada um cujo máximo alcança i
            *((li + 0x3a) as *mut u8) = pos;
            let mut j: i32 = pos as i8 as i32;
            while (j as i16) < read_i16(this + 0x3e) {
                let lj = cpz3_linha(this, j as i16 as i32);
                let jogos = read_i16(this + 0xa7);
                let pj = read_i16(lj + 0x19) as i32;
                let max_j = cpz3_maximo(this, lj);
                let fim_i = (cpz3_s(li + 0x11) as i16) >= jogos;
                let fim_j = (cpz3_s(lj + 0x11) as i16) >= jogos;
                let pi = read_i16(li + 0x19) as i32;
                let sobe = if !fim_i && !fim_j {
                    max_j >= pi
                } else if !fim_j {
                    max_j >= pi
                } else if !fim_i {
                    max_j == pi
                } else {
                    max_j == pj && read_usize(this + 0xe6) != 0
                };
                if sobe {
                    *((li + 0x3a) as *mut u8) = read_u8(li + 0x3a).wrapping_add(1);
                }
                j += 1;
            }
        }
        i += 1;
    }
    // a situação de cada linha
    let um_depois = (primeiro as i32) + 1;
    let mut i: i32 = 0;
    while (i as i16) < read_i16(this + 0x3e) {
        let ii = i as i16 as i32;
        let li = cpz3_linha(this, ii);
        let alto_linha = (li as u32) & !0xff;
        let pior = cpz3_s(li + 0x3a) as i32;
        let s = read_u8(li + 0x37);
        if pior == um_depois {
            if s != 3 && s != 0xfe && s != 0xfc && s != 5 {
                crate::comparacao::ramo("cpz_3 titulo");
                cpz3_situacao(this, ii, 5, alto_linha); // cpz3: o título na pior posição = o primeiro + 1
            }
        } else {
            let f = *m(pior - 1);
            if f == 0 {
                if s == 0xff || s == 1 {
                    let d9 = read_u8(this + 0xd9);
                    if d9 & 2 != 0 {
                        if s != 0 {
                            cpz3_situacao(this, ii, 0, alto_linha);
                        }
                    } else if d9 & 4 != 0 {
                        if s != 7 {
                            cpz3_situacao(this, ii, 7, alto_linha);
                        }
                    } else if d9 & 8 != 0 {
                        if s != 6 {
                            cpz3_situacao(this, ii, 6, alto_linha);
                        }
                    }
                }
            } else if f == 1 {
                if s == 0xff {
                    crate::comparacao::ramo("cpz_3 vaga no mata-mata");
                    cpz3_situacao(this, ii, 1, alto_linha); // cpz3: a vaga no mata-mata (situação 1)
                }
            } else {
                let g = *m(cpz3_s(li + 0x39) as i32 - 1);
                if g == 3 {
                    let mut alto = alto_linha;
                    if !CPZ3_ENSAIO {
                        crate::comparacao::ramo("cpz_3 clausulas de rebaixamento");
                        let clausulas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x4cd030usize);
                        alto = clausulas(read_usize(0xae1050), read_usize(li), read_usize(this + 4)) & !0xff; // cpz3: as cláusulas de rebaixamento
                    }
                    let li = cpz3_linha(this, ii);
                    let s = read_u8(li + 0x37);
                    if s == 0xff || s == 2 || s == 0xfc {
                        cpz3_situacao(this, ii, 3, alto);
                    }
                } else if g == 2 {
                    if s == 0xff || s == 0xfe {
                        cpz3_situacao(this, ii, 2, alto_linha);
                    }
                }
            }
        }
        // entre as faixas, sem faixa no meio: 0xfe
        if read_u8(this + 0xd9) & 0x20 != 0 {
            let li = cpz3_linha(this, ii);
            let c0 = cpz3_s(this + 0xbe) as i32;
            let c2 = cpz3_s(this + 0xc0) as i32;
            let melhor = cpz3_s(li + 0x39);
            if (melhor as i32) > c0 + c2 {
                let c3 = cpz3_s(this + 0xc1) as i32;
                let pior = cpz3_s(li + 0x3a);
                if (pior as i32) <= (read_i16(this + 0x3e) as i32) - c3 - c2 {
                    let mut livre = true;
                    let mut a: i16 = (melhor as i16) - 1;
                    while a < pior as i16 {
                        if !livre {
                            break;
                        }
                        if *m(a as i32) != 0xff {
                            livre = false;
                        }
                        a += 1;
                    }
                    if livre && read_u8(li + 0x37) == 0xff {
                        *((li + 0x37) as *mut u8) = 0xfe;
                        cpz3_anunciar(this, ii, 0xffff_fffe, (li as u32) & 0xffff_0000);
                    }
                }
            }
        }
        // a vaga europeia de quem já tem a posição decidida
        let li = cpz3_linha(this, ii);
        let melhor = cpz3_s(li + 0x39);
        if melhor == cpz3_s(li + 0x3a) && melhor < 9 && read_u8(this + 0x44) == 0xff {
            let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
            if conta(cpz3_i(read_usize(this + 4))) != 0 && cpz3_i(this + 0x1c) == -1 {
                let vaga = read_usize(0xae2d14) + (cpz3_i(read_usize(li)) as usize).wrapping_mul(6);
                let reg = read_usize(this + 4);
                let nacao = read_usize(reg + 0x5d);
                if cpz3_i(vaga) == -1 && nacao != 0 && cpz3_i(nacao) != cpz3_i(0x9cf45c) {
                    let cont = read_usize(nacao + 0x71);
                    if cont != 0 && cpz3_i(cont) == cpz3_i(0x9cfa10) {
                        let (mut cl, mut uefa) = (0i8, 0i8);
                        let vagas: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8) = core::mem::transmute(0x905c60usize);
                        vagas(read_usize(0xde1f58), nacao, &mut cl, &mut uefa);
                        let li = cpz3_linha(this, ii);
                        let melhor = cpz3_s(li + 0x39) as i32;
                        let vaga = read_usize(0xae2d14) + (cpz3_i(read_usize(li)) as usize).wrapping_mul(6);
                        if melhor <= cl as i32 {
                            core::ptr::write_unaligned(vaga as *mut i32, cpz3_i(0x9cf6e8));
                        } else if melhor <= (cl as i32) + (uefa as i32) - 1 {
                            core::ptr::write_unaligned(vaga as *mut i32, cpz3_i(0x9cf6f0));
                        }
                    }
                }
            }
        }
        // a situação nas estatísticas (virtual +0x68)
        if !CPZ3_ENSAIO {
            let li = cpz3_linha(this, ii);
            // ponytail: a original empurra a pior, a melhor, a posição e a fase em registradores com lixo nos bits de
            // cima; as +0x68 (0x573260, 0x578660, Comp::set_club_progress 0x48e380) só leem o byte.
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + 0x68));
            f(this, read_usize(li), read_u8(this + 0x44) as u32, (i as u8).wrapping_add(1) as u32, read_u8(li + 0x39) as u32, read_u8(li + 0x3a) as u32);
        }
        i += 1;
    }
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(mapa);
}

/// As regiões que a 0x684cd0 grava: a tabela inteira (+0xb1, +0x3e linhas de 0x3b bytes), +0xa7 e a vaga europeia
/// ([0xae2d14] + 6 × id, 4 bytes) de cada clube da tabela.
unsafe fn cpz3_regioes(this: usize) -> Vec<(usize, usize)> {
    let n = read_i16(this + 0x3e).max(0) as usize;
    let mut v = vec![(this + 0xa7, 2)];
    let tab = read_usize(this + 0xb1);
    if n > 0 && tab != 0 {
        v.push((tab, n * 0x3b));
        let vagas = read_usize(0xae2d14);
        if vagas != 0 {
            for i in 0..n {
                let c = read_usize(tab + i * 0x3b);
                if c != 0 {
                    let a = vagas + (cpz3_i(c) as usize).wrapping_mul(6);
                    if !v.iter().any(|r| r.0 == a) {
                        v.push((a, 4));
                    }
                }
            }
        }
    }
    v
}

/// Comparador da 0x684cd0, em ensaio (modelo CPV6_ENSAIO): a original roda inteira primeiro, gravando a fita dos
/// anúncios (CPZ3_FITA); a memória volta e a cópia roda sem as virtuais +0x40 e +0x68 nem as cláusulas (que mandam
/// notícia, gravam registros e sorteiam o moral), repondo da fita o que a +0x44 de cada anúncio gravou na tabela. As
/// duas a partir do mesmo estado da tabela, de +0xa7 e das vagas europeias dos clubes da tabela, que têm de sair
/// iguais; no fim fica o estado da original. Sem linhas ou com +0xdd: só a original.
unsafe extern "thiscall" fn comparar_posicoes_e_situacao_na_liga(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1386);
    if read_usize(this + 0xdd) != 0 || read_i16(this + 0x3e) <= 0 || read_usize(this + 0xb1) == 0 {
        return o(this);
    }
    let regioes = cpz3_regioes(this);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    *core::ptr::addr_of_mut!(CPZ3_FITA) = Vec::new();
    CPZ3_LIDO = 0;
    *core::ptr::addr_of_mut!(CPZ3_GRAVAR) = Some(regioes.clone());
    o(this);
    *core::ptr::addr_of_mut!(CPZ3_GRAVAR) = None;
    let mo = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    CPZ3_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| posicoes_e_situacao_na_liga(this));
    CPZ3_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    mo.escrever();
    PLACAR.registrar(mo == mr, || format!("posicoes_e_situacao_na_liga({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// A fase seguinte das ligas com mata-mata (o corpo de 0x572ff0 e 0x5769e0): na última fase (+0x30 ≥ +0x2c − 1, com
/// sinal) a caixa da linha `linha`; senão +0x30 + 1 e, se chegou a 0, o mata-mata (`mata_mata`, thiscall(this)).
unsafe fn cpz3_fase_seguinte(this: usize, arquivo: usize, linha: u32, mata_mata: usize, nome: &'static str) {
    let f = cpz3_i(this + 0x30);
    if f >= cpz3_i(this + 0x2c).wrapping_sub(1) {
        return aviso_em(arquivo, linha);
    }
    let f = f.wrapping_add(1);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, f);
    if f == 0 {
        crate::comparacao::ramo(nome);
        let criar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(mata_mata);
        criar(this);
    }
}

/// { 0x572ff0 } (thiscall, ret 0, eng_first.cpp): `EngFirst::advance_stage()` (a virtual +0x28): fonte 0x9c4374,
/// linha 0x2ce; o mata-mata 0x573080.
unsafe extern "thiscall" fn fase_seguinte_da_first(this: usize) {
    cpz3_fase_seguinte(this, 0x9c4374, 0x2ce, 0x573080, "cpz_3 mata-mata da first")
}

/// { 0x5769e0 } (thiscall, ret 0, eng_second.cpp): `EngSecond::advance_stage()` (a virtual +0x28): fonte 0x9c43e8,
/// linha 0x294; o mata-mata 0x576a70.
unsafe extern "thiscall" fn fase_seguinte_da_second(this: usize) {
    cpz3_fase_seguinte(this, 0x9c43e8, 0x294, 0x576a70, "cpz_3 mata-mata da second")
}

/// O comparador das duas: na fase que não chega a 0 (só +0x30 muda) a original e a cópia lado a lado em +0x30; na
/// última fase (a caixa) e na que chega a 0 (o mata-mata aloca a fase): só a original.
unsafe fn cpz3_comparar_fase(nome: &str, this: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let f = cpz3_i(this + 0x30);
    if f >= cpz3_i(this + 0x2c).wrapping_sub(1) || f.wrapping_add(1) == 0 {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x30, 4)], original, copia);
    PLACAR.registrar(igual, || format!("{}({:#x}) original={:x?} rust={:x?}", nome, this, mo, mr));
}

unsafe extern "thiscall" fn comparar_fase_seguinte_da_first(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1387);
    cpz3_comparar_fase("fase_seguinte_da_first", this, || o(this), || fase_seguinte_da_first(this))
}

unsafe extern "thiscall" fn comparar_fase_seguinte_da_second(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1388);
    cpz3_comparar_fase("fase_seguinte_da_second", this, || o(this), || fase_seguinte_da_second(this))
}

// ---------- 260a leva: o fim da temporada da liga: os prêmios em dinheiro, os registros pela situação do clube e a notícia da situação (league.cpp) ----------




/// O registro de finanças do clube (FinanceTable::get_club_finance 0x5a0590 de [0xae2a38]).
unsafe fn cpx11_financas(clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    f(read_usize(0xae2a38), clube)
}

/// O ensaio do comparador da 0x682e10: a cópia roda sem a notícia do prêmio (0x48e3e0, que aloca e não pode rodar duas
/// vezes).
static mut CPX11_ENSAIO: bool = false;

/// O divisor dos prêmios: 1 + 2 + … + n (16 bits, como o ax da original).
unsafe fn cpx11_soma_das_posicoes(n: i16) -> i32 {
    let (mut s, mut k) = (0i32, 0i32);
    if n > 0 {
        loop {
            s = s.wrapping_add(k).wrapping_add(1);
            k += 1;
            if k as i16 >= n {
                break;
            }
        }
    }
    s as i16 as i32
}

/// { 0x682e10 } (thiscall, ret 0, league.cpp): `League::pay_prize_money()`: com +0xe1 = 2, nada. Com +0xe1 = 3, a
/// competição ([0xadadfc] pelo id de +4) paga pela virtual +0x6c (sem a tabela das competições: a caixa da linha
/// 0x1df). Senão o fundo (+0x28) dividido por 1 + 2 + … + n (n = +0x3e linhas da tabela) é a parte; o clube da linha i
/// (+0xb1, 0x3b cada) recebe (n − i) partes no registro de finanças (0x5a0590; 0x5999a0 soma no saldo, em +0xa8 e
/// +0x148) e, acima de 1000, a notícia do prêmio (0x48e3e0 com a posição i + 1).
unsafe extern "thiscall" fn pagar_premios_da_liga(liga: usize) {
    let e1 = read_u8(liga + 0xe1);
    if e1 == 2 {
        return;
    }
    if e1 == 3 {
        let t = read_usize(0xadadfc);
        if t == 0 {
            return aviso_em(0x9f15f4, 0x1df);
        }
        let comp = read_usize(t.wrapping_add(read_usize(read_usize(liga + 4)).wrapping_mul(4)));
        let pagar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(comp) + 0x6c));
        return pagar(comp, liga);
    }
    let s = cpx11_soma_das_posicoes(read_i16(liga + 0x3e));
    let fundo = read_usize(liga + 0x28) as i32;
    if s == 0 || (fundo == i32::MIN && s == -1) {
        // ponytail: a original divide por zero (ou estoura) e cai; só a original, para cair do mesmo jeito
        return orig::<unsafe extern "thiscall" fn(usize)>(1335)(liga);
    }
    let parte = fundo / s;
    if read_i16(liga + 0x3e) <= 0 {
        return;
    }
    let somar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5999a0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, i32) = core::mem::transmute(0x48e3e0usize);
    let mut i: i16 = 0;
    loop {
        let valor = (read_i16(liga + 0x3e) as i32).wrapping_sub(i as i32).wrapping_mul(parte);
        let clube = read_usize(read_usize(liga + 0xb1).wrapping_add(0x3b * i as usize));
        let fin = cpx11_financas(clube);
        crate::comparacao::ramo("cpx_11 premio pago");
        somar(fin, valor); // cpx11: o prêmio
        if valor > 1000 && !CPX11_ENSAIO {
            // ponytail: a original empurra a posição no dl e o resto do edx (29 × i, sobra da conta da linha); a
            // 0x48e3e0 lê só o byte
            let pos = ((29 * i as i32 as u32) & !0xff) | (i as u8).wrapping_add(1) as u32;
            noticia(liga, clube, valor as u32, (valor >> 31) as u32, pos, 0, 0, -2);
        }
        i = i.wrapping_add(1);
        if i >= read_i16(liga + 0x3e) {
            break;
        }
    }
}

/// Comparador: com +0xe1 = 3 (a virtual da competição paga por conta própria) ou o divisor zero: só a original. Nos
/// outros casos, o ensaio: a cópia sem a notícia do prêmio (0x48e3e0 aloca) e a original a partir do mesmo estado, nos
/// registros de finanças (0x167 bytes) de todos os clubes da tabela.
unsafe extern "thiscall" fn comparar_pagar_premios_da_liga(liga: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1335);
    let n = read_i16(liga + 0x3e);
    let e1 = read_u8(liga + 0xe1);
    if e1 == 3 || cpx11_soma_das_posicoes(n) == 0 {
        return o(liga); // ponytail: a virtual ou a divisão por zero, só a original
    }
    let mut regioes: Vec<(usize, usize)> = Vec::new();
    if e1 != 2 {
        for i in 0..n.max(0) as usize {
            let fin = cpx11_financas(read_usize(read_usize(liga + 0xb1) + 0x3b * i));
            if fin != 0 && !regioes.iter().any(|r| r.0 == fin) {
                regioes.push((fin, 0x167));
            }
        }
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPX11_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| pagar_premios_da_liga(liga)));
    CPX11_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(liga);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("pagar_premios_da_liga({:x}) +0xe1 {} n {} original={:x?} rust={:x?}", liga, e1, n, mo, mr));
}

/// { 0x686940 } (thiscall, ret 0x18, league.cpp): `League::update_staff_records_for_status(clube, situação, fase,
/// saída, _, _)` (a virtual +0x44 da liga): situação 5, o título (Staff_Records 0x868c50 de [0xdd7f00] com o clube e
/// a competição +4); 0, o acesso (0x869480 com peso 100); 3, o rebaixamento (0x86a1c0). Devolve 0 e não grava a saída.
unsafe extern "thiscall" fn registros_pela_situacao_na_liga(liga: usize, clube: usize, situacao: u32, _fase: u32, _saida: usize, _a: u32, _b: u32) -> u32 {
    let registros = read_usize(0xdd7f00);
    let comp = read_usize(liga + 4);
    match situacao as u8 {
        5 => {
            crate::comparacao::ramo("cpx_11 registro do titulo");
            let titulo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x868c50usize);
            titulo(registros, clube, comp);
        }
        0 => {
            crate::comparacao::ramo("cpx_11 registro do acesso");
            let acesso: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869480usize);
            acesso(registros, clube, comp, 0x64);
        }
        3 => {
            crate::comparacao::ramo("cpx_11 registro do rebaixamento");
            let rebaixamento: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86a1c0usize);
            rebaixamento(registros, clube, comp);
        }
        _ => {}
    }
    0
}

// ponytail: os três registros acrescentam ao histórico das pessoas e mexem na reputação (alocam): só a original; a
// cópia leva os ramos.
unsafe extern "thiscall" fn comparar_registros_pela_situacao_na_liga(liga: usize, clube: usize, situacao: u32, fase: u32, saida: usize, a: u32, b: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> u32>(1336)(liga, clube, situacao, fase, saida, a, b)
}

/// { 0x686670 } (thiscall, ret 0x14, league.cpp, quadro SEH sem destrutor): `League::announce_team_status(clube,
/// situação, fase, _, _)`: amistoso (0x5c6710), 0. A virtual +0x44 da competição ([0xadadfc] pelo id de +4) com o
/// clube, a situação, a fase e a saída (0xff); −1 dela: 0. A rodada (+0x3a; com fases, +0x44 = 0xff, e fase ≠ 0xff:
/// a da fase). A notícia 0xc (local): 0 o clube, 1 a competição, 2 a fase, 3 a situação, 4 a rodada, 5 o que a
/// virtual devolveu. Saída ≠ 0xff: a entrega pelo alcance (0x786090 com a nação do clube e o continente). Senão pela
/// situação: 5, à nação da competição (0x7863f0) sem o +0x1c, à divisão (0x7862d0) com ele; 0 e 3, à divisão e à
/// divisão de acesso (+0x1c) ou de rebaixamento (+0x20) da competição, se houver; 1 e 2, à divisão; 6, 7 e 0xfe, ao
/// técnico do clube (0x7861e0). Devolve 1.
unsafe extern "thiscall" fn anunciar_situacao_na_liga(liga: usize, clube: usize, situacao: u32, fase: u32, _a: u32, _b: u32) -> u32 {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let a_divisao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7862d0usize);
    let a_nacao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
    let pelo_alcance: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x786090usize);
    // ponytail: a original grava só o byte da saída num dword da pilha e empurra o dword inteiro para a 0x786090, que
    // lê o byte; a cópia passa o alto zerado
    let mut saida: u32 = 0xff;
    let mut rodada = read_i16(liga + 0x3a);
    if amistoso(read_usize(0xae2c74), read_usize(liga + 4)) != 0 {
        return 0;
    }
    let comp = read_usize(read_usize(0xadadfc).wrapping_add(read_usize(read_usize(liga + 4)).wrapping_mul(4)));
    let virtual44: unsafe extern "thiscall" fn(usize, usize, u32, u32, *mut u32, u32, u32) -> i32 = core::mem::transmute(read_usize(read_usize(comp) + 0x44));
    let r = virtual44(comp, clube, situacao, fase, &mut saida, 0, 0);
    if r == -1 {
        return 0;
    }
    if read_u8(liga + 0x44) == 0xff && fase as u8 != 0xff {
        let f = read_usize(read_usize(liga + 0xc).wrapping_add((fase as u8 as i8 as isize as usize).wrapping_mul(4)));
        rodada = read_i16(f + 0x3a);
    }
    crate::comparacao::ramo("cpx_11 noticia da situacao");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xc, 0);
    gravar(n, 0, read_usize(clube) as u32);
    gravar(n, 1, read_usize(read_usize(liga + 4)) as u32);
    gravar(n, 2, fase as u8 as i8 as i32 as u32);
    gravar(n, 3, situacao as u8 as i8 as i32 as u32);
    gravar(n, 4, rodada as i32 as u32);
    gravar(n, 5, r as u32);
    let noticias = read_usize(0xb7431c);
    if saida as u8 != 0xff {
        let nacao = read_usize(clube + 0x53);
        let continente = if nacao != 0 { read_usize(nacao + 0x71) } else { 0 };
        pelo_alcance(noticias, n, clube, read_usize(liga + 4), nacao, continente, saida);
        return 1;
    }
    // a divisão de acesso (+0x1c) ou de rebaixamento (+0x20) da competição, registros de 0x6b em [0xae23d0]
    let outra = |desl: usize| {
        let c = read_usize(read_usize(0xadadfc).wrapping_add(read_usize(read_usize(liga + 4)).wrapping_mul(4)));
        let e = read_usize(c + desl) as i32;
        if e != -1 {
            a_divisao(noticias, n, read_usize(0xae23d0).wrapping_add((e as usize).wrapping_mul(0x6b)));
        }
    };
    match situacao as u8 {
        5 => {
            if read_usize(liga + 0x1c) as i32 == -1 {
                a_nacao(noticias, n, read_usize(read_usize(liga + 4) + 0x5d));
            } else {
                a_divisao(noticias, n, read_usize(liga + 4));
            }
        }
        0 => {
            a_divisao(noticias, n, read_usize(liga + 4));
            outra(0x1c);
        }
        7 | 6 | 0xfe => {
            ao_clube(noticias, n, clube);
        }
        1 | 2 => {
            a_divisao(noticias, n, read_usize(liga + 4));
        }
        3 => {
            a_divisao(noticias, n, read_usize(liga + 4));
            outra(0x20);
        }
        _ => {}
    }
    1
}

// ponytail: a virtual +0x44 acrescenta registros e a notícia vai às caixas dos técnicos (alocam): só a original; a
// cópia leva o ramo.
// Dentro do comparador da 0x684cd0 (CPZ3_GRAVAR), guarda na fita dele o que o anúncio gravou nas regiões.
unsafe extern "thiscall" fn comparar_anunciar_situacao_na_liga(liga: usize, clube: usize, situacao: u32, fase: u32, a: u32, b: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32>(1337);
    let Some(regioes) = (*core::ptr::addr_of!(CPZ3_GRAVAR)).clone() else {
        return o(liga, clube, situacao, fase, a, b);
    };
    let antes = crate::comparacao::Memoria::ler(&regioes);
    let r = o(liga, clube, situacao, fase, a, b);
    let depois = crate::comparacao::Memoria::ler(&regioes);
    let mut bytes = Vec::new();
    for ((base, x), (_, y)) in antes.0.iter().zip(depois.0.iter()) {
        bytes.extend((0..x.len()).filter(|&k| x[k] != y[k]).map(|k| (base + k, y[k])));
    }
    (*core::ptr::addr_of_mut!(CPZ3_FITA)).push((clube, situacao as u8, bytes));
    r
}

// ---------- 259a leva: a tabela da liga: o resultado na tabela, a cópia da tabela e as posições previstas (league.cpp) ----------



/// O ensaio do comparador da 0x685d30: a cópia roda sem a estatística da competição (0x49f570, que acha ou cria os
/// registros dos jogadores e não pode rodar duas vezes).
static mut CPW11_ENSAIO: bool = false;

/// { 0x685d30 } (thiscall, ret 8, league.cpp): `League::add_result(partida, jogo)`: grava a data do dia em +0xc9
/// (0x417c10 de [0xae2c90]); liga com fases (+0x44 = 0xff) e partida de uma fase (+0x42 ≠ 0xff): a fase
/// ([+0xc] + 4 × fase, até +0x30) recebe a partida pela virtual +4 e o que ela devolve sai; fase fora da faixa: as
/// caixas e 0. Partida de outra fase (+0x42 ≠ +0x44, que não é 0xfe): 0. Senão soma a partida na tabela (0x685f60 na
/// tabela +0xb1 com as +0x3e linhas, marcando) e, fora das fases 0xfe, com a competição ([0xadadfc] pelo id de +4) e a
/// estatística dela (+8) e com o jogo: 0x49f570 com os pontos dos dois; devolve 1. Partida nula: a caixa e −1.
unsafe extern "thiscall" fn somar_resultado_na_liga(liga: usize, p: usize, jogo: usize) -> i32 {
    if p == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(1299)(liga, p, jogo);
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    copia(liga + 0xc9, local.as_mut_ptr(), 0xae2c90);
    let fase_da_liga = *((liga + 0x44) as *const u8);
    let fase = *((p + 0x42) as *const u8);
    if fase_da_liga == 0xff && fase != 0xff {
        let k = fase as i8 as i32;
        if k > read_usize(liga + 0x30) as i32 || k < 0 {
            // as caixas da fase fora da faixa (linhas 0xc8e e 0xc8f): a original (a data já está gravada, igual)
            return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(1299)(liga, p, jogo);
        }
        let filha = read_usize(read_usize(liga + 0xc).wrapping_add(4 * k as usize));
        let somar: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(read_usize(read_usize(filha) + 4));
        return somar(filha, p, jogo);
    }
    if fase_da_liga != fase && fase_da_liga != 0xfe {
        return 0;
    }
    let aplicar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut i16, *mut i16) = core::mem::transmute(0x685f60usize);
    let (mut pts_casa, mut pts_fora) = (0i16, 0i16);
    // ponytail: a original empurra o +0x3e com o alto do ecx (sobra da chamada anterior); a 0x685f60 lê só o curto
    crate::comparacao::ramo("cpw_11 resultado aplicado");
    aplicar(liga, p, read_usize(liga + 0xb1), read_i16(liga + 0x3e) as u16 as u32, 1, &mut pts_casa, &mut pts_fora); // cpw11: a tabela
    if *((liga + 0x44) as *const u8) == 0xfe || read_usize(0xadadfc) == 0 {
        return 1;
    }
    let comp = read_usize(read_usize(0xadadfc).wrapping_add(read_usize(read_usize(liga + 4)).wrapping_mul(4)));
    if comp == 0 || read_usize(comp + 8) == 0 || jogo == 0 {
        return 1;
    }
    if !CPW11_ENSAIO {
        // ponytail: a original empurra os pontos como dwords das locais com o alto sem valor; a 0x49f570 lê os curtos
        let estatistica: unsafe extern "thiscall" fn(usize, usize, usize, i32, i32) -> u32 = core::mem::transmute(0x49f570usize);
        estatistica(read_usize(comp + 8), p, jogo, pts_casa as i32, pts_fora as i32);
    }
    1
}

/// Comparador: a partida nula e a da fase (a virtual da fase recebe a partida e compara por conta própria): só a
/// original. Nos outros casos, o ensaio: a cópia sem a estatística (0x49f570 acha ou cria registros) e a original a
/// partir do mesmo estado: a data em +0xc9, as linhas da tabela com os confrontos (as regiões da 0x685f60), o +0x3d/+0x3e
/// da partida e a devolução.
unsafe extern "thiscall" fn comparar_somar_resultado_na_liga(liga: usize, p: usize, jogo: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(1299);
    if p == 0 || (*((liga + 0x44) as *const u8) == 0xff && *((p + 0x42) as *const u8) != 0xff) {
        return o(liga, p, jogo); // ponytail: a caixa ou a fase, só a original
    }
    let n = read_i16(liga + 0x3e) as u16 as u32;
    let mut regioes = vec![(liga + 0xc9, 8)];
    regioes.extend(cpe2_regioes(p, read_usize(liga + 0xb1), n, core::ptr::null_mut(), core::ptr::null_mut()));
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW11_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| somar_resultado_na_liga(liga, p, jogo)));
    CPW11_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let ro = o(liga, p, jogo);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(ro == rr && mo == mr, || format!("somar_resultado_na_liga({:x}, partida {:x}) original={} {:x?} rust={} {:x?}", liga, p, ro, mo, rr, mr));
    ro
}

/// { 0x687710 } (thiscall, ret 0, league.cpp): `League::copy_table()`: sem tabela (+0xb1) ou sem linhas (+0x3e): 0.
/// Com +0xa7 = −1, grava nele (linhas − 1) × +0x3c (16 bits; mais de 50 linhas: a caixa da linha 0x1059 antes, pela
/// original). Aloca as linhas (0x944e46, 0x3b cada; sem memória: a caixa 0x1061 e 0) e copia; com os confrontos na
/// primeira linha (+5), cada linha ganha um vetor novo de (linhas − 1) × 14 bytes com a cópia dos dela; com a forma
/// (+9), um de +0xa7 bytes. Devolve a cópia. As alocações dos vetores não são conferidas (como na original).
unsafe extern "thiscall" fn copiar_tabela_da_liga(liga: usize) -> usize {
    let tab = read_usize(liga + 0xb1);
    let n = read_i16(liga + 0x3e);
    if tab == 0 || n == 0 {
        return 0;
    }
    if read_i16(liga + 0xa7) == -1 && n > 0 {
        if n > 0x32 {
            return orig::<unsafe extern "thiscall" fn(usize) -> usize>(1300)(liga); // ponytail: a caixa, só a original
        }
        let v = n.wrapping_sub(1).wrapping_mul(read_i16(liga + 0x3c));
        core::ptr::write_unaligned((liga + 0xa7) as *mut i16, v);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha = |i: i32| (i as usize).wrapping_mul(0x3b);
    let c = alocar((n as i32 * 0x3b) as u32);
    if c == 0 {
        aviso_em(0x9f15f4, 0x1061);
        return 0;
    }
    core::ptr::copy_nonoverlapping(tab as *const u8, c as *mut u8, n as i32 as usize * 0x3b);
    if read_usize(tab + 5) != 0 && n > 0 {
        let mut i: i16 = 0;
        while i < read_i16(liga + 0x3e) {
            let m = read_i16(liga + 0x3e) as i32;
            let h = alocar((14 * m - 14) as u32);
            core::ptr::write_unaligned((c + linha(i as i32) + 5) as *mut usize, h);
            let de = read_usize(read_usize(liga + 0xb1) + linha(i as i32) + 5);
            core::ptr::copy_nonoverlapping(de as *const u8, h as *mut u8, (14 * read_i16(liga + 0x3e) as i32 - 14) as usize);
            i += 1;
        }
    }
    if read_usize(read_usize(liga + 0xb1) + 9) != 0 && read_i16(liga + 0x3e) > 0 {
        let mut i: i16 = 0;
        while i < read_i16(liga + 0x3e) {
            let t = read_i16(liga + 0xa7) as i32;
            let h = alocar(t as u32);
            core::ptr::write_unaligned((c + linha(i as i32) + 9) as *mut usize, h);
            let de = read_usize(read_usize(liga + 0xb1) + linha(i as i32) + 9);
            core::ptr::copy_nonoverlapping(de as *const u8, h as *mut u8, read_i16(liga + 0xa7) as i32 as usize);
            i += 1;
        }
    }
    c
}

/// Comparador: aloca (não roda duas vezes): só a original, com o plano completo conferido depois: o +0xa7 esperado
/// (calculado antes) e a cópia devolvida igual à tabela, linha a linha (fora os ponteiros dos vetores novos), com os
/// vetores de confrontos e de forma iguais aos da tabela.
unsafe extern "thiscall" fn comparar_copiar_tabela_da_liga(liga: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(1300);
    let tab = read_usize(liga + 0xb1);
    let n = read_i16(liga + 0x3e);
    if tab == 0 || n <= 0 || n > 0x32 {
        return o(liga); // ponytail: nada a copiar ou a caixa: só a original
    }
    let a7 = if read_i16(liga + 0xa7) == -1 { n.wrapping_sub(1).wrapping_mul(read_i16(liga + 0x3c)) } else { read_i16(liga + 0xa7) };
    let c = o(liga);
    let mut erros = Vec::new();
    if read_i16(liga + 0xa7) != a7 {
        erros.push(format!("+0xa7 {} esperado {}", read_i16(liga + 0xa7), a7));
    }
    if c != 0 {
        let (h2h, forma) = (read_usize(tab + 5) != 0, read_usize(tab + 9) != 0);
        let n = n as usize;
        for i in 0..n {
            let (lt, lc) = (tab + 0x3b * i, c + 0x3b * i);
            for b in 0..0x3b {
                if (h2h && (5..9).contains(&b)) || (forma && (9..13).contains(&b)) {
                    continue;
                }
                if *((lt + b) as *const u8) != *((lc + b) as *const u8) {
                    erros.push(format!("linha {} +{:#x}", i, b));
                    break;
                }
            }
            let iguais = |o_: usize, t: usize| {
                let (a, b) = (read_usize(lt + o_), read_usize(lc + o_));
                b != 0 && core::slice::from_raw_parts(a as *const u8, t) == core::slice::from_raw_parts(b as *const u8, t)
            };
            if h2h && n > 1 && !iguais(5, 14 * (n - 1)) {
                erros.push(format!("linha {} confrontos", i));
            }
            if forma && a7 > 0 && !iguais(9, a7 as usize) {
                erros.push(format!("linha {} forma", i));
            }
        }
    }
    PLACAR.registrar(c != 0 && erros.is_empty(), || format!("copiar_tabela_da_liga({:x}) n {} copia {:x}: {}", liga, n, c, erros.join("; ")));
    c
}

/// { 0x68a850 } (thiscall, ret 0, league.cpp): `League::set_predicted_positions()`: sem estatística (+8): a caixa
/// (linha 0x189e). Copia os clubes das +0x3e linhas da tabela (+0xb1, 0x3b cada) para um vetor (0x944e46; sem memória:
/// a caixa 0x18a6, sem soltar), ordena pela força (0x4c08f0) e grava a posição i + 1 de cada clube não nulo na
/// estatística (0x4a2540); solta o vetor (0x9452ca).
unsafe extern "thiscall" fn posicoes_previstas_da_liga(liga: usize) {
    if read_usize(liga + 8) == 0 {
        return aviso_em(0x9f15f4, 0x189e);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, i32) = core::mem::transmute(0x4c08f0usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    let v = alocar(((read_i16(liga + 0x3e) as i32) << 2) as u32);
    if v == 0 {
        return aviso_em(0x9f15f4, 0x18a6);
    }
    let mut i = 0i32;
    while i < read_i16(liga + 0x3e) as i32 {
        *((v + 4 * i as usize) as *mut u32) = read_usize(read_usize(liga + 0xb1) + 0x3b * i as usize) as u32;
        i += 1;
    }
    ordenar(v, read_i16(liga + 0x3e) as i32);
    let mut i = 0i32;
    while i < read_i16(liga + 0x3e) as i32 {
        let c = read_usize(v + 4 * i as usize);
        if c != 0 {
            // ponytail: a original empurra o ecx com a posição no byte de baixo (o resto é sobra); a 0x4a2540 lê o byte
            gravar(read_usize(liga + 8), c, (i as u8).wrapping_add(1) as u32);
        }
        i += 1;
    }
    liberar(v);
}

/// Comparador: grava e pode criar registros na estatística (não roda duas vezes): o plano sai antes, sobre uma cópia
/// do vetor ordenada pelo núcleo da 0x4c08f0 (chaves num Vec); depois, o registro de cada clube (a última vez que ele
/// aparece) tem de ter em +0x72 a posição planejada.
unsafe extern "thiscall" fn comparar_posicoes_previstas_da_liga(liga: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1301);
    let n = read_i16(liga + 0x3e) as i32;
    if read_usize(liga + 8) == 0 || n <= 0 {
        return o(liga); // ponytail: a caixa ou nada a ordenar, só a original
    }
    let mut v: Vec<u32> = (0..n as usize).map(|i| read_usize(read_usize(liga + 0xb1) + 0x3b * i) as u32).collect();
    let mut k = vec![0i32; n as usize];
    cpt13_ordenar_com(v.as_mut_ptr(), k.as_mut_ptr(), n);
    o(liga);
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut erros = Vec::new();
    for (i, &c) in v.iter().enumerate() {
        if c == 0 || v[i + 1..].contains(&c) {
            continue;
        }
        let p = (i as u8).wrapping_add(1);
        let e = buscar(read_usize(liga + 8), c as usize);
        if e == 0 || *((e + 0x72) as *const u8) != p {
            erros.push(format!("clube {:x} esperado {} obtido {}", c, p, if e == 0 { -1 } else { *((e + 0x72) as *const u8) as i32 }));
        }
    }
    PLACAR.registrar(erros.is_empty(), || format!("posicoes_previstas_da_liga({:x}) {} clubes: {}", liga, n, erros.join("; ")));
}

// ---------- 254a leva: a projeção da liga para a diretoria: as posições projetadas do clube e as duas ordens da projeção (league.cpp) ----------



/// A entrada da projeção (0xe bytes, no bloco da 0x944cf1): +0 o clube, +4 os pontos por jogo (float), +8 os pontos,
/// +0xa os pontos projetados, +0xc os jogos.
unsafe fn cpr11_entrada(buf: usize, i: i32) -> usize {
    buf.wrapping_add((i as isize * 0xe) as usize)
}

/// { 0x68afb0 } (cdecl, league.cpp): `compare_projection_by_reputation_desc(a, b)`: o qsort das entradas da projeção
/// pela reputação do clube (+0 → +0x80, palavra com sinal), a maior primeiro. Quente (qsort): sem contador.
unsafe extern "C" fn ordem_da_projecao_pela_reputacao(a: usize, b: usize) -> i32 {
    let ra = read_i16(read_usize(a) + 0x80) as i32;
    let rb = read_i16(read_usize(b) + 0x80) as i32;
    rb - ra // o clube de maior reputação primeiro
}

/// { 0x68afd0 } (cdecl, league.cpp): `compare_projection_by_points_desc(a, b)`: o qsort das entradas da projeção pelos
/// pontos projetados (+0xa, palavra com sinal), o maior primeiro. Quente (qsort): sem contador.
unsafe extern "C" fn ordem_da_projecao_pelos_pontos(a: usize, b: usize) -> i32 {
    read_i16(b + 0xa) as i32 - read_i16(a + 0xa) as i32 // os mais pontos primeiro
}

/// A posição (1..) do clube na projeção já ordenada (a primeira entrada dele; `None` sem ela).
unsafe fn cpr11_posicao(buf: usize, n: i16, clube: usize) -> Option<i32> {
    (0..n as i32).find(|&i| read_usize(cpr11_entrada(buf, i)) == clube)
}

/// O clube com outra forma: os pontos por jogo (+4) vezes `fator` (o float guardado; o x87 segue com o de 53 bits) e
/// os pontos projetados de novo com ele.
unsafe fn cpr11_outra_forma(this: usize, e: usize, fator: f64) -> i32 {
    let r = *((e + 4) as *const f32) as f64 * fator;
    *((e + 4) as *mut f32) = r as f32; // fst: guarda o float e segue com o de 53 bits
    let resta = read_i16(this + 0xa7) as i32 - read_i16(e + 0xc) as i32;
    *((e + 0xa) as *mut i16) = (ftol(resta as f64 * r) as i16).wrapping_add(read_i16(e + 8));
    resta
}

/// { 0x68ac40 } (thiscall, ret 4, league.cpp; virtual +0x84 das ligas, chamada só por Comp_Stats 0x4ae300, que a
/// diretoria usa na confiança semanal 0x697c40 e no motivo da saída do técnico 0x69b350):
/// `League::update_projected_positions(clube)`: com o clube na tabela (+0xb1, 0x3b bytes, conta +0x3e: +0 o clube,
/// +0x11 os jogos, +0x19 os pontos), monta num bloco (0x944cf1) uma entrada de 0xe bytes por linha com os pontos por
/// jogo; no primeiro terço da temporada (jogos do clube × 3 < +0xa7), troca-os por uma escala pela reputação
/// (0,83 × +0xc2 do primeiro ao 0,23 × +0xc2 abaixo do último). Projeta os pontos no fim da temporada, ordena e acha a
/// posição do clube na forma normal, melhor (× 1,5) e pior (× 0,444); libera o bloco e entrega as três à virtual
/// +0x68 (`set_club_progress(clube, competição +0x44, normal, melhor, pior)`, que grava os bytes +0x73..+0x75 do
/// registro do clube no Comp_Stats). Devolve 1; fora da tabela, 0.
unsafe extern "thiscall" fn projetar_posicoes_da_liga(this: usize, clube: usize) -> u32 {
    let n = read_i16(this + 0x3e);
    let tabela = read_usize(this + 0xb1);
    let linha = match (0..n as i32).map(|i| tabela.wrapping_add((i as isize * 0x3b) as usize)).find(|&l| read_usize(l) == clube) {
        Some(l) => l,
        None => return 0,
    };
    if linha == 0 || n == 0 {
        return 0;
    }
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let buf = novo((n as i32 * 0xe) as u32);
    let mut i: i16 = 0;
    while i < read_i16(this + 0x3e) {
        let l = read_usize(this + 0xb1).wrapping_add((i as isize * 0x3b) as usize);
        let e = cpr11_entrada(buf, i as i32);
        core::ptr::write_unaligned(e as *mut usize, read_usize(l));
        *((e + 8) as *mut i16) = read_i16(l + 0x19);
        let jogos = *((l + 0x11) as *const i8);
        *((e + 0xc) as *mut i16) = jogos as i16;
        *((e + 4) as *mut f32) = if jogos > 0 { (read_i16(l + 0x19) as f64 / jogos as f64) as f32 } else { 0.0 };
        i += 1;
    }
    // os três locais da original (dwords): o byte baixo de cada um vira a posição; os bytes altos são o que sobrou
    // neles (o float A e a conta das linhas na escala pela reputação, o que resta de jogos do clube no terceiro)
    let (mut normal, mut melhor, mut pior) = (0u32, 0u32, 0u32);
    if (*((linha + 0x11) as *const i8) as i32) * 3 < read_i16(this + 0xa7) as i32 {
        crate::comparacao::ramo("cpr_11 projecao pela reputacao");
        let n = read_i16(this + 0x3e);
        qsort(buf, n as i32 as u32, 0xe, 0x68afb0);
        let c = *((this + 0xc2) as *const i8) as f64;
        let a = (c * f64_em(0x96d0e8)) as f32; // fstp no local: o primeiro arredonda para float
        normal = a.to_bits();
        let b = c * f64_em(0x96d0e0);
        if n > 0 {
            let d = a as f64 - b;
            let passo = d / n as f64;
            for k in 0..n as i32 {
                *((cpr11_entrada(buf, k) + 4) as *mut f32) = (a as f64 - passo * k as f64) as f32; // do maior ao menor
            }
            melhor = n as i32 as u32;
        }
    }
    let mut i: i16 = 0;
    while i < read_i16(this + 0x3e) {
        let e = cpr11_entrada(buf, i as i32);
        let resta = read_i16(this + 0xa7) as i32 - read_i16(e + 0xc) as i32;
        pior = resta as u32;
        *((e + 0xa) as *mut i16) = (ftol(resta as f64 * *((e + 4) as *const f32) as f64) as i16).wrapping_add(read_i16(e + 8));
        i += 1;
    }
    qsort(buf, read_i16(this + 0x3e) as i32 as u32, 0xe, 0x68afd0);
    if let Some(k) = cpr11_posicao(buf, read_i16(this + 0x3e), clube) {
        normal = (normal & !0xff) | (k as u8).wrapping_add(1) as u32; // a posição na forma normal
        pior = cpr11_outra_forma(this, cpr11_entrada(buf, k), f64_em(0x968940)) as u32;
    }
    qsort(buf, read_i16(this + 0x3e) as i32 as u32, 0xe, 0x68afd0);
    if let Some(k) = cpr11_posicao(buf, read_i16(this + 0x3e), clube) {
        melhor = (melhor & !0xff) | (k as u8).wrapping_add(1) as u32;
        pior = cpr11_outra_forma(this, cpr11_entrada(buf, k), f64_em(0x96d0d8)) as u32;
    }
    qsort(buf, read_i16(this + 0x3e) as i32 as u32, 0xe, 0x68afd0);
    for k in 0..read_i16(this + 0x3e) as i32 {
        if read_usize(cpr11_entrada(buf, k)) == clube {
            pior = (pior & !0xff) | (k as u8).wrapping_add(1) as u32; // a última (o clube está uma vez só)
        }
    }
    liberar(buf);
    crate::comparacao::ramo("cpr_11 projecao");
    // ponytail: os bytes altos que a original deixa nos três locais fora da escala pela reputação são sobra de pilha
    // (aqui, zero); as 69 versões da virtual +0x68 só leem o byte baixo de cada argumento (ou repassam os dwords a
    // outra que lê)
    let progresso: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) =
        core::mem::transmute(read_usize(read_usize(this) + 0x68));
    let competicao = (melhor & !0xff) | *((this + 0x44) as *const u8) as u32; // dl sobre o edx do segundo local
    progresso(this, clube, competicao, normal, melhor, pior);
    1
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_da_projecao_pela_reputacao(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1108)(a, b);
    let n = ordem_da_projecao_pela_reputacao(a, b);
    PLACAR.registrar(r == n, || format!("ordem_da_projecao_pela_reputacao({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_da_projecao_pelos_pontos(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1109)(a, b);
    let n = ordem_da_projecao_pelos_pontos(a, b);
    PLACAR.registrar(r == n, || format!("ordem_da_projecao_pelos_pontos({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: o bloco da projeção é alocado e liberado dentro (não fica nada dele); o efeito é a virtual +0x68, que
/// grava os bytes +0x73..+0x75 do registro do clube no Comp_Stats da liga (+8, achado pela 0x4a4850, que dentro da
/// 0x4ae300 devolve o registro provisório dela): lado a lado nesses 3 bytes e na devolução. As duas versões que repassam
/// a outra competição (0x48e380, 0x7ef100) gravam fora da região: aí só a devolução pesa.
unsafe extern "thiscall" fn comparar_projetar_posicoes_da_liga(this: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1107);
    let stats = read_usize(this + 8);
    let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let reg = if stats != 0 { busca(stats, clube) } else { 0 };
    if reg == 0 {
        return o(this, clube); // ponytail: sem registro para olhar, só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(reg + 0x73, 3)], || o(this, clube),
        || projetar_posicoes_da_liga(this, clube));
    PLACAR.registrar(igual && r == n, || format!("projetar_posicoes_da_liga(liga {:x}, clube {:x}) original={} rust={} {:x?} x {:x?}",
        this, clube, r, n, mo, mr));
    r
}

// ---------- 249a leva: a tabela da liga: iniciar a linha do clube, o vetor dos clubes e as posições finais (league.cpp) ----------


unsafe fn cpm2_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}



/// Linhas da tabela: 0x3b bytes (+0 clube, +4 posição, +5 confrontos diretos de 14 bytes, +9 histórico das posições,
/// +0x11/+0x1f/+0x2d os totais, em casa e fora, +0x37 situação, +0x38 marca, +0x39, +0x3a clubes).
const CPM2_LINHA: usize = 0x3b;

/// A regra 4 (confrontos diretos) entre as 4 regras de desempate (+0xc5, até a primeira 0).
unsafe fn cpm2_regra_dos_confrontos(this: usize) -> bool {
    for k in 0..4 {
        match read_u8(this + 0xc5 + k) {
            0 => return false,
            4 => return true,
            _ => {}
        }
    }
    false
}

/// { 0x687430 } (thiscall, ret 0x10, league.cpp): `League::init_table_row(i, clube, marca, linhas)`: põe o clube na
/// linha `i` (char) da tabela `linhas` (nula: a da liga, +0xb1): o clube com situação +0x64 = 3 passa a 2; o total de
/// rodadas +0xa7 ainda −1 vira (clubes +0x3e − 1) × turnos +0x3c (mais de 50 clubes: caixa e segue); a linha recebe o
/// clube, a posição `i`, os três blocos de totais zerados, a marca (+0x38), situação 0xff, 1 em +0x39 e os clubes
/// (+0x3a); com turnos, o histórico das posições (+9, +0xa7 bytes; sem memória, caixa e fim); com a regra 4, os
/// confrontos diretos (+5, 14 bytes por adversário, zerados). Na última linha com confrontos, cada linha recebe os
/// adversários na ordem da tabela.
unsafe extern "thiscall" fn iniciar_linha_da_tabela(this: usize, i: u32, clube: usize, marca: u32, linhas: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    if clube != 0 && read_u8(clube + 0x64) == 3 {
        *((clube + 0x64) as *mut u8) = 2;
    }
    if read_i16(this + 0xa7) == -1 && read_i16(this + 0x3e) > 0 {
        if read_i16(this + 0x3e) > 0x32 {
            aviso_em(0x9f15f4, 0xfe9); // mais de 50 clubes: a caixa, e segue
        }
        let total = (read_i16(this + 0x3e) - 1).wrapping_mul(read_i16(this + 0x3c));
        core::ptr::write_unaligned((this + 0xa7) as *mut i16, total);
    }
    let tab = if linhas == 0 { read_usize(this + 0xb1) } else { linhas };
    let idx = i as u8 as i8 as isize;
    let l = tab.wrapping_add((idx * CPM2_LINHA as isize) as usize);
    cpm2_por(l, clube);
    *((l + 4) as *mut u8) = i as u8;
    for k in 0..3 {
        let e = l + 0x11 + 14 * k;
        cpm2_por(e - 4, 0);
        core::ptr::write_bytes(e as *mut u8, 0, 4);
        core::ptr::write_bytes((e + 4) as *mut u8, 0, 6);
    }
    *((l + 0x38) as *mut u8) = marca as u8;
    *((l + 0x37) as *mut u8) = 0xff;
    *((l + 0x39) as *mut u8) = 1;
    *((l + 0x3a) as *mut u8) = read_u8(this + 0x3e);
    cpm2_por(l + 5, 0);
    if read_i16(this + 0x3c) > 0 {
        let p = alocar(read_i16(this + 0xa7) as i32 as u32);
        cpm2_por(l + 9, p);
        if p == 0 {
            aviso_em(0x9f15f4, 0x100c);
            return;
        }
    } else {
        cpm2_por(l + 9, 0);
    }
    if cpm2_regra_dos_confrontos(this) {
        let n = read_i16(this + 0x3e) as i32;
        cpm2_por(l + 5, alocar((n * 14 - 14) as u32));
        // os confrontos zerados (+4..+0xd de cada um; a original grava o +4 duas vezes, sem conferir a memória)
        let mut k = 0i32;
        while k < read_i16(this + 0x3e) as i32 - 1 {
            let e = read_usize(l + 5) + 14 * k as usize;
            core::ptr::write_bytes((e + 4) as *mut u8, 0, 10);
            k += 1;
        }
    }
    let n = read_i16(this + 0x3e) as isize;
    if idx == n - 1 && read_usize(l + 5) != 0 {
        crate::comparacao::ramo("cpm_2 confrontos diretos");
        let mut a = 0isize;
        while a < read_i16(this + 0x3e) as isize {
            let mut k = 0usize;
            let mut j = 0isize;
            while j < read_i16(this + 0x3e) as isize {
                if a != j {
                    let p = read_usize(tab + a as usize * CPM2_LINHA + 5);
                    cpm2_por(p + 14 * k, read_usize(tab + j as usize * CPM2_LINHA)); // o clube adversário
                    k += 1;
                }
                j += 1;
            }
            a += 1;
        }
    }
}

/// { 0x687a20 } (thiscall, ret 8, league.cpp): `League::get_club_array(saida, n)`: um vetor novo (malloc 0x944e46) com
/// os clubes das linhas da tabela (+0xb1) em `*saida` e a conta (+0x3e) em `*n`; sem memória, caixa e `*n` = 0.
unsafe extern "thiscall" fn vetor_dos_clubes_da_liga(this: usize, saida: *mut usize, n: *mut i16) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let p = alocar((read_i16(this + 0x3e) as i32 * 4) as u32);
    *saida = p;
    if p == 0 {
        aviso_em(0x9f15f4, 0x10cc);
        *n = 0;
        return;
    }
    crate::comparacao::ramo("cpm_2 vetor dos clubes");
    let mut i = 0i16;
    while i < read_i16(this + 0x3e) {
        cpm2_por(p + 4 * i as usize, read_usize(read_usize(this + 0xb1) + i as usize * CPM2_LINHA));
        i += 1;
    }
    *n = read_i16(this + 0x3e); // a conta
}

/// { 0x68aa30 } (thiscall, league.cpp): `League::record_final_positions()`: cada clube da tabela (+0xb1, na ordem) que
/// ainda está na divisão da liga (+0x57 = +4 dela) guarda a divisão como a da última temporada (+0x5b) e a posição
/// (linha + 1) em +0x5f.
unsafe extern "thiscall" fn gravar_posicoes_finais_da_liga(this: usize) {
    let mut i = 0i16;
    while i < read_i16(this + 0x3e) {
        let divisao = read_usize(this + 4);
        let c = read_usize(read_usize(this + 0xb1) + i as usize * CPM2_LINHA);
        if read_usize(c + 0x57) == divisao {
            crate::comparacao::ramo("cpm_2 posicao final");
            cpm2_por(c + 0x5b, divisao);
            *((c + 0x5f) as *mut u8) = (i as u8).wrapping_add(1); // a posição final
        }
        i += 1;
    }
}

/// Comparador: a original aloca (histórico e confrontos): roda só ela e confere a linha com o plano feito antes (o
/// total de rodadas, a situação do clube, os campos, os blocos zerados, os vetores presentes e, na última linha, os
/// adversários de cada linha).
unsafe extern "thiscall" fn comparar_iniciar_linha_da_tabela(this: usize, i: u32, clube: usize, marca: u32, linhas: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize, u32, usize)>(928);
    let a7 = read_i16(this + 0xa7);
    let n = read_i16(this + 0x3e);
    let a7_esperado = if a7 == -1 && n > 0 { (n - 1).wrapping_mul(read_i16(this + 0x3c)) } else { a7 };
    let situacao = if clube != 0 { read_u8(clube + 0x64) } else { 0 };
    let situacao_esperada = if situacao == 3 { 2 } else { situacao };
    let tab = if linhas == 0 { read_usize(this + 0xb1) } else { linhas };
    let confrontos = cpm2_regra_dos_confrontos(this);
    o(this, i, clube, marca, linhas);
    let idx = i as u8 as i8 as isize;
    let l = tab.wrapping_add((idx * CPM2_LINHA as isize) as usize);
    let mut ok = read_usize(l) == clube && read_u8(l + 4) == i as u8 && read_u8(l + 0x37) == 0xff && read_u8(l + 0x38) == marca as u8
        && read_u8(l + 0x39) == 1 && read_u8(l + 0x3a) == n as u8 && read_i16(this + 0xa7) == a7_esperado
        && (clube == 0 || read_u8(clube + 0x64) == situacao_esperada)
        && (read_usize(l + 9) != 0) == (read_i16(this + 0x3c) > 0) && (read_usize(l + 5) != 0) == confrontos;
    for k in 0..3 {
        let e = l + 0x11 + 14 * k;
        ok &= (e - 4..e + 10).all(|a| read_u8(a) == 0);
    }
    let ultima = idx == n as isize - 1 && read_usize(l + 5) != 0;
    if read_usize(l + 5) != 0 && !ultima {
        let p = read_usize(l + 5);
        ok &= (0..(n as usize).saturating_sub(1)).all(|k| (p + 14 * k + 4..p + 14 * k + 14).all(|a| read_u8(a) == 0));
    }
    if ultima {
        for a in 0..n as usize {
            let p = read_usize(tab + a * CPM2_LINHA + 5);
            let outros = (0..n as usize).filter(|&j| j != a);
            ok &= outros.enumerate().all(|(k, j)| read_usize(p + 14 * k) == read_usize(tab + j * CPM2_LINHA));
        }
    }
    PLACAR.registrar(ok, || format!("iniciar_linha_da_tabela(linha {}, clube {:x}, marca {}) clubes {}, rodadas {} (esperava {}), confrontos {} ({:x}), histórico {:x}",
        idx, clube, marca as u8, n, read_i16(this + 0xa7), a7_esperado, confrontos, read_usize(l + 5), read_usize(l + 9)));
}

/// Comparador: a original aloca o vetor: roda só ela e confere a conta e os clubes, na ordem da tabela.
unsafe extern "thiscall" fn comparar_vetor_dos_clubes_da_liga(this: usize, saida: *mut usize, n: *mut i16) {
    orig::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(929)(this, saida, n);
    let p = *saida;
    if p == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = read_i16(this + 0x3e);
    let ok = *n == conta && (0..conta.max(0) as usize).all(|i| read_usize(p + 4 * i) == read_usize(read_usize(this + 0xb1) + i * CPM2_LINHA));
    PLACAR.registrar(ok, || format!("vetor_dos_clubes_da_liga conta {} (tabela {})", *n, conta));
}

/// Comparador: lado a lado pela divisão da última temporada e pela posição (+0x5b..+0x5f) de cada clube da tabela.
unsafe extern "thiscall" fn comparar_gravar_posicoes_finais_da_liga(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(930);
    let reg: Vec<(usize, usize)> = (0..read_i16(this + 0x3e).max(0) as usize)
        .map(|i| (read_usize(read_usize(this + 0xb1) + i * CPM2_LINHA) + 0x5b, 5))
        .collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || gravar_posicoes_finais_da_liga(this));
    PLACAR.registrar(igual, || format!("gravar_posicoes_finais_da_liga {:x?} x {:x?}", mo, mr));
}

// ---------- 242a leva: o resultado na tabela da liga e os pontos do resultado (league.cpp) ----------


unsafe fn cpe2_mais1(a: usize) {
    *(a as *mut u8) = (*(a as *const u8)).wrapping_add(1);
}

unsafe fn cpe2_somar(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, core::ptr::read_unaligned(a as *const i16).wrapping_add(v));
}

/// Linhas da tabela: 0x3b bytes; +0 clube, +5 confrontos (entradas de 14), +0x11..+0x1a os totais, +0x1f..+0x28 em
/// casa (jogos, vitórias, empates, derrotas, gols pró, contra, pontos), +0x2d..+0x36 fora.
const CPE2_LINHA: usize = 0x3b;

/// { 0x685f60 } (thiscall, ret 0x18, league.cpp): `League::apply_result_to_table(partida, tabela, n, marcar,
/// pts_casa*, pts_fora*)`: acha as linhas do mandante (+0x1c) e do visitante (+0x20) entre as `n` (curto) da tabela
/// (para no primeiro clube que não é nenhum dos dois depois de achar ambos) e soma o jogo: jogos (+0x1f/+0x2d), gols
/// (+0x23/+0x25 e +0x31/+0x33: o placar de +0x47/+0x48 quando ≠ 0xff, senão o de +0x43/+0x44). O resultado pelo placar
/// +0x43/+0x44; empatado: pela prorrogação (+0x38 bit 2, +0x47/+0x48; `marcar` grava 3 em +0x3d/+0x3e) e pelos pênaltis
/// (bits 0x20/1, +0x49/+0x4a; vitória do mandante conta como empate dele; 5 com o 0x20, senão 4); empate puro com os
/// pontos de empate da liga (+0xc3). Os pontos de vitória pela virtual +0x90 (0x68aad0) da competição de tipo 2
/// ([0xadadfc], +0x43) ou da própria liga, lida a cada chamada. Refaz os totais das duas linhas e devolve os pontos
/// (+0x19) em `pts_casa`/`pts_fora` (0 sem a linha). Com confrontos na primeira linha (+5), acha o adversário em cada
/// uma e soma o confronto (0x6869a0). Sem SEH, sem sorteio, sem caixa, locais todas gravadas antes de lidas.
unsafe extern "thiscall" fn aplicar_resultado_na_tabela(liga: usize, p: usize, tab: usize, n: u32, marcar: u32, pts_casa: *mut i16, pts_fora: *mut i16) {
    let mut pontuador = liga;
    let id = read_usize(liga + 4);
    if id != 0 && read_usize(0xadadfc) != 0 {
        let comp = read_usize(read_usize(0xadadfc).wrapping_add(read_usize(id).wrapping_mul(4)));
        if *((comp + 0x43) as *const u8) == 2 {
            pontuador = comp;
        }
    }
    let n = n as i16;
    let linha = |k: i16| tab.wrapping_add(CPE2_LINHA.wrapping_mul(k as i32 as usize));
    let (mut casa, mut fora): (i16, i16) = (-1, -1);
    let mut k: i16 = 0;
    if n > 0 {
        loop {
            let c = read_usize(linha(k));
            if c == read_usize(p + 0x1c) {
                casa = k;
            } else if c == read_usize(p + 0x20) {
                fora = k;
            } else if casa != -1 && fora != -1 {
                break;
            }
            k = k.wrapping_add(1);
            if k >= n {
                break;
            }
        }
    }
    let pb = |o: usize| *((p + o) as *const i8);
    let noventa = || *((p + 0x47) as *const u8) != 0xff;
    if casa > -1 {
        let r = linha(casa);
        cpe2_mais1(r + 0x1f);
        let (pro, contra) = if noventa() { (pb(0x47), pb(0x48)) } else { (pb(0x43), pb(0x44)) };
        cpe2_somar(r + 0x23, pro as i16);
        cpe2_somar(r + 0x25, contra as i16);
    }
    if fora > -1 {
        let r = linha(fora);
        cpe2_mais1(r + 0x2d);
        let (pro, contra) = if noventa() { (pb(0x48), pb(0x47)) } else { (pb(0x44), pb(0x43)) };
        cpe2_somar(r + 0x31, pro as i16);
        cpe2_somar(r + 0x33, contra as i16);
    }
    let pontos = |clube: usize| -> i16 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(pontuador) + 0x90));
        f(pontuador, clube, p) as u8 as i8 as i16
    };
    // vence o mandante: +0x20 e os pontos (+0x27) dele, derrota (+0x30) do visitante; perde: +0x22 e +0x2e com os pontos (+0x35)
    let vence_casa = |vitoria: usize| {
        if casa > -1 {
            let r = linha(casa);
            cpe2_mais1(r + vitoria);
            cpe2_somar(r + 0x27, pontos(read_usize(p + 0x1c))); // os pontos do mandante
        }
        if fora > -1 {
            cpe2_mais1(linha(fora) + 0x30);
        }
    };
    let vence_fora = |vitoria: usize| {
        if casa > -1 {
            cpe2_mais1(linha(casa) + 0x22);
        }
        if fora > -1 {
            let r = linha(fora);
            cpe2_mais1(r + vitoria);
            cpe2_somar(r + 0x35, pontos(read_usize(p + 0x20)));
        }
    };
    let (gc, gf) = (pb(0x43), pb(0x44));
    let flags = core::ptr::read_unaligned((p + 0x38) as *const u16);
    let penaltis = if *((p + 0x38) as *const u8) & 0x20 != 0 { 5u8 } else { 4 };
    if gc > gf {
        vence_casa(0x20);
    } else if gc < gf {
        vence_fora(0x2e);
    } else if flags & 2 != 0 && pb(0x47) > pb(0x48) {
        vence_casa(0x20);
        if marcar != 0 {
            *((p + 0x3d) as *mut u8) = 3;
        }
    } else if flags & 2 != 0 && pb(0x47) < pb(0x48) {
        vence_fora(0x2e);
        if marcar != 0 {
            *((p + 0x3e) as *mut u8) = 3;
        }
    } else if flags & 0x21 != 0 && pb(0x49) > pb(0x4a) {
        vence_casa(0x21);
        if marcar != 0 {
            *((p + 0x3d) as *mut u8) = penaltis;
        }
    } else if flags & 0x21 != 0 && pb(0x49) < pb(0x4a) {
        vence_fora(0x2f);
        if marcar != 0 {
            *((p + 0x3e) as *mut u8) = penaltis;
        }
    } else if flags & 0x21 == 0 {
        let empate = *((liga + 0xc3) as *const i8) as i16; // o empate puro
        if casa > -1 {
            cpe2_mais1(linha(casa) + 0x21);
            cpe2_somar(linha(casa) + 0x27, empate);
        }
        if fora > -1 {
            cpe2_mais1(linha(fora) + 0x2f);
            cpe2_somar(linha(fora) + 0x35, empate);
        }
    }
    let totais = |r: usize| -> i16 {
        for (t, f, c) in [(0x11usize, 0x2d, 0x1f), (0x12, 0x2e, 0x20), (0x13, 0x2f, 0x21), (0x14, 0x30, 0x22)] {
            *((r + t) as *mut u8) = (*((r + f) as *const u8)).wrapping_add(*((r + c) as *const u8));
        }
        for (t, f, c) in [(0x15usize, 0x31, 0x23), (0x17, 0x33, 0x25), (0x19, 0x35, 0x27)] {
            let v = core::ptr::read_unaligned((r + f) as *const i16).wrapping_add(core::ptr::read_unaligned((r + c) as *const i16));
            core::ptr::write_unaligned((r + t) as *mut i16, v);
        }
        core::ptr::read_unaligned((r + 0x19) as *const i16)
    };
    for (k, pts) in [(casa, pts_casa), (fora, pts_fora)] {
        let v = if k > -1 { totais(linha(k)) } else { 0 };
        if !pts.is_null() {
            core::ptr::write_unaligned(pts, v);
        }
    }
    if read_usize(tab + 5) != 0 {
        let m = n as i32 - 1;
        let (mut ia, mut ib) = (-1i32, -1i32);
        let entrada = |linha_: i16, k: i16| read_usize(read_usize(linha(linha_) + 5).wrapping_add(14 * k as i32 as usize));
        let mut k: i16 = 0;
        if m > 0 {
            loop {
                if casa > -1 && entrada(casa, k) == read_usize(p + 0x20) {
                    ia = k as i32;
                    if ib > -1 {
                        break;
                    }
                }
                if fora > -1 && entrada(fora, k) == read_usize(p + 0x1c) {
                    ib = k as i32;
                    if ia > -1 {
                        break;
                    }
                }
                k = k.wrapping_add(1);
                if k as i32 >= m {
                    break;
                }
            }
        }
        let confronto: unsafe extern "thiscall" fn(usize, usize, i32, i32, i8, i8) = core::mem::transmute(0x6869a0usize);
        confronto(liga, tab, casa as i32, ia, gc, gf);
        confronto(liga, tab, fora as i32, ib, gf, gc);
    }
}

/// As regiões da 0x685f60: as `n` linhas, os confrontos de cada linha ((n − 1) entradas de 14, se a primeira os tem),
/// +0x3d/+0x3e da partida e os dois pontos devolvidos.
unsafe fn cpe2_regioes(p: usize, tab: usize, n: u32, pts_casa: *mut i16, pts_fora: *mut i16) -> Vec<(usize, usize)> {
    let n = (n as i16).max(0) as usize;
    let mut reg = vec![(p + 0x3d, 2)];
    if tab != 0 && n > 0 {
        reg.push((tab, CPE2_LINHA * n));
        if read_usize(tab + 5) != 0 && n > 1 {
            for i in 0..n {
                let h = read_usize(tab + CPE2_LINHA * i + 5);
                if h != 0 {
                    reg.push((h, 14 * (n - 1)));
                }
            }
        }
    }
    for pts in [pts_casa, pts_fora] {
        if !pts.is_null() {
            reg.push((pts as usize, 2));
        }
    }
    reg
}

unsafe extern "thiscall" fn comparar_aplicar_resultado_na_tabela(liga: usize, p: usize, tab: usize, n: u32, marcar: u32, pts_casa: *mut i16, pts_fora: *mut i16) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut i16, *mut i16)>(802);
    let reg = cpe2_regioes(p, tab, n, pts_casa, pts_fora);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(liga, p, tab, n, marcar, pts_casa, pts_fora),
        || aplicar_resultado_na_tabela(liga, p, tab, n, marcar, pts_casa, pts_fora));
    PLACAR.registrar(igual, || format!("aplicar_resultado_na_tabela({:x}, partida {:x}, {:x}, n {}, marcar {}) {:x?} x {:x?}",
        liga, p, tab, n as i16, marcar, mo, mr));
}

/// { 0x68aad0 } (thiscall, ret 8, league.cpp): `League::points_for_result(clube, partida)`: os pontos de vitória
/// (+0xc2) se o clube venceu (mandante +0x1c com +0x43 > +0x44, visitante +0x20 com menos), senão 0; empatado: pela
/// prorrogação (+0xab bit 2, +0x47/+0x48, os de vitória), pelos pênaltis (bits 0x21, +0x49/+0x4a: os de empate +0xc3
/// a quem venceu, 0 a quem perdeu) ou os de empate. Só o al é escrito: os 3 bytes altos do eax são os da partida.
unsafe extern "thiscall" fn pontos_do_resultado(liga: usize, clube: usize, p: usize) -> u32 {
    let pb = |o: usize| *((p + o) as *const i8);
    let (casa, fora) = (read_usize(p + 0x1c), read_usize(p + 0x20));
    let venceu = |a: i8, b: i8| (clube == casa && a > b) || (clube == fora && a < b);
    let ganho = |o: usize| *((liga + o) as *const u8);
    let flags = *((liga + 0xab) as *const u8);
    let v = if pb(0x43) != pb(0x44) {
        if venceu(pb(0x43), pb(0x44)) { ganho(0xc2) } else { 0 } // os pontos da vitória
    } else if flags & 2 != 0 && pb(0x47) != pb(0x48) {
        if venceu(pb(0x47), pb(0x48)) { ganho(0xc2) } else { 0 }
    } else if flags & 0x21 != 0 && pb(0x49) != pb(0x4a) {
        if venceu(pb(0x49), pb(0x4a)) { ganho(0xc3) } else { 0 }
    } else {
        ganho(0xc3)
    };
    (p as u32 & 0xffff_ff00) | v as u32
}

comparar_valor!(comparar_pontos_do_resultado, pontos_do_resultado, 803, (clube: usize, p: usize) -> u32);

// ---------- 242a leva: a ordem pelo confronto guardado, a ordem das datas e a linha do clube na tabela da liga (league.cpp) ----------



/// { 0x683410 } (cdecl, league.cpp): `league_compare_table_rows_h2h(a, b)` (callback do qsort da 0x6827d0 na regra
/// 4 e da 0x6471a0 da Série B italiana): linha vazia empata; a comparação própria [0xb67a00] quando não dá 0 (chamada
/// de novo); os pontos (+0x19 da coluna [[0xb679fc]+0xad], b − a); os dois sem jogos (+0x11): o id dos clubes (a − b);
/// a posição do confronto guardada em +4 (b − a); por fim o nome dos clubes (+4, strcmp: −1, 0 ou 1).
unsafe extern "C" fn ordem_pela_posicao_do_confronto(a: usize, b: usize) -> i32 {
    let col = read_usize(read_usize(0xb679fc) + 0xad);
    if read_usize(a) == 0 || read_usize(b) == 0 {
        return 0;
    }
    if read_usize(0xb67a00) != 0 {
        let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(read_usize(0xb67a00));
        if f(a, b) != 0 {
            let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(read_usize(0xb67a00));
            return f(a, b);
        }
    }
    let o = col.wrapping_mul(14);
    let (pa, pb) = (read_i16(a + o + 0x19), read_i16(b + o + 0x19));
    if pa != pb {
        return pb as i32 - pa as i32;
    }
    if *((a + o + 0x11) as *const u8) == 0 && *((b + o + 0x11) as *const u8) == 0 {
        return (read_usize(read_usize(a)) as i32).wrapping_sub(read_usize(read_usize(b)) as i32);
    }
    let dif = *((b + 4) as *const i8) as i32 - *((a + 4) as *const i8) as i32; // a posição do confronto
    if dif != 0 {
        return dif;
    }
    let (mut x, mut y) = (read_usize(a) + 4, read_usize(b) + 4);
    loop {
        let (cx, cy) = (*(x as *const u8), *(y as *const u8));
        if cx != cy {
            return if cx < cy { -1 } else { 1 };
        }
        if cx == 0 {
            return 0;
        }
        x += 1;
        y += 1;
    }
}

unsafe extern "C" fn comparar_ordem_pela_posicao_do_confronto(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(804)(a, b);
    let n = ordem_pela_posicao_do_confronto(a, b);
    PLACAR.registrar(r == n, || format!("ordem_pela_posicao_do_confronto({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x685cf0 } (cdecl, league.cpp): `league_compare_dates(a, b)` (callback do bsearch da 0x685640 na tabela de
/// datas da liga): o ano (+2, sem sinal) e depois o dia (+0, com sinal): −1, 0 ou 1.
unsafe extern "C" fn ordem_das_datas_da_liga(a: usize, b: usize) -> i32 {
    let (ya, yb) = (read_i16(a + 2) as u16, read_i16(b + 2) as u16);
    if ya < yb { // o ano antes
        return -1;
    }
    if ya > yb {
        return 1;
    }
    let (da, db) = (read_i16(a), read_i16(b));
    if da < db {
        -1
    } else {
        (da > db) as i32
    }
}

unsafe extern "C" fn comparar_ordem_das_datas_da_liga(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(805)(a, b);
    let n = ordem_das_datas_da_liga(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_datas_da_liga({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x68ab70 } (thiscall, ret 8, league.cpp): `League::find_club_row(clube, linhas)`: a linha (0x3b bytes) do
/// clube entre as +0x3e (curto) de `linhas` (nula: a tabela da liga, +0xb1), ou 0.
unsafe extern "thiscall" fn linha_do_clube(liga: usize, clube: usize, linhas: usize) -> usize {
    let t = if linhas == 0 { read_usize(liga + 0xb1) } else { linhas };
    let n = read_i16(liga + 0x3e);
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let r = t.wrapping_add(0x3b * i as i32 as usize);
            if read_usize(r) == clube { // a linha do clube
                return r;
            }
            i = i.wrapping_add(1);
            if i >= n {
                break;
            }
        }
    }
    0
}

comparar_valor!(comparar_linha_do_clube, linha_do_clube, 806, (clube: usize, linhas: usize) -> usize);

/// { 0x68abd0 } (thiscall, ret 4, league.cpp; virtual +0x80 das ligas): `League::contains_club(clube)`: 1 se o
/// clube tem linha na tabela da liga (+0xb1, +0x3e linhas; a linha no endereço 0 conta como não), senão 0.
unsafe extern "thiscall" fn liga_tem_o_clube(liga: usize, clube: usize) -> u32 {
    let t = read_usize(liga + 0xb1);
    let n = read_i16(liga + 0x3e);
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let r = t.wrapping_add(0x3b * i as i32 as usize);
            if read_usize(r) == clube {
                return (r != 0) as u32; // a liga tem o clube
            }
            i = i.wrapping_add(1);
            if i >= n {
                break;
            }
        }
    }
    0
}

comparar_valor!(comparar_liga_tem_o_clube, liga_tem_o_clube, 807, (clube: usize) -> u32);

// ---------- 241a leva: a tabela da liga: ordenar, a minitabela do confronto direto, liberar e conferir o fim (league.cpp) ----------




/// Linhas da tabela: 0x3b bytes; +0 clube, +4 posição, +5 confrontos diretos (entradas de 14 bytes), +9 histórico
/// das posições (um byte por rodada), +0x11 jogos e, a cada 14 bytes por coluna (+0xad), os pontos em +0x19.
const CPA1_LINHA: usize = 0x3b;

/// { 0x6827d0 } (thiscall, league.cpp): `League::sort_table(linhas)`: ordena a tabela (`linhas` nula: a da liga,
/// +0xb1). Antes acerta o total de rodadas +0xa7 (−1: (+0x3e − 1) × +0x3c, com clubes e +0x45). A regra 6 (+0xc5,
/// sem coluna +0xad) só ordena (qsort 0x945333 com 0x683210); a regra 4 com confrontos monta a minitabela
/// (0x682b30) e, sendo a primeira regra, ordena com 0x683410 e passa bolha nas linhas da liga (+0xb1) enquanto
/// trocar: empate em pontos (ou na comparação própria +0xa3) decidido pela 0x683210; senão qsort com 0x683210.
/// Por fim (+0x44 ≠ 0xfe, sem coluna) grava a posição em +4 e no histórico +9 (rodada +0x11 até +0xa7). Guarda a
/// liga em [0xb679fc] e a comparação própria em [0xb67a00]. Devolve 1. Sem SEH, sem sorteio.
unsafe extern "thiscall" fn ordenar_tabela_da_liga(liga: usize, linhas: usize) -> u32 {
    let n = || read_i16(liga + 0x3e);
    let linhas = if linhas == 0 { read_usize(liga + 0xb1) } else { linhas };
    if read_i16(liga + 0xa7) == -1 && n() > 0 && read_usize(liga + 0x45) != 0 {
        core::ptr::write_unaligned((liga + 0xa7) as *mut i16, (n() - 1).wrapping_mul(read_i16(liga + 0x3c)));
    }
    if linhas == 0 {
        return 1;
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let ordem: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x683210usize);
    *(0xb679fc as *mut usize) = liga;
    let coluna = read_usize(liga + 0xad);
    let mut so_ordenar = false;
    let mut confronto = false;
    for k in 0..4usize {
        let regra = read_u8(liga + 0xc5 + k);
        if coluna == 0 && regra == 6 {
            so_ordenar = true;
            break;
        }
        if regra == 0 {
            break;
        }
        if regra == 4 && n() != 0 && read_usize(linhas + 5) != 0 {
            confronto = k == 0;
            let mini: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x682b30usize);
            mini(liga, linhas);
            break;
        }
    }
    if so_ordenar {
        qsort(linhas, n() as i32, CPA1_LINHA as u32, 0x683210);
    } else {
        *(0xb67a00 as *mut usize) = read_usize(liga + 0xa3);
        if confronto {
            qsort(linhas, n() as i32, CPA1_LINHA as u32, 0x683410);
            loop {
                let mut trocou = false;
                if n() as i32 - 1 <= 0 {
                    break;
                }
                let mut i: i8 = 0;
                loop {
                    let (oa, ob) = (CPA1_LINHA * i as i32 as usize, CPA1_LINHA * (i as i32 + 1) as usize);
                    let (a, b) = (read_usize(liga + 0xb1) + oa, read_usize(liga + 0xb1) + ob);
                    let col = read_usize(*(0xb679fc as *const usize) + 0xad);
                    let decidir = read_usize(a) == 0 || read_usize(b) == 0 || cpa1_empate(a, b, col);
                    if decidir && ordem(read_usize(liga + 0xb1) + oa, read_usize(liga + 0xb1) + ob) > 0 {
                        let t = read_usize(liga + 0xb1);
                        core::ptr::swap_nonoverlapping((t + oa) as *mut u8, (t + ob) as *mut u8, CPA1_LINHA); // a troca das linhas
                        trocou = true;
                    }
                    i = i.wrapping_add(1);
                    if i as i32 >= n() as i32 - 1 {
                        break;
                    }
                }
                if !trocou {
                    break;
                }
            }
        } else {
            qsort(linhas, n() as i32, CPA1_LINHA as u32, 0x683210);
        }
    }
    if read_u8(liga + 0x44) != 0xfe && read_usize(liga + 0xad) == 0 && n() > 0 {
        let mut c: i8 = 0;
        loop {
            let r = linhas + CPA1_LINHA * c as i32 as usize;
            *((r + 4) as *mut i8) = c; // a posição
            let hist = read_usize(r + 9);
            let rodada = read_u8(r + 0x11) as i8;
            if hist != 0 && rodada > 0 && rodada as i16 <= read_i16(liga + 0xa7) {
                *((hist + rodada as usize - 1) as *mut i8) = c;
            }
            c = c.wrapping_add(1);
            if c as i16 >= n() {
                break;
            }
        }
    }
    1
}

/// A bolha da 0x6827d0 decide o par (a, b) pela 0x683210? Só quando empatam: na comparação própria [0xb67a00]
/// (chamada de novo quando dá diferente de 0) ou, sem ela, nos pontos (+0x19 da coluna) e, com os dois sem jogos
/// (+0x11), no id dos clubes; com jogos, no confronto direto (gols +5/+7 e +8/+0xa da entrada do clube b).
unsafe fn cpa1_empate(a: usize, b: usize, col: usize) -> bool {
    let propria = *(0xb67a00 as *const usize);
    if propria != 0 {
        let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(propria);
        if f(a, b) != 0 {
            let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(*(0xb67a00 as *const usize));
            return f(a, b) == 0;
        }
    }
    let o = col.wrapping_mul(14);
    let (pa, pb) = (read_i16(a + o + 0x19), read_i16(b + o + 0x19));
    if pa != pb {
        return false;
    }
    if read_u8(a + o + 0x11) == 0 && read_u8(b + o + 0x11) == 0 {
        let (ca, cb) = (read_usize(a), read_usize(b));
        if ca == 0 || cb == 0 {
            return true;
        }
        return *(ca as *const i32) == *(cb as *const i32);
    }
    let m = read_i16(*(0xb679fc as *const usize) + 0x3e) as i32 - 1;
    if m <= 0 {
        return true;
    }
    let (h, cb) = (read_usize(a + 5), read_usize(b));
    let mut k: i16 = 0;
    loop {
        let e = h.wrapping_add(14 * k as i32 as usize);
        if read_usize(e) == cb {
            if e == 0 {
                return true;
            }
            return read_u8(e + 5) == read_u8(e + 7) && read_i16(e + 8) == read_i16(e + 0xa);
        }
        k = k.wrapping_add(1);
        if k as i32 >= m {
            return true;
        }
    }
}

/// As regiões da ordenação: as linhas pedidas e as da liga, o total de rodadas, as duas globais e o histórico de
/// posições de cada linha (o byte gravado vai até a rodada +0x11, um byte com sinal: no máximo 127).
unsafe fn cpa1_regioes_da_ordem(liga: usize, linhas: usize) -> Vec<(usize, usize)> {
    let n = read_i16(liga + 0x3e).max(0) as usize;
    let linhas = if linhas == 0 { read_usize(liga + 0xb1) } else { linhas };
    let mut reg = vec![(liga + 0xa7, 2), (0xb679fc, 4), (0xb67a00, 4)];
    if linhas == 0 {
        return reg;
    }
    reg.push((linhas, CPA1_LINHA * n));
    let propria = read_usize(liga + 0xb1);
    if propria != 0 && propria != linhas {
        reg.push((propria, CPA1_LINHA * n));
    }
    let mut total = read_i16(liga + 0xa7);
    if total == -1 && n > 0 && read_usize(liga + 0x45) != 0 {
        total = (n as i16 - 1).wrapping_mul(read_i16(liga + 0x3c));
    }
    let tam = total.clamp(0, 127) as usize;
    if tam > 0 {
        for i in 0..n {
            let hist = read_usize(linhas + CPA1_LINHA * i + 9);
            if hist != 0 {
                reg.push((hist, tam));
            }
        }
    }
    reg
}

unsafe extern "thiscall" fn comparar_ordenar_tabela_da_liga(liga: usize, linhas: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(773);
    let reg = cpa1_regioes_da_ordem(liga, linhas);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(liga, linhas), || ordenar_tabela_da_liga(liga, linhas));
    PLACAR.registrar(igual && r == n, || format!("ordenar_tabela_da_liga({:x}, {:x}) {}/{} {:x?} x {:x?}", liga, linhas, r, n, mo, mr));
    r
}

/// { 0x682b30 } (thiscall, league.cpp): `League::build_h2h_table(linhas)` (sem nome no mapa da comunidade): uma
/// minitabela (calloc 0x945501 de +0x3e linhas) com o clube de cada linha e, para cada outro clube com os mesmos
/// pontos (+0x19), a soma do confronto direto contra ele (entrada de +5: bytes +4..+7 em +0x11..+0x14, palavras
/// +8/+0xa/+0xc em +0x15/+0x17/+0x19); ordena com 0x6834f0 e grava em +4 de cada linha a posição, que começa no
/// número de clubes e desce a cada par da minitabela que a 0x6834f0 separa. Solta a minitabela (0x9452ca). Sem
/// memória: a original (a caixa do league.cpp). Chamada pela 0x6827d0 na regra 4.
unsafe extern "thiscall" fn mini_tabela_do_confronto(liga: usize, linhas: usize) {
    let n = || read_i16(liga + 0x3e);
    let calloc: unsafe extern "C" fn(i32, u32) -> usize = core::mem::transmute(0x945501usize);
    let t = calloc(n() as i32, CPA1_LINHA as u32);
    if t == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(774)(liga, linhas); // caixa: só a original
    }
    let linha = |base: usize, i: i16| base + CPA1_LINHA * i as i32 as usize;
    let mut i: i16 = 0;
    if n() > 0 {
        loop {
            let (ri, mi) = (linha(linhas, i), linha(t, i));
            *(mi as *mut usize) = read_usize(ri);
            let mut j: i16 = 0;
            if n() > 0 {
                loop {
                    let rj = linha(linhas, j);
                    if i != j && read_i16(ri + 0x19) == read_i16(rj + 0x19) {
                        let m = n() as i32 - 1;
                        let (h, cj) = (read_usize(ri + 5), read_usize(rj));
                        let mut k: i16 = 0;
                        while (k as i32) < m {
                            let e = h + 14 * k as i32 as usize;
                            if read_usize(e) == cj {
                                for (de, para) in [(4, 0x11), (5, 0x12), (6, 0x13), (7, 0x14)] {
                                    *((mi + para) as *mut u8) = read_u8(mi + para).wrapping_add(read_u8(e + de)); // o confronto somado
                                }
                                for (de, para) in [(8, 0x15), (0xa, 0x17), (0xc, 0x19)] {
                                    let v = read_i16(mi + para).wrapping_add(read_i16(e + de));
                                    core::ptr::write_unaligned((mi + para) as *mut i16, v);
                                }
                                break;
                            }
                            k = k.wrapping_add(1);
                        }
                    }
                    j = j.wrapping_add(1);
                    if j >= n() {
                        break;
                    }
                }
            }
            i = i.wrapping_add(1);
            if i >= n() {
                break;
            }
        }
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(t, n() as i32, CPA1_LINHA as u32, 0x6834f0);
    let separa: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x6834f0usize);
    let mut pos = n() as u8;
    let mut a: i16 = 0;
    if n() > 0 {
        loop {
            let clube = read_usize(linha(t, a));
            let mut c: i16 = 0;
            while c < n() {
                if clube == read_usize(linha(linhas, c)) {
                    *((linha(linhas, c) + 4) as *mut u8) = pos;
                    break;
                }
                c = c.wrapping_add(1);
            }
            if (a as i32 + 1) < n() as i32 && separa(linha(t, a), linha(t, a + 1)) != 0 {
                pos = pos.wrapping_sub(1); // a posição desce
            }
            a = a.wrapping_add(1);
            if a >= n() {
                break;
            }
        }
    }
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    free(t);
}

unsafe extern "thiscall" fn comparar_mini_tabela_do_confronto(liga: usize, linhas: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(774);
    if linhas == 0 {
        return o(liga, linhas);
    }
    let reg = [(linhas, CPA1_LINHA * read_i16(liga + 0x3e).max(0) as usize)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(liga, linhas), || mini_tabela_do_confronto(liga, linhas));
    PLACAR.registrar(igual, || format!("mini_tabela_do_confronto({:x}, {:x}) {:x?} x {:x?}", liga, linhas, mo, mr));
}

/// { 0x687970 } (thiscall, league.cpp): `League::free_table(tabela)`: `tabela` nula = a da liga (+0xb1). Com clubes
/// (+0x3e ≥ 1) e tabela: solta (0x9452ca) os vetores +5 de cada linha (se o da primeira não é nulo), depois os +9
/// (idem) e a tabela; sendo a da liga, zera +0x3e e +0xb1. Sem SEH, sem sorteio.
unsafe extern "thiscall" fn liberar_tabela_da_liga(liga: usize, tabela: usize) {
    let (t, propria) = if tabela == 0 { (read_usize(liga + 0xb1), true) } else { (tabela, false) };
    if read_i16(liga + 0x3e) < 1 || t == 0 {
        return;
    }
    crate::comparacao::ramo("liga liberar tabela");
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    for campo in [5usize, 9] {
        if read_usize(t + campo) != 0 {
            let mut i = 0usize;
            while (i as i32) < read_i16(liga + 0x3e) as i32 {
                let p = read_usize(t + campo + CPA1_LINHA * i);
                if p != 0 {
                    free(p);
                }
                i += 1;
            }
        }
    }
    free(t);
    if propria { // a da liga: zera a contagem e a tabela
        core::ptr::write_unaligned((liga + 0x3e) as *mut i16, 0);
        core::ptr::write_unaligned((liga + 0xb1) as *mut usize, 0);
    }
}

/// Comparador: liberar duas vezes soltaria os mesmos blocos duas vezes.
// ponytail: só a original (como as liberações da 232ª); a prova é a carreira e o ramo da cópia no modo Rust.
unsafe extern "thiscall" fn comparar_liberar_tabela_da_liga(liga: usize, tabela: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(775)(liga, tabela)
}

/// { 0x689ad0 } (thiscall, league.cpp): `League::check_complete(fases, avisar)`: 0 sem tabela, encerrada (+0xb5) ou
/// sem +0x45. Fora da fase única (+0x44 ≠ 0xff) já completa (+0xdd): 1. Não completa: acerta o total de rodadas
/// +0xa7 (−1) e, se algum clube ainda não jogou todas (+0x11 < +0xa7), 0; senão, nas divisões de liga (0x4b0700)
/// com +0xa9 > 8 e sem +0xe6, os registros de fim de liga (0x86a070), e sem +0xe6 os prêmios (0x682e10). Com
/// `fases` = 1 na fase única: cada fase de +0xc (0..=+0x30) tem de estar completa (virtual +0x10 com 0, 1) e
/// +0x30 ≥ +0x2c − 1, senão 0. Marca +0xdd (antes, com `avisar`, a virtual +0x94) e devolve 1. Sem SEH, sem sorteio.
unsafe extern "thiscall" fn liga_completa(liga: usize, fases: u32, avisar: u32) -> u8 {
    let tab = read_usize(liga + 0xb1);
    if tab == 0 || read_u8(liga + 0xb5) != 0 || read_usize(liga + 0x45) == 0 {
        return 0;
    }
    let unica = read_u8(liga + 0x44) == 0xff;
    if read_usize(liga + 0xdd) == 0 {
        if read_i16(liga + 0xa7) == -1 && read_i16(liga + 0x3e) > 0 {
            core::ptr::write_unaligned((liga + 0xa7) as *mut i16, (read_i16(liga + 0x3e) - 1).wrapping_mul(read_i16(liga + 0x3c)));
        }
        let rodadas = read_i16(liga + 0xa7);
        for i in 0..read_i16(liga + 0x3e).max(0) as usize {
            if ((read_u8(tab + CPA1_LINHA * i + 0x11) as i8) as i16) < rodadas { // ainda falta jogar
                return 0;
            }
        }
        crate::comparacao::ramo("liga completa: fim da liga");
        let divisao: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x4b0700usize);
        let id = *(read_usize(liga + 4) as *const u32);
        let mut premios = true;
        if divisao(id) != 0 && read_i16(liga + 0xa9) > 8 {
            if read_usize(liga + 0xe6) != 0 {
                premios = false;
            } else {
                let registros: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x86a070usize);
                registros(*(0xdd7f00 as *const usize), liga);
            }
        }
        if premios && read_usize(liga + 0xe6) == 0 {
            let pagar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x682e10usize);
            pagar(liga);
        }
    } else if !unica {
        return 1;
    }
    if fases == 1 && read_u8(liga + 0x44) == 0xff {
        crate::comparacao::ramo("liga completa: fases");
        let mut k = 0usize;
        if read_usize(liga + 0x30) as i32 >= 0 {
            loop {
                let fase = read_usize(read_usize(liga + 0xc) + 4 * k);
                let completa: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(fase) + 0x10));
                if completa(fase, 0, 1) == 0 {
                    return 0;
                }
                k += 1;
                if k as i32 > read_usize(liga + 0x30) as i32 {
                    break;
                }
            }
        }
        if (read_usize(liga + 0x30) as i32) < (read_usize(liga + 0x2c) as i32).wrapping_sub(1) {
            return 0;
        }
    }
    if read_usize(liga + 0xdd) == 0 {
        if avisar != 0 {
            let fim: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(liga) + 0x94));
            fim(liga);
        }
        core::ptr::write_unaligned((liga + 0xdd) as *mut u32, 1);
    }
    1
}

/// O caminho da 0x689ad0 que só lê (e acerta +0xa7)? Escrito à parte da cópia: sem tabela, encerrada ou sem +0x45;
/// completa fora da fase única, ou na única sem pedir as fases; ou algum clube sem todos os jogos.
unsafe fn cpa1_completa_so_le(liga: usize, fases: u32) -> bool {
    let tab = read_usize(liga + 0xb1);
    if tab == 0 || read_u8(liga + 0xb5) != 0 || read_usize(liga + 0x45) == 0 {
        return true;
    }
    if read_usize(liga + 0xdd) != 0 {
        return read_u8(liga + 0x44) != 0xff || fases != 1;
    }
    let n = read_i16(liga + 0x3e);
    let mut total = read_i16(liga + 0xa7);
    if total == -1 && n > 0 {
        total = (n - 1).wrapping_mul(read_i16(liga + 0x3c));
    }
    (0..n.max(0) as usize).any(|i| ((read_u8(tab + 0x3b * i + 0x11) as i8) as i16) < total)
}

unsafe extern "thiscall" fn comparar_liga_completa(liga: usize, fases: u32, avisar: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(776);
    if !cpa1_completa_so_le(liga, fases) {
        return o(liga, fases, avisar); // fim de liga (registros, prêmios, fases, aviso): só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(liga + 0xa7, 2)], || o(liga, fases, avisar), || liga_completa(liga, fases, avisar));
    PLACAR.registrar(igual && r == n, || format!("liga_completa({:x}, {}, {}) {}/{} {:x?} x {:x?}", liga, fases, avisar, r, n, mo, mr));
    r
}

// ---------- 241a leva: o fim de temporada da liga: os direitos de TV, as honras da temporada e o registro no histórico de competições (league.cpp, comp_util.cpp) ----------



/// O pagamento da 0x683010 (`noticia` falso: sem a notícia da 0x48ea40, para o comparador).
unsafe fn cpa2_pagar_tv(liga: usize, valor: i32, filtro: usize, noticia: bool) {
    let financas = *(0xae2a38 as *const usize);
    if financas == 0 || read_i16(liga + 0x3e) <= 0 {
        return;
    }
    crate::comparacao::ramo("liga direitos de tv");
    let financas_do_clube: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let somar_tv: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5987c0usize);
    let mut i: i16 = 0;
    loop {
        let linha = read_usize(liga + 0xb1) + 0x3b * i as i32 as usize;
        let clube = read_usize(linha);
        if filtro == 0 || read_usize(clube + 0x5b) == filtro {
            somar_tv(financas_do_clube(*(0xae2a38 as *const usize), clube), valor); // o dinheiro da TV
            if noticia {
                let avisar: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(0x48ea40usize);
                avisar(liga, read_usize(read_usize(liga + 0xb1) + 0x3b * i as i32 as usize), valor);
            }
        }
        i = i.wrapping_add(1);
        if i >= read_i16(liga + 0x3e) {
            break;
        }
    }
}

/// { 0x683010 } (thiscall, league.cpp): `League::pay_tv_rights(valor, filtro)`: com a tabela de finanças
/// [0xae2a38], a cada clube da tabela (+0xb1; com `filtro`, só os de +0x5b igual a ele) soma `valor` à receita de TV
/// (0x5a0590 → 0x5987c0) e manda a notícia (Comp::send_tv_rights_news 0x48ea40, só para clube de técnico humano).
/// Sem SEH, sem sorteio, sem locais.
unsafe extern "thiscall" fn pagar_direitos_de_tv(liga: usize, valor: i32, filtro: usize) {
    cpa2_pagar_tv(liga, valor, filtro, true)
}

/// Comparador: as finanças (0x167 bytes) de cada clube da tabela; a cópia roda sem a notícia (a da original fica, uma
/// vez só). Clube nulo na tabela (a caixa da 0x5a0590): só a original.
unsafe extern "thiscall" fn comparar_pagar_direitos_de_tv(liga: usize, valor: i32, filtro: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, usize)>(777);
    let financas = *(0xae2a38 as *const usize);
    let n = read_i16(liga + 0x3e).max(0) as usize;
    let clubes: Vec<usize> = (0..n).map(|i| read_usize(read_usize(liga + 0xb1) + 0x3b * i)).collect();
    if financas == 0 || clubes.contains(&0) {
        return o(liga, valor, filtro);
    }
    let financas_do_clube: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let reg: Vec<(usize, usize)> = clubes.iter().map(|&c| (financas_do_clube(financas, c), 0x167)).collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(liga, valor, filtro), || cpa2_pagar_tv(liga, valor, filtro, false));
    PLACAR.registrar(igual, || format!("pagar_direitos_de_tv({:x}, {}, {:x}) {:x?} x {:x?}", liga, valor, filtro, mo, mr));
}

/// { 0x684640 } (thiscall, league.cpp): `League::record_season_honours()`: campeão, vice e (com mais de 2 clubes)
/// terceiro da tabela (+0xb1). Se a divisão do campeão (+0x57) é esta competição (+4), zera nos clubes ([0xae23b4],
/// 0x245 bytes, [0xae235c] − [0xae2350]) o +0x5b que aponta para ela. Se a virtual +0x88 da liga da competição
/// ([0xadadfc][id]) pede, o destaque é a seleção (0x53b3d0) do país-sede (HostCountry::find_hosts 0x5fa730 com o ano
/// +0x40): com duas sedes, a de maior +0x8e (empate: a primeira). Grava no histórico (0x4afce0) e devolve o que ela
/// devolve. Sem SEH, sem sorteio; as duas locais das sedes só são lidas quando a 0x5fa730 as grava.
unsafe extern "thiscall" fn registrar_honras_da_temporada(liga: usize) -> usize {
    crate::comparacao::ramo("liga honras da temporada");
    let tab = read_usize(liga + 0xb1);
    let vice = read_usize(tab + 0x3b);
    let campeao = read_usize(tab);
    let terceiro = if read_i16(liga + 0x3e) > 2 { read_usize(tab + 0x76) } else { 0 };
    if read_usize(campeao + 0x57) == read_usize(liga + 4) {
        let quantos = || (*(0xae235c as *const i32)).wrapping_sub(*(0xae2350 as *const i32));
        let mut k = 0i32;
        while k < quantos() {
            let p = (*(0xae23b4 as *const usize)).wrapping_add(0x245 * k as usize + 0x5b);
            if read_usize(p) == read_usize(liga + 4) {
                core::ptr::write_unaligned(p as *mut usize, 0); // a divisão anterior
            }
            k += 1;
        }
    }
    let comp = read_usize(liga + 4);
    let alvo = *((*(0xadadfc as *const usize)).wrapping_add(4 * *(comp as *const u32) as usize) as *const usize);
    let pede: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(alvo) + 0x88));
    let mut destaque = 0usize;
    if pede(alvo) != 0 {
        let sedes: unsafe extern "thiscall" fn(usize, u32, u32, *mut i32, *mut i32, u32) -> u8 = core::mem::transmute(0x5fa730usize);
        let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
        let (mut a, mut b) = (0i32, 0i32);
        let q = sedes(*(0xb5e134 as *const usize), *(read_usize(liga + 4) as *const u32), read_i16(liga + 0x40) as u16 as u32, &mut a, &mut b, 0);
        let nacao = |k: i32| (*(0xae23a8 as *const usize)).wrapping_add(k.wrapping_mul(0x122) as usize);
        if q == 2 {
            destaque = if read_i16(nacao(a) + 0x8e) < read_i16(nacao(b) + 0x8e) { selecao(nacao(b)) } else { selecao(nacao(a)) };
        } else if q != 0 {
            destaque = selecao(nacao(a));
        }
    }
    let registrar: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> usize = core::mem::transmute(0x4afce0usize);
    registrar(liga, campeao, vice, terceiro, destaque) // campeão, vice, terceiro
}

/// As regiões do histórico de competições: contagem, ponteiro e as linhas (26 bytes).
unsafe fn cpa2_regioes_do_historico() -> Vec<(usize, usize)> {
    let n = (*(0xae237c as *const i32)).max(0) as usize;
    vec![(0xae237c, 4), (0xae23d4, 4), (*(0xae23d4 as *const usize), 26 * n)]
}

/// Comparador: os clubes (o +0x5b) e o histórico de competições. Sem linha livre no histórico (a 0x4afce0 faria
/// realloc): só a original.
unsafe extern "thiscall" fn comparar_registrar_honras_da_temporada(liga: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(778);
    if cpa2_linha_livre() == 0 {
        return o(liga);
    }
    let mut reg = cpa2_regioes_do_historico();
    let quantos = (*(0xae235c as *const i32)).wrapping_sub(*(0xae2350 as *const i32)).max(0) as usize;
    reg.push((*(0xae23b4 as *const usize), 0x245 * quantos));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(liga), || registrar_honras_da_temporada(liga));
    PLACAR.registrar(igual && r == n, || format!("registrar_honras_da_temporada({:x}) {:x}/{:x} {:x?} x {:x?}", liga, r, n, mo, mr));
    r
}

/// A linha livre do histórico de competições ([0xae23d4], [0xae237c] linhas de 26 bytes): a seguinte à última com
/// +4 (competição) preenchido; nenhuma preenchida, ou a última preenchida: 0 (falta crescer).
unsafe fn cpa2_linha_livre() -> usize {
    let n = *(0xae237c as *const i32);
    let t = *(0xae23d4 as *const usize);
    let mut i = n - 1;
    while i >= 0 {
        if read_usize(t.wrapping_add(26 * i as usize + 4)) != 0 {
            break;
        }
        i -= 1;
    }
    if i < 0 || i >= n - 1 {
        return 0;
    }
    t.wrapping_add(26 * (i + 1) as usize)
}

/// { 0x4afce0 } (cdecl, comp_util.cpp): `comp_util_add_history_record(liga, primeiro, segundo, terceiro, extra)`:
/// a linha livre do histórico de competições (sem ela, cresce 20 linhas pela 0x527f60 e usa a primeira nova; sem
/// memória, 0) recebe a competição (+4 da liga), o ano [0xae2c92] (+8), e os quatro valores em +0xa, +0xe, +0x12,
/// +0x16. Devolve a linha. Sem SEH, sem sorteio, sem locais.
unsafe extern "C" fn acrescentar_ao_historico(liga: usize, a: u32, b: u32, c: u32, d: u32) -> usize {
    let mut r = cpa2_linha_livre();
    if r == 0 {
        crate::comparacao::ramo("histórico de competições crescer");
        let crescer: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x527f60usize);
        if crescer(0x14) == 0 {
            return 0;
        }
        let n = *(0xae237c as *const i32);
        r = (*(0xae23d4 as *const usize)).wrapping_add(26usize.wrapping_mul(n.wrapping_sub(20) as usize));
    }
    core::ptr::write_unaligned((r + 4) as *mut usize, read_usize(liga + 4));
    core::ptr::write_unaligned((r + 8) as *mut u16, *(0xae2c92 as *const u16)); // o ano
    for (k, v) in [a, b, c, d].into_iter().enumerate() {
        core::ptr::write_unaligned((r + 0xa + 4 * k) as *mut u32, v);
    }
    r
}

unsafe extern "C" fn comparar_acrescentar_ao_historico(liga: usize, a: u32, b: u32, c: u32, d: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32, u32, u32) -> usize>(779);
    if cpa2_linha_livre() == 0 {
        return o(liga, a, b, c, d); // cresce (realloc): só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&cpa2_regioes_do_historico(), || o(liga, a, b, c, d), || acrescentar_ao_historico(liga, a, b, c, d));
    PLACAR.registrar(igual && r == n, || format!("acrescentar_ao_historico({:x}, {:x}, {:x}, {:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", liga, a, b, c, d, r, n, mo, mr));
    r
}

// ---------- 216a leva: se a liga tem rebaixamento, a data da rodada do modelo da liga e a importância do jogo da liga (league.cpp) ----------

/// { 0x6847b0 } (thiscall, league.cpp): 1 se a liga tem vagas de rebaixamento (byte +0xc1 não nulo).
unsafe extern "thiscall" fn liga_tem_rebaixamento(liga: usize) -> u32 {
    (*((liga + 0xc1) as *const u8) != 0) as u32 // tem rebaixamento
}

/// { 0x68a160 } (cdecl, league.cpp): a data (Date::Date, 0x549f70, num Date local) de `dia`/`mes`/
/// (`anos` + `base`)/`semana` no registro `i` (palavra com sinal, 0x41 bytes) de `rodadas`: dia do ano
/// na palavra +0, ano − `base` na palavra +2, `codigo` no byte +4 e `valor` em +0x3d. Devolve o registro.
unsafe extern "C" fn data_da_rodada_da_liga(rodadas: usize, i: u32, dia: u32, mes: u32, anos: u32, semana: u32, codigo: u32, base: u32, valor: u32) -> usize {
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u16; 4];
    construir(&mut d, dia, mes, anos.wrapping_add(base), semana);
    let r = rodadas.wrapping_add((i as i16 as isize * 0x41) as usize);
    core::ptr::write_unaligned(r as *mut u16, d[0]);
    core::ptr::write_unaligned((r + 2) as *mut u16, d[1].wrapping_sub(base as u16)); // ano relativo
    *((r + 4) as *mut u8) = codigo as u8;
    core::ptr::write_unaligned((r + 0x3d) as *mut u32, valor);
    r
}

/// { 0x68a3e0 } (thiscall, ret 0xc, league.cpp): a importância do jogo da `partida` para o clube do
/// `lado` (0 = casa, +0x1c; senão +0x20): (byte +0x51 da liga − 10) / 4 + 10. Grava 0xff em `saida`;
/// com o clube na tabela (+0xb1, linhas de 59 bytes, +0x3e clubes) e mais de 10 rodadas (+0xa9): +1
/// com +0x34 da partida abaixo de 3; senão, linha sem marca (+0x37 = 0xff), +1 e a margem do último
/// dia (0x68a4d0, que pode trocar a importância) em `saida`. Só o al é devolvido (quem chama usa al).
unsafe extern "thiscall" fn importancia_do_jogo_da_liga(liga: usize, partida: usize, lado: u32, saida: *mut u8) -> u8 {
    let margem: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8 = core::mem::transmute(0x68a4d0usize);
    *saida = 0xff;
    let clube = *((partida + if lado as u8 == 0 { 0x1c } else { 0x20 }) as *const usize);
    let mut imp = ((*((liga + 0x51) as *const i8) as i32 - 10) / 4) as u8;
    imp = imp.wrapping_add(10); // importância base
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16);
    let linha = |i: i16| (*((liga + 0xb1) as *const usize)).wrapping_add((i as isize * 59) as usize);
    // ponytail: a original testa i == −1 depois da busca; i começa em 0 e para antes de `clubes`, nunca é −1
    let mut i = 0i16;
    loop {
        if i >= clubes {
            return imp;
        }
        if *(linha(i) as *const usize) == clube {
            break;
        }
        i = i.wrapping_add(1);
    }
    if core::ptr::read_unaligned((liga + 0xa9) as *const i16) <= 10 {
        return imp;
    }
    if core::ptr::read_unaligned((partida + 0x34) as *const i16) < 3 {
        return imp.wrapping_add(1);
    }
    let r = linha(i);
    if *((r + 0x37) as *const u8) != 0xff {
        return imp;
    }
    let mut local = imp.wrapping_add(1);
    *saida = margem(liga, r, &mut local);
    local
}

unsafe extern "thiscall" fn comparar_liga_tem_rebaixamento(liga: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(545)(liga);
    let n = liga_tem_rebaixamento(liga);
    PLACAR.registrar(r == n, || format!("liga_tem_rebaixamento original={} rust={}", r, n));
    r
}

unsafe extern "C" fn comparar_data_da_rodada_da_liga(rodadas: usize, i: u32, dia: u32, mes: u32, anos: u32, semana: u32, codigo: u32, base: u32, valor: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(546);
    let r = rodadas.wrapping_add((i as i16 as isize * 0x41) as usize);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 0x41)], || o(rodadas, i, dia, mes, anos, semana, codigo, base, valor),
        || data_da_rodada_da_liga(rodadas, i, dia, mes, anos, semana, codigo, base, valor));
    PLACAR.registrar(igual && a == b, || format!("data_da_rodada_da_liga({}, {}/{} +{}) {:x?} x {:x?}", i as i16, dia, mes, anos, mo, mr));
    a
}

unsafe extern "thiscall" fn comparar_importancia_do_jogo_da_liga(liga: usize, partida: usize, lado: u32, saida: *mut u8) -> u8 {
    // a 0x68a4d0 só lê a liga e grava na local e na `saida`: pode rodar nos dois lados
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, *mut u8) -> u8>(547);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(saida as usize, 1)], || o(liga, partida, lado, saida),
        || importancia_do_jogo_da_liga(liga, partida, lado, saida));
    PLACAR.registrar(igual && r == n, || format!("importancia_do_jogo_da_liga original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

// ---------- 214a leva: a ordem das linhas da tabela, a grade de turnos da liga e a margem do último dia (league.cpp) ----------

/// { 0x683210 } (cdecl, league.cpp): comparação do qsort das linhas da tabela (`a` e `b`: [+0] clube).
/// Liga corrente em [0xb679fc], a coluna da tabela em +0xad (14 bytes por coluna nas linhas). Linha
/// sem clube: 0. Com a comparação particular em [0xb67a00] e resposta dela não nula: ela de novo. Senão
/// pontos (+0x19) decrescentes; sem jogos (+0x11) dos dois, pelo id do clube. Depois os quatro
/// critérios da liga (+0xc5): 1 saldo, 2 gols pró, 3 byte +0x12, 4 byte +4, 5 palavra +0x31 (na
/// coluna 0, critério 6 = byte +4 e fim); na coluna 0 com os dois no número de jogos da liga (+0xa7), o
/// id em +0x69 dos clubes; por fim o id do clube.
unsafe extern "C" fn ordem_das_linhas_da_tabela(a: usize, b: usize) -> i32 {
    let liga = *(0xb679fc as *const usize);
    let coluna = core::ptr::read_unaligned((liga + 0xad) as *const i32);
    if *(a as *const usize) == 0 || *(b as *const usize) == 0 {
        return 0;
    }
    let propria = *(0xb67a00 as *const usize);
    if propria != 0 {
        let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(propria);
        if f(a, b) != 0 {
            let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(*(0xb67a00 as *const usize));
            return f(a, b);
        }
    }
    let c = (coluna as usize).wrapping_mul(14);
    let w = |r: usize, o: usize| core::ptr::read_unaligned(r.wrapping_add(c).wrapping_add(o) as *const i16) as i32;
    let y = |r: usize, o: usize| *(r.wrapping_add(c).wrapping_add(o) as *const i8) as i32;
    let (pa, pb) = (w(a, 0x19), w(b, 0x19));
    if pa != pb {
        return pb - pa; // pontos
    }
    let id = |r: usize| *(*(r as *const usize) as *const i32);
    if y(a, 0x11) == 0 && y(b, 0x11) == 0 {
        return id(a).wrapping_sub(id(b));
    }
    for k in 0..4usize {
        let criterio = *((liga + 0xc5 + k) as *const i8);
        if coluna == 0 && criterio == 6 {
            return *((a + 4) as *const i8) as i32 - *((b + 4) as *const i8) as i32;
        }
        if criterio == 0 {
            break;
        }
        let (va, vb) = match criterio {
            1 => (w(a, 0x15) - w(a, 0x17), w(b, 0x15) - w(b, 0x17)),
            2 => (w(a, 0x15), w(b, 0x15)),
            3 => (y(a, 0x12), y(b, 0x12)),
            4 => (*((a + 4) as *const i8) as i32, *((b + 4) as *const i8) as i32),
            5 => (core::ptr::read_unaligned((a + 0x31) as *const i16) as i32, core::ptr::read_unaligned((b + 0x31) as *const i16) as i32),
            _ => continue,
        };
        if va != vb {
            return vb - va;
        }
    }
    if coluna == 0 {
        let jogos = core::ptr::read_unaligned((liga + 0xa7) as *const i16);
        if *((a + 0x11) as *const i8) as i16 == jogos && *((b + 0x11) as *const i8) as i16 == jogos {
            let xb = core::ptr::read_unaligned((*(b as *const usize) + 0x69) as *const usize);
            let xa = core::ptr::read_unaligned((*(a as *const usize) + 0x69) as *const usize);
            if xb != 0 && xa != 0 {
                let d = (*(xb as *const i32)).wrapping_sub(*(xa as *const i32));
                if d != 0 {
                    return d;
                }
            }
        }
    }
    id(a).wrapping_sub(id(b))
}

/// { 0x6844b0 } (stdcall, ret 8, league.cpp): a grade de turnos da liga de `n` clubes: `grade[1..=n]`
/// são linhas de inteiros (colunas 1..n−1 = rodadas) com o adversário de cada clube na rodada (positivo
/// em casa, negativo fora). Zera tudo; o clube `n` enfrenta 1..n−1 numa rodada que anda de 2 em 2
/// (casa para a primeira metade); depois cada clube k de n−1 até 2 preenche as vagas vazias com os
/// clubes 1..n, alternando o mando a partir da paridade de n−1−k. Devolve 1.
unsafe extern "stdcall" fn montar_grade_de_turnos(grade: *const *mut i32, n: i32) -> u32 {
    let celula = |i: i32, j: i32| (*grade.offset(i as isize)).offset(j as isize);
    for i in 1..=n {
        for j in 1..n {
            *celula(i, j) = 0;
        }
    }
    let metade = n / 2;
    let mut col = n - 1;
    for i in 1..n {
        if i <= metade {
            *celula(i, col) = n;
            *celula(n, col) = -i;
        } else {
            *celula(i, col) = -n;
            *celula(n, col) = i;
        }
        col = if col == n - 2 { 1 } else if col == n - 1 { 2 } else { col + 2 };
    }
    let mut k = n - 1;
    while k >= 2 {
        let menos_k = -k;
        let mut fora = (n - 1 + menos_k) % 2;
        let mut col = k - 1;
        let mut contra = -1;
        for j in 1..=n {
            let c = celula(j, col);
            if *c == 0 {
                if fora != 0 {
                    *c = k; // vaga preenchida
                    *celula(k, col) = contra;
                } else {
                    *c = menos_k;
                    *celula(k, col) = j;
                }
                fora = 1 - fora;
            }
            col = if col == n - 1 { 1 } else { col + 1 };
            contra -= 1;
        }
        k -= 1;
    }
    1
}

/// A linha `i` (59 bytes) da tabela [+0xb1] da liga.
unsafe fn linha_da_liga(liga: usize, i: i32) -> usize {
    (*((liga + 0xb1) as *const usize)).wrapping_add((i as isize * 59) as usize)
}

/// A margem de gols (+1, mínimo 1, senão 0xff) que falta para alcançar a linha `alvo` pelo critério da
/// liga (+0xc5): 1 saldo (+0x15 − +0x17), 2 gols pró (+0x15).
unsafe fn margem_para(liga: usize, linha: usize, alvo: usize) -> u8 {
    let b = |r: usize, o: usize| *((r + o) as *const u8);
    let c = if *((liga + 0xc5) as *const u8) == 1 {
        b(alvo, 0x15).wrapping_sub(b(alvo, 0x17)).wrapping_sub(b(linha, 0x15)).wrapping_add(b(linha, 0x17))
    } else {
        b(alvo, 0x15).wrapping_sub(b(linha, 0x15))
    }
    .wrapping_add(1);
    if (c as i8) < 1 { 0xff } else { c }
}

/// { 0x68a4d0 } (thiscall, ret 8, league.cpp): no último dia (jogos +0x11 ≥ rodadas da liga +0xa9 − 1)
/// e linha sem marca (+0x37 = 0xff), a margem de gols que o clube precisa para o título, as vagas de
/// cima (+0xbe/+0xbf) ou fugir das de baixo (+0xc0/+0xc1); 0xff se nenhuma. Com pontos iguais a quem
/// está à frente sem margem possível, grava 0x14 em `saida` (decisivo) e devolve 0 ou 0xff. Só o al é
/// devolvido (quem chama usa al).
unsafe extern "thiscall" fn margem_do_ultimo_dia(liga: usize, linha: usize, saida: *mut u8) -> u8 {
    let p = |r: usize| core::ptr::read_unaligned((r + 0x19) as *const i16) as i32;
    let extra = *((liga + 0xc2) as *const i8) as i32;
    let pos = *((linha + 4) as *const i8);
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16);
    let tabela = *((liga + 0xb1) as *const usize);
    let criterio = *((liga + 0xc5) as *const u8);
    if (*((linha + 0x11) as *const i8) as i32) < core::ptr::read_unaligned((liga + 0xa9) as *const i16) as i32 - 1 || *((linha + 0x37) as *const u8) != 0xff {
        return 0xff; // fora do último dia
    }
    // a linha `alvo` à frente, sem marca: pontos + extra acima dela, 0xff; iguais (extra 0), a margem
    let ver = |alvo: usize| -> Option<u8> {
        if p(linha) + extra > p(alvo) {
            return Some(0xff);
        }
        if extra == 0 && (criterio == 1 || criterio == 2) {
            return Some(margem_para(liga, linha, alvo));
        }
        None
    };
    // a primeira linha sem marca de `de` até `ate` (exclusive), com pontos + extra contra os da `linha`
    let decidir = |de: i16, ate: i16| -> Option<u8> {
        let mut i = de;
        while i < ate {
            let r = linha_da_liga(liga, i as i32);
            if *((r + 0x37) as *const u8) == 0xff {
                let v = p(r) + extra;
                if v > p(linha) {
                    *saida = 0x14;
                    return Some(0xff);
                }
                if v != p(linha) {
                    return Some(0xff);
                }
                *saida = 0x14;
                return Some(0);
            }
            i += 1;
        }
        None
    };
    if *((liga + 0xd9) as *const u8) & 1 != 0 && *((tabela + 0x37) as *const u8) != 5 {
        if pos > 0 {
            let v = p(linha) + extra;
            if v > p(tabela) {
                return 0xff;
            }
            if v == p(tabela) && (criterio == 1 || criterio == 2) {
                return margem_para(liga, linha, tabela);
            }
        } else if clubes > 1 {
            let v = core::ptr::read_unaligned((tabela + 0x54) as *const i16) as i32 + extra;
            if v > p(tabela) {
                *saida = 0x14;
                return 0xff;
            }
            if v != p(tabela) {
                return 0xff;
            }
            *saida = 0x14;
            return 0;
        }
    }
    let (s1, s2) = (*((liga + 0xbe) as *const u8), *((liga + 0xbf) as *const u8));
    if s1 != 0 || s2 != 0 {
        let vagas = s2.wrapping_add(s1) as i8;
        if pos < vagas {
            if let Some(r) = decidir(vagas as i16, clubes) {
                return r;
            }
        } else {
            let mut i = 0i16;
            while i < vagas as i16 {
                let r = linha_da_liga(liga, i as i32);
                if *((r + 0x37) as *const u8) == 0xff {
                    if let Some(m) = ver(r) {
                        return m;
                    }
                }
                i += 1;
            }
        }
    }
    let (d1, d2) = (*((liga + 0xc1) as *const u8), *((liga + 0xc0) as *const u8));
    if d1 == 0 && d2 == 0 {
        return 0xff;
    }
    let descem = d2.wrapping_add(d1) as i8;
    let limite = clubes as i32 - descem as i32;
    if (pos as i32) < limite {
        return decidir(clubes.wrapping_sub(descem as i16), clubes).unwrap_or(0xff);
    }
    let mut i = 0i16;
    while (i as i32) < limite {
        let r = linha_da_liga(liga, i as i32);
        if *((r + 0x37) as *const u8) == 0xff {
            if let Some(m) = ver(r) {
                return m;
            }
        }
        i += 1;
    }
    0xff
}

unsafe extern "C" fn comparar_ordem_das_linhas_da_tabela(a: usize, b: usize) -> i32 {
    // a comparação particular de [0xb67a00] (se houver) roda nos dois lados: só lê as linhas
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(534)(a, b);
    let n = ordem_das_linhas_da_tabela(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_linhas_da_tabela original={} rust={}", r, n));
    r
}

unsafe extern "stdcall" fn comparar_montar_grade_de_turnos(grade: *const *mut i32, n: i32) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(*const *mut i32, i32) -> u32>(535);
    let regioes: Vec<(usize, usize)> = (1..=n.max(0)).map(|i| (*grade.offset(i as isize) as usize + 4, (n.max(1) as usize - 1) * 4)).collect();
    let (r, s, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(grade, n), || montar_grade_de_turnos(grade, n));
    PLACAR.registrar(igual && r == s, || format!("montar_grade_de_turnos({}) {:x?} x {:x?}", n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_margem_do_ultimo_dia(liga: usize, linha: usize, saida: *mut u8) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8>(536);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(saida as usize, 1)], || o(liga, linha, saida),
        || margem_do_ultimo_dia(liga, linha, saida));
    PLACAR.registrar(igual && r == n, || format!("margem_do_ultimo_dia original={:#x} rust={:#x} {:x?} x {:x?}", r, n, mo, mr));
    r
}

// ---------- 196a leva: o confronto direto da liga e os termos oferecidos na transferência (league.cpp, transfer_manager.cpp) ----------

/// { 0x6834f0 } (cdecl, league.cpp): a ordem da minitabela do confronto direto (callback do qsort):
/// linha vazia empata; senão pontos (+0x19), e depois os critérios da liga ([0xb679fc]+0xc5, até 4;
/// 0 encerra): 1 saldo (+0x15 − +0x17), 2 gols pró (+0x15), 3 o byte +0x12, 5 a palavra +0x31; 4 e os
/// outros não decidem. Devolve b − a no primeiro critério que difere.
unsafe extern "C" fn ordem_do_confronto(a: usize, b: usize) -> i32 {
    if *(a as *const u32) == 0 || *(b as *const u32) == 0 {
        return 0;
    }
    let w = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const i16) as i32;
    if w(a, 0x19) != w(b, 0x19) {
        return w(b, 0x19) - w(a, 0x19);
    }
    let criterios = *(0xb679fc as *const usize) + 0xc5;
    for k in 0..4 {
        let (x, y) = match *((criterios + k) as *const i8) {
            0 => return 0,
            1 => (w(a, 0x15) - w(a, 0x17), w(b, 0x15) - w(b, 0x17)),
            2 => (w(a, 0x15), w(b, 0x15)),
            3 => (*((a + 0x12) as *const i8) as i32, *((b + 0x12) as *const i8) as i32),
            5 => (w(a, 0x31), w(b, 0x31)),
            _ => continue,
        };
        if x != y {
            return y - x;
        }
    }
    0
}

/// O registro de 14 bytes do confronto da linha `linha` contra o adversário `adversario` (a linha da
/// tabela tem 59 bytes; em +5 o ponteiro dos confrontos).
unsafe fn registro_do_confronto(tabela: usize, linha: i32, adversario: i32) -> usize {
    let confrontos = core::ptr::read_unaligned((tabela as isize + linha as isize * 59 + 5) as *const usize);
    (confrontos as isize + adversario as isize * 14) as usize
}

/// { 0x6869a0 } (thiscall, ret 0x14, league.cpp): soma um jogo ao confronto direto: jogos (+4), vitórias
/// (+5, com os pontos por vitória da liga +0xc2), empates (+6, pontos por empate +0xc3) ou derrotas (+7),
/// gols pró (+8) e contra (+0xa); os pontos em +0xc. Linha ou adversário negativo: nada.
unsafe extern "thiscall" fn atualizar_confronto_direto(liga: usize, tabela: usize, linha: i32, adversario: i32, pro: i8, contra: i8) {
    if linha < 0 || adversario < 0 {
        return;
    }
    let e = registro_do_confronto(tabela, linha, adversario);
    let byte = |o: usize| (e + o) as *mut u8;
    let palavra = |o: usize| (e + o) as *mut i16;
    let somar = |o: usize, v: i16| core::ptr::write_unaligned(palavra(o), core::ptr::read_unaligned(palavra(o)).wrapping_add(v));
    *byte(4) = (*byte(4)).wrapping_add(1);
    if pro > contra {
        *byte(5) = (*byte(5)).wrapping_add(1);
        somar(0xc, *((liga + 0xc2) as *const i8) as i16);
    } else if pro == contra {
        *byte(6) = (*byte(6)).wrapping_add(1);
        somar(0xc, *((liga + 0xc3) as *const i8) as i16);
    } else {
        *byte(7) = (*byte(7)).wrapping_add(1);
    }
    somar(8, pro as i16);
    somar(0xa, contra as i16);
}

/// { 0x8c1f80 } (thiscall, ret 4, transfer_manager.cpp): marca que a oferta `k` (fora de empréstimo,
/// +0x1f ≠ 1) teve os termos oferecidos ao jogador: o estado +0x2c vira 8 se era 6, senão 7. Índice
/// fora da lista ([+4] ofertas): a original (caixa de erro).
unsafe extern "thiscall" fn marcar_termos_oferecidos(gerente: usize, k: i32) {
    if k < 0 || k >= *((gerente + 4) as *const i32) {
        return orig::<unsafe extern "thiscall" fn(usize, i32)>(462)(gerente, k);
    }
    let oferta = *((*(gerente as *const usize) as isize + k as isize * 4) as *const usize);
    if oferta != 0 && *((oferta + 0x1f) as *const u8) != 1 {
        let estado = (oferta + 0x2c) as *mut u8;
        *estado = (*estado == 6) as u8 + 7;
    }
}

unsafe extern "C" fn comparar_ordem_do_confronto(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(460)(a, b);
    let n = ordem_do_confronto(a, b);
    PLACAR.registrar(r == n, || format!("ordem_do_confronto original={} rust={}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_atualizar_confronto_direto(liga: usize, tabela: usize, linha: i32, adversario: i32, pro: i8, contra: i8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i32, i32, i8, i8)>(461);
    if linha < 0 || adversario < 0 {
        return o(liga, tabela, linha, adversario, pro, contra);
    }
    let e = registro_do_confronto(tabela, linha, adversario);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(e, 14)], || o(liga, tabela, linha, adversario, pro, contra),
        || atualizar_confronto_direto(liga, tabela, linha, adversario, pro, contra));
    PLACAR.registrar(igual, || format!("atualizar_confronto_direto({}, {}, {}x{}) {:x?} x {:x?}", linha, adversario, pro, contra, mo, mr));
}

unsafe extern "thiscall" fn comparar_marcar_termos_oferecidos(gerente: usize, k: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(462);
    if k < 0 || k >= *((gerente + 4) as *const i32) {
        return o(gerente, k);
    }
    let oferta = *((*(gerente as *const usize) as isize + k as isize * 4) as *const usize);
    if oferta == 0 {
        return o(gerente, k);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(oferta + 0x2c, 1)], || o(gerente, k), || marcar_termos_oferecidos(gerente, k));
    PLACAR.registrar(igual, || format!("marcar_termos_oferecidos({}) {:x?} x {:x?}", k, mo, mr));
}

// ---------- 177a leva: o bit do contrato, a marca da competicao e o registro do clube na liga ----------

/// { 0x7a7eb0 } (stdcall, ret 4, player_regen.cpp): os bits 4..5 do byte +0xb do registro de 79 bytes
/// da pessoa `*p` em [0xae2d0c]. Devolve só `al`.
unsafe extern "stdcall" fn bits_do_registro(p: *const i32) -> u8 {
    let r = (*(0xae2d0c as *const usize)).wrapping_add((*p as usize).wrapping_mul(79));
    (*((r + 0xb) as *const u8) >> 4) & 3
}

/// { 0x684790 } (thiscall, league.cpp): a competição tem o byte +0xbe e a marca 2 de +0xd9?
unsafe extern "thiscall" fn competicao_marcada(t: usize) -> u32 {
    (*((t + 0xbe) as *const u8) != 0 && *((t + 0xd9) as *const u8) & 2 != 0) as u32
}

/// { 0x6826d0 } (thiscall, ret 4, league.cpp): o registro `i` (59 bytes) da tabela [this+0xb1].
unsafe extern "thiscall" fn registro_da_liga(t: usize, i: i32) -> usize {
    core::ptr::read_unaligned((t + 0xb1) as *const usize).wrapping_add((i as usize).wrapping_mul(59))
}

comparar_valor!(comparar_competicao_marcada, competicao_marcada, 404, () -> u32);
comparar_valor!(comparar_registro_da_liga, registro_da_liga, 405, (i: i32) -> usize);

unsafe extern "stdcall" fn comparar_bits_do_registro(p: *const i32) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(*const i32) -> u8>(403)(p);
    let n = bits_do_registro(p);
    PLACAR.registrar(r == n, || format!("bits_do_registro({:p}) original={} rust={}", p, r, n));
    r
}

// ---------- centesima sexagesima quarta leva: o calendario das ligas (league.cpp, fix_man.cpp) ----------

/// { 0x68a030 } (cdecl, league.cpp): a próxima posição `pos` no rodízio do calendário de `n` (palavra)
/// clubes. Na liga especial (`liga` = [0x9cf6e8]): 2 vira 4, 5 vira 3, o resto avança 1. Com menos
/// de 2 turnos ou de 8 rodadas, ou a marca 0x40, avança 1; com 0x80, avança 1 trocando n/2 por
/// n−1 e n por n/2. Senão o estado `*passo` (0..3) alterna saltos: 1 e 2 voltam 1 (abaixo de 0,
/// n−1) e vão a 2 e 3; 3 salta 3 e volta a 0; 0 salta 3 (indo a 1) se `pos` < n−5 e sorteio(4) ≥ 2,
/// senão avança 1; os saltos dão a volta em n.
unsafe extern "C" fn girar_rodada(pos: i32, passo: *mut i8, liga: i32, rodadas: i16, turnos: i16, n: i16, marcas: u8) -> i32 {
    if liga == *(0x9cf6e8 as *const i32) {
        return match pos {
            2 => 4,
            5 => 3,
            _ => pos + 1,
        };
    }
    if turnos < 2 || rodadas < 8 || marcas & 0x40 != 0 {
        return pos + 1;
    }
    let n = n as i32;
    if marcas & 0x80 != 0 {
        let p = pos + 1;
        return if p == n / 2 { n - 1 } else if p == n { n / 2 } else { p };
    }
    let volta = |p: i32| if p < n { p } else { p - n };
    match *passo {
        1 | 2 => {
            *passo += 1;
            if pos - 1 >= 0 { pos - 1 } else { n - 1 }
        }
        3 => {
            *passo = 0;
            volta(pos + 3)
        }
        _ => {
            let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
            if pos < n - 5 && sorteio(4) >= 2 {
                *passo = 1;
                volta(pos + 3)
            } else {
                volta(pos + 1)
            }
        }
    }
}

/// { 0x68a1c0 } (cdecl, league.cpp): grava o jogo `j` 0..7 da rodada `rodada` (0x41 bytes cada,
/// 7 por jogo a partir de `base`): +9 `a` (−1..6), +0xa `b` (−1..2), +0xb `c` (−1..4) e +5 o dword
/// `valor`. Fora das faixas: a original (caixas de erro, e grava assim mesmo).
unsafe extern "C" fn registrar_jogo_da_rodada(base: usize, rodada: i16, j: i16, a: i16, b: i8, c: i8, valor: u32) {
    if !(0..=7).contains(&j) || !(-1..=6).contains(&a) || !(-1..=2).contains(&b) || !(-1..=4).contains(&c) {
        return orig::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32)>(365)(base, rodada, j, a, b, c, valor);
    }
    let e = base.wrapping_add((rodada as isize * 0x41 + j as isize * 7) as usize);
    *((e + 9) as *mut i8) = a as i8;
    *((e + 0xa) as *mut i8) = b;
    *((e + 0xb) as *mut i8) = c;
    core::ptr::write_unaligned((e + 5) as *mut u32, valor);
}

/// { 0x5af180 } (cdecl, fix_man.cpp): marca o ano (+0x28, palavra) do jogo `jogo` nas entradas dos
/// dois clubes (+0xc e +0x10) da tabela [0xae2a5c] (palavras: ano × 2 + bit 0). Entrada vazia: o
/// bit 0 liga, ou a entrada zera em 2001, e o ano entra; senão em 2001 o bit 0 apaga e o ano sobe se
/// for maior. Tabela nula: a original (caixa de erro).
unsafe extern "C" fn marcar_ano_no_calendario(jogo: usize) {
    let tabela = *(0xae2a5c as *const usize);
    if tabela == 0 {
        return orig::<unsafe extern "C" fn(usize)>(366)(jogo);
    }
    let ano = core::ptr::read_unaligned((jogo + 0x28) as *const u16);
    for o in [0xc, 0x10] {
        let e = tabela.wrapping_add((*((jogo + o) as *const i32) as usize).wrapping_mul(2)) as *mut u16;
        let v = core::ptr::read_unaligned(e);
        if v & 0xfffe == 0 {
            let w = if ano == 0x7d1 { 0 } else { v | 1 };
            core::ptr::write_unaligned(e, w & 1 | ano << 1);
        } else {
            if v & 1 != 0 && ano == 0x7d1 {
                core::ptr::write_unaligned(e, v & 0xfffe);
            }
            let v = core::ptr::read_unaligned(e);
            if ano > v >> 1 {
                core::ptr::write_unaligned(e, ano.wrapping_mul(2) | v & 1);
            }
        }
    }
}

unsafe extern "C" fn comparar_girar_rodada(pos: i32, passo: *mut i8, liga: i32, rodadas: i16, turnos: i16, n: i16, marcas: u8) -> i32 {
    let o = orig::<unsafe extern "C" fn(i32, *mut i8, i32, i16, i16, i16, u8) -> i32>(364);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((passo as usize, 1));
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pos, passo, liga, rodadas, turnos, n, marcas), || crate::aleatorio::sem_contar(|| girar_rodada(pos, passo, liga, rodadas, turnos, n, marcas)));
    PLACAR.registrar(igual && r == v, || format!("girar_rodada({}, {}, {}, {}, {}, {:#x}) original={} {:x?} rust={} {:x?}", pos, liga, rodadas, turnos, n, marcas, r, mo, v, mr));
    r
}

unsafe extern "C" fn comparar_registrar_jogo_da_rodada(base: usize, rodada: i16, j: i16, a: i16, b: i8, c: i8, valor: u32) {
    let o = orig::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32)>(365);
    let e = base.wrapping_add((rodada as isize * 0x41 + j as isize * 7) as usize);
    let reg = [(e + 5, 7)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(base, rodada, j, a, b, c, valor), || registrar_jogo_da_rodada(base, rodada, j, a, b, c, valor));
    PLACAR.registrar(igual, || format!("registrar_jogo_da_rodada({:#x}, {}, {}, {}, {}, {}, {:#x}) {:x?} x {:x?}", base, rodada, j, a, b, c, valor, mo, mr));
}

unsafe extern "C" fn comparar_marcar_ano_no_calendario(jogo: usize) {
    let o = orig::<unsafe extern "C" fn(usize)>(366);
    let tabela = *(0xae2a5c as *const usize);
    let reg: Vec<(usize, usize)> = if tabela == 0 {
        vec![]
    } else {
        [0xc, 0x10].iter().map(|&o| (tabela.wrapping_add((*((jogo + o) as *const i32) as usize).wrapping_mul(2)), 2)).collect()
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(jogo), || marcar_ano_no_calendario(jogo));
    PLACAR.registrar(igual, || format!("marcar_ano_no_calendario({:#x}) {:x?} x {:x?}", jogo, mo, mr));
}

