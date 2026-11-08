// ---------- %%LEVA%%a leva: as telas da partida (8): os painéis das estatísticas do time, das notas dos jogadores e das estatísticas da partida (match_screens.cpp) ----------

type CelulaDaLista = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
type TextoDaCelulaDaLista = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
type ListaDoDisplay = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;

/// O `Display` do humano da vez ([0xb74c4c + 0x300 x (i16)[0xb77c7a]]).
unsafe fn current_human_display() -> usize {
    *((0xb74c4c + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize)
}

/// A cor (u16) guardada em `a`, estendida com zero. Onde a original empurra um registrador inteiro com
/// lixo imprevisível nos 16 bits de cima (o que sobrou de uma chamada), vai o valor estendido.
unsafe fn colour_word(a: usize) -> usize {
    *(a as *const u16) as usize
}

/// { 0x7217a0 } (cdecl, match_screens.cpp): desenha a tabela das estatísticas de um time de `*topo` a
/// `*base`. Fora da partida (parâmetro 0 nulo) e sem estatísticas ([parâmetro 0x20 + 0x4c] ≠ 0): só a
/// mensagem "There are no team stats available." (0x5ed070, as linhas que cabem na fonte 3). Senão: a
/// moldura (0x55fba0), a tabela do cabeçalho de 22 colunas, as setas de página nas colunas 4 e 5 ("<<"
/// e ">>", ou '*' e '+' pela fonte de símbolos [0x9cc2c4] ≠ -1; o item da 4 no parâmetro 0x1a), a seta
/// da página atual (parâmetro 0x1c: 1 ou 2) desligada (0x800580), os títulos das colunas 7 a 0x13 pela
/// tabela da página (0xa16178 / 0xa1618c; o texto em [0xa16118 + 4k], com os títulos de comentário
/// próprios de cada página), a tabela dos valores, o lado `lado` no parâmetro 0x1b, -1 no 0x1a e o
/// preenchimento (0x721cd0).
unsafe extern "C" fn draw_team_stats_panel(lado: u32, topo: *mut i32, base: *mut i32) {
    crate::comparacao::ramo("telas painel das estatisticas do time");
    let celula: CelulaDaLista = core::mem::transmute(0x8040a0usize);
    let texto_da_celula: TextoDaCelulaDaLista = core::mem::transmute(0x804100usize);
    let lista: ListaDoDisplay = core::mem::transmute(0x55fba0usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;

    let pagina = get_screen_param(0x1c) as u8;
    let mut cabecalho: [u8; 22] = [1, 1, 1, 0x49, 8, 8, 3, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xe, 0];
    let mut valores: [u8; 22] = [0, 0xe, 1, 5, 1, 0x35, 0x15, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 7, 7];
    if get_screen_param(0) == 0 && *((get_screen_param(0x20) + 0x4c) as *const u8) != 0 {
        let altura: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x5e5cd0usize);
        let a = altura(3);
        let n = ((*base - *topo - 0x10) / a) as u8;
        texto(BUF, 0xa16a08);
        let mensagem: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
            core::mem::transmute(0x5ed070usize);
        mensagem(0x6e, *topo as usize, 0x30c, *base as usize, 2, 0, n as usize, 3, colour_word(0xae3184), BUF, 0);
        return;
    }

    lista(current_human_display(), 0x6e, *topo as usize, 0x30c, *base as usize, 1, 0, 1, 0, 2, 0, usize::MAX);
    let tabela = lista(current_human_display(), 0x6e, *topo as usize, 0x30c, *base as usize, 0x16, cabecalho.as_mut_ptr() as usize, 0x15, 0, 1, 0, usize::MAX);
    // a seta "<<": com a fonte normal a original empurra eax = [0x9cc2c4] (-1) com a cor em ax
    let r = if *(0x9cc2c4 as *const i32) != -1 {
        formatar(BUF, 0x99ec1c, 0x2a);
        celula(sm, tabela, 4, 0, 0x30, colour_word(0xaebe04), 0xc, 0, colour_word(0xae3184), BUF, 9, 0)
    } else {
        celula(sm, tabela, 4, 0, 0x30, colour_word(0xaebe04), 0xc, 1, 0xffff_0000 | colour_word(0xae3184), 0x98e7c4, 9, 0)
    };
    set_screen_param(0x1a, r as u16 as i16 as i32 as usize);
    if *(0x9cc2c4 as *const i32) != -1 {
        formatar(BUF, 0x99ec1c, 0x2b);
        celula(sm, tabela, 5, 0, 0x30, colour_word(0xaebe04), 0xc, 0, colour_word(0xae3184), BUF, 0xa, 0);
    } else {
        celula(sm, tabela, 5, 0, 0x30, colour_word(0xaebe04), 0xc, 1, colour_word(0xae3184), 0x98e7a8, 0xa, 0);
    }
    // a tabela das colunas da página; numa página fora de 1 e 2 a original lê pelo valor do ponteiro
    // `base` (o lugar do argumento é que recebe a tabela), e nenhum título sai
    let (colunas, seta) = match pagina {
        1 => (0xa16178usize, r),
        2 => (0xa1618c, r.wrapping_add(1)),
        _ => (base as usize, r.wrapping_add(1)),
    };
    let desligar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x800580usize);
    desligar(sm, seta, 1);
    for c in 7..0x14usize {
        let k = *((colunas + c - 2) as *const u8) as usize;
        let rotulo = *((0xa16118 + k * 4) as *const usize);
        if rotulo == 0 {
            continue;
        }
        let proprio = match pagina {
            1 => match c {
                0xb => Some(0xa16968usize),
                0xe => Some(0xa169e8),
                0x11 => Some(0xa169cc),
                0x12 => Some(0xa169a8),
                0x13 => Some(0xa16988),
                _ => None,
            },
            2 => match c {
                9 => Some(0xa16968usize),
                0xc => Some(0xa169cc),
                0xd => Some(0xa169a8),
                0xe => Some(0xa16944),
                0x10 => Some(0xa16924),
                0x13 => Some(0xa16988),
                _ => None,
            },
            _ => continue,
        };
        match proprio {
            Some(f) => texto(BUF, f, 0xad9c64),
            None => texto(BUF, rotulo, 0),
        };
        texto_da_celula(sm, tabela, c, 0, 0x30, colour_word(0xae317e), 0xc, 1, colour_word(0xae3184), BUF);
    }
    let tabela_dos_valores = lista(current_human_display(), 0x6e, *topo as usize, 0x30c, *base as usize, 0x16, valores.as_mut_ptr() as usize, 0x15, 0, 1, 0, usize::MAX);
    set_screen_param(0x1b, lado as u8 as i8 as i32 as usize);
    set_screen_param(0x1a, usize::MAX);
    let preencher: unsafe extern "C" fn(u32, usize) = core::mem::transmute(0x721cd0usize);
    preencher(1, tabela_dos_valores);
}

