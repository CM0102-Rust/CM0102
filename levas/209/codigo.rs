// ---------- 209a leva: as telas da partida (14): a montagem e a resposta da tela da partida ao vivo e a montagem da tela de detalhes (match_screens.cpp) ----------

/// Os 25 registros de aba (0xd4 bytes: +4 o título, +0xcc a marca, +0xd0 o valor) que a barra de abas
/// (0x5ed780) lê, construídos (0x4568f0) e destruídos (0x5a53c0) pelos iteradores do runtime
/// (0x9463af/0x9462bb) como na original.
type AbasDaPartida = [u32; 0x14b4 / 4];

unsafe fn construir_abas_da_partida(abas: &mut AbasDaPartida) {
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(0x9463afusize);
    vetor(abas.as_mut_ptr() as usize, 0xd4, 0x19, 0x4568f0, 0x5a53c0);
}

unsafe fn destruir_abas_da_partida(abas: &mut AbasDaPartida) {
    let fim_do_vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(0x9462bbusize);
    fim_do_vetor(abas.as_mut_ptr() as usize, 0xd4, 0x19, 0x5a53c0);
}

/// O título da aba `k` (texto em +4) e a marca e o valor dela.
unsafe fn titulo_da_aba(abas: &mut AbasDaPartida, k: usize) -> usize {
    abas.as_mut_ptr() as usize + k * 0xd4 + 4
}
unsafe fn marcar_aba(abas: &mut AbasDaPartida, k: usize, marca: u32, valor: u32) {
    abas[k * 53 + 51] = marca;
    abas[k * 53 + 52] = valor;
}

/// Libera a foto da partida do parâmetro 0x1d (0xd7a bytes; o vetor de 2 x 0x625 em +0x11f, 0x453c10)
/// e zera o parâmetro.
unsafe fn liberar_foto_da_partida() {
    if parametro_da_tela(0x1d) != 0 {
        let f = parametro_da_tela(0x1d);
        if f != 0 {
            let fim_do_vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(0x9462bbusize);
            let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
            fim_do_vetor(f + 0x11f, 0x625, 2, 0x453c10);
            apagar(f);
        }
        gravar_parametro_da_tela(0x1d, 0);
    }
}

/// O fundo das telas da partida: a foto (0x41c670), senão o da competição (0x41c410), senão o do clube
/// (0x41c250 e 0x41bfb0). Devolve o `eax` da última chamada (a original empilha o byte da cor do
/// uniforme por cima dele).
unsafe fn fundo_da_partida(foto: usize, competicao: usize, clube: usize) -> usize {
    let pela_foto: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x41c670usize);
    let pela_competicao: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x41c410usize);
    let pelo_clube: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x41c250usize);
    let aplicar: unsafe extern "C" fn() -> usize = core::mem::transmute(0x41bfb0usize);
    if foto != 0 {
        let r = pela_foto(foto, 0, 1);
        if r != 0 {
            return r;
        }
    }
    let r = pela_competicao(competicao, 0, 1);
    if r != 0 {
        return r;
    }
    pelo_clube(clube, 0, 1);
    aplicar()
}

/// A caixa "Error" de match_screens.cpp na linha `linha` (sem parar o jogo) e [0xb67a34] = 0.
unsafe fn erro_das_telas_da_partida(linha: u32) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let juntar: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa161d4, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = juntar(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s);
    caixa(0x9870e0, 0xae24d0, 0);
    *(0xb67a34 as *mut u32) = 0;
}

/// O nome do time no texto 0xde1f64 do cabeçalho da partida: só o nome ("  %s") com `so_o_nome`;
/// senão a seleção B ("'B'", 0x53b900) ou o time reserva ("Res.").
unsafe fn nome_do_time_na_partida(time: usize, so_o_nome: u8) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let selecao_b: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if so_o_nome != 0 {
        formatar(0xde1f64, 0x987ff0, time + 0x38);
    } else if selecao_b(time) != 0 {
        let c = *((time + 0x37) as *const i8) as i32;
        texto(0xde1f64, 0xa164a4, c, c, time + 4);
    } else {
        let c = *((time + 0x52) as *const i8) as i32;
        texto(0xde1f64, 0xa16478, c, c, time + 0x38);
    }
}

/// Uma linha da velocidade da narração `valor` na lista `lista`, marcada (0x2002) quando é a atual `v`
/// (a devolução de 0x5fcff0). A original empilha uma das três cores (0xae31aa, 0xaebdec, 0xaebdea, nessa
/// ordem) no `eax` que sobrou de 0x5fcff0 (os 16 bits de cima conhecidos) e as outras em registradores
/// sujos (aqui estendidas); `sujo` diz qual, (marcada, não marcada).
unsafe fn linha_de_velocidade(lista: usize, valor: u16, v: u32, sujo: (usize, usize)) -> usize {
    let linha: LinhaDoDisplay = core::mem::transmute(0x55f990usize);
    let marcada = v as u16 == valor;
    let k = if marcada { sujo.0 } else { sujo.1 };
    let enderecos = [0xae31aausize, 0xaebdec, 0xaebdea];
    let cor = |i: usize| if i == k { cor_sobre(v as usize, enderecos[i]) } else { cor_em(enderecos[i]) };
    let marca = if marcada { 0x2002 } else { 2 };
    linha(display_do_humano(), marca, 0, 0, 0, 0, 0, 0, 0x10, cor(0), cor(1), 1, 1, cor(2), 0xde1f64, 0, 0xc, valor as usize, lista)
}

