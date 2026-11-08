// ---------- %%LEVA%%a leva: as telas da partida (13): a barra da posse, o painel da visão geral da partida e a tabela da liga ao vivo (match_screens.cpp) ----------

/// { 0x71d9e0 } (cdecl, match_screens.cpp): na partida (parâmetro 0), sem disputa de pênaltis
/// ([estado + 0x53] = 0), pega as cores dos dois clubes (0x53b640 com os uniformes +0x152 e +0x777) e,
/// se a posse (+0x142) mudou desde a cópia do parâmetro 0x1d (ou sem cópia), redesenha a barra da posse
/// da visão geral: o percentual (50 sem valor ou sem a marca +0x767) em "%d%%" nos itens 3 e 4 a partir
/// do parâmetro 0x21, repartidos de 0x115 a 0x2c9 na altura do parâmetro 0x22, e a cor do clube que tem
/// mais posse no item 5. No fim guarda o estado na cópia (que nasce aqui se não existe).
unsafe extern "C" fn refresh_possession_bar() {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let texto: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x801c70usize);
    let retangulo: unsafe extern "thiscall" fn(usize, u32, i32, i32, i32, i32) = core::mem::transmute(0x800890usize);
    let fundo: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x801040usize);
    let cores_do_clube: unsafe extern "C" fn(usize, u32, *mut u32, *mut u32) = core::mem::transmute(0x53b640usize);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;

    let p0 = get_screen_param(0);
    let copia = get_screen_param(0x1d);
    let item = get_screen_param(0x21) as u32;
    let altura = get_screen_param(0x22) as i32;
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
            formatar(BUF, FORMAT_PCT, p);
            let x = p * 0x1b4 / 100 + 0x115;
            retangulo(sm, item.wrapping_add(3), 0x115, altura - 0x23, x, altura - 0x14);
            texto(sm, item.wrapping_add(3), BUF);
            formatar(BUF, FORMAT_PCT, 100 - p);
            retangulo(sm, item.wrapping_add(4), x, altura - 0x23, 0x2c9, altura - 0x14);
            texto(sm, item.wrapping_add(4), BUF);
            fundo(sm, item.wrapping_add(5), if p < 0x32 { c2 } else { c1 });
        }
    }
    save_match_snapshot(copia, e);
}

/// A linha (0x49 bytes) do jogador `idx` do lado `lado` no estado da partida `e` (+0x11f + 0x71).
fn match_player_row(e: usize, lado: u8, idx: u8) -> usize {
    (e as isize + lado as i8 as isize * 0x625 + idx as i8 as isize * 0x49) as usize
}

