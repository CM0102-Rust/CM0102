// ---------- %%LEVA%%a leva: as telas da partida (10): os painéis das zonas de ação e do relatório da partida e a tela dos jogos do dia (match_screens.cpp) ----------

type DisplayLineFn = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
type ManagerBoxFn = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
type ManagerTextFn = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;

/// A cor (u16) em `a` com os 16 bits de cima de `x`: onde a original troca só o `ax`/`cx`/`dx` de um
/// registrador que tem um valor conhecido (a devolução de uma chamada, uma conta) e o empurra inteiro.
unsafe fn colour_with_high_bits(x: usize, a: usize) -> usize {
    (x & 0xffff_0000) | colour_word(a)
}

/// A mensagem de "não há" dos painéis da partida (0x5ed070 com as linhas que cabem na fonte 3).
unsafe fn show_panel_empty_message(topo: *mut i32, base: *mut i32, texto_id: usize) {
    let altura: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x5e5cd0usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let mensagem: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(0x5ed070usize);
    let a = altura(3);
    let n = ((*base - *topo - 0x10) / a) as u8;
    texto(0xde1f64, texto_id);
    mensagem(0x6e, *topo as usize, 0x30c, *base as usize, 2, 0, n as usize, 3, colour_word(0xae3184), 0xde1f64, 0);
}

