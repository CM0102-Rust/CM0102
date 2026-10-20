// ---------- %%LEVA%%a leva: as telas da partida (4): abrir a tela da partida e as respostas dos últimos placares e das estatísticas do time (match_screens.cpp) ----------

/// O bloco de parâmetros da tela atual ([gerente + 0x7e2 + 0x300 x vaga]; o parâmetro k em +0x14 + 8k,
/// a marca de "liberar ao trocar" em +0x18 + 8k, 60 parâmetros) e as marcas 0xb6b7a8/ac/b0.
unsafe fn regioes_dos_parametros() -> Vec<(usize, usize)> {
    let vaga = *((GERENTE_DE_TELAS + 0x382a) as *const i16) as isize;
    let bloco = core::ptr::read_unaligned((GERENTE_DE_TELAS as isize + 0x7e2 + vaga * 0x300) as *const usize);
    let mut r = vec![(0xb6b7a8, 12)];
    if bloco != 0 {
        r.push((bloco, 0x200));
    }
    r
}

/// O id do item `item` na lista de exibição do humano da vez (0 vira o padrão [0xdd6bde]).
unsafe fn id_do_item(item: i16) -> u16 {
    let display = *((0xb74c4c + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize);
    let w = core::ptr::read_unaligned((display as isize + item as isize * 0x18c + 0xba966) as *const u16);
    if w == 0 { *(0xdd6bde as *const u16) } else { w }
}

/// { 0x718aa0 } (cdecl, match_screens.cpp): abre a tela da partida `partida` na vaga `vaga` (0x7fe4a0
/// com a montagem 0x718e90, a resposta 0x71a240 e o fechamento 0x71b120) e grava os parâmetros dela
/// (0x7ff130; o 0xe é [partida+0x63]); marca o humano da vez como espectador ([[partida+0x5b]+0xc5+4h],
/// contando em +0x108) e, sem confirmação ([partida+0x57]), confirma pela rede (0x6b5260) e, se
/// confirmou, grava 2 no parâmetro 1. Vaga ou partida nula: a original (caixa de erro).
unsafe extern "C" fn abrir_tela_da_partida(vaga: u32, partida: usize) {
    if vaga == 0 || partida == 0 {
        return orig::<unsafe extern "C" fn(u32, usize)>(%%I0%%)(vaga, partida);
    }
    crate::comparacao::ramo("telas abrir a partida");
    let abrir: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(0x7fe4a0usize);
    if abrir(GERENTE_DE_TELAS, vaga, 0x718e90, 0x71a240, 1, 0x71b120, 0) == 0 {
        return;
    }
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32) = core::mem::transmute(0x7ff130usize);
    const M: usize = usize::MAX;
    let e = *((partida + 0x63) as *const usize);
    for (k, v) in [
        (0u32, partida), (1, 1), (2, 1), (3, M), (4, M), (5, M), (6, M), (7, 0), (8, M), (9, M), (0xa, 3), (0xb, M), (0xc, 0), (0xd, M),
        (0xe, e), (0xf, 0), (0x10, M), (0x11, 1), (0x12, 0), (0x13, 1), (0x14, 0), (0x15, 0), (0x16, 0), (0x17, 0), (0x18, 0), (0x19, 0),
        (0x1a, M), (0x1b, 0), (0x1c, 1), (0x1d, 0), (0x21, M), (0x22, 0), (0x23, M), (0x24, 0), (0x25, 0), (0x26, 0), (0x27, 0),
        (0x28, 0), (0x29, 0), (0x2d, 0), (0x2f, 0), (0x30, 0),
    ] {
        gravar(GERENTE_DE_TELAS, vaga, k, v, 0);
    }
    let espectadores = *((partida + 0x5b) as *const usize);
    let marca = (espectadores as isize + 0xc5 + *(0xb77c7a as *const i16) as isize * 4) as *mut u32;
    if *marca == 0 {
        *marca = 1;
        let n = (espectadores + 0x108) as *mut u8;
        *n = (*n).wrapping_add(1);
    }
    if *((partida + 0x57) as *const u32) == 0 {
        let confirmar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x6b5260usize);
        confirmar(*(0xb67a50 as *const usize), partida);
        if *((partida + 0x57) as *const u32) != 0 {
            let gravar_atual: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(0x7ff270usize);
            gravar_atual(GERENTE_DE_TELAS, 1, 2, 0);
        }
    }
}