/// { 0x722ed0 } (cdecl, match_screens.cpp): desenha as notas dos jogadores dos dois times de `*topo` a
/// `*base`: uma tabela de 16 colunas (8 por time) e 20 linhas por time. Os dados: na partida (parâmetro
/// 0) [p0 + 0x5f] e a partida p0 + 4; fora dela os parâmetros 0x20 e 0x1f. Por linha `j` do time `i`
/// (registro de 73 bytes em dados + 1573i + 73j + 0x190): o número da camisa (ou j + 1) com fundo por
/// titular/reserva (as 11 primeiras e até 0xb + [partida+0x4d]&0xf; o item da primeira no parâmetro
/// 0xd), dois separadores, o nome (0x53e140; "(c)" no capitão, [dados + 1573i + 0x153]) como botão
/// (0x8040a0, item 0x7dc ou 0x7d7 pelo saldo de [dados] ou [dados+4]) quando a pessoa vale (0x53e990),
/// senão texto; vazio (0xfb em +0x1d) sem nome; e as três células da nota. No fim, 0x723350.
unsafe extern "C" fn draw_player_ratings_panel(topo: *mut i32, base: *mut i32) {
    crate::comparacao::ramo("telas painel das notas dos jogadores");
    let celula: CelulaDaLista = core::mem::transmute(0x8040a0usize);
    let texto_da_celula: TextoDaCelulaDaLista = core::mem::transmute(0x804100usize);
    let lista: ListaDoDisplay = core::mem::transmute(0x55fba0usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let cor_reserva: unsafe extern "C" fn(u32, u32, u32, u32) -> usize = core::mem::transmute(0x5e4800usize);
    let nome_da_pessoa: unsafe extern "C" fn(u32, u32, u32, usize, u32, u32, i32) = core::mem::transmute(0x53e140usize);
    let pessoa_vale: unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32 = core::mem::transmute(0x53e990usize);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;

    let mut colunas: [u8; 16] = [0xe, 1, 5, 1, 0x46, 0x15, 7, 7, 0xe, 1, 5, 1, 0x46, 0x15, 7, 7];
    let p0 = get_screen_param(0);
    let (dados, partida) = if p0 != 0 {
        (core::ptr::read_unaligned((p0 + 0x5f) as *const usize), p0 + 4)
    } else {
        let d = get_screen_param(0x20);
        (d, get_screen_param(0x1f))
    };
    let tabela = lista(current_human_display(), 0x6e, *topo as usize, 0x30c, *base as usize, 0x10, colunas.as_mut_ptr() as usize, 0x14, 0, 2, 0, usize::MAX);
    for i in 0..2usize {
        let col = 8 * i;
        for j in 0..0x14usize {
            let reg = dados + 1573 * i + 73 * j + 0x190;
            let camisa = *((reg + 0x1c) as *const i8);
            formatar(BUF, 0x98806c, if camisa > 0 { camisa as i32 } else { j as i32 + 1 });
            let starters = (*((partida + 0x4d) as *const u8) & 0xf) as usize + 0xb;
            // as 11 primeiras: a original empurra eax = a partida com a cor em ax
            let fundo = if j < 0xb { (partida & 0xffff_0000) | colour_word(0xaebe04) } else { cor_reserva(0, 0, 0x40, 0) };
            let r = if j < starters {
                texto_da_celula(sm, tabela, col, j, 0x30, fundo, 0xc, 1, colour_word(0xae3184), BUF)
            } else {
                texto_da_celula(sm, tabela, col, j, 0x30, fundo, 0x2c, 1, colour_word(0xae3184), 0xad9c64)
            };
            if i == 0 && j == 0 {
                set_screen_param(0xd, r as u16 as i16 as i32 as usize);
            }
            texto_da_celula(sm, tabela, col + 2, j, 1, colour_word(0xaebdec), 0xc, 1, colour_word(0xaebdea), 0);
            texto_da_celula(sm, tabela, col + 2, j, 1, colour_word(0xae31a8), 0xc, 1, colour_word(0xaebdea), 0);
            if *((reg + 0x1d) as *const u8) == 0xfb {
                texto_da_celula(sm, tabela, col + 4, j, 1, 0, 1, 2, colour_word(0xae317e), 0);
            } else {
                let id = core::ptr::read_unaligned(reg as *const i32);
                let a = core::ptr::read_unaligned((reg + 8) as *const u32);
                let b = core::ptr::read_unaligned((reg + 0xc) as *const u32);
                let c = core::ptr::read_unaligned((reg + 0x10) as *const u32);
                let mut nome = [0u8; 0xc8];
                nome_da_pessoa(a, b, c, nome.as_mut_ptr() as usize, 0xc8, 1, id);
                let capitao = *((dados + 1573 * i + 0x153) as *const i8) as i32;
                if j as i32 == capitao {
                    texto(BUF, 0xa16a7c, nome.as_ptr() as usize);
                } else {
                    copy_to_work_text(nome.as_ptr() as usize);
                }
                let mut botao = false;
                if id >= 0 && id < *(0xae2364 as *const i32) {
                    let pessoa = (*(0xae23bc as *const usize) as isize + id as isize * 110) as usize;
                    if pessoa_vale(pessoa, a, b, c, reg + 0x14) != 0 {
                        botao = true;
                        let saldo = if i == 0 { *(dados as *const i32) } else { *((dados + 4) as *const i32) };
                        let limit = (*(0xae235c as *const i32)).wrapping_sub((*(0xae2350 as *const i32)).wrapping_mul(2));
                        let item = if saldo >= limit { 0x7dc } else { 0x7d7 };
                        celula(sm, tabela, col + 4, j, 1, 0, 1, 2, colour_word(0xae317e), BUF, item, pessoa);
                    }
                }
                if !botao {
                    texto_da_celula(sm, tabela, col + 4, j, 1, 0, 1, 2, colour_word(0xae317e), BUF);
                }
            }
            texto_da_celula(sm, tabela, col + 5, j, 1, 0, 1, 1, colour_word(0xae3154), 0);
            texto_da_celula(sm, tabela, col + 6, j, 1, 0, 0xc, 2, colour_word(0xaebde8), 0);
            texto_da_celula(sm, tabela, col + 7, j, 1, 0, 0xc, 1, colour_word(0xaebdec), 0);
        }
    }
    let preencher: unsafe extern "C" fn() = core::mem::transmute(0x723350usize);
    preencher();
}

/// Os rótulos das 13 linhas das estatísticas da partida e as cores deles.
const ROTULOS_DAS_ESTATISTICAS_DA_PARTIDA: [(usize, usize); 13] = [
    (0xa16870, 0xae3184), // Shots On Goal
    (0xa16864, 0xae3184), // On Target
    (0xa16858, 0xae3184), // Off Target
    (0xa16850, 0xae3184), // Corners
    (0xa16844, 0xae3184), // Free Kicks
    (0xa16838, 0xae3184), // Throw-Ins
    (0xa16830, 0xae3184), // Fouls
    (0xa16824, 0xae3184), // Offsides
    (0xa16810, 0xae3184), // Passes Completed
    (0xa16804, 0xae3184), // Tackles Won
    (0xa167f8, 0xae3184), // Headers Won
    (0x99caf4, 0xaebdec), // Yellow Cards
    (0x99cae8, 0xae31a8), // Red Cards
];

/// { 0x71e040 } (cdecl, match_screens.cpp): desenha as estatísticas da partida de `*topo` a `*base`: a
/// tabela de 3 colunas (1, 5, 1) e 13 linhas, cada uma com as células dos dois times (colunas 0 e 2;
/// o item da primeira no parâmetro 9) e o rótulo no meio; depois os valores (0x71e750).
unsafe extern "C" fn draw_match_stats_panel(topo: *mut i32, base: *mut i32) {
    crate::comparacao::ramo("telas painel das estatisticas da partida");
    let texto_da_celula: TextoDaCelulaDaLista = core::mem::transmute(0x804100usize);
    let lista: ListaDoDisplay = core::mem::transmute(0x55fba0usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;

    let mut colunas: [u8; 3] = [1, 5, 1];
    let tabela = lista(current_human_display(), 0x6e, *topo as usize, 0x30c, *base as usize, 3, colunas.as_mut_ptr() as usize, 0xd, 0, 2, 0, usize::MAX);
    for (k, &(rotulo, cor)) in ROTULOS_DAS_ESTATISTICAS_DA_PARTIDA.iter().enumerate() {
        let r = texto_da_celula(sm, tabela, 0, k, 0x30, colour_word(0xaebe04), 0xc, 3, colour_word(0xaebde8), 0);
        if k == 0 {
            set_screen_param(9, r as u16 as i16 as i32 as usize);
        }
        texto(BUF, rotulo);
        texto_da_celula(sm, tabela, 1, k, 1, 0, 0xc, 4, colour_word(cor), BUF);
        texto_da_celula(sm, tabela, 2, k, 0x30, colour_word(0xaebe04), 0xc, 3, colour_word(0xaebde8), 0);
    }
    let preencher: unsafe extern "C" fn() = core::mem::transmute(0x71e750usize);
    preencher();
}

/// Os três painéis são comparados de verdade: as duas desenham a partir do mesmo estado e o gerente de
/// telas, o `Display` da tela atual, o bloco de parâmetros com as marcas 0xb6b7a8/ac/b0 e o texto
/// 0xde1f64 têm de sair iguais. A lista (0x55fba0) também escreve num buffer de rede ([0xae2908] na
/// posição [0xae290c]): a posição entra nas regiões (volta para a segunda rodada escrever no mesmo
/// lugar e o tamanho escrito é comparado); o conteúdo desse buffer não.
unsafe fn comparar_painel_pela_lista(nome: &str, original: impl FnOnce(), rust: impl FnOnce()) {
    let mut reg = screen_build_regions();
    reg.extend(screen_param_regions());
    reg.push((0xde1f64, 0x100));
    reg.push((0xae290c, 4));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, original, rust);
    PLACAR.registrar(igual, || {
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("{}: diferencas em {:?}", nome, dif)
    });
}

unsafe extern "C" fn comparar_draw_team_stats_panel(lado: u32, topo: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(u32, *mut i32, *mut i32)>(%%I0%%);
    comparar_painel_pela_lista("draw_team_stats_panel", || o(lado, topo, base), || draw_team_stats_panel(lado, topo, base));
}

unsafe extern "C" fn comparar_draw_player_ratings_panel(topo: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(%%I1%%);
    // ponytail: sem a copia do estado (parametro 0x1d) a atualizacao dentro do painel a aloca, e cada
    // lado alocaria a sua (enderecos diferentes): so a original; o modo Rust prova esse caminho
    if get_screen_param(0x1d) == 0 {
        return o(topo, base);
    }
    comparar_painel_pela_lista("draw_player_ratings_panel", || o(topo, base), || draw_player_ratings_panel(topo, base));
}

unsafe extern "C" fn comparar_draw_match_stats_panel(topo: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(%%I2%%);
    // ponytail: sem a copia do estado (parametro 0x1d) a atualizacao dentro do painel a aloca, e cada
    // lado alocaria a sua (enderecos diferentes): so a original; o modo Rust prova esse caminho
    if get_screen_param(0x1d) == 0 {
        return o(topo, base);
    }
    comparar_painel_pela_lista("draw_match_stats_panel", || o(topo, base), || draw_match_stats_panel(topo, base));
}
