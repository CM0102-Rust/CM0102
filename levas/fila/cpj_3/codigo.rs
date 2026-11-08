// ---------- %%LEVA%%a leva: as notícias para os técnicos humanos: para todos, para os da divisão e para os da nação (clube ou seleção) ----------

unsafe fn cpj3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O técnico humano `i` (0 a 15: os últimos 16 de [0xae23bc], 0x6e bytes), se ativo (+0x5f = 1).
unsafe fn active_human_manager(i: usize) -> Option<usize> {
    let p = cpj3_d(0xae23bc).wrapping_add(cpj3_d(0xae2364).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(0x6e));
    (*((p + 0x5f) as *const u8) == 1).then_some(p)
}

/// Entrega a notícia a cada técnico humano ativo que `passa` (0x785d40 com 0), com o alcance `alcance` em +0xde.
/// Devolve se todas as entregas foram aceitas (a primeira recusada encerra).
unsafe fn deliver_to_human_managers(this: usize, noticia: usize, alcance: u8, passa: impl Fn(usize) -> bool) -> bool {
    *((noticia + 0xde) as *mut u8) = alcance;
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x785d40usize);
    for i in 0..0x10 {
        if let Some(p) = active_human_manager(i) {
            if passa(p) && entregar(this, noticia, p, 0) == 0 {
                return false; // a entrega recusada encerra
            }
        }
    }
    true
}

/// { 0x786780 } (thiscall, ret 4, news.cpp): `News::send_to_all_managers(noticia)`: alcance de todos (+0xde = 4) e a
/// entrega (0x785d40) a cada técnico humano ativo. Devolve 1, ou 0 na primeira recusada. Notícia nula: a original
/// (caixa "Error").
unsafe extern "thiscall" fn send_to_all_managers(this: usize, noticia: usize) -> u32 {
    if noticia == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, noticia); // caixa: só a original
    }
    deliver_to_human_managers(this, noticia, 4, |_| true) as u32
}

/// { 0x7862d0 } (thiscall, ret 8, news.cpp): `News::send_to_division_managers(noticia, divisao)`: alcance da divisão
/// (+0xde = 1) e a entrega a cada técnico humano ativo com clube (+0x39) na `divisao` (+0x57). Devolve 1, ou 0 na
/// primeira recusada. Notícia ou divisão nulas: a original (caixa "Error").
unsafe extern "thiscall" fn send_to_division_managers(this: usize, noticia: usize, divisao: usize) -> u32 {
    if noticia == 0 || divisao == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%)(this, noticia, divisao); // caixa: só a original
    }
    deliver_to_human_managers(this, noticia, 1, |p| {
        let c = cpj3_d(p + 0x39);
        c != 0 && cpj3_d(c + 0x57) == divisao // a divisão do clube
    }) as u32
}

/// { 0x7863f0 } (thiscall, ret 8, news.cpp): `News::send_to_nation_managers(noticia, nacao)`: alcance da nação (+0xde
/// = 2) e a entrega a cada técnico humano ativo cujo clube (+0x39) ou seleção (+0x24) é da `nacao` (+0x53). Devolve 0.
/// Notícia ou nação nulas: a original (caixa "Error").
unsafe extern "thiscall" fn send_to_nation_managers(this: usize, noticia: usize, nacao: usize) -> u32 {
    if noticia == 0 || nacao == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%)(this, noticia, nacao); // caixa: só a original
    }
    deliver_to_human_managers(this, noticia, 2, |p| {
        let (c, s) = (cpj3_d(p + 0x39), cpj3_d(p + 0x24));
        (c != 0 && cpj3_d(c + 0x53) == nacao) || (s != 0 && cpj3_d(s + 0x53) == nacao) // o clube ou a seleção
    });
    0
}

/// Comparadores: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
unsafe extern "thiscall" fn comparar_send_to_all_managers(this: usize, noticia: usize) -> u32 {
    crate::comparacao::ramo("send_to_all_managers");
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, noticia)
}

unsafe extern "thiscall" fn comparar_send_to_division_managers(this: usize, noticia: usize, divisao: usize) -> u32 {
    crate::comparacao::ramo("send_to_division_managers");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%)(this, noticia, divisao)
}

unsafe extern "thiscall" fn comparar_send_to_nation_managers(this: usize, noticia: usize, nacao: usize) -> u32 {
    crate::comparacao::ramo("send_to_nation_managers");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%)(this, noticia, nacao)
}