/// { 0x71f500 } (cdecl, match_screens.cpp): desenha as zonas de ação de `*topo` a `*base`. Fora da
/// partida (parâmetro 0 nulo) e sem dados ([parâmetro 0x20 + 0x4c] ≠ 0): a mensagem "There are no
/// action zones available.". Senão: a caixa "View" com a lista dos cinco períodos (o item do primeiro no
/// parâmetro 0x23; o texto do período do parâmetro 0xa fica guardado), `*topo` += 0x19, o campo
/// (0x8ac600, que devolve o retângulo), as duas linhas que o dividem em três, o título do período, as
/// três zonas (botão do parâmetro 0xb na primeira), em cada zona o painel com as barras das cores dos
/// dois clubes (0x53b640) e a linha do meio, o painel de baixo igual, o retângulo nos parâmetros 0x16 a
/// 0x19, `*base` no 0xc e os valores (0x720320).
unsafe extern "C" fn draw_action_zones_panel(topo: *mut i32, base: *mut i32) {
    crate::comparison::branch("screens action zones panel");
    let caixa: ManagerBoxFn = core::mem::transmute(0x804030usize);
    let txt: ManagerTextFn = core::mem::transmute(0x803fd0usize);
    let lista: DisplayListFn = core::mem::transmute(0x55fba0usize);
    let linha: DisplayLineFn = core::mem::transmute(0x55f990usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;

    let guarda = 0xb6b7b4 as *mut u8;
    if *guarda & 1 == 0 {
        *guarda |= 1;
        *(0xb6b7a4 as *mut u16) = *(0xaebdec as *const u16);
    }
    let p0 = get_screen_param(0);
    let vista = get_screen_param(0xa) as u8;
    let dados = if p0 != 0 {
        core::ptr::read_unaligned((p0 + 0x5f) as *const usize)
    } else {
        let d = get_screen_param(0x20);
        if *((d + 0x4c) as *const u8) != 0 {
            return show_panel_empty_message(topo, base, 0xa16880);
        }
        d
    };

    texto(BUF, 0x988004);
    let t = *topo as usize;
    let c = caixa(sm, 0x6e, t, 0xeb, t + 0x14, 0x30, colour_word(0xae317e), 0xc, 1, colour_word(0xae3184), BUF, 0, 0);
    let periodos = lista(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, colour_word(0xae31aa), c);
    // ponytail: com a vista fora de 0 a 4 a original passa o texto de uma variável local sem valor; aqui vazio
    let mut titulo = [0u8; 200];
    for (k, &(rotulo, item)) in [(0xa16738usize, 4u8), (0xa167ec, 0), (0xa167e0, 1), (0xa168a8, 2), (0x98800c, 3)].iter().enumerate() {
        texto(BUF, rotulo);
        let r = linha(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x90, colour_word(0xae31aa), colour_word(0xaebdec), 0x21, 1, colour_word(0xaebdea), BUF, 0, 0, item as usize, periodos);
        if k == 0 {
            set_screen_param(0x23, r as u16 as i16 as i32 as usize);
        }
        if vista == item {
            for i in 0..200 {
                let b = *((BUF + i) as *const u8);
                titulo[i] = b;
                if b == 0 {
                    break;
                }
            }
        }
    }

    *topo += 0x19;
    lista(current_human_display(), 0x6e, *topo as usize, 0x30c, *base as usize, 1, 0, 1, 0, 2, 0, usize::MAX);
    let (mut x0, mut y0, mut x1, mut y1) = (0i32, 0i32, 0i32, 0i32);
    let campo: unsafe extern "C" fn(i32, i32, i32, i32, *mut i32, *mut i32, *mut i32, *mut i32, u32, i32, u32) =
        core::mem::transmute(0x8ac600usize);
    campo(0xb9, *topo + 10, 0x2c1, *base - 0x37, &mut x0, &mut y0, &mut x1, &mut y1, 1, -1, 0x4080_0000);
    let w = x1 - x0;
    let terco = w / 3;
    let dois = w.wrapping_mul(2) / 3;
    let u = |v: i32| v as usize;

    linha(current_human_display(), 1, u(x0 + terco), u(y0), u(x0 + terco), u(y1), 0, 0, 0x2400, colour_with_high_bits(u(w), 0xb6b7a4), 0, 0, 0, 0, 0, 0, 0, 0, usize::MAX);
    linha(current_human_display(), 1, u(x0 + dois), u(y0), u(x0 + dois), u(y1), 0, 0, 0x2400, colour_with_high_bits(u(w.wrapping_mul(2)), 0xb6b7a4), 0, 0, 0, 0, 0, 0, 0, 0, usize::MAX);
    txt(sm, u(x0 + terco + 10), u(y0 + 5), u(x0 + dois - 10), u(y0 + 0x19), 2, 0, 0xc, 2, colour_word(0xae3184), titulo.as_ptr() as usize, 1);
    let r = txt(sm, u(x0), u(y0), u(x0 + terco), u(y1), 1, 0, 0xc, 4, colour_word(0xaebdec), 0, 1);
    set_screen_param(0xb, r as u16 as i16 as i32 as usize);
    txt(sm, u(x0 + terco), u(y0), u(x0 + dois), u(y1), 1, 0, 0xc, 4, colour_word(0xaebdec), 0, 1);
    txt(sm, u(x0 + dois), u(y0), u(x1), u(y1), 1, 0, 0xc, 4, colour_word(0xaebdec), 0, 1);
    txt(sm, u(x0 + dois + 10), u(y0 + 5), u(x1 - 10), u(y0 + 0x19), 1, 0, 0xc, 1, colour_word(0xaebdec), 0, 1);

    // as cores dos dois clubes (0x53b640 grava u16; o resto da variável da original é lixo da pilha, aqui 0)
    let cores_do_clube: unsafe extern "C" fn(usize, u32, *mut u32, *mut u32) = core::mem::transmute(0x53b640usize);
    let (mut c1a, mut c1b, mut c2a, mut c2b) = (0u32, 0u32, 0u32, 0u32);
    let clubes = *(0xae23b4 as *const usize);
    let id1 = core::ptr::read_unaligned(dados as *const i32);
    cores_do_clube((clubes as isize + id1 as isize * 0x245) as usize, *((dados + 0x152) as *const u8) as u32, &mut c1a, &mut c1b);
    let id2 = core::ptr::read_unaligned((dados + 4) as *const i32);
    cores_do_clube((clubes as isize + id2 as isize * 0x245) as usize, *((dados + 0x777) as *const u8) as u32, &mut c2a, &mut c2b);

    // uma zona: a moldura, o painel de `l` a `r`, as barras das duas cores e a linha do meio; devolve a da linha
    let zona = |a: (i32, i32), cor_a: usize, l: i32, r: i32, cor_b: (usize, usize)| -> usize {
        txt(sm, u(a.0), u(y1 - 0x19), u(a.1), u(y1 - 5), 0x30, cor_a, 0xc, 1, colour_word(0xae3184), 0, 1);
        txt(sm, u(l), u(y1 - 0x14), u(r), u(y1 - 10), 0x60, cor_b.0, 0xc, 1, cor_b.1, 0, 1);
        let d = r - l;
        let meio = (d * 25 - 100) * 2 / 100 + l + 2;
        txt(sm, u(l + 2), u(y1 - 0x12), u(meio), u(y1 - 12), 0x10, c1b as usize, 0xc, 1, c1a as usize, 0, 1);
        txt(sm, u(meio), u(y1 - 0x12), u(r - 2), u(y1 - 12), 0x10, c2b as usize, 0xc, 1, c2a as usize, 0, 1);
        let xm = l + d / 2;
        let sinal = if d < 0 { 0xffff_0000 } else { 0 };
        linha(current_human_display(), 1, u(xm), u(y1 - 0x12), u(xm), u(y1 - 0x10), 0, 0, 0x400, sinal | colour_word(0xaebdea), 0, 0, 0, 0, 0, 0, 0, 0, usize::MAX)
    };
    let r1 = zona((x0 + 5, x0 + terco - 5), colour_word(0xae317e), x0 + 10, x0 + terco - 10, (colour_with_high_bits(u(y1), 0xae317e), colour_word(0xae3184)));
    let r2 = zona((x0 + terco + 5, x0 + dois - 5), colour_with_high_bits(r1, 0xae317e), x0 + terco + 10, x0 + dois - 10, (colour_with_high_bits(u(terco), 0xae317e), colour_with_high_bits(u(w), 0xae3184)));
    zona((x0 + dois + 5, x1 - 5), colour_with_high_bits(r2, 0xae317e), x0 + dois + 10, x1 - 10, (colour_word(0xae317e), colour_with_high_bits(u(dois), 0xae3184)));

    let b = *base;
    let r = txt(sm, 0xb9, u(b - 0x2d), 0x2c1, u(b - 10), 0x30, colour_word(0xae317e), 0xc, 1, colour_word(0xae3184), 0, 1);
    txt(sm, 0xd2, u(b - 0x25), 0x2a8, u(b - 0x12), 0x60, colour_word(0xae317e), 0xc, 1, colour_with_high_bits(r, 0xae3184), 0, 1);
    txt(sm, 0xd4, u(b - 0x23), 0x1bd, u(b - 0x14), 0x10, c1b as usize, 0xc, 1, c1a as usize, 0, 1);
    let r = txt(sm, 0x1bd, u(b - 0x23), 0x2a6, u(b - 0x14), 0x10, c2b as usize, 0xc, 1, c2a as usize, 0, 1);
    linha(current_human_display(), 1, 0x1bd, u(b - 0x23), 0x1bd, u(b - 0x20), 0, 0, 0x400, colour_with_high_bits(r, 0xaebdea), 0, 0, 0, 0, 0, 0, 0, 0, usize::MAX);
    set_screen_param(0x16, u(x0));
    set_screen_param(0x17, u(y0));
    set_screen_param(0x18, u(x1));
    set_screen_param(0x19, u(y1));
    set_screen_param(0xc, u(*base));
    let valores: unsafe extern "C" fn() = core::mem::transmute(0x720320usize);
    valores();
}

/// { 0x721000 } (cdecl, match_screens.cpp): desenha o relatório da partida de `*topo` a `*base`: uma
/// tabela de 2 colunas (1, 5) com uma linha por texto de cada lance do relatório (0x6d8d10, " %s", cor
/// pelo tipo do lance) e o minuto (0x66f4e0 ou, no idioma 2, 0x90c310) na primeira linha de cada lance
/// de minuto novo; completa com linhas vazias até 20. O relatório: na partida, [[p0 + 0x57] + 0x8f7a];
/// fora dela, o do parâmetro 0x2d, que nasce aqui (0x944cf1 de 0x2db1 bytes, 0x6d5bf0, montado pelos
/// dados + 0xd69 em 0x6d9c40; se não monta, é destruído e sai a mensagem). Os lances do código
/// [dados + 0x4f] 0x1ef, 0x3de (minuto ≥ 45), 0x483 (≥ 90) e 0x528 (≥ 105) ficam de fora. Sem dados
/// ([parâmetro 0x20 + 0x4c]) ou sem linhas: "There is no match report available.". Relatório nulo ou
/// sem memória: a original (caixa de erro).
unsafe extern "C" fn draw_match_report_panel(topo: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(%%I1%%);
    let texto_da_celula: ListCellTextFn = core::mem::transmute(0x804100usize);
    let lista: DisplayListFn = core::mem::transmute(0x55fba0usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;
    const SEM_RELATORIO: usize = 0xa16900;

    let p0 = get_screen_param(0);
    let (dados, partida, relatorio);
    if p0 == 0 {
        dados = get_screen_param(0x20);
        partida = get_screen_param(0x1f);
        if *((dados + 0x4c) as *const u8) != 0 {
            crate::comparison::branch("screens match report panel");
            return show_panel_empty_message(topo, base, SEM_RELATORIO);
        }
        if get_screen_param(0x2d) == 0 {
            let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
            let memoria = novo(0x2db1);
            if memoria == 0 {
                // ponytail: sem memória a original mostra o erro (e tenta alocar de novo, sem efeito)
                return o(topo, base);
            }
            crate::comparison::branch("screens match report panel (build)");
            // ponytail: o quadro SEH da original só libera a memória se o construtor lançar exceção, o que ele não faz
            let construir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x6d5bf0usize);
            let r = construir(memoria, 0);
            set_screen_param(0x2d, r);
            let montar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6d9c40usize);
            if montar(r, dados + 0xd69) as u8 == 0 {
                show_panel_empty_message(topo, base, SEM_RELATORIO);
                let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5a8f60usize);
                destruir(r);
                let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
                liberar(r);
                set_screen_param(0x2d, 0);
                return;
            }
            relatorio = r;
        } else {
            relatorio = get_screen_param(0x2d);
        }
    } else {
        dados = core::ptr::read_unaligned((get_screen_param(0) + 0x5f) as *const usize);
        partida = get_screen_param(0) + 4;
        relatorio = core::ptr::read_unaligned((core::ptr::read_unaligned((get_screen_param(0) + 0x57) as *const usize) + 0x8f7a) as *const usize);
    }
    if relatorio == 0 {
        return o(topo, base);
    }
    crate::comparison::branch("screens match report panel");

    let lances = |r: usize| *((r + 6) as *const i16);
    let mut colunas: [u8; 2] = [1, 5];
    let mut ultimo_minuto: u8 = 0xff;
    let mut linhas: i32 = 0;
    let mut tabela: usize = 0;
    // a cor da linha: as cores u16 trocam só os 16 bits de baixo (os de cima começam com lixo na original)
    let mut cor: usize = 0;
    if lances(relatorio) <= 0 {
        return show_panel_empty_message(topo, base, SEM_RELATORIO);
    }
    let mut i: i16 = 0;
    loop {
        let entrada = if i >= 0 && i < lances(relatorio) { relatorio + 0x720 + i as usize * 14 } else { 0 };
        let codigo = core::ptr::read_unaligned((dados + 0x4f) as *const u16);
        let pular = codigo == 0x1ef
            || (codigo == 0x3de && *((entrada + 4) as *const u8) >= 0x2d)
            || (codigo == 0x483 && ultimo_minuto >= 0x5a)
            || (codigo == 0x528 && ultimo_minuto >= 0x69);
        if !pular {
            let mut primeira: i32 = -1;
            let mut j: u32 = 0;
            loop {
                let id = if partida != 0 { *(partida as *const u32) } else { u32::MAX };
                let texto_do_lance: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x6d8d10usize);
                let t = texto_do_lance(relatorio, entrada, dados, id, j);
                j += 1;
                if t == 0 {
                    break;
                }
                if linhas == 0 {
                    let (b, a) = (*base, *topo);
                    let altura: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x5e5cd0usize);
                    let n = ((b - a) / altura(2)) as u8;
                    // ponytail: os 3 bytes de cima das linhas são lixo da pilha na original; aqui 0
                    tabela = lista(current_human_display(), 0x6e, a as usize, 0x30c, b as usize, 2, colunas.as_mut_ptr() as usize, n as usize, 0, 1, 0, usize::MAX);
                }
                let cor_rgb: unsafe extern "C" fn(u32, u32, u32, u32) -> usize = core::mem::transmute(0x5e4800usize);
                let k = *(entrada as *const i16) as i32;
                let palavra = |a: usize| (cor & 0xffff_0000) | colour_word(a);
                cor = match k {
                    0x1f56 | 0x1f57 | 0x1fce | 0x2008 => cor_rgb(0, 0xc0, 0, 0),
                    0x1fcc | 0x1fcf | 0x2000 | 0x2013..=0x201b | 0x2114 => cor_rgb(0, 0x96, 0xff, 0),
                    0x1fcd => palavra(0xaebdd4),
                    0x1ff5..=0x1ff8 => palavra(0xae31a8),
                    0x1ff9 => palavra(0xaebdec),
                    _ => palavra(0xae3154),
                };
                if primeira == -1 {
                    primeira = linhas;
                }
                formatar(BUF, 0x98ed14, t);
                texto_da_celula(sm, tabela, 1, linhas as usize, 2, 0, 1, 2, cor, BUF);
                linhas += 1;
            }
            if primeira != -1 {
                let minuto = *((entrada + 4) as *const u8);
                let mut k = primeira;
                if ultimo_minuto != minuto {
                    ultimo_minuto = minuto;
                    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
                    if idioma() != 2 {
                        texto(BUF, 0xa168ec, minuto as u32 + 1);
                    } else {
                        let por_extenso: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x90c310usize);
                        formatar(BUF, 0xa168e4, por_extenso(minuto.wrapping_add(1) as u32, 2));
                    }
                    texto_da_celula(sm, tabela, 0, k as usize, 2, 0, 0xc, 2, colour_word(0xaebdec), BUF);
                    k += 1;
                }
                while k < linhas {
                    texto_da_celula(sm, tabela, 0, k as usize, 2, 0, 0xc, 2, colour_word(0xae3184), 0xad9c64);
                    k += 1;
                }
            }
        }
        i = i.wrapping_add(1);
        if i >= lances(relatorio) {
            break;
        }
    }
    if linhas == 0 {
        return show_panel_empty_message(topo, base, SEM_RELATORIO);
    }
    while linhas < 0x14 {
        let r = texto_da_celula(sm, tabela, 0, linhas as usize, 2, 0, 0xc, 3, colour_word(0xae3184), 0xad9c64);
        texto_da_celula(sm, tabela, 1, linhas as usize, 2, 0, 1, 2, colour_with_high_bits(r, 0xae3184), 0xad9c64);
        linhas += 1;
    }
}