/// { 0x725860 } (cdecl, match_screens.cpp): a resposta do painel dos últimos placares ao item `item`.
/// A cada 5 s (parâmetro 0x14 + 5000 ms, 0x90c510) passa à página seguinte (parâmetro 0x11, de 1 até
/// o 0x13, se não estiver parado pelo 0x12) e, sem [0xb6b7b0], redesenha o painel (0x724520) e liga
/// [0xb6b7a8]. Itens: 5 troca a competição (0xe) e volta à página 1, 6 e 8 a página anterior e a
/// seguinte (em círculo), 7 para ou solta a troca automática; cada um liga [0xb6b7b0]. Devolve 0.
unsafe extern "C" fn responder_ultimos_placares_da_partida(item: u32) -> i32 {
    crate::comparacao::ramo("telas responder ao painel dos placares");
    let pagina = parametro_da_tela(0x11) as u8 as i8;
    let parado = parametro_da_tela(0x12);
    let paginas = parametro_da_tela(0x13) as u8 as i8;
    let quando = parametro_da_tela(0x14) as u32;
    let agora: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x90c510usize);
    let seguinte = if pagina == paginas { 1 } else { (pagina as i32 + 1) as usize };
    if agora() >= quando.wrapping_add(5000) {
        if paginas > 1 && parado == 0 {
            gravar_parametro_da_tela(0x11, seguinte);
        }
        if *(0xb6b7b0 as *const u32) == 0 {
            let redesenhar: unsafe extern "C" fn() = core::mem::transmute(0x724520usize);
            redesenhar();
            *(0xb6b7a8 as *mut u32) = 1;
        }
    }
    let item16 = item as u16 as i16;
    if item16 == -1 {
        return 0;
    }
    let b0 = 0xb6b7b0 as *mut u32;
    if id_do_item(item16) == 5 {
        let display = *((0xb74c4c + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize);
        let dado = core::ptr::read_unaligned((display as isize + item16 as isize * 0x18c + 0xba9a6) as *const usize);
        gravar_parametro_da_tela(0xe, dado);
        gravar_parametro_da_tela(0x11, 1);
        *b0 = 1;
    }
    if id_do_item(item16) == 6 {
        // a página lida no início (a original não relê depois do item 5)
        gravar_parametro_da_tela(0x11, if pagina == 1 { paginas as i32 as usize } else { (pagina as i32 - 1) as usize });
        *b0 = 1;
    }
    if id_do_item(item16) == 7 {
        gravar_parametro_da_tela(0x12, (parado == 0) as usize);
        *b0 = 1;
    }
    if id_do_item(item16) == 8 {
        gravar_parametro_da_tela(0x11, seguinte);
        *b0 = 1;
    }
    0
}

/// { 0x722dc0 } (cdecl, match_screens.cpp): a resposta das estatísticas do time ao item `item`. Com
/// dados novos ([0xb6b7ac]) e a marca 4 do menu da partida ([[parâmetro 0]+0x5b]), sem [0xb6b7b0],
/// redesenha o painel (0x721cd0 com 0 e −1) e liga [0xb6b7a8]; os itens 9 e 0xa descem e sobem a
/// linha do parâmetro 0x1c e ligam [0xb6b7b0]. Devolve 0.
unsafe extern "C" fn responder_estatisticas_do_time(item: u32) -> i32 {
    crate::comparacao::ramo("telas responder as estatisticas do time");
    let p0 = parametro_da_tela(0);
    let linha = parametro_da_tela(0x1c) as u8 as i8;
    if *(0xb6b7ac as *const u32) != 0 && *(*((p0 + 0x5b) as *const usize) as *const u8) & 4 != 0 && *(0xb6b7b0 as *const u32) == 0 {
        let redesenhar: unsafe extern "C" fn(u32, i32) = core::mem::transmute(0x721cd0usize);
        redesenhar(0, -1);
        *(0xb6b7a8 as *mut u32) = 1;
    }
    let item16 = item as u16 as i16;
    if item16 == -1 {
        return 0;
    }
    if id_do_item(item16) == 9 {
        gravar_parametro_da_tela(0x1c, (linha as i32 - 1) as usize);
        *(0xb6b7b0 as *mut u32) = 1;
    }
    if id_do_item(item16) == 0xa {
        gravar_parametro_da_tela(0x1c, (linha as i32 + 1) as usize);
        *(0xb6b7b0 as *mut u32) = 1;
    }
    0
}

// ponytail: abrir a tela mexe em telas e na rede (não roda duas vezes): no modo comparação só a original
unsafe extern "C" fn comparar_abrir_tela_da_partida(vaga: u32, partida: usize) {
    orig::<unsafe extern "C" fn(u32, usize)>(%%I0%%)(vaga, partida)
}

/// As duas respostas são comparadas de verdade nos cliques (o bloco de parâmetros da tela e as marcas
/// 0xb6b7a8/ac/b0, mais a devolução); quando há redesenho (a troca de 5 s dos placares, os dados novos
/// das estatísticas) só a original roda, porque o redesenho grava a hora no parâmetro 0x14 e mexe na
/// lista de exibição.
unsafe fn comparar_resposta(nome: &str, item: u32, o: unsafe extern "C" fn(u32) -> i32, r: unsafe extern "C" fn(u32) -> i32, redesenha: bool) -> i32 {
    if item as u16 == 0xffff || redesenha {
        return o(item);
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes_dos_parametros(), || o(item), || r(item));
    PLACAR.registrar(igual && ro == rr, || format!("{}(item {:#x}) original={} rust={} {:?} x {:?}", nome, item, ro, rr, mo, mr));
    ro
}

unsafe extern "C" fn comparar_responder_ultimos_placares_da_partida(item: u32) -> i32 {
    let agora: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x90c510usize);
    // folga de 1 s: a hora pode virar entre os dois lados
    let redesenha = agora().wrapping_add(1000) >= (parametro_da_tela(0x14) as u32).wrapping_add(5000);
    comparar_resposta("responder_ultimos_placares_da_partida", item, orig(%%I1%%), responder_ultimos_placares_da_partida, redesenha)
}

unsafe extern "C" fn comparar_responder_estatisticas_do_time(item: u32) -> i32 {
    let p0 = parametro_da_tela(0);
    let redesenha = *(0xb6b7ac as *const u32) != 0 && *(*((p0 + 0x5b) as *const usize) as *const u8) & 4 != 0;
    comparar_resposta("responder_estatisticas_do_time", item, orig(%%I2%%), responder_estatisticas_do_time, redesenha)
}
