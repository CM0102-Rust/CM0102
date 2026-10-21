// ---------- %%LEVA%%a leva: as telas da partida (14): a montagem e a resposta da tela da partida ao vivo e a montagem da tela de detalhes (match_screens.cpp) ----------

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
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, original, rust);
    PLACAR.registrar(igual, || format!("{}: diferencas em {:?}", nome, diferencas_da_tela(&mo, &mr)));
}

/// A tela da partida é comparada pela lista de exibição, com o que ela grava fora da tela: a ficha do
/// humano (+0 e +0xea), a marca dele nos espectadores e a contagem, o foco das velocidades no `Display`
/// do humano (+0x12f4fe) e o volume do narrador (+0x2db0). Só a original quando há o que não roda duas
/// vezes: a foto do parâmetro 0x1d a liberar, o lance do parâmetro 7 a narrar (som, 0x6d7b00) ou as
/// abas 1 (o resumo também narra), 3 (as zonas alocam a foto) e 10 (a tabela ao vivo).
unsafe extern "C" fn comparar_montar_tela_da_partida() {
    let o = orig::<unsafe extern "C" fn()>(%%I0%%);
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
    orig::<unsafe extern "C" fn(u32) -> i32>(%%I1%%)(item)
}

/// A tela de detalhes é comparada pela lista de exibição quando o registro e a foto da partida já
/// existem (parâmetros 0x1f e 0x20) e não há foto 0x1d a liberar. Só a original quando eles nascem aqui
/// (alocação) ou nas abas 1 (o resumo narra com som), 3 (as zonas alocam a foto) e 4 (o relatório nasce).
unsafe extern "C" fn comparar_montar_detalhes_da_partida() {
    let o = orig::<unsafe extern "C" fn()>(%%I2%%);
    let aba = parametro_da_tela(2) as u8;
    // as abas 2 (estatisticas) e 6 (notas) alocam a copia do estado 0x1d: tambem so a original
    if parametro_da_tela(0x1d) != 0 || parametro_da_tela(0x1f) == 0 || parametro_da_tela(0x20) == 0 || matches!(aba, 1 | 2 | 3 | 4 | 6) {
        // ponytail: alocar, liberar e narrar não rodam duas vezes; esses casos se provam no modo Rust
        return o();
    }
    comparar_montagem_da_partida("montar_detalhes_da_partida", &[], || o(), || montar_detalhes_da_partida());
}