/// { 0x717db0 } (cdecl, match_screens.cpp): monta a tela dos jogos do dia do grupo do parâmetro 1 (+4 a
/// +8 as partidas do dia em [[0xb67a50]+0x19], 105 bytes cada): o fundo e as cores da competição
/// (0x41c410, 0x53b7c0), o título com o nome (Friendlies/Reserves com mais de uma partida) e o selo
/// (fora dos amistosos e das seleções), a barra de menu (0x75db00 com 4), o título "Fixtures"/"Results"
/// (singular ou plural; o dia pela data 0x54d040/0x54d130), com mais de uma rodada ([+0x15] > 1) a
/// árvore dos grupos de competições do dia (cabeçalho do grupo e uma linha por competição), o selo
/// "Competition", a tabela de 9 colunas com o cabeçalho de cada rodada nova e uma linha por partida
/// (0x49db00), os botões de voltar e avançar, a rolagem até a partida do humano (parâmetro 4) ou à do
/// parâmetro 2 e a marca de vista do humano da vez no grupo (+0x14 + 4h).
unsafe extern "C" fn build_day_fixtures_screen() {
    crate::comparison::branch("screens build the day matches");
    let caixa: ManagerBoxFn = core::mem::transmute(0x804030usize);
    let texto_da_celula: ListCellTextFn = core::mem::transmute(0x804100usize);
    let lista: DisplayListFn = core::mem::transmute(0x55fba0usize);
    let linha: DisplayLineFn = core::mem::transmute(0x55f990usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let selecao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7e6f20usize);
    let sem_selo = |c: usize| {
        let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c2990usize);
        amistoso(*(0xae2c74 as *const usize), c) == 0 && selecao(*(0xb74434 as *const usize), c) == 0
    };
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let rd32 = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;

    let mut colunas: [u8; 9] = [3, 2, 0x12, 1, 4, 1, 0x12, 2, 3];
    let grupo = get_screen_param(1);
    let comp = (*(0xae23d0 as *const usize) as isize + rd32(grupo) as isize * 107) as usize;
    let fundo: unsafe extern "C" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x41c410usize);
    fundo(comp, 0, 0);
    // as cores da competição (0x53b7c0 grava u16; o resto da variável da original é lixo da pilha, aqui 0)
    let (mut ca, mut cb) = (0u32, 0u32);
    let cores: unsafe extern "C" fn(usize, *mut u32, *mut u32) = core::mem::transmute(0x53b7c0usize);
    cores(comp, &mut ca, &mut cb);
    let mais_de_uma = rd32(grupo + 8) > rd32(grupo + 4);
    let chave = *(comp as *const usize);
    if chave == *(0x9cf668 as *const usize) && mais_de_uma {
        texto(BUF, 0xa1631c);
    } else if chave == *(0x9cf684 as *const usize) && mais_de_uma {
        texto(BUF, 0xa16310);
    } else {
        copy_to_work_text(comp + 4);
    }
    let titulo: unsafe extern "thiscall" fn(usize, u32, u32, usize) = core::mem::transmute(0x803f30usize);
    titulo(sm, cb, ca, BUF);
    if sem_selo(comp) {
        linha(current_human_display(), 6, 0x7d, 0xa, 0x316, 0x46, 0, 0, 1, 0, 0, 0xc, 7, 0, 0, 0, 0x7d0, comp, usize::MAX);
    }
    let menu: unsafe extern "C" fn(u32, u32) = core::mem::transmute(0x75db00usize);
    menu(4, 0);

    // o título do dia; o byte da data vai inteiro (as rotinas só leem o byte)
    let md = rd(0xb67a50);
    let ini = rd32(grupo + 4);
    let jogado = *((rd(rd(md + 0x19) + ini as usize * 105) + 0x43) as *const u8) != 0xff;
    let formato = match (jogado, rd32(grupo + 8) > ini) {
        (false, true) => 0xa162f0usize,
        (false, false) => 0xa162d0,
        (true, true) => 0xa162b0,
        (true, false) => 0xa16290,
    };
    let dia_a: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x54d040usize);
    let dia_b: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x54d130usize);
    let r1 = dia_a(*(rd(0xb67a50) as *const u8) as u32, 1);
    let r2 = dia_b(*(rd(0xb67a50) as *const u8) as u32) as u8 as i8 as i32;
    let r3 = dia_b(*(rd(0xb67a50) as *const u8) as u32) as u8 as i8 as i32;
    texto(BUF, formato, r3, r2, r1);
    let titulo_do_dia: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x803f80usize);
    titulo_do_dia(sm, BUF, 1);

    // os 3 bytes de cima do número de linhas da tabela: o contador dos grupos que a original deixou na pilha
    let mut resto: usize = 0;
    if rd32(rd(0xb67a50) + 0x15) > 1 {
        let cabeca = caixa(sm, 0x69, 0xf, 0x78, 0x23, 0x30, cb as usize, 0xc, 1, ca as usize, 0, 0, 0);
        let raiz = lista(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_word(0xae31aa), cabeca);
        let mut i: i32 = 0;
        loop {
            let md = rd(0xb67a50);
            let n = rd32(md + 9);
            if i >= n {
                break;
            }
            let e = rd(md + 1) + i as usize * 0x18;
            let pai = if n == 1 {
                raiz
            } else {
                let tipo = *(e as *const i32);
                match tipo {
                    0 => {
                        texto(BUF, 0xa16284);
                    }
                    1 => {
                        texto(BUF, 0xa16248, 0xad9c64);
                    }
                    2 => {
                        texto(BUF, 0xa16210, 0xad9c64);
                    }
                    3..=7 => {
                        let mut nome = [0u8; 0x64];
                        let continente: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(0x6177f0usize);
                        continente(*((0xb63d64 + tipo as isize * 72) as *const usize), nome.as_mut_ptr(), 0x64, 6);
                        copy_to_work_text(nome.as_ptr() as usize);
                    }
                    _ => copy_to_work_text(*((0xb63d60 + tipo as isize * 72) as *const usize) + 4),
                }
                if *((e + 0x14) as *const u32) != 0 {
                    let r = linha(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, colour_with_high_bits(raiz, 0xae31aa), colour_word(0xaebdea), 1, 1, colour_word(0xae3184), BUF, 0, 0, 0, raiz);
                    lista(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_word(0xae31aa), r)
                } else {
                    let r = linha(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, colour_with_high_bits(raiz, 0xae31aa), colour_word(0xaebdec), 1, 1, colour_word(0xaebdea), BUF, 0, 0, 0, raiz);
                    lista(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_with_high_bits(r, 0xae31aa), r)
                }
            };
            let mut k = rd32(e + 4);
            while k <= rd32(e + 8) {
                let m = (rd(rd(0xb67a50) + 0xd) as isize + k as isize * 0x54) as usize;
                let c = (*(0xae23d0 as *const usize) as isize + *(m as *const i32) as isize * 107) as usize;
                panel_competition_name(c, m);
                let v = *((m + 0x10) as *const u32) as usize;
                if v != 0 {
                    linha(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, colour_word(0xae31aa), colour_word(0xaebdea), 1, 1, colour_with_high_bits(v, 0xae3184), BUF, 0, 1, m, pai);
                } else {
                    linha(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, colour_word(0xae31aa), colour_word(0xaebdec), 1, 1, colour_word(0xaebdea), BUF, 0, 1, m, pai);
                }
                k += 1;
            }
            i += 1;
        }
        resto = i as usize;
    }

    let mut esquerda = 0x91usize;
    if sem_selo(comp) {
        let r = texto(BUF, 0xa16204) as u32 as usize;
        caixa(sm, 0x28f, 0x91, 0x30c, 0xa5, 0x30, colour_with_high_bits(r, 0xae317e), 0xc, 1, colour_word(0xae3184), BUF, 0x7d0, comp);
        esquerda = 0xaa;
    }
    let altura: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x5e5cd0usize);
    let n = ((0x207 - esquerda as i32) / altura(3)) as u8;
    let tabela = lista(current_human_display(), 0x6e, esquerda, 0x30c, 0x217, 9, colunas.as_mut_ptr() as usize, (resto & !0xff) | n as usize, 0, 2, 0, usize::MAX);

    let mut k: i32 = 0;
    let mut alvo: i32 = -1;
    let mut anterior: usize = 0;
    let humano_manda: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x600c90usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let mut j = rd32(grupo + 4);
    while j <= rd32(grupo + 8) {
        let partidas = rd(rd(0xb67a50) + 0x19);
        let fx = rd(partidas + j as usize * 105);
        let w30 = *((fx + 0x30) as *const i16);
        let w32 = *((fx + 0x32) as *const i16);
        let mesma_rodada = anterior != 0 && w32 == *((anterior + 0x32) as *const i16) && w30 == *((anterior + 0x30) as *const i16);
        if !mesma_rodada && !(w32 == 0 && w30 == 0) {
            if anterior != 0 {
                // a original empurra edx = o ponteiro das partidas com a cor em dx
                texto_da_celula(sm, tabela, 0, k as usize, 1, 0, 1, 1, colour_with_high_bits(partidas, 0xae3184), 0);
                k += 1;
            }
            let rodada: unsafe extern "C" fn(i32, i32, usize, u32, u32) = core::mem::transmute(0x4bffc0usize);
            rodada(w32 as i32, w30 as i32, BUF, 0x7d0, 0);
            let largura_do_texto: unsafe extern "C" fn(usize, *const u8) -> i32 = core::mem::transmute(0x5e5ad0usize);
            let mut largura = largura_do_texto(3, BUF as *const u8) + 0x73;
            if largura >= 0x2f1 {
                largura = 0x2f1;
            }
            let largura = largura as usize;
            if selecao(*(0xb74434 as *const usize), comp) == 0 {
                let base_id = *(rd(fx + 0x14) as *const u32);
                let id = (((base_id << 11) | (w32 as u32 & 0x7ff)) << 11) | (w30 as u32 & 0x7ff);
                linha(current_human_display(), 2, 0x73, 0, largura, 0, 0, k as usize, 1, 0, 0, 1, 3, colour_with_high_bits(id as usize, 0xaebdec), BUF, 0, 0x7e5, id as usize, tabela);
            } else {
                linha(current_human_display(), 1, 0x73, 0, largura, 0, 0, k as usize, 1, 0, 0, 1, 3, colour_word(0xaebdec), BUF, 0, 0, 0, tabela);
            }
            linha(current_human_display(), 1, 0x73, 0, largura, 0, 0, (k + 1) as usize, 0xc400, colour_word(0xaebdec), 0, 0, 0, 0, 0, 0, 0, 0, tabela);
            k += 2;
        }
        let (c1, c2) = (rd(fx + 0x1c), rd(fx + 0x20));
        if humano_manda(c1, 0, 1) != 0 || humano_manda(c2, 0, 1) != 0 {
            alvo = k;
        } else if alvo == -1 && (humano(c1, 0, 1, 0, 0) != 0 || humano(c2, 0, 1, 0, 0) != 0) {
            alvo = k;
        }
        let partida_na_tabela: unsafe extern "C" fn(usize, usize, *mut i32, u32) = core::mem::transmute(0x49db00usize);
        partida_na_tabela(fx, tabela, &mut k, 2);
        anterior = fx;
        j += 1;
    }

    let sem_avancar: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x7febb0usize);
    let sem_voltar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x7fec60usize);
    let avancar = (sem_avancar(sm) == 0) as u32;
    let voltar = (sem_voltar(sm, 0) == 0) as u32;
    let botoes: unsafe extern "C" fn(u32, u32) -> u32 = core::mem::transmute(0x5edcc0usize);
    botoes(voltar, avancar);

    let item = tabela as u16 as i16;
    let rolar = |pos: usize| {
        if item != -1 {
            let ir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x403320usize);
            ir((current_human_display() as isize + item as isize * 3057 + 4) as usize, pos);
        }
    };
    if get_screen_param(4) != 0 {
        if alvo > -1 {
            let p = alvo - (n >> 1) as i32;
            rolar(if p <= 0 { 0 } else { p as usize });
        } else {
            rolar(get_screen_param(2));
        }
        set_screen_param(3, item as i32 as usize);
        set_screen_param(4, 0);
    } else {
        rolar(get_screen_param(2));
        set_screen_param(3, item as i32 as usize);
    }
    *((grupo as isize + 0x14 + *(0xb77c7a as *const i16) as isize * 4) as *mut u32) = 1;
}

