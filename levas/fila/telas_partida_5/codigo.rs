// ---------- %%LEVA%%a leva: as telas da partida (5): o painel dos últimos placares, a tela seguinte e a cor do título (match_screens.cpp, scrman.cpp, Setup.cpp) ----------

/// { 0x7febb0 } (thiscall, scrman.cpp): o gerente de telas não tem para onde avançar? (o bloco de
/// parâmetros da tela atual sem a tela seguinte em +0x1f8). Sem bloco: a original (caixa de erro e saída).
unsafe extern "thiscall" fn is_forward_unavailable(gerente: usize) -> u32 {
    let vaga = *((gerente + 0x382a) as *const i16) as isize;
    let bloco = core::ptr::read_unaligned((gerente as isize + 0x7e2 + vaga * 0x300) as *const usize);
    if bloco == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(gerente);
    }
    (*((bloco + 0x1f8) as *const usize) == 0) as u32
}

/// { 0x839590 } (cdecl, Setup.cpp): a cor do título: [0xae3184] com `x` ≠ 0, senão [0xae31a8]. A
/// original só troca `ax`: os 16 bits de cima da devolução são os de `x`.
unsafe extern "C" fn get_heading_colour(x: u32) -> u32 {
    let c = if x != 0 { *(0xae3184 as *const u16) } else { *(0xae31a8 as *const u16) };
    (x & 0xffff_0000) | c as u32
}

/// Copia o texto C de `origem` (com o zero) para o texto de trabalho 0xde1f64.
unsafe fn copy_to_work_text(origem: usize) {
    let mut i = 0;
    loop {
        let b = *((origem + i) as *const u8);
        *((0xde1f64 + i) as *mut u8) = b;
        if b == 0 {
            break;
        }
        i += 1;
    }
}

/// O nome da competição do registro `c` (107 bytes) no texto 0xde1f64 do jeito do painel: "Friendlies"
/// e "Reserves" para as dos amistosos ([0x9cf668]) e reservas ([0x9cf684]) com mais de uma rodada
/// (+8 > +4 de `rodadas`); senão o nome (+0x38).
unsafe fn panel_competition_name(c: usize, rodadas: usize) {
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let mais_de_uma = core::ptr::read_unaligned((rodadas + 8) as *const i32) > core::ptr::read_unaligned((rodadas + 4) as *const i32);
    let chave = *(c as *const usize);
    if chave == *(0x9cf668 as *const usize) && mais_de_uma {
        texto(0xde1f64, 0xa1631c);
    } else if chave == *(0x9cf684 as *const usize) && mais_de_uma {
        texto(0xde1f64, 0xa16310);
    } else {
        copy_to_work_text(c + 0x38);
    }
}

