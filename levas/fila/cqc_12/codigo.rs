// ---------- %%LEVA%%a leva: o dia das seleções e as buscas da sessão do técnico humano (national_teams.cpp, scout_manager.cpp) ----------

unsafe fn cqc12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc12_uw(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqc12_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A data `r` é a de hoje (o ano e o dia)?
unsafe fn cqc12_e_hoje(r: usize, hoje: usize) -> bool {
    cqc12_uw(r + 2) == cqc12_uw(hoje + 2) && cqc12_w(r) == cqc12_w(hoje)
}

/// Marca os clubes com jogo na data `d` e nos dias vizinhos (0x7775c0 na data, na véspera e no dia seguinte; a data
/// volta a ser a mesma no fim: 0x54c8e0/0x54c5b0 e 0x417c10).
unsafe fn cqc12_marcar_tres_dias(this: usize, d: usize) {
    let marcar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7775c0usize);
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    // 0x7775c0: push &data; mov ecx, edi -> thiscall, ret 4
    let (mut t, mut u) = ([0u8; 8], [0u8; 8]);
    let (pt, pu) = (t.as_mut_ptr() as usize, u.as_mut_ptr() as usize);
    crate::comparacao::ramo("cqc_12 marca os clubes");
    marcar(this, d);
    copiar(d, pu, menos(d, pt, 1));
    marcar(this, d);
    copiar(d, pu, mais(d, pt, 2));
    marcar(this, d);
    copiar(d, pu, menos(d, pt, 1));
}

/// Os jogos da lista `l` (do dia daqui a duas semanas): para cada um, as convocações (0x76bab0) e, fora das fases
/// finais (a competição +0x14 numa das chaves 0x9cf79c, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0,
/// 0x9cf78c, 0x9cf964), com mandante que não é a seleção B (0x5409b0) e a marca +0x40: os convocados por clube
/// (0x7771c0) das duas seleções; com o bit 0x200 de +0x4d, também os das seleções B (0x53b4f0 da nação +0x53 de cada
/// uma) e, sem ele e com o 0x100, os das duas com `b` = 1.
unsafe fn cqc12_jogos(this: usize, l: usize) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let convocar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x76bab0usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let selecao_b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let registrar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x7771c0usize);
    // 0x76bab0: push p; mov ecx, edi -> thiscall ret 4; 0x5409b0 e 0x53b4f0: push; add esp, 4 -> cdecl;
    // 0x7771c0: push p; push b; push time; mov ecx, edi -> thiscall ret 0xc
    cqc12_pd(l + 0xa, cqc12_d(l + 2));
    let mut p = proximo(l);
    while p != 0 {
        crate::comparacao::ramo("cqc_12 convoca");
        convocar(this, p); // cqc12: as convocações para o jogo
        let comp = cqc12_d(p + 0x14);
        let final_ = comp != 0
            && [0x9cf79cusize, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964]
                .iter().any(|&k| cqc12_d(comp) == cqc12_d(k));
        if !final_ && segunda(cqc12_d(p + 0x1c)) == 0 && *((p + 0x40) as *const u8) != 0 {
            registrar(this, cqc12_d(p + 0x1c), 0, p);
            registrar(this, cqc12_d(p + 0x20), 0, p);
            let f = cqc12_uw(p + 0x4d);
            if f & 0x200 != 0 {
                let b1 = selecao_b(cqc12_d(cqc12_d(p + 0x1c) + 0x53));
                registrar(this, b1, 0, p);
                let b2 = selecao_b(cqc12_d(cqc12_d(p + 0x20) + 0x53));
                registrar(this, b2, 0, p);
            } else if f & 0x100 != 0 {
                registrar(this, cqc12_d(p + 0x1c), 1, p);
                registrar(this, cqc12_d(p + 0x20), 1, p);
            }
        }
        p = proximo(l);
    }
}