/// Como `compare_painel_pela_lista`, com mais regiões (o que a rotina grava fora da tela).
unsafe fn compare_pela_lista_com(nome: &str, extra: &[(usize, usize)], original: impl FnOnce(), rust: impl FnOnce()) {
    let mut reg = screen_build_regions();
    reg.extend(screen_param_regions());
    reg.push((0xde1f64, 0x100));
    reg.push((0xae290c, 4));
    reg.extend_from_slice(extra);
    // a foto de fundo (0x41c410) sorteia: o estado do gerador volta para a segunda rodada (sem ele o
    // gerador andava duas vezes no modo comparação e a carreira se deslocava; 204ª)
    reg.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, original, || crate::rng::without_counting(rust));
    SCOREBOARD.record(igual, || {
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("{}: diferencas em {:?}", nome, dif)
    });
}

/// As zonas de ação são comparadas de verdade, com o `*topo` (+0x19), a cor fixa 0xb6b7a4 e a guarda
/// 0xb6b7b4 e a foto da partida que os valores (0x720320) comparam e regravam (parâmetro 0x1d, 0xd7a
/// bytes). Sem a foto ainda (o primeiro desenho), os valores a alocam: só a original roda.
unsafe extern "C" fn compare_draw_action_zones_panel(topo: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(%%I0%%);
    let foto = get_screen_param(0x1d);
    if foto == 0 {
        // ponytail: a alocação não roda duas vezes; esse desenho se prova no modo Rust
        return o(topo, base);
    }
    let extra = [(topo as usize, 4), (0xb6b7a4, 2), (0xb6b7b4, 1), (foto, 0xd7a)];
    compare_pela_lista_com("draw_action_zones_panel", &extra, || o(topo, base), || draw_action_zones_panel(topo, base));
}

/// O relatório é comparado de verdade quando já existe (na partida ou no parâmetro 0x2d); quando nasce
/// aqui (alocação e construção) só a original roda.
unsafe extern "C" fn compare_draw_match_report_panel(topo: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(%%I1%%);
    if get_screen_param(0) == 0 && *((get_screen_param(0x20) + 0x4c) as *const u8) == 0 && get_screen_param(0x2d) == 0 {
        // ponytail: a alocação do relatório não roda duas vezes; esse caminho se prova no modo Rust
        return o(topo, base);
    }
    compare_painel_pela_lista("draw_match_report_panel", || o(topo, base), || draw_match_report_panel(topo, base));
}

/// A montagem é comparada de verdade, com a marca de vista do humano no grupo (+0x14 + 4h).
unsafe extern "C" fn compare_build_day_fixtures_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I2%%);
    let marca = (get_screen_param(1) as isize + 0x14 + *(0xb77c7a as *const i16) as isize * 4) as usize;
    compare_pela_lista_com("build_day_fixtures_screen", &[(marca, 4)], || o(), || build_day_fixtures_screen());
}
