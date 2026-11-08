// ---------- %%LEVA%%a leva: as telas da partida (12): as linhas das estatísticas do time, o redesenho dos últimos placares e a espera dos outros jogadores (match_screens.cpp) ----------

/// Cor (16 bits) guardada em `a`. A original empilha registradores com os 16 bits de cima sujos; quem
/// recebe (0x55f990 pelas células, 0x801960/0x801040) só usa a palavra de baixo (ver o painel 0x723d20).
unsafe fn colour_word_u32(a: usize) -> u32 {
    *(a as *const u16) as u32
}

/// As regiões que uma rotina das telas da partida mexe: as da tela, a posição do buffer de rede
/// ([0xae290c]) e, nas atualizações (0x801c70 e companhia, que viram um pacote para o `Display`), o
/// `Display` local do gerente (+0x3844). Atualização que vai pela rede (vaga com [+0x7d4] ≠ 0): `None`
/// (o pacote seria enviado duas vezes).
unsafe fn screen_update_regions(montagem: bool) -> Option<Vec<(usize, usize)>> {
    let mut r = screen_build_regions();
    r.push((0xae290c, 4));
    if montagem {
        return Some(r);
    }
    let vaga = *((SCREEN_MANAGER + 0x382a) as *const i16) as isize;
    if core::ptr::read_unaligned((SCREEN_MANAGER as isize + 0x7d4 + vaga * 0x300) as *const u16) != 0 {
        return None;
    }
    let local = SCREEN_MANAGER + 0x3844;
    if !r.iter().any(|x| x.0 == local) {
        r.push((local, 0x12f550));
    }
    Some(r)
}

/// O primeiro byte diferente de cada região (para a mensagem do comparador).
fn first_differences(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> Vec<String> {
    mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
        let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
        Some(format!("{:#x}+{:#x}", a, k))
    }).collect()
}

/// Acrescenta o texto em `origem` ao fim do texto 0xde1f64 (o strcat da original).
unsafe fn append_to_text_buffer(origem: *const u8) {
    let mut fim = 0xde1f64usize;
    while *(fim as *const u8) != 0 {
        fim += 1;
    }
    let mut i = 0;
    loop {
        let b = *origem.add(i);
        *((fim + i) as *mut u8) = b;
        if b == 0 {
            break;
        }
        i += 1;
    }
}

/// Colunas 5 a 24 das estatísticas do time: a cor e o campo (byte da linha do jogador) mostrado.
const TEAM_STATS_FIELDS: [(usize, usize); 20] = [
    (0xaebdec, 0x1ce), (0xaebdec, 0x1cf), (0xaebdec, 0x1d0), (0xaebdd4, 0x1d1), (0xaebdd4, 0x1d2),
    (0xaebdd4, 0x1d3), (0xaebdec, 0x1d4), (0xaebdec, 0x1d5), (0xaebdec, 0x1d6), (0xaebdd4, 0x1b7),
    (0xaebdd4, 0x1b5), (0xaebdd4, 0x1b6), (0xaebdec, 0x1d7), (0xaebdec, 0x1b8), (0xaebdec, 0x1bb),
    (0xaebdd4, 0x1b9), (0xaebdd4, 0x1ba), (0xaebdd4, 0x1b0), (0xaebde8, 0x1c5), (0xaebdec, 0x1b1),
];