/// { 0x718e90 } (cdecl, SEH, match_screens.cpp, ~5 KB): monta a tela da partida ao vivo. Liga a marca
/// +0xea da ficha do humano (0x765480) e libera a foto do parâmetro 0x1d. Modo 1 (parâmetro 1): só a
/// figura de fundo vazia (0x41c0d0). Senão: tira a marca de "saiu" (−911) do humano nos espectadores
/// ([partida+0x5b] +0xc5 + 4h, contando em +0x108); o fundo; os cabeçalhos dos dois times (cores do
/// uniforme 0x53b640, nome, caixa do clube, placar ou "A" de abandonada) e o agregado ([+0x4f] ≥ 0;
/// somado ao placar fora do modo 2 de [+0x67]); a área da narração (fundo "match.mbr"), a do relógio
/// (minuto [+0x4d] no parâmetro 3), as setas "<<<"/">>>" (ou os símbolos da fonte [0x9cc2c4]), o
/// controle (0x725a70, parâmetro 4) e os botões táticos dos dois times (parâmetros 5 e 6; o do
/// parâmetro 0x15 marcado); com o humano no comando de um dos times (0x600c90) ou sem humano nenhum
/// (0x600a50), as velocidades da narração (0x5fcff0; a atual marcada, o foco [Display+0x12f4ff] e o
/// volume do narrador [+0x2db0]). Por fim a ficha +0 = 3, as abas (Overview, Stats, Zones, Report, as
/// estatísticas dos times, Ratings, Latest Scores e, para liga de verdade sem seleções, League Table),
/// −1 no parâmetro 8, o painel da aba (parâmetro 2), o estádio e, com o parâmetro 8 em −1, o painel de
/// narração (com o lance do parâmetro 7 narrado por 0x6d7b00) no parâmetro 8.
unsafe extern "C" fn montar_tela_da_partida() {
    crate::comparacao::ramo("telas montar a partida");
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;
    const MENOS1: usize = usize::MAX;
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let texto_do_gerente: TextoDoGerente = core::mem::transmute(0x803fd0usize);
    let caixa: CaixaDoGerente = core::mem::transmute(0x804030usize);
    let celula: CelulaDaLista = core::mem::transmute(0x8040a0usize);
    let texto_da_celula: TextoDaCelulaDaLista = core::mem::transmute(0x804100usize);
    let lista: ListaDoDisplay = core::mem::transmute(0x55fba0usize);
    let linha: LinhaDoDisplay = core::mem::transmute(0x55f990usize);
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x765480usize);
    let cores: unsafe extern "C" fn(usize, usize, *mut u32, *mut u32) = core::mem::transmute(0x53b640usize);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let rb = |a: usize| *(a as *const u8);
    let sx = |a: usize| *(a as *const i8) as i32;
    let velocidade = || {
        let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5fcff0usize);
        f(*(0xb63c98 as *const usize), 0)
    };
    let gravar_item = |k: u32, r: usize| gravar_parametro_da_tela(k, r as i16 as isize as usize);

    let mut abas: AbasDaPartida = [0; 0x14b4 / 4];
    construir_abas_da_partida(&mut abas);
    *((ficha(0) + 0xea) as *mut u32) = 1;
    liberar_foto_da_partida();
    let p = parametro_da_tela(0);
    let modo = parametro_da_tela(1) as u8;
    let aba = parametro_da_tela(2) as u8;
    let p15 = parametro_da_tela(0x15);
    parametro_da_tela(0xe);
    if modo == 1 {
        let figura: unsafe extern "C" fn(usize) = core::mem::transmute(0x41c0d0usize);
        figura(0);
        destruir_abas_da_partida(&mut abas);
        return;
    }
    let espectadores = rd(p + 0x5b);
    let st = rd(p + 0x5f);
    let h = *(0xb77c7a as *const i16) as isize;
    let marca = (espectadores as isize + 0xc5 + h * 4) as *mut i32;
    if core::ptr::read_unaligned(marca) == -911 {
        core::ptr::write_unaligned(marca, 0);
        let n = (espectadores + 0x108) as *mut u8;
        *n = (*n).wrapping_sub(1);
    }
    fundo_da_partida(rd(p + 0x1c), rd(p + 0x18), rd(p + 0x20));

    // os cabeçalhos: as cores u16 (o resto das variáveis da original é lixo da pilha, aqui 0); o byte do
    // uniforme vai num registrador sujo (estendido)
    let (mut cor_a, mut cor_b) = (0u32, 0u32);
    cores(rd(p + 0x20), rb(st + 0x152) as usize, &mut cor_a, &mut cor_b);
    nome_do_time_na_partida(rd(p + 0x20), rb(p + 0x44));
    texto_do_gerente(sm, 0x64, 0xa, 0x1bc, 0x46, 0x30, cor_b as usize, 1, 7, cor_a as usize, BUF, 1);
    caixa(sm, 0x64, 0xa, 0x1bc, 0x46, 1, 0, 1, 7, 0, 0, 1, rd(p + 0x20));
    let r = if rb(st + 0x11f) == 0xfd { formatar(BUF, 0x9b760c) } else { formatar(BUF, 0x98806c, rb(st + 0x11f) as u32) };
    texto_do_gerente(sm, 0x18f, 0xf, 0x1b7, 0x41, 0x30, cor_em(0xae3184), 0xc, 6, cor_sobre(r as usize, 0xaebdea), BUF, 1);
    cores(rd(p + 0x24), rb(st + 0x777) as usize, &mut cor_a, &mut cor_b);
    nome_do_time_na_partida(rd(p + 0x24), rb(p + 0x45));
    texto_do_gerente(sm, 0x1be, 0xa, 0x316, 0x46, 0x30, cor_b as usize, 1, 7, cor_a as usize, BUF, 1);
    caixa(sm, 0x1be, 0xa, 0x316, 0x46, 1, 0, 1, 7, 0, 0, 1, rd(p + 0x24));
    let r = if rb(st + 0x11f) == 0xfd { formatar(BUF, 0x9b760c) } else { formatar(BUF, 0x98806c, rb(st + 0x744) as u32) };
    texto_do_gerente(sm, 0x2e9, 0xf, 0x311, 0x41, 0x30, cor_em(0xae3184), 0xc, 6, cor_sobre(r as usize, 0xaebdea), BUF, 1);
    let agregado = sx(p + 0x4f);
    if agregado > -1 {
        let (a, b) = if rb(p + 0x67) != 2 {
            (agregado + rb(st + 0x11f) as i32, sx(p + 0x50) + rb(st + 0x744) as i32)
        } else {
            (agregado, sx(p + 0x50))
        };
        texto(BUF, 0xa16454, a, b);
        texto_do_gerente(sm, 0x1c3, 0x37, 0x2e4, 0x41, 1, 0, 0x40, 1, cor_a as usize, BUF, 1);
    }

    // a narração, o relógio e as setas
    // o y2 e o imediato do push em 0x719345: 0x257 no exe 800x600, 0x437 no cm0102-1080 (resolucao.py)
    let y2 = core::ptr::read_unaligned(0x719346 as *const u32) as usize;
    let narracao = lista(display_do_humano(), 0, 0, 0x59, y2, 1, 0, 1, 0, 1, 0, MENOS1);
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let fundo = caminho(0x9b5348, 0xa16448);
    linha(display_do_humano(), 0x400, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0xc, 1, 0, fundo, 0, 0, 0, narracao);
    let relogio = lista(display_do_humano(), 5, 0xa, 0x55, 0x24e, 1, 0, 0xd, 0, 1, 0, MENOS1);
    formatar(BUF, 0x98806c, *((st + 0x4d) as *const i16) as i32);
    let r = texto_da_celula(sm, relogio, 0, 0, 0x1061, 0, 0xc, 5, cor_em(0xaebdec), BUF);
    gravar_item(3, r);
    let setas = lista(display_do_humano(), 5, 0xa, 0x55, 0x24e, 2, 0, 0xd, 0, 1, 0, MENOS1);
    if *(0x9cc2c4 as *const i32) != -1 {
        formatar(BUF, 0x99ec1c, 0x28);
        celula(sm, setas, 0, 1, 0x1021, 0, 0xc, 0, cor_em(0xaebdec), BUF, (-2isize) as usize, 0);
    } else {
        // a original empilha o eax com [0x9cc2c4] = −1 em cima da cor
        celula(sm, setas, 0, 1, 0x1021, 0, 0xc, 1, cor_sobre(MENOS1, 0xae3184), 0xa16444, (-2isize) as usize, 0);
    }
    if *(0x9cc2c4 as *const i32) != -1 {
        formatar(BUF, 0x99ec1c, 0x29);
        celula(sm, setas, 1, 1, 0x1021, 0, 0xc, 0, cor_em(0xaebdec), BUF, (-3isize) as usize, 0);
    } else {
        celula(sm, setas, 1, 1, 0x1021, 0, 0xc, 1, cor_em(0xae3184), 0xa16440, (-3isize) as usize, 0);
    }

    // o controle da partida (0x725a70 lê só o byte do modo; a cor da original fica nos 16 bits de baixo
    // da variável que guardava os espectadores)
    let rotulo: unsafe extern "C" fn(u32, usize, usize, u32, *mut u32) -> u32 = core::mem::transmute(0x725a70usize);
    let mut marcas: u32 = 0;
    rotulo(modo as u32, espectadores, BUF, 0x7d0, &mut marcas);
    let r = linha(display_do_humano(), 2, 0, 0, 0, 0, 0, 2, marcas as usize, 0, 0, 0xc, 1, cor_sobre(espectadores, 0xae3184), BUF, 0, 2, 0, relogio);
    gravar_item(4, r);

    // os botões táticos dos dois times (o do parâmetro 0x15 marcado)
    for (k, lado, param) in [(3usize, 0x20usize, 5u32), (4, 0x24, 6)] {
        let t = rd(p + lado);
        let c = sx(t + 0x52);
        texto(BUF, 0xa16428, c, c, t + 0x38);
        let r = if t == p15 {
            // no segundo time a original empilha a cor sobre o ponteiro do time (eax)
            let cor = if k == 4 { cor_sobre(t, 0xae3184) } else { cor_em(0xae3184) };
            celula(sm, relogio, 0, k, 0x1061, 0, 0xc, 1, cor, BUF, 1, t)
        } else {
            celula(sm, relogio, 0, k, 0x1021, 0, 0xc, 1, cor_em(0xae3184), BUF, 1, t)
        };
        gravar_item(param, r);
    }

    // as velocidades da narração
    let manda: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x600c90usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let mostrar = manda(rd(p + 0x20), 0, 1) != 0
        || manda(rd(p + 0x24), 0, 1) != 0
        || (humano(rd(p + 0x20), 0, 1, 0, 0) == 0 && humano(rd(p + 0x24), 0, 1, 0, 0) == 0);
    if mostrar {
        texto(BUF, 0xa16414);
        let r = linha(display_do_humano(), 0x82, 0, 0, 0, 0, 0, 5, 0x1021, 0, 0, 0xc, 1, cor_sobre(p15, 0xae3184), BUF, 0, 0, 0, relogio);
        let v = lista(display_do_humano(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, cor_em(0xae31aa), r);
        texto(BUF, 0xa16408);
        let muito_rapida = linha_de_velocidade(v, 0x14, velocidade(), (0, 1));
        texto(BUF, 0xa16400);
        let rapida = linha_de_velocidade(v, 0x37, velocidade(), (2, 0));
        texto(BUF, 0xa163cc, 0xad9c64);
        let media = linha_de_velocidade(v, 0x5a, velocidade(), (1, 2));
        texto(BUF, 0xa163c4);
        let lenta = linha_de_velocidade(v, 0x7d, velocidade(), (0, 1));
        texto(BUF, 0xa163b8);
        let muito_lenta = linha_de_velocidade(v, 0x96, velocidade(), (2, 0));
        // o foco vai para a linha de cima da atual (a muito lenta para a muito rápida e as fora da tabela)
        let foco = match velocidade() as i16 {
            0x37 => muito_rapida,
            0x5a => rapida,
            0x7d => media,
            0x96 => lenta,
            _ => muito_lenta,
        };
        let d = display_do_humano();
        core::ptr::write_unaligned((d + 0x12f4ff) as *mut u16, foco as u16);
        *((d + 0x12f4fe) as *mut u8) = 0x20;
        let narrador = rd(rd(p + 0x57) + 0x8f7a);
        *((narrador + 0x2db0) as *mut u8) = velocidade() as u8;
    }
    *(ficha(0) as *mut u8) = 3;

    // as abas
    texto(titulo_da_aba(&mut abas, 0), 0xa163a8);
    marcar_aba(&mut abas, 0, 3, 1);
    texto(titulo_da_aba(&mut abas, 1), 0xa1639c);
    marcar_aba(&mut abas, 1, 3, 2);
    texto(titulo_da_aba(&mut abas, 2), 0xa1638c);
    marcar_aba(&mut abas, 2, 3, 3);
    texto(titulo_da_aba(&mut abas, 3), 0xa1637c);
    marcar_aba(&mut abas, 3, 3, 4);
    let t1 = rd(p + 0x20);
    texto(titulo_da_aba(&mut abas, 4), 0xa16364, sx(t1 + 0x52), sx(t1 + 0x52), t1 + 0x38);
    marcar_aba(&mut abas, 4, 3, 6);
    texto(titulo_da_aba(&mut abas, 5), 0xa16354);
    marcar_aba(&mut abas, 5, 3, 7);
    texto(titulo_da_aba(&mut abas, 6), 0xa16328, 0xad9c64);
    marcar_aba(&mut abas, 6, 3, 8);
    let mut n: usize = 7;
    // a tabela da liga da partida (a de seleções pelo continente do clube do humano; jogo de copa pela fase)
    let ligas = |k: isize| rd((rd(0xadadfc) as isize + k * 4) as usize);
    let comp = rd(p + 4) as i32 as isize;
    let entrada = ligas(comp);
    let mut liga: usize = 0;
    if entrada != 0 {
        let selecoes = *(0x9cf590 as *const i32) as isize;
        let candidata = if comp == selecoes {
            if sx(p + 0x46) >= 2 {
                None
            } else {
                let vaga = rd((0xb74c26 + h * 0x300) as usize);
                let lado = if rd(rd(p + 0x24) + 0xcf) == vaga { rd(p + 0x24) } else { rd(p + 0x20) };
                let continente: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x909ec0usize);
                let selecoes = || *(0x9cf590 as *const i32) as isize;
                Some(if continente(lado) as u16 == 0x414 {
                    rd(rd(ligas(selecoes()) + 0xc) + 4)
                } else if continente(lado) as u16 == 0x47b {
                    rd(rd(ligas(selecoes()) + 0xc))
                } else {
                    ligas(selecoes())
                })
            }
        } else {
            let fase = *((p + 0x46) as *const i8);
            Some(if fase == -1 { entrada } else { rd((rd(entrada + 0xc) as isize + fase as isize * 4) as usize) })
        };
        if let Some(l) = candidata {
            liga = l;
            let selecao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7e6f20usize);
            if l != 0 && rb(l + 0x43) == 2 && selecao(*(0xb74434 as *const usize), rd(l + 4)) == 0 {
                texto(titulo_da_aba(&mut abas, 7), 0x99c280);
                marcar_aba(&mut abas, 7, 3, 0xa);
                n = 8;
            }
        }
    }
    let t2 = rd(p + 0x24);
    texto(titulo_da_aba(&mut abas, n), 0xa16364, sx(t2 + 0x52), sx(t2 + 0x52), t2 + 0x38);
    marcar_aba(&mut abas, n, 3, 9);
    let (mut topo, mut base) = (0x50i32, 0x221i32);
    let barra: unsafe extern "C" fn(usize, u32, i32, *mut i32, *mut i32, u32) -> u32 = core::mem::transmute(0x5ed780usize);
    let aba = aba as i8 as i32;
    barra(abas.as_mut_ptr() as usize, n as u32 + 1, aba, &mut topo, &mut base, 0);
    gravar_parametro_da_tela(8, MENOS1);
    let painel = |f: usize, topo: *mut i32, base: *mut i32| {
        let g: unsafe extern "C" fn(*mut i32, *mut i32) = core::mem::transmute(f);
        g(topo, base)
    };
    let estatisticas_do_time: unsafe extern "C" fn(u32, *mut i32, *mut i32) = core::mem::transmute(0x7217a0usize);
    match aba {
        1 => painel(0x71c270, &mut topo, &mut base),
        2 => painel(0x71e040, &mut topo, &mut base),
        3 => painel(0x71f500, &mut topo, &mut base),
        4 => painel(0x721000, &mut topo, &mut base),
        6 => estatisticas_do_time(0, &mut topo, &mut base),
        7 => painel(0x722ed0, &mut topo, &mut base),
        8 => painel(0x723d20, &mut topo, &mut base),
        9 => estatisticas_do_time(1, &mut topo, &mut base),
        10 => {
            let tabela: unsafe extern "C" fn(usize, i32, i32) = core::mem::transmute(0x7280e0usize);
            tabela(liga, topo, base);
        }
        _ => {}
    }

    // o estádio ("estádio, cidade"; sem cidade só o nome, sem estádio "Unknown Stadium")
    let estadio = rd(p + 0x1c);
    let r: usize = if estadio != 0 {
        let cidade = rd(estadio + 0x38);
        if cidade != 0 {
            let (a, b) = (sx(estadio + 0x37), sx(cidade + 0x1e));
            texto(BUF, 0x9902a4, a, a, b, b, estadio + 4, cidade + 4) as usize
        } else {
            // o strcpy da original deixa eax e ecx em 0
            copiar_para_o_texto(estadio + 4);
            0
        }
    } else {
        texto(BUF, 0x99c1a4) as usize
    };
    texto_do_gerente(sm, 0x64, 0x22b, 0x316, 0x24e, 0x30, cor_em(0xae317e), 0xc, 3, cor_sobre(r, 0xae3184), BUF, 1);

    // o painel de narração
    if parametro_da_tela(8) as u16 == 0xffff {
        if parametro_da_tela(7) != 0 {
            let narrador = rd(rd(p + 0x57) + 0x8f7a);
            let r = texto_do_gerente(sm, 0x64, 0x22b, 0x316, 0x24e, 1, 0, 0xc, 3, 0, 0, 1);
            let narrar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x6d7b00usize);
            narrar(narrador, rd(rd(p + 0x5b) + 1), st, rd(p + 4), r, 0x2b2, 0, 3, 0);
            gravar_item(8, r);
        } else {
            let r = texto_do_gerente(sm, 0x64, 0x22b, 0x316, 0x24e, 1, 0, 0xc, 3, 0, 0, 1);
            gravar_item(8, r);
        }
    }
    destruir_abas_da_partida(&mut abas);
}