/// { 0x723d20 } (cdecl, match_screens.cpp): desenha o painel dos últimos placares a partir de `*topo`
/// até `*base`. Título (dentro da partida, parâmetro 0 ≠ 0: "<competição> Latest" à esquerda 0x6e e
/// largura 0x30c, `*topo` += 0x28; na tela avulsa o nome em 0x64/0x316, += 0x41) e o selo da competição
/// (0x804030, fora dos amistosos 0x5c2990 e das seleções 0x7e6f20). Com mais de uma rodada no dia
/// ([[0xb67a50]+0x2a] > 1) a lista dos grupos do dia ([+1], 0x18 bytes, [+9] quantos) com as partidas
/// de cada um ([+0xd], 84 bytes, de +4 a +8 do grupo): cabeçalho do grupo (Copa do Mundo, Internacional,
/// Continental ou o nome da nação) e uma linha por partida (0x55f990). Depois a tabela de 4 colunas
/// com as linhas que cabem (0x5e5cd0, a altura da fonte 3; quantas no parâmetro 0xf, o primeiro item no 0x10),
/// os três botões do alto (0x804030, 6 a 8) e o redesenho dos placares (0x724520).
unsafe extern "C" fn draw_latest_scores_panel(topo: *mut i32, base: *mut i32) {
    crate::comparacao::ramo("telas painel dos ultimos placares");
    type T9 = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    type T11 = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    type T12 = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    type T18 = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    let titulo: T11 = core::mem::transmute(0x803fd0usize);
    let caixa: T12 = core::mem::transmute(0x804030usize);
    let celula: T11 = core::mem::transmute(0x8040a0usize);
    let texto_da_celula: T9 = core::mem::transmute(0x804100usize);
    let linha: T18 = core::mem::transmute(0x55f990usize);
    let lista: T11 = core::mem::transmute(0x55fba0usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let cor = |a: usize| *(a as *const u16) as usize;
    let display = || *((0xb74c4c + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;
    let sem_selo = |c: usize| {
        let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c2990usize);
        let selecao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7e6f20usize);
        amistoso(*(0xae2c74 as *const usize), c) == 0 && selecao(*(0xb74434 as *const usize), c) == 0
    };

    let partida = get_screen_param(0);
    let entrada = get_screen_param(0xe);
    let comp = (*(0xae23d0 as *const usize) as isize + *(entrada as *const i32) as isize * 107) as usize;
    let t = *topo as usize;
    let (esquerda, largura);
    if partida != 0 {
        esquerda = 0x6e;
        largura = 0x30c;
        let mais_de_uma = core::ptr::read_unaligned((entrada + 8) as *const i32) > core::ptr::read_unaligned((entrada + 4) as *const i32);
        let chave = *(comp as *const usize);
        if chave == *(0x9cf668 as *const usize) && mais_de_uma {
            texto(BUF, 0xa16b28, 0xad9c64);
        } else if chave == *(0x9cf684 as *const usize) && mais_de_uma {
            texto(BUF, 0xa16aec, 0xad9c64);
        } else {
            let g = *((comp + 0x52) as *const i8) as i32;
            texto(BUF, 0xa16ac8, g, g, comp + 0x38);
        }
        titulo(sm, 0x6e, t, 0x30c, t + 0x23, 2, 0, 0xc, 3, cor(0xaebdec), BUF, 1);
        if sem_selo(comp) {
            caixa(sm, 0x9b, t, 0x276, t + 0x23, 1, 0, 0xc, 3, cor(0xaebdec), 0, 0x7d0, comp);
        }
        *topo += 0x28;
    } else {
        esquerda = 0x64;
        largura = 0x316;
        panel_competition_name(comp, entrada);
        titulo(sm, 0x64, t, 0x316, t + 0x2d, 2, 0, 0xc, 6, cor(0xaebdec), BUF, 1);
        if sem_selo(comp) {
            caixa(sm, 0x91, t, 0x280, t + 0x2d, 1, 0, 0xc, 6, cor(0xaebdec), 0, 0x7d0, comp);
        }
        *topo += 0x41;
    }

    let md = *(0xb67a50 as *const usize);
    if *((md + 0x2a) as *const i8) > 1 {
        let cabeca = caixa(sm, esquerda + 5, t + 5, esquerda + 0x14, t + 0x19, 0x30, cor(0xaebdea), 0xc, 1, cor(0xae3184), 0, 0, 0);
        let h = lista(display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, cor(0xae31aa), cabeca);
        let grupos = core::ptr::read_unaligned((md + 9) as *const i32);
        for i in 0..grupos.max(0) as usize {
            let g = core::ptr::read_unaligned((md + 1) as *const usize) + i * 0x18;
            if *((g + 0x14) as *const u32) == 0 {
                continue;
            }
            let pai = if *((md + 0x29) as *const u8) == 1 {
                h
            } else {
                let tipo = *(g as *const i32);
                match tipo {
                    0 => {
                        texto(BUF, 0xa16284);
                    }
                    1 => {
                        texto(BUF, 0xa16248, 0xad9c64);
                    }
                    3..=7 => {
                        texto(BUF, 0xa16a90, 0xad9c64);
                    }
                    _ => {
                        let nacao = *((0xb63d60 + tipo as isize * 72) as *const usize);
                        if nacao != 0 {
                            copy_to_work_text(nacao + 4);
                        } else {
                            texto(BUF, 0xa16a90, 0xad9c64);
                        }
                    }
                }
                let r = linha(display(), 2, 0, 0, 0, 0, 0, 0, 0x10, cor(0xae31aa), cor(0xaebdec), 1, 1, cor(0xaebdea), BUF, 0, 0, 0, h);
                lista(display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, cor(0xae31aa), r)
            };
            let mut k = *((g + 4) as *const i32);
            while k <= *((g + 8) as *const i32) {
                let m = (core::ptr::read_unaligned((md + 0xd) as *const usize) as isize + k as isize * 84) as usize;
                if *((m + 0x10) as *const u32) != 0 {
                    let c = (*(0xae23d0 as *const usize) as isize + *(m as *const i32) as isize * 107) as usize;
                    panel_competition_name(c, m);
                    linha(display(), 2, 0, 0, 0, 0, 0, 0, 0x10, cor(0xae31aa), cor(0xaebdec), 1, 1, cor(0xaebdea), BUF, 0, 5, m, pai);
                }
                k += 1;
            }
        }
    }

    let altura: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x5e5cd0usize);
    let a = altura(3);
    let n = ((*base - *topo - 0x10) / a) as u8;
    set_screen_param(0xf, n as usize);
    let mut colunas: [u8; 4] = [0x16, 4, 0x16, 4];
    // a original passa o byte das linhas por cima do ponteiro `base` na pilha (os 3 bytes de cima ficam)
    let linhas = (base as usize & !0xff) | n as usize;
    let tabela = lista(display(), 0x6e, *topo as usize, 0x30c, *base as usize, 4, colunas.as_mut_ptr() as usize, linhas, 0, 2, 0, usize::MAX);
    for s in 0..n as usize {
        let r = celula(sm, tabela, 0, s, 1, 0, 1, 3, cor(0xaebde8), 0, 0, 0) as u16 as i16;
        if s == 0 {
            set_screen_param(0x10, r as i32 as usize);
        }
        for _ in 0..3 {
            texto_da_celula(sm, tabela, 1, s, 1, 0, 0xc, 3, cor(0xaebdec), 0);
        }
        celula(sm, tabela, 2, s, 1, 0, 1, 3, cor(0xaebde8), 0, 0, 0);
        texto_da_celula(sm, tabela, 3, s, 1, 0, 0xc, 2, cor(0xae3184), 0);
    }
    let marca = if *(0x9cc2c4 as *const i32) != -1 { 0 } else { 1 };
    caixa(sm, largura - 0x7c, t + 5, largura - 0x68, t + 0x19, 1, cor(0xaebe04), 0xc, marca, cor(0xae3184), 0, 6, 0);
    caixa(sm, largura - 0x66, t + 5, largura - 0x1b, t + 0x19, 1, 0, 0xc, 2, cor(0xaebdec), 0, 7, 0);
    caixa(sm, largura - 0x19, t + 5, largura - 5, t + 0x19, 1, cor(0xaebe04), 0xc, marca, cor(0xae3184), 0, 8, 0);
    let redesenhar: unsafe extern "C" fn() = core::mem::transmute(0x724520usize);
    redesenhar();
}