/// { 0x721cd0 } (cdecl, match_screens.cpp, ~4,2 KB): as 20 linhas de jogador das estatísticas do time
/// do lado [parâmetro 0x1b] (dados: na partida [p0 + 0x5f] e p0 + 4, fora dela os parâmetros 0x20 e
/// 0x1f; linha i em dados + 1573·lado + 73·i, vazia com 0xfb em +0x1ad). As 22 colunas vêm da tabela
/// do parâmetro 0x1c (1: 0xa16178, 2: 0xa1618c): número (ou i + 1), dois marcadores, o nome ("(c)" no
/// capitão; botão 0x7dc/0x7d7 quando a pessoa confere, 0x53e990), o evento (lesão, expulsão, entrada,
/// saída) e os números. Com o parâmetro 0x1d (a foto anterior), célula sem mudança fica sem texto.
/// `modo` ≠ 0 monta as células na lista `area` (0x8040a0 botão, 0x804100 texto; o primeiro item no
/// parâmetro 0x1a se estiver em −1); `modo` 0 atualiza os itens a partir do parâmetro 0x1a (0x801c70,
/// 0x800d30, 0x801960). Parâmetro 0x1c fora de 1 e 2 (a original lê a tabela sem valor): a original.
unsafe extern "C" fn fill_team_stats_rows(modo: u32, area: usize) {
    type T9 = unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, u32, u32, u32, u32, usize) -> u32;
    type T11 = unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, u32, u32, u32, u32, usize, u32, usize) -> u32;
    let celula: T11 = core::mem::transmute(0x8040a0usize);
    let texto_da_celula: T9 = core::mem::transmute(0x804100usize);
    let texto_do_item: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x801c70usize);
    let margem: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x800d30usize);
    let cor_do_texto: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x801960usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let rgb: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(0x5e4800usize);
    let nome_de: unsafe extern "C" fn(u32, u32, u32, *mut u8, u32, u32, u32) = core::mem::transmute(0x53e140usize);
    let confere: unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32 = core::mem::transmute(0x53e990usize);
    const BUF: usize = 0xde1f64;
    let sm = SCREEN_MANAGER;

    let lado = get_screen_param(0x1b) as u8 as i8;
    let p0 = get_screen_param(0);
    let antes = get_screen_param(0x1d);
    let tabela = match get_screen_param(0x1c) as u8 {
        1 => 0xa16178usize,
        2 => 0xa1618c,
        _ => return original_fn::<unsafe extern "C" fn(u32, usize)>(%%I0%%)(modo, area),
    };
    let dados = if p0 != 0 {
        core::ptr::read_unaligned((p0 + 0x5f) as *const usize)
    } else {
        let d = get_screen_param(0x20);
        get_screen_param(0x1f);
        d
    };
    let mut item = get_screen_param(0x1a) as u32;
    let base_lado = (lado as i32).wrapping_mul(1573);
    // o estado das células passa de uma para a outra, como na original (a coluna só volta a 1 na coluna 0)
    let (mut coluna, mut cor, mut fundo, mut fonte, mut marcas, mut estilo) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    for i in 0..20i32 {
        let linha = base_lado.wrapping_add(73 * i) as usize;
        let reg = dados.wrapping_add(linha);
        let b = |o: usize| *((reg + o) as *const u8);
        let bi = |o: usize| *((reg + o) as *const i8);
        for j in 0..22usize {
            let mut botao: (u32, usize) = (0, 0);
            *(BUF as *mut u8) = 0;
            let foto = if antes != 0 { linha.wrapping_add(antes).wrapping_add(0x190) } else { 0 };
            // o campo `o` da linha igual ao da foto anterior
            let igual = |o: usize| *((foto + o - 0x190) as *const u8) == b(o);
            let id = core::ptr::read_unaligned((reg + 0x190) as *const i32);
            if b(0x1ad) == 0xfb {
                continue;
            }
            let codigo = *((tabela + j) as *const u8) as usize;
            if codigo > 0x18 {
                continue;
            }
            match codigo {
                0 => {
                    cor = colour_word_u32(0xae3184);
                    coluna = 1;
                    fonte = 0xc;
                    fundo = if i < 0xb { colour_word_u32(0xaebe04) } else { rgb(0, 0, 0x40, 0) };
                    let n = bi(0x1ac);
                    marcas = 0x30;
                    estilo = 1;
                    formatar(BUF, 0x98806c, if n > 0 { n as i32 } else { i + 1 });
                    // ponytail: a original compara i com ([info + 0x4d] & 0xf) + 0xb, mas os dois ramos dão 0x30
                }
                1 | 2 => {
                    cor = colour_word_u32(0xae317e);
                    marcas = 1;
                    coluna = codigo as u32 + 2;
                    fonte = 0xc;
                    fundo = 0;
                    estilo = 1;
                    if !(foto != 0 && igual(0x1bc) && igual(0x1bd)) {
                        let (a, d) = (b(0x1bc), b(0x1bd));
                        // a cor do cartao vai no fundo da celula ([esp+0x1c] na original), nao no texto
                        if codigo == 1 {
                            if a != 0 {
                                marcas = 0x20030;
                                fundo = colour_word_u32(0xaebdec);
                            } else if d != 0 {
                                marcas = 0x20030;
                                fundo = colour_word_u32(0xae31a8);
                            }
                        } else if a != 0 && d != 0 {
                            fundo = colour_word_u32(0xae31a8);
                            marcas = 0x40030;
                        }
                    }
                }
                3 => {
                    let (n1, n2, n3) = (
                        core::ptr::read_unaligned((reg + 0x198) as *const u32),
                        core::ptr::read_unaligned((reg + 0x19c) as *const u32),
                        core::ptr::read_unaligned((reg + 0x1a0) as *const u32),
                    );
                    marcas = 1;
                    estilo = 2;
                    coluna = 5;
                    fonte = 1;
                    fundo = 0;
                    let mut nome = [0u8; 0xc8];
                    nome_de(n1, n2, n3, nome.as_mut_ptr(), 0xc8, 1, id as u32);
                    let capitao = *((dados.wrapping_add(base_lado as usize) + 0x153) as *const i8) as i32;
                    if i == capitao {
                        texto(BUF, 0xa16a7c, nome.as_ptr());
                    } else {
                        copy_to_work_text(nome.as_ptr() as usize);
                    }
                    if id >= 0 && id < *(0xae2364 as *const i32) {
                        let pessoa = (*(0xae23bc as *const usize)).wrapping_add(id as usize * 110);
                        if confere(pessoa, n1, n2, n3, reg + 0x1a4) != 0 {
                            let v = if lado == 0 { *(dados as *const i32) } else { *((dados + 4) as *const i32) };
                            let limit = (*(0xae235c as *const i32)).wrapping_sub((*(0xae2350 as *const i32)).wrapping_mul(2));
                            botao = (if v >= limit { 0x7dc } else { 0x7d7 }, pessoa);
                        }
                    }
                    if core::ptr::read_unaligned((reg + 0x194) as *const u32) == core::ptr::read_unaligned((dados + 0x3c) as *const u32) {
                        cor = if *(0xb74349 as *const u8) != 0 { colour_word_u32(0xaebdec) } else { colour_word_u32(0xaebde8) };
                    } else if bi(0x1c3) > -1 || bi(0x1c4) > -1 || bi(0x1c0) > -1 {
                        cor = colour_word_u32(0xae317e);
                    } else if i <= 0xa || bi(0x1c2) != -1 {
                        cor = colour_word_u32(0xae3184);
                    } else {
                        cor = colour_word_u32(0xae317e);
                    }
                }
                4 => {
                    cor = colour_word_u32(0xae3154);
                    estilo = 1;
                    coluna = 6;
                    fonte = 1;
                    fundo = 0;
                    marcas = 1;
                    if !(i >= 0xb && bi(0x1c2) <= -1)
                        && !(foto != 0 && igual(0x1c3) && igual(0x1c4) && igual(0x1c0) && igual(0x1c2))
                    {
                        if bi(0x1c3) > -1 {
                            texto(BUF, 0xa16a68, bi(0x1c3) as i32 + 1);
                        } else if bi(0x1c4) > -1 {
                            texto(BUF, 0xa16a54, bi(0x1c4) as i32 + 1);
                        } else if bi(0x1c0) > -1 {
                            texto(BUF, 0xa16a40, bi(0x1c0) as i32 + 1);
                        } else if bi(0x1c2) > -1 {
                            texto(BUF, 0xa16a2c, bi(0x1c2) as i32 + 1);
                            cor = colour_word_u32(0xaebdd4);
                        }
                    }
                }
                _ => {
                    let (endereco_da_cor, campo) = TEAM_STATS_FIELDS[codigo - 5];
                    coluna = coluna.wrapping_add(1);
                    cor = colour_word_u32(endereco_da_cor);
                    marcas = 1;
                    fonte = 0xc;
                    fundo = 0;
                    estilo = 1;
                    if !(i >= 0xb && bi(0x1c2) <= -1) && !(foto != 0 && igual(campo) && igual(0x1c2)) {
                        match codigo {
                            22 => {
                                formatar(BUF, 0x98806c, bi(campo) as i32);
                                cor = colour_word_u32(0xaebdde);
                            }
                            24 => {
                                if b(campo) != 0 {
                                    formatar(BUF, 0x98806c, b(campo) as u32);
                                }
                            }
                            _ => {
                                formatar(BUF, 0x98806c, b(campo) as u32);
                            }
                        }
                    }
                }
            }
            if modo != 0 {
                let r = if botao.1 != 0 {
                    celula(sm, area, coluna, i + 1, marcas, fundo, fonte, estilo, cor, BUF, botao.0, botao.1)
                } else {
                    texto_da_celula(sm, area, coluna, i + 1, marcas, fundo, fonte, estilo, cor, BUF)
                };
                if item as u16 == 0xffff {
                    set_screen_param(0x1a, r as u16 as i16 as i32 as usize);
                    item = r;
                }
            } else {
                texto_do_item(sm, item, BUF);
                margem(sm, item, marcas);
                cor_do_texto(sm, item, cor);
                item = item.wrapping_add(1);
            }
        }
    }
}