/// { 0x71a240 } (cdecl, match_screens.cpp): a resposta da tela da partida ao vivo ao item `item` (−1:
/// a passagem do tempo). Em rede ([0xb77c84]) ou com a partida a confirmar no modo 1 sem confirmação:
/// 0; espectadores fechados ([+0x119]): −9; modo 1: confirma pela rede (0x6b5260) e, confirmado, passa
/// ao modo 2 e −11. O item de id 0xc troca o volume do narrador ([+0x2db0]); volume diferente da
/// velocidade do técnico (0x5fcff0) repassa (0x5fd100) e −11. Sem item: com o humano conectado
/// (0x5fd210), os modos 2 e 3 viram 4 (deixa de assistir) e o 5 para a narração (0x802a50, −9); no modo
/// 4 com lance novo (0x6b55c0) lê as marcas do menu da partida (+0: 1 e 2 refazem tudo, 4 o relógio, 8
/// narra o lance (0x6d7b00) e, nos de substituição, marca o time do humano no parâmetro 0x15, 0x20 e
/// 0x40 os modos 3 e 5); a marca de "esperando" (parâmetro 0x24) e o rótulo do controle (0x725a70); sem
/// time marcado, procura o do humano (parâmetros 0x2f/0x30). Com um time a abrir (o marcado, ou o
/// item de id 1): abre a tática dele (0x899a50, −4) se o jogo permite ([+0x8eb2] sem lance), senão
/// remarca os botões (0x800d30) e o parâmetro 0x15. Item de id 2: assistir ou não (modos 2/3/4), 5
/// para (−9); id 3: troca a aba (parâmetro 2). A barra de menu (0x7643d0; −4 marca o humano como
/// saindo, −911) e depois a aba decide (os painéis 0x71d9e0, 0x71e750, 0x720320 (id 4 troca o período
/// do parâmetro 0xa), 0x723350; as respostas 0x722dc0 e 0x725860; a 10 converte o parâmetro 0x26 no
/// 0x25). Por fim −11 com [0xb6b7b0] ou o redesenho (0x802d00) com [0xb6b7a8].
unsafe extern "C" fn responder_tela_da_partida(item: u32) -> i32 {
    crate::comparacao::ramo("telas responder a partida");
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;
    let g = |a: usize| *(a as *const u32);
    let por = |a: usize, v: u32| *(a as *mut u32) = v;
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let rb = |a: usize| *(a as *const u8);
    let h = || *(0xb77c7a as *const i16);
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x765480usize);
    let margem: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x800d30usize);
    let texto_do_item: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x801c70usize);
    let humano_no_time: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let usa: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    let indice: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x7fffa0usize);
    let parar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x802a50usize);
    let rotulo: unsafe extern "C" fn(u32, usize, usize, u32, *mut u32) -> u32 = core::mem::transmute(0x725a70usize);
    if g(0xb77c84) != 0 {
        return 0;
    }
    let p = parametro_da_tela(0);
    let mut modo = parametro_da_tela(1) as u8;
    let aba = parametro_da_tela(2) as u8;
    let p3 = parametro_da_tela(3);
    let p4 = parametro_da_tela(4);
    let p5 = parametro_da_tela(5);
    let p6 = parametro_da_tela(6);
    let p8 = parametro_da_tela(8);
    let mut p15 = parametro_da_tela(0x15);
    let m = rd(p + 0x5b);
    if rd(m + 0x119) != 0 {
        return -9;
    }
    if modo == 1 {
        if rd(p + 0x57) == 0 {
            let confirmar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x6b5260usize);
            confirmar(*(0xb67a50 as *const usize), p);
        }
        if rd(p + 0x57) == 0 {
            return 0;
        }
        gravar_parametro_da_tela(1, 2);
        return -11;
    }
    let pm = rd(p + 0x57);
    let st = rd(p + 0x5f);
    let item16 = item as u16 as i16;
    let registro = |i: i16| (display_do_humano() as isize + i as isize * 0x18c) as usize;
    if id_do_item_ou_padrao(item16) == 0xc {
        let v = if item16 != -1 { rb(registro(item16) + 0xba9a6) } else { 0 };
        *((rd(pm + 0x8f7a) + 0x2db0) as *mut u8) = v;
    }
    let volume = rb(rd(pm + 0x8f7a) + 0x2db0);
    let velocidade: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5fcff0usize);
    if volume as u16 != velocidade(*(0xb63c98 as *const usize), 0) as u16 {
        // a original empilha o ecx com os 16 bits de cima do que sobrou de 0x5fcff0 (aqui estendido)
        let repassar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x5fd100usize);
        repassar(*(0xb63c98 as *const usize), rb(rd(pm + 0x8f7a) + 0x2db0) as u32, 0);
        return -11;
    }
    por(0xb6b7ac, 0);
    por(0xb6b7b0, 0);
    por(0xb6b7a8, 0);
    let espectador = || (m as isize + 0xc5 + h() as isize * 4) as *mut i32;
    let quantos = (m + 0x108) as *mut u8;
    let assistir = || {
        if core::ptr::read_unaligned(espectador()) == 0 {
            core::ptr::write_unaligned(espectador(), 1);
            *quantos = (*quantos).wrapping_add(1);
        }
    };
    let deixar_de_assistir = || {
        if core::ptr::read_unaligned(espectador()) != 0 {
            core::ptr::write_unaligned(espectador(), 0);
            *quantos = (*quantos).wrapping_sub(1);
        }
    };
    let t_do_menu = |d: usize| {
        let x = rd(pm + 0x4792);
        if x != 0 { rd(x + d) } else { 0 }
    };
    // o time `t` é o do humano da vez (0x600a50 e a vaga dele em 0x7fffa0)? então é o do parâmetro 0x15
    let marcar_se_do_humano = |t: usize, p15: &mut usize| {
        if humano_no_time(t, 0, 1, 0, 0) != 0 && indice(sm, usa(t)) == h() as u16 {
            *p15 = t;
            gravar_parametro_da_tela(0x15, t);
        }
    };

    // o time a abrir (Some) ou seguir para o item
    let alvo: Option<usize> = if item16 == -1 {
        let mut ao_lance = false;
        if rd((0xb74c26 + h() as isize * 0x300) as usize) != 0 {
            let conectado: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5fd210usize);
            if conectado(*(0xb63c98 as *const usize), 0) != 0 {
                if modo == 2 || modo == 3 {
                    gravar_parametro_da_tela(1, 4);
                    modo = 4;
                    deixar_de_assistir();
                    *((p + 0x67) as *mut u8) = 1;
                    por(0xb6b7b0, 1);
                    ao_lance = true;
                } else if modo == 5 {
                    parar(sm, -1);
                    return -9;
                }
            }
        }
        if !ao_lance && modo == 4 {
            ao_lance = true;
        }
        let lance: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6b55c0usize);
        if ao_lance && lance(*(0xb67a50 as *const usize), p) != 0 {
            let marcas = || rb(m);
            if marcas() & 1 != 0 {
                por(0xb6b7b0, 1);
            }
            if marcas() & 2 != 0 {
                por(0xb6b7b0, 1);
            }
            if marcas() & 4 != 0 && g(0xb6b7b0) == 0 {
                let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
                formatar(BUF, 0x98806c, *((st + 0x4d) as *const i16) as i32);
                texto_do_item(sm, p3, BUF);
                por(0xb6b7a8, 1);
            }
            if marcas() & 8 != 0 {
                if g(0xb6b7b0) == 0 {
                    let narrador = rd(pm + 0x8f7a);
                    // (largura, quebrar, fonte, modo); o modo da original é um sete em dl sobre um edx sujo
                    let (largura, quebrar, fonte, modo_n) = if aba == 1 && rd(st + 0x53) == 0 { (0x262, 1, 5, 0) } else { (0x2b2, 0, 3, (aba == 5) as u32) };
                    let comp = rd(rd(pm + 0x4792));
                    let narrar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x6d7b00usize);
                    narrar(narrador, rd(m + 1), st, comp, p8, largura, quebrar, fonte, modo_n);
                    por(0xb6b7a8, 1);
                }
                gravar_parametro_da_tela(7, 1);
                let lance = rd(m + 1);
                let codigo = *(lance as *const u16);
                if matches!(codigo, 0x1ff8 | 0x1ff7 | 0x1ff5 | 0x1ff6 | 0x1f56) {
                    let t = t_do_menu(0x1c);
                    if rd(pm + 0x4792) != 0 && t != 0 && rb(rd(m + 1) + 6) == 0 {
                        marcar_se_do_humano(t, &mut p15);
                    }
                    let t = t_do_menu(0x20);
                    if rd(pm + 0x4792) != 0 && t != 0 && rb(rd(m + 1) + 6) == 1 {
                        marcar_se_do_humano(t, &mut p15);
                    }
                }
            } else if parametro_da_tela(7) != 0 {
                if g(0xb6b7b0) == 0 {
                    margem(sm, p8, 1);
                    texto_do_item(sm, p8, 0xad9c64);
                    por(0xb6b7a8, 1);
                }
                gravar_parametro_da_tela(7, 0);
            }
            if marcas() & 0x20 != 0 {
                gravar_parametro_da_tela(1, 3);
                modo = 3;
                assistir();
                por(0xb6b7b0, 1);
            }
            if marcas() & 0x40 != 0 {
                gravar_parametro_da_tela(1, 5);
                modo = 5;
                por(0xb6b7b0, 1);
            }
            por(0xb6b7ac, 1);
        }
        // a marca de "esperando" (parâmetro 0x24): o humano saiu da partida que outros assistem
        let esperando = *quantos != 0 && core::ptr::read_unaligned(espectador()) == 0;
        if esperando == (parametro_da_tela(0x24) == 0) {
            if g(0xb6b7b0) == 0 {
                let mut cor: u32 = 0;
                rotulo(modo as u32, m, BUF, 0x7d0, &mut cor);
                margem(sm, p4, cor);
                texto_do_item(sm, p4, BUF);
                por(0xb6b7a8, 1);
            }
            gravar_parametro_da_tela(0x24, esperando as usize);
        }
        if p15 == 0 {
            let fase = rb(pm + 0x8eb3);
            if fase == 1 || fase == 4 {
                let t = t_do_menu(0x1c);
                if rd(pm + 0x4792) != 0 && t != 0 && parametro_da_tela(0x2f) == 0 {
                    marcar_se_do_humano(t, &mut p15);
                }
                let t = t_do_menu(0x20);
                if rd(pm + 0x4792) != 0 && t != 0 && parametro_da_tela(0x30) == 0 {
                    marcar_se_do_humano(t, &mut p15);
                }
            }
        }
        if p15 != 0 { Some(p15) } else { None }
    } else if id_do_item(item16) == 1 {
        Some(rd(registro(item16) + 0xba9a6))
    } else {
        None
    };

    if let Some(alvo) = alvo {
        if parametro_da_tela(7) == 0 && rb(pm + 0x8eb2) != 0 {
            // abre a tática do time
            let mut lado: u32 = 0;
            if humano_no_time(alvo, 0, 1, 0, 0) != 0 {
                let vaga = || rd((0xb74c26 + h() as isize * 0x300) as usize);
                if usa(alvo) != vaga() {
                    let v = vaga();
                    let u = usa(alvo);
                    let trocar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7ff810usize);
                    trocar(sm, v, u);
                }
            }
            if (alvo == rd(p + 0x20) && rb(p + 0x44) == 0) || (alvo == rd(p + 0x24) && rb(p + 0x45) == 0) {
                lado = 1;
            }
            gravar_parametro_da_tela(0x15, 0);
            assistir();
            *((ficha(0) + 0xea) as *mut u32) = 0;
            let tatica: unsafe extern "C" fn(usize, u32, usize, u32, i32, u32) = core::mem::transmute(0x899a50usize);
            let fase = rb(pm + 0x8eb3);
            if (fase == 1 || fase == 4) && humano_no_time(alvo, 0, 1, 0, 0) != 0 && indice(sm, usa(alvo)) == h() as u16 {
                let k = if alvo == t_do_menu(0x1c) { 0x2f } else { 0x30 };
                gravar_parametro_da_tela(k, 1);
                tatica(alvo, lado, pm, 0, -1, 1);
                return -4;
            }
            let estado = rb(p + 0x67);
            if estado == 0xff || estado == 1 || estado == 4 {
                tatica(alvo, lado, pm, 1, -1, 0);
            } else {
                tatica(alvo, lado, pm, 0, -1, 0);
            }
            return -4;
        }
        if alvo != p15 {
            if g(0xb6b7b0) == 0 {
                margem(sm, p5, if t_do_menu(0x1c) == alvo { 0x1061 } else { 0x1021 });
                margem(sm, p6, if t_do_menu(0x20) == alvo { 0x1061 } else { 0x1021 });
                por(0xb6b7a8, 1);
            }
            gravar_parametro_da_tela(0x15, alvo);
        }
    }

    if item16 != -1 {
        if id_do_item(item16) == 2 {
            match modo {
                2 | 3 => {
                    gravar_parametro_da_tela(1, 4);
                    modo = 4;
                    deixar_de_assistir();
                    *((p + 0x67) as *mut u8) = 1;
                    por(0xb6b7b0, 1);
                }
                4 => {
                    if core::ptr::read_unaligned(espectador()) == 0 {
                        core::ptr::write_unaligned(espectador(), 1);
                        *quantos = (*quantos).wrapping_add(1);
                    } else {
                        core::ptr::write_unaligned(espectador(), 0);
                        *quantos = (*quantos).wrapping_sub(1);
                    }
                    por(0xb6b7b0, 1);
                }
                5 => {
                    parar(sm, -1);
                    return -9;
                }
                _ => {}
            }
        }
        if id_do_item(item16) == 3 && rd(registro(item16) + 0xba9a6) != parametro_da_tela(2) {
            gravar_parametro_da_tela(2, rd(registro(item16) + 0xba9a6));
            por(0xb6b7b0, 1);
        }
    }

    let ligacao: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x7643d0usize);
    if ligacao(item) == -4 {
        if core::ptr::read_unaligned(espectador()) == 0 {
            core::ptr::write_unaligned(espectador(), -911);
            *quantos = (*quantos).wrapping_add(1);
        }
        *((ficha(0) + 0xea) as *mut u32) = 0;
        return -4;
    }
    // a original lê o parâmetro 0 antes de olhar [0xb6b7ac]
    let menu_da_partida = |bit: u8| {
        let q = parametro_da_tela(0);
        g(0xb6b7ac) != 0 && rb(rd(q + 0x5b)) & bit != 0
    };
    let refazer = |f: usize| {
        let r: unsafe extern "C" fn() = core::mem::transmute(f);
        r();
        por(0xb6b7a8, 1);
    };
    match aba as i8 {
        1 => {
            let q = parametro_da_tela(0);
            if g(0xb6b7ac) != 0 {
                let f = rd(q + 0x5b);
                if rb(f) & 0x20 != 0 {
                    por(0xb6b7b0, 1);
                }
                if rb(f) & 0x10 != 0 {
                    por(0xb6b7b0, 1);
                }
                if rb(f) & 4 != 0 {
                    if g(0xb6b7b0) != 0 {
                        return -11;
                    }
                    refazer(0x71d9e0);
                }
            }
        }
        2 => {
            if menu_da_partida(4) {
                if g(0xb6b7b0) != 0 {
                    return -11;
                }
                refazer(0x71e750);
            }
        }
        3 => {
            if menu_da_partida(4) && g(0xb6b7b0) == 0 {
                refazer(0x720320);
            }
            if item16 != -1 && id_do_item(item16) == 4 && rd(registro(item16) + 0xba9a6) != parametro_da_tela(0xa) {
                gravar_parametro_da_tela(0xa, rd(registro(item16) + 0xba9a6));
                por(0xb6b7b0, 1);
                return -11;
            }
        }
        4 => {
            if menu_da_partida(0x20) {
                por(0xb6b7b0, 1);
                return -11;
            }
        }
        6 | 9 => {
            let estatisticas_do_time: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x722dc0usize);
            let r = estatisticas_do_time(item);
            if r != 0 {
                return r;
            }
        }
        7 => {
            if menu_da_partida(4) {
                if g(0xb6b7b0) != 0 {
                    return -11;
                }
                refazer(0x723350);
            }
        }
        8 => {
            let trocar: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x725860usize);
            let r = trocar(item);
            if r != 0 {
                return r;
            }
        }
        10 => {
            let converter: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x803320usize);
            let v = parametro_da_tela(0x26);
            let r = converter(sm, v);
            gravar_parametro_da_tela(0x25, r);
            if modo != 5 && rb(m) & 4 != 0 {
                por(0xb6b7b0, 1);
                return -11;
            }
        }
        _ => {}
    }
    if g(0xb6b7b0) != 0 {
        return -11;
    }
    if g(0xb6b7a8) != 0 {
        let redesenhar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x802d00usize);
        redesenhar(sm, 0, 0);
    }
    0
}