unsafe extern "thiscall" fn comparar_is_forward_unavailable(gerente: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(gerente);
    let n = is_forward_unavailable(gerente);
    PLACAR.registrar(r == n, || format!("is_forward_unavailable original={} rust={}", r, n));
    r
}

unsafe extern "C" fn comparar_get_heading_colour(x: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(%%I2%%)(x);
    let n = get_heading_colour(x);
    PLACAR.registrar(r == n, || format!("get_heading_colour({:#x}) original={:#x} rust={:#x}", x, r, n));
    r
}

/// O painel é comparado de verdade: as duas desenham a partir do mesmo estado e o gerente de telas, o
/// `Display` da tela atual, os parâmetros (menos o 0x14, a hora que o redesenho 0x724520 grava), o
/// `*topo` e o texto 0xde1f64 têm de sair iguais.
unsafe extern "C" fn comparar_draw_latest_scores_panel(topo: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(%%I0%%);
    let mut reg = screen_build_regions();
    let vaga = *((GERENTE_DE_TELAS + 0x382a) as *const i16) as isize;
    let bloco = core::ptr::read_unaligned((GERENTE_DE_TELAS as isize + 0x7e2 + vaga * 0x300) as *const usize);
    if bloco != 0 {
        reg.push((bloco, 0xb4));
        reg.push((bloco + 0xb8, 0x200 - 0xb8));
    }
    reg.push((topo as usize, 4));
    reg.push((0xde1f64, 0x100));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(topo, base), || draw_latest_scores_panel(topo, base));
    PLACAR.registrar(igual, || {
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("draw_latest_scores_panel: diferencas em {:?}", dif)
    });
}