/// { 0x76b100 } (thiscall, ret 4, national_teams.cpp, quadro SEH só das datas locais, sem destrutor):
/// `NationalTeams::daily_update(hoje)`, pelo dia do jogo ([0xb74318], 0x5ccb07). A data daqui a duas semanas
/// (0x54c770); a tabela dos clubes com jogo (+0x24) sai (0x9452ca) e zera. Em 1º de janeiro, 15 de abril e 1º de agosto
/// (0x549f70 no ano de hoje), as reservas das seleções se refazem (0x76abf0). Nas 3 fatias do dia daqui a duas
/// semanas, as listas dos grupos 0 e 1 (0x5abf10): com jogo em alguma e sem a tabela, os clubes com jogo em volta da
/// data são marcados (cqc12_marcar_tres_dias); os jogos das duas listas (cqc12_jogos). Com a tabela, os jogos dos clubes
/// com convocados se remarcam (0x7777e0). Da lista em +0 saem (0x68d1f0) os itens do dia de hoje (+0x10). Com o ranking
/// (+0x1c), a atualização dele (0x58d460); no dia múltiplo de 70, a poda dos registros de convocação (0x7774b0); por fim
/// as notícias dos classificados (0x7779e0).
unsafe extern "thiscall" fn dia_das_selecoes(this: usize, hoje: usize) {
    let semanas: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c770usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let dma: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let refazer: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x76abf0usize);
    let lista: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x5abf10usize);
    let remarcar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7777e0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    let ranking: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x58d460usize);
    let podar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7774b0usize);
    let classificados: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7779e0usize);
    // 0x54c770: push 2; push saida; mov ecx, hoje -> thiscall ret 8; free: add esp, 4 -> cdecl; 0x549f70: 4 pushes;
    // lea ecx, saida -> thiscall ret 0x10; 0x76abf0, 0x7774b0, 0x7779e0: mov ecx, edi sem pushes -> thiscall;
    // 0x5abf10: 4 pushes; mov ecx, [0xae2a58] -> thiscall ret 0x10; 0x7777e0: push &data; mov ecx, edi -> thiscall
    // ret 4; 0x68d1f0: push item; mov ecx, [edi] -> thiscall ret 4; 0x58d460: push hoje; mov ecx, [edi + 0x1c] ->
    // thiscall ret 4
    let mut dd = [0u8; 8];
    let d = dd.as_mut_ptr() as usize;
    semanas(hoje, d, 2);
    if cqc12_d(this + 0x24) != 0 {
        liberar(cqc12_d(this + 0x24));
        cqc12_pd(this + 0x24, 0);
    }
    // ponytail: a original empurra o ano (palavra) e a semana com sobra de registrador em cima; a 0x549f70 lê as palavras
    let ano = cqc12_uw(hoje + 2) as u32;
    let mut t = [0u8; 8];
    let pt = t.as_mut_ptr() as usize;
    if cqc12_e_hoje(dma(pt, 1, 0, ano, u32::MAX), hoje) // cqc12: 1º de janeiro
        || cqc12_e_hoje(dma(pt, 0xf, 3, ano, u32::MAX), hoje)
        || cqc12_e_hoje(dma(pt, 1, 7, ano, u32::MAX), hoje)
    {
        crate::comparacao::ramo("cqc_12 refaz as reservas");
        refazer(this);
    }
    let fixman = cqc12_d(0xae2a58);
    for fatia in 0u32..3 {
        // os dwords da original: o ano (com os 2 bytes seguintes da data) e o dia (com o ano em cima); ponytail: a
        // fatia vai sem a sobra de pilha dos bytes de cima (a 0x5abf10 faz and 0xff)
        let l0 = lista(fixman, cqc12_d(d + 2) as u32, cqc12_d(d) as u32, fatia, 0);
        let l1 = lista(fixman, cqc12_d(d + 2) as u32, cqc12_d(d) as u32, fatia, 1);
        if ((l0 != 0 && cqc12_w(l0) != 0) || (l1 != 0 && cqc12_w(l1) != 0)) && cqc12_d(this + 0x24) == 0 {
            cqc12_marcar_tres_dias(this, d); // cqc12: os clubes com jogo em volta da data
        }
        if l0 != 0 {
            cqc12_jogos(this, l0);
        }
        if l1 != 0 {
            cqc12_jogos(this, l1);
        }
    }
    if cqc12_d(this + 0x24) != 0 {
        remarcar(this, d);
    }
    if cqc12_d(this) != 0 {
        let dia = cqc12_w(hoje);
        let l = cqc12_d(this);
        cqc12_pd(l + 0xa, cqc12_d(l + 2));
        let mut p = proximo(cqc12_d(this));
        while p != 0 {
            if cqc12_w(p + 0x10) == dia {
                tirar(cqc12_d(this), p);
            }
            p = proximo(cqc12_d(this));
        }
    }
    if cqc12_d(this + 0x1c) != 0 {
        ranking(cqc12_d(this + 0x1c), hoje);
    }
    if cqc12_w(hoje) as i32 % 0x46 == 0 {
        podar(this);
    }
    classificados(this);
}

/// Quantos itens da lista em +0 são do dia `dia` (+0x10), percorrendo os nós sem mexer no cursor.
unsafe fn cqc12_do_dia(this: usize, dia: i16) -> u32 {
    let l = cqc12_d(this);
    if l == 0 {
        return 0;
    }
    let mut n = 0;
    let mut no = cqc12_d(l + 2);
    while no != 0 {
        if cqc12_w(cqc12_d(no) + 0x10) == dia {
            n += 1;
        }
        no = cqc12_d(no + 6);
    }
    n
}