/// { 0x71b630 } (cdecl, SEH, match_screens.cpp): monta a tela de detalhes de uma partida guardada:
/// libera a foto do parâmetro 0x1d; sem o registro (parâmetro 0x1f) lê o da partida do parâmetro 0x1e
/// das estatísticas (0x729e30, 0x4f bytes) e sem a foto (0x20) a monta (0x729600, 0xd7a bytes; erro:
/// caixa "Error" e nada mais); o fundo, os cabeçalhos dos dois times (cores 0x53b640, nome, caixa do
/// clube, placar ou "Match Abandoned"), o agregado ([+0x4b] ≥ 0), a barra de menu, as 7 abas, os
/// botões de voltar e avançar e o painel da aba (parâmetro 2).
unsafe extern "C" fn montar_detalhes_da_partida() {
    crate::comparacao::ramo("telas montar os detalhes");
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let texto_do_gerente: TextoDoGerente = core::mem::transmute(0x803fd0usize);
    let caixa: CaixaDoGerente = core::mem::transmute(0x804030usize);
    let cores: unsafe extern "C" fn(usize, usize, *mut u32, *mut u32) = core::mem::transmute(0x53b640usize);
    let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944cf1usize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let rb = |a: usize| *(a as *const u8);
    let sx = |a: usize| *(a as *const i8) as i32;

    let mut abas: AbasDaPartida = [0; 0x14b4 / 4];
    construir_abas_da_partida(&mut abas);
    liberar_foto_da_partida();
    let aba = parametro_da_tela(2) as u8;
    let indice = parametro_da_tela(0x1e);
    let mut reg = parametro_da_tela(0x1f);
    let mut foto = parametro_da_tela(0x20);
    let estatisticas = *(0xb6b7c0 as *const usize);
    if reg == 0 {
        reg = novo(0x4f);
        let ler: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x729e30usize);
        if ler(estatisticas, indice, reg) == 0 {
            erro_das_telas_da_partida(0x90a);
            apagar(reg);
            destruir_abas_da_partida(&mut abas);
            return;
        }
        gravar_parametro_da_tela(0x1f, reg);
    }
    if foto == 0 {
        let f = novo(0xd7a);
        if f != 0 {
            let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(0x9463afusize);
            vetor(f + 0x11f, 0x625, 2, 0x453c30, 0x453c10);
        }
        let montar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x729600usize);
        if montar(*(0xb6b7c0 as *const usize), indice, f) == 0 {
            erro_das_telas_da_partida(0x91a);
            if f != 0 {
                let fim_do_vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(0x9462bbusize);
                fim_do_vetor(f + 0x11f, 0x625, 2, 0x453c10);
                apagar(f);
            }
            destruir_abas_da_partida(&mut abas);
            return;
        }
        gravar_parametro_da_tela(0x20, f);
        foto = f;
    }
    let r = fundo_da_partida(rd(reg + 0x18), rd(reg + 0x14), rd(reg + 0x1c));
    // as cores u16 (o resto das variáveis da original é lixo da pilha, aqui 0)
    let (mut cor_a, mut cor_b) = (0u32, 0u32);
    cores(rd(reg + 0x1c), (r & !0xff) | rb(foto + 0x152) as usize, &mut cor_a, &mut cor_b);
    formatar(BUF, 0x987ff0, rd(reg + 0x1c) + 0x38);
    texto_do_gerente(sm, 0x64, 0xa, 0x1bc, 0x46, 0x30, cor_b as usize, 1, 7, cor_a as usize, BUF, 1);
    caixa(sm, 0x64, 0xa, 0x1bc, 0x46, 1, 0, 1, 7, 0, 0, 2, rd(reg + 0x1c));
    let r = if rb(foto + 0x11f) == 0xfd { texto(BUF, 0xa164c4, 0xad9c64) } else { formatar(BUF, 0x98806c, rb(foto + 0x11f) as u32) };
    texto_do_gerente(sm, 0x18f, 0xf, 0x1b7, 0x41, 0x30, cor_em(0xae3184), 0xc, 6, cor_sobre(r as usize, 0xaebdea), BUF, 1);
    // o byte do uniforme do segundo time vai num ecx sujo (estendido)
    cores(rd(reg + 0x20), rb(foto + 0x777) as usize, &mut cor_a, &mut cor_b);
    formatar(BUF, 0x987ff0, rd(reg + 0x20) + 0x38);
    texto_do_gerente(sm, 0x1be, 0xa, 0x316, 0x46, 0x30, cor_b as usize, 1, 7, cor_a as usize, BUF, 1);
    caixa(sm, 0x1be, 0xa, 0x316, 0x46, 1, 0, 1, 7, 0, 0, 2, rd(reg + 0x20));
    let r = if rb(foto + 0x11f) == 0xfd { texto(BUF, 0xa164c4, 0xad9c64) } else { formatar(BUF, 0x98806c, rb(foto + 0x744) as u32) };
    texto_do_gerente(sm, 0x2e9, 0xf, 0x311, 0x41, 0x30, cor_sobre(r as usize, 0xae3184), 0xc, 6, cor_em(0xaebdea), BUF, 1);
    let agregado = sx(reg + 0x4b);
    if agregado > -1 {
        texto(BUF, 0xa16454, agregado, sx(reg + 0x4c));
        texto_do_gerente(sm, 0x1c3, 0x37, 0x2e4, 0x41, 1, 0, 0x40, 1, cor_a as usize, BUF, 1);
    }
    let menu: unsafe extern "C" fn(u32, u32) = core::mem::transmute(0x75db00usize);
    menu(4, 0);

    texto(titulo_da_aba(&mut abas, 0), 0xa163a8);
    marcar_aba(&mut abas, 0, 1, 1);
    texto(titulo_da_aba(&mut abas, 1), 0xa1639c);
    marcar_aba(&mut abas, 1, 1, 2);
    texto(titulo_da_aba(&mut abas, 2), 0xa1638c);
    marcar_aba(&mut abas, 2, 1, 3);
    texto(titulo_da_aba(&mut abas, 3), 0xa1637c);
    marcar_aba(&mut abas, 3, 1, 4);
    let t1 = rd(reg + 0x1c);
    texto(titulo_da_aba(&mut abas, 4), 0xa16364, sx(t1 + 0x52), sx(t1 + 0x52), t1 + 0x38);
    marcar_aba(&mut abas, 4, 1, 5);
    texto(titulo_da_aba(&mut abas, 5), 0xa16354);
    marcar_aba(&mut abas, 5, 1, 6);
    let t2 = rd(reg + 0x20);
    texto(titulo_da_aba(&mut abas, 6), 0xa16364, sx(t2 + 0x52), sx(t2 + 0x52), t2 + 0x38);
    marcar_aba(&mut abas, 6, 1, 7);
    let (mut topo, mut base) = (0x50i32, 0x221i32);
    let barra: unsafe extern "C" fn(usize, u32, i32, *mut i32, *mut i32, u32) -> u32 = core::mem::transmute(0x5ed780usize);
    let aba = aba as i8 as i32;
    barra(abas.as_mut_ptr() as usize, 7, aba, &mut topo, &mut base, 1);
    let sem_avancar: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x7febb0usize);
    let sem_voltar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x7fec60usize);
    let avancar = (sem_avancar(sm) == 0) as u32;
    let voltar = (sem_voltar(sm, 0) == 0) as u32;
    let botoes: unsafe extern "C" fn(u32, u32) -> u32 = core::mem::transmute(0x5edcc0usize);
    botoes(voltar, avancar);
    let painel = |f: usize, topo: *mut i32, base: *mut i32| {
        let g: unsafe extern "C" fn(*mut i32, *mut i32) = core::mem::transmute(f);
        g(topo, base)
    };
    let estatisticas_do_time: unsafe extern "C" fn(u32, *mut i32, *mut i32) = core::mem::transmute(0x7217a0usize);
    match aba {
        1 => painel(0x71c270, &mut topo, &mut base),
        2 => painel(0x71e040, &mut topo, &mut base),
        3 => painel(0x71f500, &mut topo, &mut base),
        4 => painel(0x721000, &mut topo, &mut base),
        5 => estatisticas_do_time(0, &mut topo, &mut base),
        6 => painel(0x722ed0, &mut topo, &mut base),
        7 => estatisticas_do_time(1, &mut topo, &mut base),
        _ => {}
    }
    destruir_abas_da_partida(&mut abas);
}

/// As montagens da tela da partida são comparadas pela lista de exibição: as regiões da tela, o bloco
/// de parâmetros sem o 0x14 (a hora que o redesenho dos últimos placares 0x724520 grava na aba 8), as
/// marcas 0xb6b7a8/ac/b0, o texto 0xde1f64 e a posição do buffer de rede ([0xae290c]), mais `extra`.
unsafe fn comparar_montagem_da_partida(nome: &str, extra: &[(usize, usize)], original: impl FnOnce(), rust: impl FnOnce()) {
    let mut reg = regioes_da_tela();
    reg.push((0xb6b7a8, 12));
    let vaga = *((GERENTE_DE_TELAS + 0x382a) as *const i16) as isize;
    let bloco = core::ptr::read_unaligned((GERENTE_DE_TELAS as isize + 0x7e2 + vaga * 0x300) as *const usize);
    if bloco != 0 {
        reg.push((bloco, 0xb4));
        reg.push((bloco + 0xb8, 0x200 - 0xb8));
    }
    reg.push((0xde1f64, 0x100));
    reg.push((0xae290c, 4));
    reg.extend_from_slice(extra);
    // a foto de fundo (0x41c410) sorteia: o estado do gerador volta para a segunda rodada (sem ele o
    // gerador andava duas vezes no modo comparação e a carreira se deslocava; 204ª)
    reg.extend_from_slice(&crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, original, || crate::aleatorio::sem_contar(rust));
    PLACAR.registrar(igual, || format!("{}: diferencas em {:?}", nome, diferencas_da_tela(&mo, &mr)));
}