/// A visão geral avisa erro (caixa "Error" de match_screens.cpp e segue)? Na partida, com o período
/// [estado + 0x4f] fora dos seis conhecidos; ou com um lance (até 50, até o lado 0xff) de tipo fora de
/// 1 a 8. Nesses casos a cópia e o comparador rodam a original.
unsafe fn overview_would_error() -> bool {
    let p0 = get_screen_param(0);
    let e = if p0 != 0 { core::ptr::read_unaligned((p0 + 0x5f) as *const usize) } else { get_screen_param(0x20) };
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
unsafe extern "C" fn draw_match_overview_panel(topo: *mut i32, base: *mut i32) {
    if overview_would_error() {
        return original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(%%I1%%)(topo, base);
    }
    let txt: ManagerTextFn = core::mem::transmute(0x803fd0usize);
    let caixa: ManagerBoxFn = core::mem::transmute(0x804030usize);
    let lista: DisplayListFn = core::mem::transmute(0x55fba0usize);
    let linha: DisplayLineFn = core::mem::transmute(0x55f990usize);
    let celula: ListCellFn = core::mem::transmute(0x8040a0usize);
    let texto_da_celula: ListCellTextFn = core::mem::transmute(0x804100usize);
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
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;
    const MAX: usize = usize::MAX;
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let b8 = |a: usize| *(a as *const u8);
    let i8_ = |a: usize| *(a as *const i8) as i32;
    let z = |v: i32| v as usize;
    let item16 = |r: usize| r as u16 as i16 as i32 as usize;

    let mut colunas3: [u8; 4] = [1, 1, 1, 0];
    let mut colunas4: [u8; 4] = [2, 1, 2, 1];
    let p0 = get_screen_param(0);
    let periodo = get_screen_param(1) as u8;
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
            0 => show_error_at(0xa161d4, if periodo == 4 { 0x1d49 } else { 0x1d5e }),
            0xa167b8 => {
                texto(BUF, rotulo, EMPTY_TEXT);
            }
            _ => {
                texto(BUF, rotulo);
            }
        }
        let t = *topo;
        txt(sm, 0x6e, z(t), 0x30c, z(t + 0x23), 2, 0, 0xc, 3, colour_word(0xaebdec), BUF, 1);
        *topo += 0x28;
        part = p0 + 4;
        if core::ptr::read_unaligned((e + 0x53) as *const u32) == 0 {
            // a barra da posse
            let b = *base;
            lista(current_human_display(), 0x6e, z(b - 0x36), 0x30c, z(b), 1, 0, 1, 0, 2, 0, MAX);
            // 0x53b640 grava u16; o resto da variável da original é lixo da pilha (só a palavra vale)
            let clube = |k: usize| (*(0xae23b4 as *const usize)).wrapping_add(u(e + k).wrapping_mul(0x245));
            let (mut c68, mut c58, mut c5c, mut c64) = (0u32, 0u32, 0u32, 0u32);
            cores_do_clube(clube(0), b8(e + 0x152) as u32, &mut c68, &mut c58);
            cores_do_clube(clube(4), b8(e + 0x777) as u32, &mut c5c, &mut c64);
            let b = *base;
            let r = txt(sm, 0x96, z(b - 0x2d), 0x2e4, z(b - 0xa), 0x30, colour_word(0xae317e), 0xc, 1, colour_word(0xae3184), 0, 1);
            set_screen_param(0x21, item16(r));
            texto(BUF, 0xa16738);
            let b = *base;
            txt(sm, 0xaf, z(b - 0x2d), 0xfa, z(b - 0xa), 1, 0, 0xc, 1, colour_word(0xae3184), BUF, 1);
            let b = *base;
            txt(sm, 0x113, z(b - 0x25), 0x2cb, z(b - 0x12), 0x60, colour_word(0xae317e), 0xc, 1, colour_with_high_bits(z(b), 0xae3184), 0, 1);
            txt(sm, 0x115, z(b - 0x23), 0x1ef, z(b - 0x14), 0x10, c58 as usize, 0xc, 1, c68 as usize, 0, 1);
            let r = txt(sm, 0x1ef, z(b - 0x23), 0x2c9, z(b - 0x14), 0x10, c64 as usize, 0xc, 1, c5c as usize, 0, 1);
            linha(current_human_display(), 1, 0x1ef, z(b - 0x23), 0x1ef, z(b - 0x20), 0, 0, 0x400, colour_with_high_bits(r, 0xaebdea), 0, 0, 0, 0, 0, 0, 0, 0, MAX);
            set_screen_param(0x22, z(*base));
            *base -= 0x37;
            // a caixa do comentário
            let t = *topo;
            let y = (*base - t - 0x46) / 2 + t;
            if get_screen_param(7) != 0 {
                crate::comparison::branch("screens overview with the commentator");
                let rel = u(u(p0 + 0x57) + 0x8f7a);
                let r = txt(sm, 0x8c, z(y), 0x2ee, z(y + 0x46), 1, 0, 0x10c, 5, 0, 0, 1);
                let m = u(p0 + 0x57);
                narrar(rel, u(u(p0 + 0x5b) + 1), e, u(u(m + 0x4792)), r, 0x262, 1, 5, 0);
                set_screen_param(8, item16(r));
            } else {
                let r = txt(sm, 0x8c, z(y), 0x2ee, z(y + 0x46), 1, 0, 0x10c, 5, 0, 0, 1);
                set_screen_param(8, item16(r));
            }
        }
    } else {
        let g = get_screen_param(0x1f);
        let st = u(g + 0x18);
        if st != 0 {
            let cidade = u(st + 0x38);
            if cidade != 0 {
                let (a, c) = (i8_(st + 0x37), i8_(cidade + 0x1e));
                texto(BUF, 0x9902a4, a, a, c, c, st + 4, cidade + 4);
            } else {
                copy_to_work_text(st + 4);
            }
        } else {
            texto(BUF, 0x99c1a4);
        }
        let t = *topo;
        txt(sm, 0x6e, z(t), 0x30c, z(t + 0x23), 2, 0, 0xc, 3, colour_word(0xaebdec), BUF, 1);
        *topo += 0x28;
        e = get_screen_param(0x20);
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
    txt(sm, 0x6e, z(t), z(w + 0x6e), z(t + 0x14), 2, 0, 0xc, 1, colour_word(0xaebde8), BUF, 1);
    placar(part, 3, BUF, 0x7d0);
    let mut aux = [0u8; 0x200];
    let pa = aux.as_mut_ptr() as usize;
    if i8_(part + 0x45) >= 0 {
        texto(pa, 0xa16714, i8_(part + 0x45), i8_(part + 0x46));
        append_to_text_buffer(aux.as_ptr());
        if i8_(part + 0x47) >= 0 {
            texto(pa, 0xa166d4, i8_(part + 0x43), i8_(part + 0x44), i8_(part + 0x47), i8_(part + 0x48));
            append_to_text_buffer(aux.as_ptr());
        }
        let pens = if b8(part + 0x38) & 0x20 != 0 { 0xa166b0 } else { 0xa1668c };
        if core::ptr::read_unaligned((e + 0x53) as *const u32) != 0 {
            texto(pa, pens, b8(e + 0x120) as u32, b8(e + 0x745) as u32);
            append_to_text_buffer(aux.as_ptr());
        } else if i8_(part + 0x49) >= 0 {
            texto(pa, pens, i8_(part + 0x49), i8_(part + 0x4a));
            append_to_text_buffer(aux.as_ptr());
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
        caixa(sm, z(0x30c - x), z(t), 0x30c, z(t + 0x14), 2, 0, 0xc, 1, colour_with_high_bits(chave as usize, 0xaebde8), BUF, 0x7e5, chave as usize);
    } else {
        let t = *topo;
        txt(sm, z(0x30c - x), z(t), 0x30c, z(t + 0x14), 2, 0, 0xc, 1, colour_word(0xaebde8), BUF, 1);
    }

    // os autores: duas colunas (casa 0, fora 2), uma linha por jogador e tipo com os minutos ao lado
    *topo += 0x19;
    let (ini, fim) = (*topo, *base);
    let a = altura(4);
    let n = ((fim - ini - 0x10) / a) as u8;
    let lst = lista(current_human_display(), 0x6e, z(ini), 0x30c, z(fim), 4, colunas4.as_mut_ptr() as usize, n as usize, 0, 2, 0, MAX);
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
        let fila = match_player_row(e, lado, idx);
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
                show_error_at(0xa161d4, 0xc04);
                continue;
            }
        }
        let pid = celula_do_nome(fila, col, cor, lado, f198, f19c, f1a0, linhas[lado_da_linha]);
        let minuto = |ev: usize| {
            let m = b8(ev + 3) as u32 + 1;
            if b8(ev + 2) == 3 {
                texto(BUF, 0xa165fc, m);
            } else {
                formatar(BUF, FORMAT_D, m);
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
                if core::ptr::read_unaligned((match_player_row(e, lj, b8(evj + 1)) + 0x190) as *const i32) != pid {
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
                    append_to_text_buffer(0xa165cc as *const u8);
                    let l = linhas[lado_da_linha];
                    texto_da_celula(sm, lst, col as usize + 1, z(l), 1, 0, 1, 4, colour_word(0xaebdec), BUF);
                    linhas[lado_da_linha] = l + 1;
                    minuto(evj);
                } else {
                    copy_to_work_text(pj);
                }
                marcas[j] = 1;
            }
        }
        let l = linhas[lado_da_linha];
        texto_da_celula(sm, lst, col as usize + 1, z(l), 1, 0, 1, 4, colour_word(0xaebdec), BUF);
        linhas[lado_da_linha] = l + 1;
        marcas[i - 1] = 1;
    }

    // a disputa de pênaltis
    if core::ptr::read_unaligned((e + 0x53) as *const u32) != 0 {
        let m = if linhas[0] > linhas[1] { linhas[0] } else { linhas[1] };
        linhas[lado_da_linha] = m;
        texto_da_celula(sm, lst, 0, z(m), 1, 0, 1, 3, colour_word(0xaebdec), EMPTY_TEXT);
        let r = m + 1;
        linhas[lado_da_linha] = r;
        texto(BUF, if b8(part + 0x38) & 0x20 != 0 { 0xa165b8 } else { 0xa165a4 });
        let mut w = largura_do_texto(3, BUF) + 0x73;
        if w >= 0x2f1 {
            w = 0x2f1;
        }
        linhas[lado_da_linha] = r + 1;
        // a original empurra edx = o ponteiro da linha com a cor em dx; 0x55f990 só lê a palavra
        linha(current_human_display(), 1, 0x73, 0, z(w), 0, 0, z(r), 1, 0, 0, 1, 3, colour_word(0xaebdec), BUF, 0, 0, 0, lst);
        linha(current_human_display(), 1, 0x73, 0, z(w), 0, 0, z(r + 1), 0xc400, colour_word(0xaebdec), 0, 0, 0, 0, 0, 0, 0, 0, lst);
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
            let fila = match_player_row(e, lado, idx);
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
    lista(current_human_display(), 0x6e, z(base23), 0x30c, z(base0), 1, 0, 1, 0, 2, 0, MAX);
    let l2 = lista(current_human_display(), 0x78, z(base23), 0x302, z(base0), 3, colunas3.as_mut_ptr() as usize, 1, 0, 1, 0, MAX);
    let arbitro = core::ptr::read_unaligned((part + 0x2e) as *const i16);
    if arbitro > -1 {
        nome_do_arbitro((*(0xae23e8 as *const usize)).wrapping_add(arbitro as usize * 0x2b), aux.as_mut_ptr(), 0xc8, 1);
        texto(BUF, 0xa16550, aux.as_ptr());
    } else {
        texto(BUF, 0xa1653c);
    }
    texto_da_celula(sm, l2, 0, 0, 1, 0, 1, 2, colour_word(0xaebdec), BUF);
    let publico = core::ptr::read_unaligned((e + 0x34) as *const u32);
    if publico != 0 {
        let w = core::ptr::read_unaligned((e + 0x4f) as *const i16);
        if w >= 0x3de || w == 0 || w == 1 {
            texto(BUF, 0xa1651c, publico);
            texto_da_celula(sm, l2, 1, 0, 1, 0, 0xc, 2, colour_word(0xaebde8), BUF);
        }
    }
    clima(*(0xde2f74 as *const usize), pa, e + 0xd76);
    texto(BUF, 0xa164f8, aux.as_ptr());
    texto_da_celula(sm, l2, 2, 0, 1, 0, 0x40, 2, colour_word(0xaebdec), BUF);
    if b8(e + 0x11f) == 0xfd {
        let t = *topo;
        let y = (*base - t - 0x46) / 2 + t;
        texto(BUF, 0xa164e8);
        txt(sm, 0x8c, z(y), 0x2ee, z(y + 0x46), 0x30, colour_word(0xaebdea), 0xc, 3, colour_word(0xae3184), BUF, 1);
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
unsafe extern "C" fn match_screens_draw_live_league_table(liga: usize, topo: i32, base: i32) {
    if liga == 0 || get_screen_param(0xe) == 0 {
        return original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I2%%)(liga, topo, base);
    }
    let txt: ManagerTextFn = core::mem::transmute(0x803fd0usize);
    let lista: DisplayListFn = core::mem::transmute(0x55fba0usize);
    let linha: DisplayLineFn = core::mem::transmute(0x55f990usize);
    let celula: ListCellFn = core::mem::transmute(0x8040a0usize);
    let texto_da_celula: ListCellTextFn = core::mem::transmute(0x804100usize);
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
    let sm = SCREEN_MANAGER;
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
    let p0 = get_screen_param(0);
    let pe = get_screen_param(0xe);
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
    get_screen_param(0x27);
    let coluna = get_screen_param(0x28);
    get_screen_param(0x29);

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
                txt(sm, 0x6e, z(topo), 0x30c, z(topo + 0x23), 2, 0, 0xc, 3, colour_word(0xaebdec), BUF, 1);
                y = topo + 0x28;
            }
            let a = altura(3);
            let n = ((base - y - 0x10) / a) as u8;
            data_texto(s + 0x30, s + 0x3d0, 0x7d0, 0xff);
            let d = dia_da_semana(s + 0x30);
            let g = genero_do_dia(idioma(), d);
            texto(BUF, 0x99c2e8, g as u8 as i8 as i32, s + 0x3d0);
            mensagem(0x6e, z(y), 0x30c, z(base), 2, 0, n as usize, 3, colour_word(0xae3184), BUF, 0);
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
    txt(sm, 0x6e, z(topo), 0x30c, z(topo + 0x23), 2, 0, 0xc, 3, colour_word(0xaebdec), titulo, 1);
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
    let lst = lista(current_human_display(), 0x6e, z(y), 0x30c, z(alto + y), 0xb, s + 0x1c, n as usize, 0, 2, 0, MAX);
    let cabecalho = |col: usize| {
        texto_da_celula(sm, lst, col, 0, 0x30, colour_word(0xae317e), 0xc, 1, colour_word(0xae3184), BUF);
    };
    texto(BUF, 0x990b3c);
    cabecalho(4);
    texto(BUF, 0x990b1c, EMPTY_TEXT);
    cabecalho(5);
    let regra = w16(liga + 0xab) as u8;
    texto(BUF, if regra & 0x20 != 0 { 0x990b18 } else if regra & 1 != 0 { 0x99c24c } else { 0x990b14 });
    cabecalho(6);
    texto(BUF, 0x990af4, EMPTY_TEXT);
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
            linha(current_human_display(), 1, 0x70, 0, x2, 0, 0, fila, 0x6400, u(s + 0x14), 0, 0, 0, 0, 0, 0, 0, 0, lst);
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
            celula(sm, lst, 0, fila, 0x30, colour_with_high_bits(pos, 0xaebe04), 0xc, 1, colour_word(0xae3184), pos, 7, r)
        } else {
            texto_da_celula(sm, lst, 0, fila, 0x30, colour_with_high_bits(pos, 0xaebe04), 0xc, 1, colour_word(0xae3184), pos)
        };
        let x = u(s + 0x38);
        *(BUF as *mut u8) = 0;
        let (agora, antes) = (b8(r + 4) as i8, b8(x + 4) as i8);
        if agora < antes {
            linha(current_human_display(), 0x20001, 0, 0, 0, 0, 1, fila, 1, colour_with_high_bits(ret, 0xaebdcc), 0, 0xc, 0, colour_word(0xaebdec), 0, 0, 0, 0, lst);
        } else if agora > antes {
            linha(current_human_display(), 0x40001, 0, 0, 0, 0, 1, fila, 1, colour_with_high_bits(ret, 0xaebdcc), 0, 0xc, 0, colour_word(0xae31a8), 0, 0, 0, 0, lst);
        }
        let cor = if humano(clube, 0, 1, 0, 0) != 0 { colour_word(0xaebdec) } else { colour_word(0xae3184) };
        celula(sm, lst, 2, fila, 1, 0, 1, 3, cor, clube + 0x38, 0x7d5, clube);
        let d = r + coluna.wrapping_mul(14);
        for (o, col) in [(0x11usize, 4usize), (0x12, 5), (0x13, 6), (0x14, 7)] {
            formatar(BUF, FORMAT_D, b8(d + o) as i8 as i32);
            texto_da_celula(sm, lst, col, fila, 1, 0, 0xc, 2, colour_word(0xae3184), BUF);
        }
        for (o, col) in [(0x15usize, 8usize), (0x17, 9)] {
            formatar(BUF, FORMAT_D, core::ptr::read_unaligned((d + o) as *const i16) as i32);
            texto_da_celula(sm, lst, col, fila, 1, 0, 0xc, 2, colour_word(0xae3184), BUF);
        }
        formatar(BUF, FORMAT_D, core::ptr::read_unaligned((d + 0x19) as *const i16) as i32);
        texto_da_celula(sm, lst, 0xa, fila, 0x30, colour_word(0xae3192), 0xc, 1, colour_word(0xae3184), BUF);
        fila += 1;
        i += 1;
    }

    let rolagem = get_screen_param(0x25);
    let disp = current_human_display();
    let id = lst as u16 as i16;
    if id != -1 {
        rolar(disp.wrapping_add((id as isize * 0xbf1) as usize) + 4, rolagem);
    }
    set_screen_param(0x26, id as i32 as usize);
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
unsafe extern "C" fn compare_refresh_possession_bar() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let p0 = get_screen_param(0);
    let copia = get_screen_param(0x1d);
    let mut reg = match screen_update_regions(false) {
        Some(r) if p0 == 0 || copia != 0 => r,
        // ponytail: a alocação da cópia e o pacote pela rede não rodam duas vezes; provados no modo Rust
        _ => return o(),
    };
    reg.extend(screen_param_regions());
    if copia != 0 {
        reg.push((copia, 0xd7a));
    }
    reg.push((0xde1f64, 0x100));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(), || refresh_possession_bar());
    SCOREBOARD.record(igual, || format!("refresh_possession_bar: diferencas em {:?}", first_differences(&mo, &mr)));
}