/// { 0x724520 } (cdecl, match_screens.cpp, ~4,9 KB): reescreve as linhas do painel dos últimos placares
/// (parâmetros 0 a partida ou nulo, 0xe a entrada do dia, 0xf quantas linhas, 0x10 o primeiro item, 6
/// itens por linha, 0x11 a página mostrada, 0x12 parado). Limpa as linhas, depois reparte as partidas
/// da entrada ([+4] a [+8], 105 bytes em [[0xb67a50]+0x19]) em páginas (0x13 = quantas): por partida o
/// cabeçalho da fase (0x4bffc0, quando muda), a linha do jogo (times, placar contado pelos eventos,
/// período 0x6b6570) e os autores dos eventos (gol, pênalti, contra, pênalti perdido, expulsão, lesão;
/// minutos do mesmo jogador juntos) e o agregado. Só a página mostrada é escrita (segunda passada).
/// No rodapé "<< Página x/y >>" (ou "Hold") com mais de uma página; no fim a hora no parâmetro 0x14.
unsafe extern "C" fn refresh_latest_scores_rows() {
    let texto_do_item: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x801c70usize);
    let margem: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x800d30usize);
    let cor_do_texto: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x801960usize);
    let cor_de_fundo: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x801040usize);
    let fonte_do_item: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x801660usize);
    let ligar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x801fc0usize);
    let dado: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x8022d0usize);
    let position_place_players: unsafe extern "thiscall" fn(usize, u32, u32, u32, i32, u32) = core::mem::transmute(0x800890usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let rgb: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(0x5e4800usize);
    let nome_da_fase: unsafe extern "C" fn(i32, i32, usize, u32, u32) -> i32 = core::mem::transmute(0x4bffc0usize);
    let largura_do_texto: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x5e5ad0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let fase: unsafe extern "C" fn(u32, u32) -> u8 = core::mem::transmute(0x6b6570usize);
    let abandono: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x6d5930usize);
    let nome_do_jogador: unsafe extern "C" fn(usize, i32, i32, *mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(0x6db2d0usize);
    let confere: unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32 = core::mem::transmute(0x53e990usize);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c2990usize);
    let selecao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7e6f20usize);
    let agora: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x90c510usize);
    const BUF: usize = 0xde1f64;
    let sm = SCREEN_MANAGER;
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let i32_em = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let i16_em = |a: usize| core::ptr::read_unaligned(a as *const i16);
    let b = |a: usize| *(a as *const u8);

    let p0 = get_screen_param(0);
    let entrada = get_screen_param(0xe);
    let n = get_screen_param(0xf) as u8 as i32;
    let primeiro = get_screen_param(0x10) as u32;
    let pagina = get_screen_param(0x11) as u8 as i8;
    let parado = get_screen_param(0x12);
    let referencia = if p0 != 0 { p0 } else { u(*(0xb67a50 as *const usize) + 0x34) };
    let item = |linha: i32, desvio: u32| primeiro.wrapping_add((linha as u32).wrapping_mul(6)).wrapping_add(desvio);

    for r in 0..n {
        let it = item(r, 0);
        position_place_players(sm, it, 0, 0, 0, 0);
        margem(sm, it, 1);
        fonte_do_item(sm, it, 3);
        texto_do_item(sm, it, 0xad9c64);
        ligar(sm, it, 0);
        texto_do_item(sm, it + 1, 0xad9c64);
        texto_do_item(sm, it + 2, 0xad9c64);
        texto_do_item(sm, it + 3, 0xad9c64);
        fonte_do_item(sm, it + 4, 3);
        texto_do_item(sm, it + 4, 0xad9c64);
        ligar(sm, it + 4, 0);
        texto_do_item(sm, it + 5, 0xad9c64);
    }
    set_screen_param(0x13, 0);
    let mut paginas: i8 = 0;
    let mut k = i32_em(entrada + 4);
    let fim = i32_em(entrada + 8);
    // os 16 bits de cima de eax no índice do rodapé (`movzx ax` sobre o índice da partida)
    let mut alto = k as u32 & 0xffff0000;
    if k <= fim {
        let mut nova_pagina = true;
        let mut anterior = 0usize;
        let (mut linha, mut folga) = (0i32, 0i32);
        let abandonada = |partida: usize| {
            let e = u(partida + 0x57);
            e != 0 && abandono(e) != 0
        };
        loop {
            let deslocamento = k.wrapping_mul(105) as usize;
            let partida = u(*(0xb67a50 as *const usize) + 0x19).wrapping_add(deslocamento);
            let m4 = partida + 4;
            let (ebp, extras) = 'refaz: loop {
                if nova_pagina {
                    paginas = paginas.wrapping_add(1);
                    anterior = 0;
                    linha = 0;
                    folga = 0;
                    set_screen_param(0x13, paginas as i32 as usize);
                    nova_pagina = false;
                }
                let (mut g1, mut g2): (u8, u8) = (0, 0);
                if abandonada(partida) {
                    g1 = 0xfd;
                    g2 = 0xfd;
                }
                let passes = if paginas == pagina { 2 } else { 1 };
                let mut resultado = (0i32, 0i32);
                for passe in 1..=passes {
                    let mut ebp = folga + linha;
                    if passe != 2 && ebp > n {
                        nova_pagina = true;
                        continue 'refaz;
                    }
                    let (w32, w30) = (i16_em(m4 + 0x32), i16_em(m4 + 0x30));
                    let mesmo = anterior != 0 && w32 == i16_em(anterior + 0x32) && w30 == i16_em(anterior + 0x30);
                    if !mesmo && !(w32 == 0 && w30 == 0) {
                        if passe == 2 {
                            nome_da_fase(w32 as i32, w30 as i32, BUF, 0x7d0, 0);
                            let mut x = largura_do_texto(3, BUF) + 0x7d;
                            if x >= 0x2f1 {
                                x = 0x2f1;
                            }
                            let comp = u(m4 + 0x14);
                            let it = item(ebp, 0);
                            if amistoso(*(0xae2c74 as *const usize), comp) == 0 && selecao(*(0xb74434 as *const usize), comp) == 0 {
                                position_place_players(sm, it, 0x7d, 0, x, 0);
                                cor_do_texto(sm, it, colour_word_u32(0xaebdec));
                                texto_do_item(sm, it, BUF);
                                ligar(sm, it, 0x7e5);
                                let chave = ((*(comp as *const u32) << 11 | (w32 as u16 as u32 & 0x7ff)) << 11) | (w30 as u16 as u32 & 0x7ff);
                                dado(sm, it, chave as usize);
                            } else {
                                position_place_players(sm, it, 0x7d, 0, x, 0);
                                cor_do_texto(sm, it, colour_word_u32(0xaebdec));
                                texto_do_item(sm, it, BUF);
                            }
                            ebp += 1;
                            let it = item(ebp, 0);
                            position_place_players(sm, it, 0x7d, 0, x, 0);
                            margem(sm, it, 0xc400);
                            cor_de_fundo(sm, it, colour_word_u32(0xaebdec));
                            ebp += 1;
                        } else {
                            if ebp + 2 > n {
                                nova_pagina = true;
                                continue 'refaz;
                            }
                            ebp += 2;
                        }
                    }
                    if passe == 2 {
                        // a linha do jogo
                        let it = item(ebp, 0);
                        for (lado, desvio) in [(0x1cusize, 0u32), (0x20, 4)] {
                            let time = u(m4 + lado);
                            formatar(BUF, 0x987ff0, time + 0x38);
                            let h = humano(time, 0, 1, 0, 0);
                            cor_do_texto(sm, it + desvio, if h != 0 { colour_word_u32(0xaebdec) } else { colour_word_u32(0xaebde8) });
                            texto_do_item(sm, it + desvio, BUF);
                            ligar(sm, it + desvio, if b(m4 + 0x40 + desvio as usize / 4) != 0 { 0x7d5 } else { 0x7d6 });
                            dado(sm, it + desvio, time);
                            if desvio == 0 {
                                match g1 {
                                    0xfe => texto(BUF, 0x99c9e4, 0xad9c64),
                                    0xfd => texto(BUF, 0x99c9c4, 0xad9c64),
                                    _ => formatar(BUF, 0xa16d1c, g1 as i8 as i32),
                                };
                                texto_do_item(sm, it + 1, BUF);
                                texto_do_item(sm, it + 2, 0x989adc);
                                // a original decide o placar de fora pelo de casa (0xfe/0xfd)
                                match g1 {
                                    0xfe => texto(BUF, 0x99c9a4, 0xad9c64),
                                    0xfd => texto(BUF, 0x99c984, 0xad9c64),
                                    _ => formatar(BUF, 0xa16d14, g2 as i8 as i32),
                                };
                                texto_do_item(sm, it + 3, BUF);
                            }
                        }
                        // o período
                        let rotulo = (|| -> usize {
                            if b(referencia + 0x67) == 0xff {
                                return 0x98ba0c;
                            }
                            let lado_p = b(partida + 0x67);
                            if lado_p == 0xff {
                                return 0x98ba0c;
                            }
                            let er = u(referencia + 0x5f);
                            if er == 0 {
                                return 0x98ba0c;
                            }
                            let ep = u(partida + 0x5f);
                            if ep == 0 {
                                return 0x98ba0c;
                            }
                            let x = fase(*((er + 0x4f) as *const u16) as u32, b(referencia + 0x67) as u32) as i8;
                            let y = fase(*((ep + 0x4f) as *const u16) as u32, lado_p as u32) as i8;
                            let e = if x < y { referencia } else { partida };
                            let cl = b(e + 0x67);
                            let w = || *((u(e + 0x5f) + 0x4f) as *const u16);
                            if cl == 1 && w() == 0x1ef {
                                0xa16d10
                            } else if cl == 4 && w() == 0x3de {
                                0xa16d0c
                            } else if cl == 1 && w() == 0x3de {
                                0xa16d08
                            } else if cl == 2 && w() == 0 {
                                0xa16d04
                            } else if cl == 1 && w() == 0x483 {
                                0xa16cfc
                            } else if cl == 4 && w() == 0x528 {
                                0xa16cf4
                            } else if cl == 1 && w() == 0x528 {
                                0xa16cec
                            } else if cl == 2 && w() == 1 {
                                0xa16ce4
                            } else {
                                0xa16ce0
                            }
                        })();
                        texto(BUF, rotulo);
                        texto_do_item(sm, it + 5, BUF);
                    } else if ebp + 2 > n {
                        nova_pagina = true;
                        continue 'refaz;
                    }

                    // os eventos (até 50, 4 bytes em [partida + 0x5f] + 0x57: lado, jogador, tipo, minuto)
                    let mut usados = [0u32; 0x32];
                    let ev = u(u(*(0xb67a50 as *const usize) + 0x19).wrapping_add(deslocamento) + 0x5f);
                    ebp += 1;
                    let (mut casa, mut fora) = (ebp, ebp);
                    let mut extras = 0i32;
                    let (mut r24, mut cor4c, mut coluna44) = (0i32, 0u32, 0u32);
                    let (mut n98, mut na0, mut na4) = (0u32, 0u32, 0u32);
                    let valido = |minuto: u8| {
                        if b(referencia + 0x67) == 0xff {
                            return false;
                        }
                        let e = u(referencia + 0x5f);
                        e != 0 && (minuto as i16) <= i16_em(e + 0x4d)
                    };
                    let registro = |lado: u8, jogador: u8| {
                        ev.wrapping_add((lado as i8 as i32 * 1573) as usize).wrapping_add((jogador as i8 as i32 * 73) as usize)
                    };
                    let mut j = 0usize;
                    while ev != 0 && j < 0x32 {
                        let e = ev + 4 * j;
                        let lado = b(e + 0x57);
                        if lado == 0xff {
                            break;
                        }
                        if usados[j] != 0 || !valido(b(e + 0x5a)) {
                            j += 1;
                            continue;
                        }
                        let minuto = b(e + 0x5a) as u32;
                        let jogador = b(e + 0x58);
                        if passe == 2 {
                            let reg = registro(lado, jogador);
                            n98 = *((reg + 0x198) as *const u32);
                            na4 = *((reg + 0x1a0) as *const u32);
                            na0 = *((reg + 0x19c) as *const u32);
                        }
                        let tipo = b(e + 0x59);
                        if tipo == 7 || tipo == 8 {
                            j += 1;
                            continue;
                        }
                        if !(1..=6).contains(&tipo) {
                            show_error_at(0xa161d4, 0x1b9c);
                            j += 1;
                            continue;
                        }
                        // o cursor do lado (o gol contra vai para o outro lado)
                        let de_casa = (lado == 0) != (tipo == 2);
                        let r2c;
                        if de_casa {
                            r24 = casa;
                            casa += 1;
                            r2c = casa;
                        } else {
                            r24 = fora;
                            fora += 1;
                            r2c = fora;
                        }
                        if passe == 2 {
                            coluna44 = if de_casa { 0 } else { 2 };
                            let mut nome = [0u8; 0xc8];
                            nome_do_jogador(ev, jogador as i8 as i32, lado as i8 as i32, nome.as_mut_ptr(), 0xc8, 0, 0, 2);
                            let (modelo, c) = match tipo {
                                1 => (0xa16be8, colour_word_u32(0xae3184)),
                                3 => (0xa16c0c, colour_word_u32(0xae3184)),
                                2 => (0xa16cb8, colour_word_u32(0xaebdd4)),
                                4 => (0xa16c8c, 0),
                                5 => (0xa16c60, colour_word_u32(0xae31a8)),
                                _ => (0xa16c34, 0),
                            };
                            texto(BUF, modelo, nome.as_ptr(), minuto + 1);
                            cor4c = if tipo == 4 || tipo == 6 { rgb(0, 0xc0, 0, 0) } else { c };
                        } else if tipo <= 3 {
                            if abandonada(partida) {
                                g1 = 0xfd;
                                g2 = 0xfd;
                            } else if de_casa {
                                g1 = g1.wrapping_add(1);
                            } else {
                                g2 = g2.wrapping_add(1);
                            }
                        }
                        // os outros eventos do mesmo jogador e do mesmo tipo (gol e pênalti juntos)
                        let reg = registro(lado, jogador);
                        let id = i32_em(reg + 0x190);
                        let gol = |t: u8| t == 1 || t == 3;
                        for m in (j + 1)..0x32 {
                            let em = ev + 4 * m;
                            let lado_m = b(em + 0x57);
                            if lado_m == 0xff {
                                break;
                            }
                            if usados[m] != 0 || !valido(b(em + 0x5a)) {
                                continue;
                            }
                            let minuto_m = b(em + 0x5a) as u32;
                            if i32_em(registro(lado_m, b(em + 0x58)) + 0x190) != id {
                                continue;
                            }
                            let tm = b(em + 0x59);
                            if tm != tipo && !(gol(tm) && gol(tipo)) {
                                continue;
                            }
                            if passe == 2 {
                                let mut mais = [0u8; 0x100];
                                if tm == 3 {
                                    texto(mais.as_mut_ptr() as usize, 0xa16bd4, minuto_m + 1);
                                } else {
                                    formatar(mais.as_mut_ptr() as usize, 0x990b40, minuto_m + 1);
                                }
                                append_to_text_buffer(mais.as_ptr());
                            } else if abandonada(partida) {
                                g1 = 0xfd;
                                g2 = 0xfd;
                            } else if gol(tm) {
                                if lado_m == 0 { g1 = g1.wrapping_add(1) } else { g2 = g2.wrapping_add(1) }
                            } else if tm == 2 {
                                if lado_m != 0 { g1 = g1.wrapping_add(1) } else { g2 = g2.wrapping_add(1) }
                            }
                            usados[m] = 1;
                        }
                        if passe == 2 {
                            let it = item(r24, coluna44 * 2);
                            cor_do_texto(sm, it, cor4c);
                            texto_do_item(sm, it, BUF);
                            if id >= 0 && id < *(0xae2364 as *const i32) {
                                let pessoa = (*(0xae23bc as *const usize)).wrapping_add(id as usize * 110);
                                if confere(pessoa, n98, na0, na4, reg + 0x1a4) != 0 {
                                    let v = if lado == 0 { i32_em(ev) } else { i32_em(ev + 4) };
                                    let limit = (*(0xae235c as *const i32)).wrapping_sub((*(0xae2350 as *const i32)).wrapping_mul(2));
                                    ligar(sm, it, if v >= limit { 0x7dc } else { 0x7d7 });
                                    dado(sm, it, pessoa);
                                }
                            }
                        } else if r2c > n {
                            nova_pagina = true;
                            continue 'refaz;
                        }
                        usados[j] = 1;
                        if (lado == 0 && casa > fora) || (lado == 1 && fora > casa) {
                            extras += 1;
                        }
                        j += 1;
                    }

                    // o agregado (ida e volta)
                    if b(m4 + 0x4b) as i8 > -1 {
                        let (r, do_fora, prox);
                        if casa <= fora {
                            r = casa;
                            casa += 1;
                            do_fora = false;
                            prox = casa;
                        } else {
                            r = fora;
                            fora += 1;
                            do_fora = true;
                            prox = fora;
                        }
                        if passe == 2 {
                            let ida = u(partida);
                            texto(BUF, 0xa16bb0, b(ida + 0x4b) as i8 as i32 + g1 as i8 as i32, b(ida + 0x4c) as i8 as i32 + g2 as i8 as i32);
                            let it = item(r, if do_fora { 4 } else { 0 });
                            fonte_do_item(sm, it, 2);
                            cor_do_texto(sm, it, colour_word_u32(0xaebdec));
                            texto_do_item(sm, it, BUF);
                        } else if prox > n {
                            nova_pagina = true;
                            continue 'refaz;
                        }
                        if (!do_fora && casa > fora) || (do_fora && fora > casa) {
                            extras += 1;
                        }
                    }
                    resultado = (casa.max(fora), extras);
                }
                break resultado;
            };
            linha = ebp;
            folga = (3 - extras).max(1);
            anterior = m4;
            k = k.wrapping_add(1);
            alto = k as u32 & 0xffff0000;
            if k > fim {
                break;
            }
        }
    }

    let it = (primeiro as usize).wrapping_add(((alto | n as u32) as usize).wrapping_mul(6)) as u32;
    if paginas > 1 {
        margem(sm, it, 0x30);
        if *(0x9cc2c4 as *const i32) != -1 {
            formatar(BUF, 0x99ec1c, 0x2a);
            texto_do_item(sm, it, BUF);
        } else {
            texto_do_item(sm, it, 0x98e7c4);
        }
        let modelo = if parado != 0 { 0xa16b8c } else { 0xa16b68 };
        texto(BUF, modelo, pagina as i32, paginas as i32);
        margem(sm, it + 1, 2);
        texto_do_item(sm, it + 1, BUF);
        margem(sm, it + 2, 0x30);
        if *(0x9cc2c4 as *const i32) != -1 {
            formatar(BUF, 0x99ec1c, 0x2b);
            texto_do_item(sm, it + 2, BUF);
        } else {
            texto_do_item(sm, it + 2, 0x98e7a8);
        }
    } else {
        for d in 0..3 {
            margem(sm, it + d, 1);
            texto_do_item(sm, it + d, 0xad9c64);
        }
    }
    set_screen_param(0x14, agora() as usize);
}