/// A tela da partida é comparada pela lista de exibição, com o que ela grava fora da tela: a ficha do
/// humano (+0 e +0xea), a marca dele nos espectadores e a contagem, o foco das velocidades no `Display`
/// do humano (+0x12f4fe) e o volume do narrador (+0x2db0). Só a original quando há o que não roda duas
/// vezes: a foto do parâmetro 0x1d a liberar, o lance do parâmetro 7 a narrar (som, 0x6d7b00) ou as
/// abas 1 (o resumo também narra), 3 (as zonas alocam a foto) e 10 (a tabela ao vivo).
unsafe extern "C" fn comparar_montar_tela_da_partida() {
    let o = orig::<unsafe extern "C" fn()>(499);
    let aba = parametro_da_tela(2) as u8;
    // as abas 2 (estatisticas) e 7 (notas) alocam a copia do estado 0x1d quando ela falta: tambem so a original
    if parametro_da_tela(0x1d) != 0 || parametro_da_tela(7) != 0 || matches!(aba, 1 | 2 | 3 | 7 | 10) {
        // ponytail: liberar, narrar com som e alocar não rodam duas vezes; esses casos se provam no modo Rust
        return o();
    }
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x765480usize);
    let f = ficha(0);
    let mut extra = Vec::new();
    if display_do_humano() != 0 {
        extra.push((display_do_humano() + 0x12f4fe, 3));
    }
    if f != 0 {
        extra.push((f, 1));
        extra.push((f + 0xea, 4));
    }
    let p = parametro_da_tela(0);
    if p != 0 && parametro_da_tela(1) as u8 != 1 {
        let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
        let espectadores = rd(p + 0x5b);
        extra.push(((espectadores as isize + 0xc5 + *(0xb77c7a as *const i16) as isize * 4) as usize, 4));
        extra.push((espectadores + 0x108, 1));
        let pm = rd(p + 0x57);
        if pm != 0 && rd(pm + 0x8f7a) != 0 {
            extra.push((rd(pm + 0x8f7a) + 0x2db0, 1));
        }
    }
    comparar_montagem_da_partida("montar_tela_da_partida", &extra, || o(), || montar_tela_da_partida());
}

// ponytail: a resposta mexe em telas, menu, rede, narração (som) e tática (não roda duas vezes): no modo
// comparação só a original roda; a cópia se prova no jogo do Lucas em modo Rust (contador "telas ...")
unsafe extern "C" fn comparar_responder_tela_da_partida(item: u32) -> i32 {
    orig::<unsafe extern "C" fn(u32) -> i32>(500)(item)
}

/// A tela de detalhes é comparada pela lista de exibição quando o registro e a foto da partida já
/// existem (parâmetros 0x1f e 0x20) e não há foto 0x1d a liberar. Só a original quando eles nascem aqui
/// (alocação) ou nas abas 1 (o resumo narra com som), 3 (as zonas alocam a foto) e 4 (o relatório nasce).
unsafe extern "C" fn comparar_montar_detalhes_da_partida() {
    let o = orig::<unsafe extern "C" fn()>(501);
    let aba = parametro_da_tela(2) as u8;
    // as abas 2 (estatisticas) e 6 (notas) alocam a copia do estado 0x1d: tambem so a original
    if parametro_da_tela(0x1d) != 0 || parametro_da_tela(0x1f) == 0 || parametro_da_tela(0x20) == 0 || matches!(aba, 1 | 2 | 3 | 4 | 6) {
        // ponytail: alocar, liberar e narrar não rodam duas vezes; esses casos se provam no modo Rust
        return o();
    }
    comparar_montagem_da_partida("montar_detalhes_da_partida", &[], || o(), || montar_detalhes_da_partida());
}

// ---------- 209a leva: as telas da partida (13): a barra da posse, o painel da visão geral da partida e a tabela da liga ao vivo (match_screens.cpp) ----------

/// { 0x71d9e0 } (cdecl, match_screens.cpp): na partida (parâmetro 0), sem disputa de pênaltis
/// ([estado + 0x53] = 0), pega as cores dos dois clubes (0x53b640 com os uniformes +0x152 e +0x777) e,
/// se a posse (+0x142) mudou desde a cópia do parâmetro 0x1d (ou sem cópia), redesenha a barra da posse
/// da visão geral: o percentual (50 sem valor ou sem a marca +0x767) em "%d%%" nos itens 3 e 4 a partir
/// do parâmetro 0x21, repartidos de 0x115 a 0x2c9 na altura do parâmetro 0x22, e a cor do clube que tem
/// mais posse no item 5. No fim guarda o estado na cópia (que nasce aqui se não existe).
unsafe extern "C" fn atualizar_barra_da_posse() {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let texto: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x801c70usize);
    let retangulo: unsafe extern "thiscall" fn(usize, u32, i32, i32, i32, i32) = core::mem::transmute(0x800890usize);
    let fundo: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x801040usize);
    let cores_do_clube: unsafe extern "C" fn(usize, u32, *mut u32, *mut u32) = core::mem::transmute(0x53b640usize);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;

    let p0 = parametro_da_tela(0);
    let copia = parametro_da_tela(0x1d);
    let item = parametro_da_tela(0x21) as u32;
    let altura = parametro_da_tela(0x22) as i32;
    if p0 == 0 {
        return;
    }
    let e = core::ptr::read_unaligned((p0 + 0x5f) as *const usize);
    if core::ptr::read_unaligned((e + 0x53) as *const u32) == 0 {
        // as cores: 0x53b640 grava u16; o resto da variável da original é lixo da pilha (0x801040 só lê a palavra)
        let clube = |k: usize| (*(0xae23b4 as *const usize)).wrapping_add(core::ptr::read_unaligned((e + k) as *const usize).wrapping_mul(0x245));
        let (mut c1, mut c1b, mut c2, mut c2b) = (0u32, 0u32, 0u32, 0u32);
        cores_do_clube(clube(0), *((e + 0x152) as *const u8) as u32, &mut c1, &mut c1b);
        cores_do_clube(clube(4), *((e + 0x777) as *const u8) as u32, &mut c2, &mut c2b);
        let v = *((e + 0x142) as *const u8);
        if copia == 0 || v != *((copia + 0x142) as *const u8) {
            let p = if v == 0 || *((e + 0x767) as *const u8) == 0 { 0x32 } else { v as i32 };
            formatar(BUF, FORMATO_PCT, p);
            let x = p * 0x1b4 / 100 + 0x115;
            retangulo(sm, item.wrapping_add(3), 0x115, altura - 0x23, x, altura - 0x14);
            texto(sm, item.wrapping_add(3), BUF);
            formatar(BUF, FORMATO_PCT, 100 - p);
            retangulo(sm, item.wrapping_add(4), x, altura - 0x23, 0x2c9, altura - 0x14);
            texto(sm, item.wrapping_add(4), BUF);
            fundo(sm, item.wrapping_add(5), if p < 0x32 { c2 } else { c1 });
        }
    }
    guardar_estado_da_partida(copia, e);
}

/// A linha (0x49 bytes) do jogador `idx` do lado `lado` no estado da partida `e` (+0x11f + 0x71).
fn linha_do_jogador_na_partida(e: usize, lado: u8, idx: u8) -> usize {
    (e as isize + lado as i8 as isize * 0x625 + idx as i8 as isize * 0x49) as usize
}

/// A visão geral avisa erro (caixa "Error" de match_screens.cpp e segue)? Na partida, com o período
/// [estado + 0x4f] fora dos seis conhecidos; ou com um lance (até 50, até o lado 0xff) de tipo fora de
/// 1 a 8. Nesses casos a cópia e o comparador rodam a original.
unsafe fn visao_geral_avisa_erro() -> bool {
    let p0 = parametro_da_tela(0);
    let e = if p0 != 0 { core::ptr::read_unaligned((p0 + 0x5f) as *const usize) } else { parametro_da_tela(0x20) };
    if p0 != 0 && ![0x1ef, 0x3de, 0, 0x483, 0x528, 1].contains(&core::ptr::read_unaligned((e + 0x4f) as *const u16)) {
        return true;
    }
    for k in 0..50 {
        let ev = e + 0x57 + 4 * k;
        if *(ev as *const u8) == 0xff {
            break;
        }
        if !(1..=8).contains(&*((ev + 2) as *const u8)) {
            return true;
        }
    }
    false
}