/// Comparador: a original convoca (0x76bab0: notícias, elencos), remarca jogos (0x7777e0), aloca a tabela (0x7775c0)
/// e libera itens (0x68d1f0): não pode rodar duas vezes. Só ela roda, conferindo o plano do que a própria rotina grava:
/// nenhum item do dia de hoje sobra na lista em +0.
unsafe extern "thiscall" fn comparar_dia_das_selecoes(this: usize, hoje: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let dia = cqc12_w(hoje);
    let antes = cqc12_do_dia(this, dia);
    o(this, hoje);
    let sobra = cqc12_do_dia(this, dia);
    PLACAR.registrar(sobra == 0, || format!("dia_das_selecoes(dia {}) itens do dia antes {} depois {}", dia, antes, sobra));
}

/// O índice (0..15) do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]) pela conta de um byte da original
/// ((id − [0xae2364]) + 16 em 8 bits); Err(()) sem técnico, Ok(None) fora (a caixa).
unsafe fn cqc12_indice() -> Result<Option<usize>, ()> {
    let t = cqc12_d(0xb74c26usize.wrapping_add((*(0xb77c7a as *const i16) as i32 as usize).wrapping_mul(0x300)));
    if t == 0 {
        return Err(());
    }
    let i = (*(t as *const u8)).wrapping_sub(*(0xae2364 as *const u8)).wrapping_add(0x10) as i8;
    Ok(if (0..=0xf).contains(&i) { Some(i as usize) } else { None })
}

/// O registro de busca `k` (0x2b0 bytes) da sessão `s`.
unsafe fn cqc12_busca(s: usize, k: i32) -> usize {
    s.wrapping_add((k as usize).wrapping_mul(0x2b0))
}

/// { 0x7f8420 } (thiscall, ret 0, scout_manager.cpp): `SCOUT_MANAGER::current_search_has_no_club()`, pelas telas das
/// buscas (0x482a9b). A sessão do técnico humano da vez ([this + 0xc + 4 × índice]): a busca atual (o byte com sinal
/// em +0 da sessão) sem clube (+0x191 = −2). Sem técnico ou sem sessão: 0; índice fora: a caixa (linha 0x454; a
/// original) e 0.
unsafe extern "thiscall" fn busca_atual_sem_clube(this: usize) -> u32 {
    let i = match cqc12_indice() {
        Err(()) => return 0,
        Ok(None) => return orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this), // a caixa
        Ok(Some(i)) => i,
    };
    let s = cqc12_d(this + 0xc + i * 4);
    if s == 0 {
        return 0;
    }
    (cqc12_d(cqc12_busca(s, *(s as *const i8) as i32) + 0x191) as i32 == -2) as u32
}

/// Comparador: pelo valor (só lê); com a caixa, só a original.
unsafe extern "thiscall" fn comparar_busca_atual_sem_clube(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    if matches!(cqc12_indice(), Ok(None)) {
        return o(this);
    }
    let r = o(this);
    let n = busca_atual_sem_clube(this);
    PLACAR.registrar(r == n, || format!("busca_atual_sem_clube() original={} rust={}", r, n));
    r
}

/// { 0x7f8500 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::is_club_in_session_searches(clube)`, pelas telas
/// (0x482aaf, 0x483040, 0x7b34e1). Alguma das 10 buscas da sessão do técnico humano da vez tem o clube (+0x191 = +0
/// do clube): 1. Sem técnico ou sem sessão: 0; índice fora: a caixa (linha 0x480; a original) e 0.
unsafe extern "thiscall" fn clube_nas_buscas_da_sessao(this: usize, clube: usize) -> u32 {
    let i = match cqc12_indice() {
        Err(()) => return 0,
        Ok(None) => return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, clube), // a caixa
        Ok(Some(i)) => i,
    };
    let s = cqc12_d(this + 0xc + i * 4);
    if s == 0 {
        return 0;
    }
    let id = cqc12_d(clube);
    // ponytail: a original testa o contador contra 0xff (nunca) para usar a busca atual (+0); fica o contador
    (0..10).any(|k| cqc12_d(cqc12_busca(s, k) + 0x191) == id) as u32
}

/// Comparador: pelo valor (só lê); com a caixa, só a original.
unsafe extern "thiscall" fn comparar_clube_nas_buscas_da_sessao(this: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    if matches!(cqc12_indice(), Ok(None)) {
        return o(this, clube);
    }
    let r = o(this, clube);
    let n = clube_nas_buscas_da_sessao(this, clube);
    PLACAR.registrar(r == n, || format!("clube_nas_buscas_da_sessao(clube {:x}) original={} rust={}", clube, r, n));
    r
}