/// A visão geral é comparada de verdade pela lista de exibição, com `*topo`, `*base` e a barra da posse
/// do fim (o `Display` do gerente e a cópia do estado). Só a original: com o aviso de erro, com o
/// narrador (parâmetro 7: escreve o lance e toca o som), na partida sem a cópia do estado (a barra a
/// aloca) e com a atualização pela rede.
unsafe extern "C" fn compare_draw_match_overview_panel(topo: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(%%I1%%);
    let p0 = get_screen_param(0);
    let foto = get_screen_param(0x1d);
    let narrador = p0 != 0
        && core::ptr::read_unaligned((core::ptr::read_unaligned((p0 + 0x5f) as *const usize) + 0x53) as *const u32) == 0
        && get_screen_param(7) != 0;
    let reg = screen_update_regions(false);
    if overview_would_error() || narrador || (p0 != 0 && foto == 0) || reg.is_none() {
        // ponytail: aviso, som, alocação e rede não rodam duas vezes; esses caminhos se provam no modo Rust
        return o(topo, base);
    }
    let mut reg = reg.unwrap_or_default();
    reg.extend(screen_param_regions());
    reg.push((0xde1f64, 0x100));
    reg.push((topo as usize, 4));
    reg.push((base as usize, 4));
    if foto != 0 {
        reg.push((foto, 0xd7a));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(topo, base), || draw_match_overview_panel(topo, base));
    SCOREBOARD.record(igual, || format!("draw_match_overview_panel: diferencas em {:?}", first_differences(&mo, &mr)));
}

/// A tabela ao vivo é comparada de verdade pela lista de exibição, com a liga (0xee bytes: [+0xad]) e a
/// tabela dela ([+0xb1], 59 bytes por clube), que a rotina reordena. As cópias locais nascem e morrem
/// dentro dela. Sem liga ou sem o parâmetro 0xe (caixa "Error"): só a original.
unsafe extern "C" fn compare_match_screens_draw_live_league_table(liga: usize, topo: i32, base: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I2%%);
    if liga == 0 || get_screen_param(0xe) == 0 {
        return o(liga, topo, base);
    }
    let mut extra = vec![(liga, 0xee)];
    let tabela = core::ptr::read_unaligned((liga + 0xb1) as *const usize);
    let n = core::ptr::read_unaligned((liga + 0x3e) as *const i16);
    if tabela != 0 && n > 0 {
        extra.push((tabela, n as usize * 59));
    }
    compare_pela_lista_com("match_screens_draw_live_league_table", &extra, || o(liga, topo, base), || match_screens_draw_live_league_table(liga, topo, base));
}