/// { 0x71c270 } (cdecl, match_screens.cpp, ~6 KB): desenha a visão geral da partida de `*topo` a
/// `*base` (`*base` −= 0x28 logo de saída). Na partida (parâmetro 0; estado em +0x5f): o título do
/// período (pelo [estado + 0x4f], com os nomes do parâmetro 1 = 4 ou os outros) e, sem disputa de
/// pênaltis ([estado + 0x53] = 0), a barra da posse (moldura, "Last 5 Mins", as cores dos clubes; o item
/// no parâmetro 0x21, a altura no 0x22; `*base` −= 0x37) e a caixa do comentário (com o parâmetro 7 o
/// narrador escreve o último lance nela, 0x6d7b00; o item no parâmetro 8). Fora dela: o estádio e a
/// cidade do parâmetro 0x1f, com o estado do parâmetro 0x20. Depois: a data, o placar (0x4bf930) com o
/// intervalo, a prorrogação e os pênaltis, a competição (com o atalho 0x7e5 quando não é amistoso nem
/// de seleção), a lista dos autores em duas colunas (gols, gol contra, pênalti perdido, expulsão,
/// lesão; os minutos do mesmo jogador e tipo juntos, quebrando a linha com mais de 10 letras), a
/// disputa de pênaltis, o árbitro, o público, o tempo, "Match Abandoned" e a barra da posse (0x71d9e0).
unsafe extern "C" fn painel_da_visao_geral_da_partida(topo: *mut i32, base: *mut i32) {
    if visao_geral_avisa_erro() {
        return orig::<unsafe extern "C" fn(*mut i32, *mut i32)>(503)(topo, base);
    }
    let txt: TextoDoGerente = core::mem::transmute(0x803fd0usize);
    let caixa: CaixaDoGerente = core::mem::transmute(0x804030usize);
    let lista: ListaDoDisplay = core::mem::transmute(0x55fba0usize);
    let linha: LinhaDoDisplay = core::mem::transmute(0x55f990usize);
    let celula: CelulaDaLista = core::mem::transmute(0x8040a0usize);
    let texto_da_celula: TextoDaCelulaDaLista = core::mem::transmute(0x804100usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let largura_do_texto: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x5e5ad0usize);
    let altura: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x5e5cd0usize);
    let rgb: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(0x5e4800usize);
    let nome_do_jogador: unsafe extern "C" fn(usize, i32, i32, *mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(0x6db2d0usize);
    let confere: unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32 = core::mem::transmute(0x53e990usize);
    let nome_do_arbitro: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(0x53eab0usize);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c2990usize);
    let selecao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7e6f20usize);
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let data_texto: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x54ba20usize);
    let placar: unsafe extern "C" fn(usize, u32, usize, u32) = core::mem::transmute(0x4bf930usize);
    let clima: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x9233a0usize);
    let narrar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x6d7b00usize);
    let cores_do_clube: unsafe extern "C" fn(usize, u32, *mut u32, *mut u32) = core::mem::transmute(0x53b640usize);
    let barra: unsafe extern "C" fn() = core::mem::transmute(0x71d9e0usize);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;
    const MAX: usize = usize::MAX;
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let b8 = |a: usize| *(a as *const u8);
    let i8_ = |a: usize| *(a as *const i8) as i32;
    let z = |v: i32| v as usize;
    let item16 = |r: usize| r as u16 as i16 as i32 as usize;

    let mut colunas3: [u8; 4] = [1, 1, 1, 0];
    let mut colunas4: [u8; 4] = [2, 1, 2, 1];
    let p0 = parametro_da_tela(0);
    let periodo = parametro_da_tela(1) as u8;
    let base0 = *base;
    *base = base0 - 0x28;
    let (part, e);
    if p0 != 0 {
        *(BUF as *mut u8) = 0;
        e = u(p0 + 0x5f);
        let w = core::ptr::read_unaligned((e + 0x4f) as *const u16);
        let rotulo = if periodo == 4 {
            match w {
                0x1ef => 0xa167ec,
                0x3de => 0xa167e0,
                0 | 1 => 0xa167b8,
                0x483 => 0xa167a0,
                0x528 => 0xa16788,
                _ => 0,
            }
        } else {
            match w {
                0x1ef => 0xa1677c,
                0x3de => 0xa16770,
                0 | 1 => 0xa167b8,
                0x483 => 0xa1675c,
                0x528 => 0xa16744,
                _ => 0,
            }
        };
        match rotulo {
            0 => aviso_em(0xa161d4, if periodo == 4 { 0x1d49 } else { 0x1d5e }),
            0xa167b8 => {
                texto(BUF, rotulo, TEXTO_VAZIO);
            }
            _ => {
                texto(BUF, rotulo);
            }
        }
        let t = *topo;
        txt(sm, 0x6e, z(t), 0x30c, z(t + 0x23), 2, 0, 0xc, 3, cor_em(0xaebdec), BUF, 1);
        *topo += 0x28;
        part = p0 + 4;
        if core::ptr::read_unaligned((e + 0x53) as *const u32) == 0 {
            // a barra da posse
            let b = *base;
            lista(display_do_humano(), 0x6e, z(b - 0x36), 0x30c, z(b), 1, 0, 1, 0, 2, 0, MAX);
            // 0x53b640 grava u16; o resto da variável da original é lixo da pilha (só a palavra vale)
            let clube = |k: usize| (*(0xae23b4 as *const usize)).wrapping_add(u(e + k).wrapping_mul(0x245));
            let (mut c68, mut c58, mut c5c, mut c64) = (0u32, 0u32, 0u32, 0u32);
            cores_do_clube(clube(0), b8(e + 0x152) as u32, &mut c68, &mut c58);
            cores_do_clube(clube(4), b8(e + 0x777) as u32, &mut c5c, &mut c64);
            let b = *base;
            let r = txt(sm, 0x96, z(b - 0x2d), 0x2e4, z(b - 0xa), 0x30, cor_em(0xae317e), 0xc, 1, cor_em(0xae3184), 0, 1);
            gravar_parametro_da_tela(0x21, item16(r));
            texto(BUF, 0xa16738);
            let b = *base;
            txt(sm, 0xaf, z(b - 0x2d), 0xfa, z(b - 0xa), 1, 0, 0xc, 1, cor_em(0xae3184), BUF, 1);
            let b = *base;
            txt(sm, 0x113, z(b - 0x25), 0x2cb, z(b - 0x12), 0x60, cor_em(0xae317e), 0xc, 1, cor_sobre(z(b), 0xae3184), 0, 1);
            txt(sm, 0x115, z(b - 0x23), 0x1ef, z(b - 0x14), 0x10, c58 as usize, 0xc, 1, c68 as usize, 0, 1);
            let r = txt(sm, 0x1ef, z(b - 0x23), 0x2c9, z(b - 0x14), 0x10, c64 as usize, 0xc, 1, c5c as usize, 0, 1);
            linha(display_do_humano(), 1, 0x1ef, z(b - 0x23), 0x1ef, z(b - 0x20), 0, 0, 0x400, cor_sobre(r, 0xaebdea), 0, 0, 0, 0, 0, 0, 0, 0, MAX);
            gravar_parametro_da_tela(0x22, z(*base));
            *base -= 0x37;
            // a caixa do comentário
            let t = *topo;
            let y = (*base - t - 0x46) / 2 + t;
            if parametro_da_tela(7) != 0 {
                crate::comparacao::ramo("telas visao geral com o narrador");
                let rel = u(u(p0 + 0x57) + 0x8f7a);
                let r = txt(sm, 0x8c, z(y), 0x2ee, z(y + 0x46), 1, 0, 0x10c, 5, 0, 0, 1);
                let m = u(p0 + 0x57);
                narrar(rel, u(u(p0 + 0x5b) + 1), e, u(u(m + 0x4792)), r, 0x262, 1, 5, 0);
                gravar_parametro_da_tela(8, item16(r));
            } else {
                let r = txt(sm, 0x8c, z(y), 0x2ee, z(y + 0x46), 1, 0, 0x10c, 5, 0, 0, 1);
                gravar_parametro_da_tela(8, item16(r));
            }
        }
    } else {
        let g = parametro_da_tela(0x1f);
        let st = u(g + 0x18);
        if st != 0 {
            let cidade = u(st + 0x38);
            if cidade != 0 {
                let (a, c) = (i8_(st + 0x37), i8_(cidade + 0x1e));
                texto(BUF, 0x9902a4, a, a, c, c, st + 4, cidade + 4);
            } else {
                copiar_para_o_texto(st + 4);
            }
        } else {
            texto(BUF, 0x99c1a4);
        }
        let t = *topo;
        txt(sm, 0x6e, z(t), 0x30c, z(t + 0x23), 2, 0, 0xc, 3, cor_em(0xaebdec), BUF, 1);
        *topo += 0x28;
        e = parametro_da_tela(0x20);
        part = g;
    }

    // a data, o placar e a competição
    let mut data = [0u8; 8];
    data_nova(data.as_mut_ptr() as usize, core::ptr::read_unaligned((part + 0x2a) as *const u16) as u32, core::ptr::read_unaligned((part + 0x28) as *const u16) as u32);
    data_texto(data.as_mut_ptr() as usize, BUF, 0x7d0, 0xff);
    let mut w = largura_do_texto(1, BUF) + 0xf;
    if w <= 0x7d {
        w = 0x7d;
    }
    let t = *topo;
    txt(sm, 0x6e, z(t), z(w + 0x6e), z(t + 0x14), 2, 0, 0xc, 1, cor_em(0xaebde8), BUF, 1);
    placar(part, 3, BUF, 0x7d0);
    let mut aux = [0u8; 0x200];
    let pa = aux.as_mut_ptr() as usize;
    if i8_(part + 0x45) >= 0 {
        texto(pa, 0xa16714, i8_(part + 0x45), i8_(part + 0x46));
        acrescentar_ao_texto(aux.as_ptr());
        if i8_(part + 0x47) >= 0 {
            texto(pa, 0xa166d4, i8_(part + 0x43), i8_(part + 0x44), i8_(part + 0x47), i8_(part + 0x48));
            acrescentar_ao_texto(aux.as_ptr());
        }
        let pens = if b8(part + 0x38) & 0x20 != 0 { 0xa166b0 } else { 0xa1668c };
        if core::ptr::read_unaligned((e + 0x53) as *const u32) != 0 {
            texto(pa, pens, b8(e + 0x120) as u32, b8(e + 0x745) as u32);
            acrescentar_ao_texto(aux.as_ptr());
        } else if i8_(part + 0x49) >= 0 {
            texto(pa, pens, i8_(part + 0x49), i8_(part + 0x4a));
            acrescentar_ao_texto(aux.as_ptr());
        }
    }
    let mut x = largura_do_texto(1, BUF) + 0xf;
    if x <= 0x7d {
        x = 0x7d;
    }
    let comp = u(part + 0x14);
    if amistoso(*(0xae2c74 as *const usize), comp) == 0 && selecao(*(0xb74434 as *const usize), u(part + 0x14)) == 0 {
        let chave = (((*(u(part + 0x14) as *const u32)) << 11 | (core::ptr::read_unaligned((part + 0x32) as *const u16) as u32 & 0x7ff)) << 11)
            | (core::ptr::read_unaligned((part + 0x30) as *const u16) as u32 & 0x7ff);
        let t = *topo;
        caixa(sm, z(0x30c - x), z(t), 0x30c, z(t + 0x14), 2, 0, 0xc, 1, cor_sobre(chave as usize, 0xaebde8), BUF, 0x7e5, chave as usize);
    } else {
        let t = *topo;
        txt(sm, z(0x30c - x), z(t), 0x30c, z(t + 0x14), 2, 0, 0xc, 1, cor_em(0xaebde8), BUF, 1);
    }

    // os autores: duas colunas (casa 0, fora 2), uma linha por jogador e tipo com os minutos ao lado
    *topo += 0x19;
    let (ini, fim) = (*topo, *base);
    let a = altura(4);
    let n = ((fim - ini - 0x10) / a) as u8;
    let lst = lista(display_do_humano(), 0x6e, z(ini), 0x30c, z(fim), 4, colunas4.as_mut_ptr() as usize, n as usize, 0, 2, 0, MAX);
    let mut marcas = [0u32; 50];
    let mut linhas = [0i32; 2];
    let mut lado_da_linha = 0usize;
    let mut col: u32;
    // [esp+0x24] da original: as cores fixas trocam só a palavra de baixo, a do verde (0x5e4800) tudo
    let mut cor = 0u32;
    let palavra = |c: u32, a: usize| (c & 0xffff_0000) | *(a as *const u16) as u32;
    let mut nome = [0u8; 200];
    let mut junto = [0u8; 0x200];
    let pj = junto.as_mut_ptr() as usize;
    let pessoas = 0xae23bc as *const usize;
    // a célula do nome: com o jogador conferido (0x53e990), a célula com o atalho para a pessoa
    // (0x7dc clube de seleção, 0x7d7 senão); senão só o texto
    let celula_do_nome = |fila: usize, col: u32, cor: u32, lado: u8, f198: u32, f19c: u32, f1a0: u32, l: i32| {
        let pid = core::ptr::read_unaligned((fila + 0x190) as *const i32);
        if pid >= 0 && pid < *(0xae2364 as *const i32) && confere((*pessoas).wrapping_add(pid as usize * 110), f198, f19c, f1a0, fila + 0x1a4) != 0 {
            let clube = if lado == 0 { u(e) } else { u(e + 4) } as i32;
            let atalho = if clube >= *(0xae235c as *const i32) - 2 * *(0xae2350 as *const i32) { 0x7dc } else { 0x7d7 };
            celula(sm, lst, col as usize, z(l), 1, 0, 1, 4, cor as usize, BUF, atalho, (*pessoas).wrapping_add(pid as usize * 110));
        } else {
            texto_da_celula(sm, lst, col as usize, z(l), 1, 0, 1, 4, cor as usize, BUF);
        }
        pid
    };
    for i in 1..=50usize {
        let ev = e + 0x57 + 4 * (i - 1);
        let lado = b8(ev);
        if lado == 0xff {
            break;
        }
        if marcas[i - 1] != 0 {
            continue;
        }
        let idx = b8(ev + 1);
        let fila = linha_do_jogador_na_partida(e, lado, idx);
        let (f198, f19c, f1a0) = (u(fila + 0x198) as u32, u(fila + 0x19c) as u32, u(fila + 0x1a0) as u32);
        let tipo = b8(ev + 2);
        match tipo {
            1..=6 => {
                // o gol contra vai para o outro lado
                let outro = (tipo == 2) != (lado != 0);
                col = if outro { 2 } else { 0 };
                lado_da_linha = outro as usize;
                nome_do_jogador(e, idx as i8 as i32, lado as i8 as i32, nome.as_mut_ptr(), 0xc8, 0, 0, 2);
                let np = nome.as_ptr();
                match tipo {
                    1 | 3 => {
                        formatar(BUF, 0x987ff0, np);
                        cor = palavra(cor, 0xae3184);
                    }
                    2 => {
                        texto(BUF, 0xa16670, 0x64, np);
                        cor = palavra(cor, 0xaebdd4);
                    }
                    5 => {
                        texto(BUF, 0xa16630, 0x64, np);
                        cor = palavra(cor, 0xae31a8);
                    }
                    _ => {
                        texto(BUF, if tipo == 4 { 0xa16650 } else { 0xa16610 }, 0x64, np);
                        cor = rgb(0, 0xc0, 0, 0);
                    }
                }
            }
            7 | 8 => continue,
            _ => {
                aviso_em(0xa161d4, 0xc04);
                continue;
            }
        }
        let pid = celula_do_nome(fila, col, cor, lado, f198, f19c, f1a0, linhas[lado_da_linha]);
        let minuto = |ev: usize| {
            let m = b8(ev + 3) as u32 + 1;
            if b8(ev + 2) == 3 {
                texto(BUF, 0xa165fc, m);
            } else {
                formatar(BUF, FORMATO_D, m);
            }
        };
        minuto(ev);
        if i < 50 {
            for j in i..50 {
                let evj = e + 0x57 + 4 * j;
                let lj = b8(evj);
                if lj == 0xff {
                    break;
                }
                if marcas[j] != 0 {
                    continue;
                }
                if core::ptr::read_unaligned((linha_do_jogador_na_partida(e, lj, b8(evj + 1)) + 0x190) as *const i32) != pid {
                    continue;
                }
                let tj = b8(evj + 2);
                if tj != tipo && !((tj == 1 || tj == 3) && (tipo == 1 || tipo == 3)) {
                    continue;
                }
                let mj = b8(evj + 3) as u32 + 1;
                if tj == 3 {
                    texto(pj, 0xa165d8, BUF, mj);
                } else {
                    formatar(pj, 0xa165d0, BUF, mj);
                }
                if junto.iter().position(|&c| c == 0).unwrap_or(0) > 10 {
                    acrescentar_ao_texto(0xa165cc as *const u8);
                    let l = linhas[lado_da_linha];
                    texto_da_celula(sm, lst, col as usize + 1, z(l), 1, 0, 1, 4, cor_em(0xaebdec), BUF);
                    linhas[lado_da_linha] = l + 1;
                    minuto(evj);
                } else {
                    copiar_para_o_texto(pj);
                }
                marcas[j] = 1;
            }
        }
        let l = linhas[lado_da_linha];
        texto_da_celula(sm, lst, col as usize + 1, z(l), 1, 0, 1, 4, cor_em(0xaebdec), BUF);
        linhas[lado_da_linha] = l + 1;
        marcas[i - 1] = 1;
    }

    // a disputa de pênaltis
    if core::ptr::read_unaligned((e + 0x53) as *const u32) != 0 {
        let m = if linhas[0] > linhas[1] { linhas[0] } else { linhas[1] };
        linhas[lado_da_linha] = m;
        texto_da_celula(sm, lst, 0, z(m), 1, 0, 1, 3, cor_em(0xaebdec), TEXTO_VAZIO);
        let r = m + 1;
        linhas[lado_da_linha] = r;
        texto(BUF, if b8(part + 0x38) & 0x20 != 0 { 0xa165b8 } else { 0xa165a4 });
        let mut w = largura_do_texto(3, BUF) + 0x73;
        if w >= 0x2f1 {
            w = 0x2f1;
        }
        linhas[lado_da_linha] = r + 1;
        // a original empurra edx = o ponteiro da linha com a cor em dx; 0x55f990 só lê a palavra
        linha(display_do_humano(), 1, 0x73, 0, z(w), 0, 0, z(r), 1, 0, 0, 1, 3, cor_em(0xaebdec), BUF, 0, 0, 0, lst);
        linha(display_do_humano(), 1, 0x73, 0, z(w), 0, 0, z(r + 1), 0xc400, cor_em(0xaebdec), 0, 0, 0, 0, 0, 0, 0, 0, lst);
        linhas[lado_da_linha] = r + 2;
        linhas[0] = r + 2;
        linhas[1] = linhas[lado_da_linha];
        for k in 0..50 {
            let ev = e + 0x57 + 4 * k;
            let lado = b8(ev);
            if lado == 0xff {
                break;
            }
            let idx = b8(ev + 1);
            let fila = linha_do_jogador_na_partida(e, lado, idx);
            let (f198, f19c, f1a0) = (u(fila + 0x198) as u32, u(fila + 0x19c) as u32, u(fila + 0x1a0) as u32);
            let tipo = b8(ev + 2);
            if tipo != 7 && tipo != 8 {
                continue;
            }
            col = if lado != 0 { 2 } else { 0 };
            lado_da_linha = (lado != 0) as usize;
            nome_do_jogador(e, idx as i8 as i32, lado as i8 as i32, nome.as_mut_ptr(), 0xc8, 0, 0, 2);
            if tipo == 7 {
                texto(BUF, 0xa16588, nome.as_ptr());
                cor = palavra(cor, 0xaebde8);
            } else {
                texto(BUF, 0xa1656c, nome.as_ptr());
                cor = rgb(0, 0xc0, 0, 0);
            }
            let l = linhas[lado_da_linha];
            celula_do_nome(fila, col, cor, lado, f198, f19c, f1a0, l);
            linhas[lado_da_linha] = l + 1;
        }
    }

    // o árbitro, o público e o tempo
    let base23 = base0 - 0x23;
    lista(display_do_humano(), 0x6e, z(base23), 0x30c, z(base0), 1, 0, 1, 0, 2, 0, MAX);
    let l2 = lista(display_do_humano(), 0x78, z(base23), 0x302, z(base0), 3, colunas3.as_mut_ptr() as usize, 1, 0, 1, 0, MAX);
    let arbitro = core::ptr::read_unaligned((part + 0x2e) as *const i16);
    if arbitro > -1 {
        nome_do_arbitro((*(0xae23e8 as *const usize)).wrapping_add(arbitro as usize * 0x2b), aux.as_mut_ptr(), 0xc8, 1);
        texto(BUF, 0xa16550, aux.as_ptr());
    } else {
        texto(BUF, 0xa1653c);
    }
    texto_da_celula(sm, l2, 0, 0, 1, 0, 1, 2, cor_em(0xaebdec), BUF);
    let publico = core::ptr::read_unaligned((e + 0x34) as *const u32);
    if publico != 0 {
        let w = core::ptr::read_unaligned((e + 0x4f) as *const i16);
        if w >= 0x3de || w == 0 || w == 1 {
            texto(BUF, 0xa1651c, publico);
            texto_da_celula(sm, l2, 1, 0, 1, 0, 0xc, 2, cor_em(0xaebde8), BUF);
        }
    }
    clima(*(0xde2f74 as *const usize), pa, e + 0xd76);
    texto(BUF, 0xa164f8, aux.as_ptr());
    texto_da_celula(sm, l2, 2, 0, 1, 0, 0x40, 2, cor_em(0xaebdec), BUF);
    if b8(e + 0x11f) == 0xfd {
        let t = *topo;
        let y = (*base - t - 0x46) / 2 + t;
        texto(BUF, 0xa164e8);
        txt(sm, 0x8c, z(y), 0x2ee, z(y + 0x46), 0x30, cor_em(0xaebdea), 0xc, 3, cor_em(0xae3184), BUF, 1);
    }
    barra();
}