/// { 0x727fb0 } (cdecl, match_screens.cpp): a resposta da tela de espera do jogo em rede. Sem ficha do
/// humano da vez (0x765480) ou sem partida nela (+0xe2): −6. Outro humano (0x7ffd80) não ausente
/// (0x7ffdd0) com a mesma partida: guarda a chave dele em [0xb77c4a] e −6. Senão fecha as telas
/// (0x803d80; bloqueado: −11), mostra "Waiting for other players..." (0x802f80) e abre a tela da
/// partida (0x718aa0) na vaga do humano: −4.
unsafe extern "C" fn match_screens_handle_waiting_screen() -> i32 {
    crate::comparacao::ramo("telas espera dos outros jogadores");
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x765480usize);
    let humanos: unsafe extern "thiscall" fn(usize, *mut usize, *mut i16, i32) = core::mem::transmute(0x7ffd80usize);
    let ausente: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7ffdd0usize);
    let fechar_tudo: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x803d80usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let mostrar: unsafe extern "thiscall" fn(usize, usize, i32, u32, i32) = core::mem::transmute(0x802f80usize);
    let abrir_partida: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x718aa0usize);
    let sm = SCREEN_MANAGER;
    let h = || *(0xb77c7a as *const i16);
    let chave_atual = || core::ptr::read_unaligned((0xb74c26 + h() as isize * 0x300) as *const usize);
    if ficha(0) == 0 {
        return -6;
    }
    let partida = core::ptr::read_unaligned((ficha(0) + 0xe2) as *const usize);
    if partida == 0 {
        return -6;
    }
    let mut chaves = [0usize; 16];
    let mut quantos: i16 = 0;
    humanos(sm, chaves.as_mut_ptr(), &mut quantos, h() as i32);
    let mut k = 0usize;
    while (k as i32) < quantos as i32 {
        let c = chaves[k];
        if c != chave_atual() && ausente(sm, c) == 0 && core::ptr::read_unaligned((ficha(c) + 0xe2) as *const usize) == partida {
            *(0xb77c4a as *mut usize) = c;
            return -6;
        }
        k += 1;
    }
    if fechar_tudo(sm) == 0 {
        return -11;
    }
    texto(0xde1f64, 0xa16d94);
    mostrar(sm, 0xde1f64, 1, 0, h() as i32);
    abrir_partida(chave_atual(), partida);
    -4
}