/// { 0x7280e0 } (cdecl, match_screens.cpp, ~3,4 KB; o mapa diz `(void)`, mas a original lê três
/// argumentos: a liga, `topo` e `base`, por valor): desenha a tabela da liga ao vivo. Faz uma cópia
/// local da liga (League 0x682200 + 0x682380, o temporário destruído com 0x682300) e, na partida com
/// placares ao vivo ([[p0+0x57]+0x8ed0]), aplica nela (0x685f60) os jogos do dia do grupo do parâmetro
/// 0xe da competição dele: a partida do humano com o placar do estado (+0x11f/+0x744, pênaltis
/// +0x120/+0x745), as outras sem resultado ainda com 0 a 0, as que já têm placar pela contagem até o
/// minuto (0x728e90); ordena a cópia. Com a data do sorteio ([liga+0xb6], [+0xb8]+[+0x40]) ainda por
/// vir: só o nome da competição e "The draw will be made on ..." (0x5ed070). Senão: o título, a tabela
/// de 11 colunas (as linhas pelas vagas de acesso e rebaixamento ou pelo espaço), os cabeçalhos, a
/// liga com [+0xad] = parâmetro 0x28 ordenada (0x6827d0) para as setas de posição, uma linha por clube
/// (separadores 0x6400, posição, seta de subida/descida, nome com atalho 0x7d5 e cor do humano, e as
/// colunas do registro pelo parâmetro 0x28), a rolagem (parâmetro 0x25) e o item da tabela no 0x26.
/// Sem liga ou sem o parâmetro 0xe (caixa "Error"): a original.
unsafe extern "C" fn tabela_da_liga_ao_vivo(liga: usize, topo: i32, base: i32) {
    if liga == 0 || parametro_da_tela(0xe) == 0 {
        return orig::<unsafe extern "C" fn(usize, i32, i32)>(504)(liga, topo, base);
    }
    let txt: TextoDoGerente = core::mem::transmute(0x803fd0usize);
    let lista: ListaDoDisplay = core::mem::transmute(0x55fba0usize);
    let linha: LinhaDoDisplay = core::mem::transmute(0x55f990usize);
    let celula: CelulaDaLista = core::mem::transmute(0x8040a0usize);
    let texto_da_celula: TextoDaCelulaDaLista = core::mem::transmute(0x804100usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let altura: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x5e5cd0usize);
    let mensagem: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(0x5ed070usize);
    let liga_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let liga_fim: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x682300usize);
    let copiar_liga: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x682380usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let registro: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x6826d0usize);
    let aplicar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize, usize) = core::mem::transmute(0x685f60usize);
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let contar: unsafe extern "C" fn(usize, u32, usize) = core::mem::transmute(0x728e90usize);
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let data_texto: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x54ba20usize);
    let dia_da_semana: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x54a3f0usize);
    let idioma: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x66f4d0usize);
    let genero_do_dia: unsafe extern "C" fn(u32, u32) -> u32 = core::mem::transmute(0x54d550usize);
    let competicao_curta: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x4b01d0usize);
    let nome_da_competicao: unsafe extern "C" fn(usize, u32, u32, u32) = core::mem::transmute(0x4b4850usize);
    let por_extenso: unsafe extern "C" fn(i32, i32) -> usize = core::mem::transmute(0x90c310usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let rolar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x403320usize);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;
    const MAX: usize = usize::MAX;
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let w16 = |a: usize| core::ptr::read_unaligned(a as *const u16);
    let b8 = |a: usize| *(a as *const u8);
    let z = |v: i32| v as usize;

    // o quadro da original ([esp+N] depois dos quatro registros): as variáveis nos mesmos lugares
    // (+0x1c colunas, +0x30 data do sorteio, +0x38 hoje / registro da liga, +0x40 e +0x180 jogos,
    // +0x90 a liga local, +0x1d0 a temporária, +0x3d0 a data por extenso, +0xba0 o título)
    let mut quadro = [0u32; 0x1370 / 4];
    let s = quadro.as_mut_ptr() as usize;
    let gravar8 = |a: usize, v: u8| *(a as *mut u8) = v;
    for (k, &v) in [4u8, 2, 0x10, 0, 4, 4, 4, 4, 4, 4, 4].iter().enumerate() {
        gravar8(s + 0x1c + k, v);
    }
    let local = s + 0x90;
    liga_nova(local);
    copiar_liga(local, s + 0x1d0, liga);
    liga_fim(s + 0x1d0);
    let p0 = parametro_da_tela(0);
    let pe = parametro_da_tela(0xe);
    *((s + 0x14) as *mut usize) = pe;

    // os jogos do dia do grupo na cópia local
    let m = u(p0 + 0x57);
    if m != 0 && w16(m + 0x8ed0) != 0 {
        let mut k = core::ptr::read_unaligned((pe + 4) as *const i32);
        while k <= core::ptr::read_unaligned((pe + 8) as *const i32) {
            let jogo = u(*(0xb67a50 as *const usize) + 0x19).wrapping_add((k as usize).wrapping_mul(0x69)) + 4;
            if u(pe) == u(jogo) && b8(jogo + 0x43) != 0xfd {
                if b8(jogo + 0x43) == 0xff {
                    let a = s + 0x40;
                    core::ptr::copy_nonoverlapping(jogo as *const u8, a as *mut u8, 0x4f);
                    if u(a + 0xc) == u(p0 + 0x10) {
                        let e = u(p0 + 0x5f);
                        gravar8(a + 0x43, b8(e + 0x11f));
                        gravar8(a + 0x44, b8(e + 0x744));
                        gravar8(a + 0x4b, 0xff);
                        gravar8(a + 0x4c, 0xff);
                        gravar8(a + 0x49, b8(e + 0x120));
                        gravar8(a + 0x4a, b8(e + 0x745));
                    } else {
                        gravar8(a + 0x43, 0);
                        gravar8(a + 0x44, 0);
                        gravar8(a + 0x4b, 0xff);
                        gravar8(a + 0x4c, 0xff);
                        gravar8(a + 0x49, 0xff);
                        gravar8(a + 0x4a, 0xff);
                    }
                    gravar8(a + 0x46, 0);
                    gravar8(a + 0x45, 0);
                    aplicar(local, a, u(local + 0xb1), u(local + 0x3e) as u32, 1, 0, 0);
                } else {
                    let a = s + 0x180;
                    core::ptr::copy_nonoverlapping(jogo as *const u8, a as *mut u8, 0x4f);
                    contar(p0, k as u32, a);
                    aplicar(local, a, u(local + 0xb1), u(local + 0x3e) as u32, 1, 0, 0);
                }
            }
            k += 1;
        }
    }
    ordenar(local, 0);
    parametro_da_tela(0x27);
    let coluna = parametro_da_tela(0x28);
    parametro_da_tela(0x29);

    // o sorteio ainda por vir
    let sorteio = w16(liga + 0xb6);
    if sorteio != 0xffff {
        data_nova(s + 0x30, sorteio as u32, w16(liga + 0xb8).wrapping_add(w16(liga + 0x40)) as u32);
        core::ptr::copy_nonoverlapping(0xae2c90 as *const u8, (s + 0x38) as *mut u8, 8);
        let (hoje, dia) = (w16(s + 0x3a), w16(s + 0x32));
        if hoje < dia || (hoje == dia && (w16(s + 0x38) as i16) < (w16(s + 0x30) as i16)) {
            let mut y = topo;
            let c = core::ptr::read_unaligned((liga + 0x3a) as *const i16);
            if c != 0 {
                let mut k = competicao_curta(c as i32);
                if k == -1 {
                    k = c as i32;
                }
                nome_da_competicao(BUF, 0x7d0, k as u32, 0);
                txt(sm, 0x6e, z(topo), 0x30c, z(topo + 0x23), 2, 0, 0xc, 3, cor_em(0xaebdec), BUF, 1);
                y = topo + 0x28;
            }
            let a = altura(3);
            let n = ((base - y - 0x10) / a) as u8;
            data_texto(s + 0x30, s + 0x3d0, 0x7d0, 0xff);
            let d = dia_da_semana(s + 0x30);
            let g = genero_do_dia(idioma(), d);
            texto(BUF, 0x99c2e8, g as u8 as i8 as i32, s + 0x3d0);
            mensagem(0x6e, z(y), 0x30c, z(base), 2, 0, n as usize, 3, cor_em(0xae3184), BUF, 0);
            liga_fim(local);
            return;
        }
    }

    // o título e a tabela
    let titulo = s + 0xba0;
    gravar8(s + 0x3d0, 0);
    let c = w16(liga + 0x3a);
    if c != 0 {
        nome_da_competicao(titulo, 0x7d0, c as u32, 0);
    } else {
        texto(titulo, 0xa17b74);
    }
    txt(sm, 0x6e, z(topo), 0x30c, z(topo + 0x23), 2, 0, 0xc, 3, cor_em(0xaebdec), titulo, 1);
    let y = topo + 0x28;
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16);
    let (alto, n);
    if clubes <= 8 && core::ptr::read_unaligned((liga + 0x3c) as *const i16) > 0 && u(u(liga + 4)) != u(0x9cf590) {
        let mut k = (clubes as u8).wrapping_add(1);
        if b8(liga + 0xbe) != 0 || b8(liga + 0xd9) & 1 != 0 {
            k = k.wrapping_add(1);
        }
        for o in [0xbf, 0xc0, 0xc1] {
            if b8(liga + o) != 0 {
                k = k.wrapping_add(1);
            }
        }
        n = k;
        alto = altura(3) * n as i32 + 0x10;
    } else {
        alto = base - y;
        let a = altura(3);
        n = ((alto - 0x10) / a) as u8;
    }
    let lst = lista(display_do_humano(), 0x6e, z(y), 0x30c, z(alto + y), 0xb, s + 0x1c, n as usize, 0, 2, 0, MAX);
    let cabecalho = |col: usize| {
        texto_da_celula(sm, lst, col, 0, 0x30, cor_em(0xae317e), 0xc, 1, cor_em(0xae3184), BUF);
    };
    texto(BUF, 0x990b3c);
    cabecalho(4);
    texto(BUF, 0x990b1c, TEXTO_VAZIO);
    cabecalho(5);
    let regra = w16(liga + 0xab) as u8;
    texto(BUF, if regra & 0x20 != 0 { 0x990b18 } else if regra & 1 != 0 { 0x99c24c } else { 0x990b14 });
    cabecalho(6);
    texto(BUF, 0x990af4, TEXTO_VAZIO);
    cabecalho(7);
    texto(BUF, 0x990af0);
    cabecalho(8);
    texto(BUF, 0x990aec);
    cabecalho(9);
    texto(BUF, 0x990ae8);
    cabecalho(0xa);

    // a liga de verdade ordenada pela coluna do parâmetro 0x28, para as setas de posição
    *((liga + 0xad) as *mut usize) = coluna;
    ordenar(liga, 0);
    let mut fila = 1usize;
    let c8 = |o: usize| b8(local + o) as i8 as i32;
    let mut i = 0i32;
    while i < core::ptr::read_unaligned((local + 0x3e) as *const i16) as i32 {
        let total = core::ptr::read_unaligned((local + 0x3e) as *const i16) as i32;
        // os separadores: vagas de cima (+0xbe, ou a 1 com +0xd9 & 1), de playoff (+0xbf), de baixo (+0xc0, +0xc1)
        let (v0, v1, v2, v3) = (c8(0xbe), c8(0xbf), c8(0xc0), c8(0xc1));
        let separador = (|| {
            if v0 == 0 {
                if b8(local + 0xd9) & 1 != 0 && i == 1 {
                    return Some(0xaebdec);
                }
            } else if v0 > 0 && i == v0 {
                return Some(0xaebdec);
            }
            if v1 > 0 && i == v1 + v0 {
                return Some(0xaebdd4);
            }
            if v2 > 0 && i == total - v3 - v2 {
                return Some(0xaebdd4);
            }
            if v3 > 0 && i == total - v3 {
                return Some(0xaebdec);
            }
            None
        })();
        if let Some(a) = separador {
            // [esp+0x14] da original: a cor na palavra de baixo, o parâmetro 0xe na de cima
            *((s + 0x14) as *mut u16) = *(a as *const u16);
            let x2 = if total as i16 <= n as i16 { 0x30a } else { 0x2f4 };
            linha(display_do_humano(), 1, 0x70, 0, x2, 0, 0, fila, 0x6400, u(s + 0x14), 0, 0, 0, 0, 0, 0, 0, 0, lst);
            fila += 1;
        }
        let r = registro(local, i);
        let clube = u(r);
        let nl = core::ptr::read_unaligned((liga + 0x3e) as *const i16) as i32;
        if nl > 0 {
            let mut j = 0;
            loop {
                let x = registro(liga, j);
                *((s + 0x38) as *mut usize) = x;
                if clube == u(x) {
                    break;
                }
                j += 1;
                if j >= core::ptr::read_unaligned((liga + 0x3e) as *const i16) as i32 {
                    break;
                }
            }
        }
        let pos = por_extenso(b8(r + 4) as i8 as i32 + 1, 3);
        let ret = if core::ptr::read_unaligned((liga + 0x3c) as *const i16) != 0 {
            celula(sm, lst, 0, fila, 0x30, cor_sobre(pos, 0xaebe04), 0xc, 1, cor_em(0xae3184), pos, 7, r)
        } else {
            texto_da_celula(sm, lst, 0, fila, 0x30, cor_sobre(pos, 0xaebe04), 0xc, 1, cor_em(0xae3184), pos)
        };
        let x = u(s + 0x38);
        *(BUF as *mut u8) = 0;
        let (agora, antes) = (b8(r + 4) as i8, b8(x + 4) as i8);
        if agora < antes {
            linha(display_do_humano(), 0x20001, 0, 0, 0, 0, 1, fila, 1, cor_sobre(ret, 0xaebdcc), 0, 0xc, 0, cor_em(0xaebdec), 0, 0, 0, 0, lst);
        } else if agora > antes {
            linha(display_do_humano(), 0x40001, 0, 0, 0, 0, 1, fila, 1, cor_sobre(ret, 0xaebdcc), 0, 0xc, 0, cor_em(0xae31a8), 0, 0, 0, 0, lst);
        }
        let cor = if humano(clube, 0, 1, 0, 0) != 0 { cor_em(0xaebdec) } else { cor_em(0xae3184) };
        celula(sm, lst, 2, fila, 1, 0, 1, 3, cor, clube + 0x38, 0x7d5, clube);
        let d = r + coluna.wrapping_mul(14);
        for (o, col) in [(0x11usize, 4usize), (0x12, 5), (0x13, 6), (0x14, 7)] {
            formatar(BUF, FORMATO_D, b8(d + o) as i8 as i32);
            texto_da_celula(sm, lst, col, fila, 1, 0, 0xc, 2, cor_em(0xae3184), BUF);
        }
        for (o, col) in [(0x15usize, 8usize), (0x17, 9)] {
            formatar(BUF, FORMATO_D, core::ptr::read_unaligned((d + o) as *const i16) as i32);
            texto_da_celula(sm, lst, col, fila, 1, 0, 0xc, 2, cor_em(0xae3184), BUF);
        }
        formatar(BUF, FORMATO_D, core::ptr::read_unaligned((d + 0x19) as *const i16) as i32);
        texto_da_celula(sm, lst, 0xa, fila, 0x30, cor_em(0xae3192), 0xc, 1, cor_em(0xae3184), BUF);
        fila += 1;
        i += 1;
    }

    let rolagem = parametro_da_tela(0x25);
    let disp = display_do_humano();
    let id = lst as u16 as i16;
    if id != -1 {
        rolar(disp.wrapping_add((id as isize * 0xbf1) as usize) + 4, rolagem);
    }
    gravar_parametro_da_tela(0x26, id as i32 as usize);
    let t = u(local + 0xb1);
    if t != 0 {
        liberar_tabela(local, t);
    }
    let v = u(local + 0xba);
    if v != 0 {
        liberar(v);
    }
    liga_fim(local);
}

/// A barra é comparada de verdade quando já existe a cópia do estado (parâmetro 0x1d) e a atualização
/// é local: o `Display` do gerente, os parâmetros, a cópia (0xd7a bytes) e o texto 0xde1f64. Sem a cópia
/// (nasce aqui) ou pela rede: só a original.
unsafe extern "C" fn comparar_atualizar_barra_da_posse() {
    let o = orig::<unsafe extern "C" fn()>(502);
    let p0 = parametro_da_tela(0);
    let copia = parametro_da_tela(0x1d);
    let mut reg = match regioes_da_atualizacao(false) {
        Some(r) if p0 == 0 || copia != 0 => r,
        // ponytail: a alocação da cópia e o pacote pela rede não rodam duas vezes; provados no modo Rust
        _ => return o(),
    };
    reg.extend(regioes_dos_parametros());
    if copia != 0 {
        reg.push((copia, 0xd7a));
    }
    reg.push((0xde1f64, 0x100));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(), || atualizar_barra_da_posse());
    PLACAR.registrar(igual, || format!("atualizar_barra_da_posse: diferencas em {:?}", diferencas_da_tela(&mo, &mr)));
}

/// A visão geral é comparada de verdade pela lista de exibição, com `*topo`, `*base` e a barra da posse
/// do fim (o `Display` do gerente e a cópia do estado). Só a original: com o aviso de erro, com o
/// narrador (parâmetro 7: escreve o lance e toca o som), na partida sem a cópia do estado (a barra a
/// aloca) e com a atualização pela rede.
unsafe extern "C" fn comparar_painel_da_visao_geral_da_partida(topo: *mut i32, base: *mut i32) {
    let o = orig::<unsafe extern "C" fn(*mut i32, *mut i32)>(503);
    let p0 = parametro_da_tela(0);
    let foto = parametro_da_tela(0x1d);
    let narrador = p0 != 0
        && core::ptr::read_unaligned((core::ptr::read_unaligned((p0 + 0x5f) as *const usize) + 0x53) as *const u32) == 0
        && parametro_da_tela(7) != 0;
    let reg = regioes_da_atualizacao(false);
    if visao_geral_avisa_erro() || narrador || (p0 != 0 && foto == 0) || reg.is_none() {
        // ponytail: aviso, som, alocação e rede não rodam duas vezes; esses caminhos se provam no modo Rust
        return o(topo, base);
    }
    let mut reg = reg.unwrap_or_default();
    reg.extend(regioes_dos_parametros());
    reg.push((0xde1f64, 0x100));
    reg.push((topo as usize, 4));
    reg.push((base as usize, 4));
    if foto != 0 {
        reg.push((foto, 0xd7a));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(topo, base), || painel_da_visao_geral_da_partida(topo, base));
    PLACAR.registrar(igual, || format!("painel_da_visao_geral_da_partida: diferencas em {:?}", diferencas_da_tela(&mo, &mr)));
}

/// A tabela ao vivo é comparada de verdade pela lista de exibição, com a liga (0xee bytes: [+0xad]) e a
/// tabela dela ([+0xb1], 59 bytes por clube), que a rotina reordena. As cópias locais nascem e morrem
/// dentro dela. Sem liga ou sem o parâmetro 0xe (caixa "Error"): só a original.
unsafe extern "C" fn comparar_tabela_da_liga_ao_vivo(liga: usize, topo: i32, base: i32) {
    let o = orig::<unsafe extern "C" fn(usize, i32, i32)>(504);
    if liga == 0 || parametro_da_tela(0xe) == 0 {
        return o(liga, topo, base);
    }
    let mut extra = vec![(liga, 0xee)];
    let tabela = core::ptr::read_unaligned((liga + 0xb1) as *const usize);
    let n = core::ptr::read_unaligned((liga + 0x3e) as *const i16);
    if tabela != 0 && n > 0 {
        extra.push((tabela, n as usize * 59));
    }
    comparar_pela_lista_com("tabela_da_liga_ao_vivo", &extra, || o(liga, topo, base), || tabela_da_liga_ao_vivo(liga, topo, base));
}