/// As células são comparadas de verdade: montagem (`modo` ≠ 0) pela lista de exibição, atualização
/// local pelo `Display` do gerente; os parâmetros (o 0x1a) e o texto 0xde1f64 também. Atualização que
/// vai pela rede ou parâmetro 0x1c fora de 1 e 2: só a original.
unsafe extern "C" fn comparar_fill_team_stats_rows(modo: u32, area: usize) {
    let o = original_fn::<unsafe extern "C" fn(u32, usize)>(%%I0%%);
    let coluna = get_screen_param(0x1c) as u8;
    let mut reg = match screen_update_regions(modo != 0) {
        Some(r) if coluna == 1 || coluna == 2 => r,
        _ => return o(modo, area),
    };
    reg.extend(screen_param_regions());
    reg.push((0xde1f64, 0x100));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(modo, area), || fill_team_stats_rows(modo, area));
    PLACAR.registrar(igual, || format!("fill_team_stats_rows({}, {:#x}): diferencas em {:?}", modo, area, first_differences(&mo, &mr)));
}

/// O redesenho é comparado de verdade (atualização local): o `Display` do gerente, os parâmetros (menos
/// o 0x14, a hora que a rotina grava no fim) e o texto 0xde1f64. Pela rede: só a original.
unsafe extern "C" fn comparar_refresh_latest_scores_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let mut reg = match screen_update_regions(false) {
        Some(r) => r,
        None => return o(),
    };
    reg.push((0xb6b7a8, 12));
    let vaga = *((SCREEN_MANAGER + 0x382a) as *const i16) as isize;
    let bloco = core::ptr::read_unaligned((SCREEN_MANAGER as isize + 0x7e2 + vaga * 0x300) as *const usize);
    if bloco != 0 {
        reg.push((bloco, 0xb4));
        reg.push((bloco + 0xb8, 0x200 - 0xb8));
    }
    reg.push((0xde1f64, 0x100));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(), || refresh_latest_scores_rows());
    PLACAR.registrar(igual, || format!("refresh_latest_scores_rows: diferencas em {:?}", first_differences(&mo, &mr)));
}

// ponytail: a espera fecha as telas, mostra a mensagem e abre a tela da partida (rede); não roda duas
// vezes: no modo comparação só a original (a cópia é provada no modo Rust pelo ramo)
unsafe extern "C" fn comparar_match_screens_handle_waiting_screen() -> i32 {
    original_fn::<unsafe extern "C" fn() -> i32>(%%I2%%)()
}
