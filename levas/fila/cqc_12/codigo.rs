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
unsafe fn is_today(r: usize, hoje: usize) -> bool {
    cqc12_uw(r + 2) == cqc12_uw(hoje + 2) && cqc12_w(r) == cqc12_w(hoje)
}

/// Marca os clubes com jogo na data `d` e nos dias vizinhos (0x7775c0 na data, na véspera e no dia seguinte; a data
/// volta a ser a mesma no fim: 0x54c8e0/0x54c5b0 e 0x417c10).
unsafe fn mark_clubs_three_days(this: usize, d: usize) {
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
unsafe fn select_squads_for_fixtures(this: usize, l: usize) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let convocar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x76bab0usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let b_national_team: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
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
                let b1 = b_national_team(cqc12_d(cqc12_d(p + 0x1c) + 0x53));
                registrar(this, b1, 0, p);
                let b2 = b_national_team(cqc12_d(cqc12_d(p + 0x20) + 0x53));
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
/// data são marcados (mark_clubs_three_days); os jogos das duas listas (select_squads_for_fixtures). Com a tabela, os jogos dos clubes
/// com convocados se remarcam (0x7777e0). Da lista em +0 saem (0x68d1f0) os itens do dia de hoje (+0x10). Com o ranking
/// (+0x1c), a atualização dele (0x58d460); no dia múltiplo de 70, a poda dos registros de convocação (0x7774b0); por fim
/// as notícias dos classificados (0x7779e0).
unsafe extern "thiscall" fn national_teams_daily_update(this: usize, hoje: usize) {
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
    if is_today(dma(pt, 1, 0, ano, u32::MAX), hoje) // cqc12: 1º de janeiro
        || is_today(dma(pt, 0xf, 3, ano, u32::MAX), hoje)
        || is_today(dma(pt, 1, 7, ano, u32::MAX), hoje)
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
            mark_clubs_three_days(this, d); // cqc12: os clubes com jogo em volta da data
        }
        if l0 != 0 {
            select_squads_for_fixtures(this, l0);
        }
        if l1 != 0 {
            select_squads_for_fixtures(this, l1);
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
unsafe fn count_items_on_day(this: usize, dia: i16) -> u32 {
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
unsafe extern "thiscall" fn comparar_national_teams_daily_update(this: usize, hoje: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let dia = cqc12_w(hoje);
    let antes = count_items_on_day(this, dia);
    o(this, hoje);
    let sobra = count_items_on_day(this, dia);
    PLACAR.registrar(sobra == 0, || format!("national_teams_daily_update(dia {}) itens do dia antes {} depois {}", dia, antes, sobra));
}

/// O índice (0..15) do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]) pela conta de um byte da original
/// ((id − [0xae2364]) + 16 em 8 bits); Err(()) sem técnico, Ok(None) fora (a caixa).
unsafe fn current_human_index() -> Result<Option<usize>, ()> {
    let t = cqc12_d(0xb74c26usize.wrapping_add((*(0xb77c7a as *const i16) as i32 as usize).wrapping_mul(0x300)));
    if t == 0 {
        return Err(());
    }
    let i = (*(t as *const u8)).wrapping_sub(*(0xae2364 as *const u8)).wrapping_add(0x10) as i8;
    Ok(if (0..=0xf).contains(&i) { Some(i as usize) } else { None })
}

/// O registro de busca `k` (0x2b0 bytes) da sessão `s`.
unsafe fn search_record(s: usize, k: i32) -> usize {
    s.wrapping_add((k as usize).wrapping_mul(0x2b0))
}

/// { 0x7f8420 } (thiscall, ret 0, scout_manager.cpp): `SCOUT_MANAGER::current_search_has_no_club()`, pelas telas das
/// buscas (0x482a9b). A sessão do técnico humano da vez ([this + 0xc + 4 × índice]): a busca atual (o byte com sinal
/// em +0 da sessão) sem clube (+0x191 = −2). Sem técnico ou sem sessão: 0; índice fora: a caixa (linha 0x454; a
/// original) e 0.
unsafe extern "thiscall" fn current_search_has_no_club(this: usize) -> u32 {
    let i = match current_human_index() {
        Err(()) => return 0,
        Ok(None) => return original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this), // a caixa
        Ok(Some(i)) => i,
    };
    let s = cqc12_d(this + 0xc + i * 4);
    if s == 0 {
        return 0;
    }
    (cqc12_d(search_record(s, *(s as *const i8) as i32) + 0x191) as i32 == -2) as u32
}

/// Comparador: pelo valor (só lê); com a caixa, só a original.
unsafe extern "thiscall" fn comparar_current_search_has_no_club(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    if matches!(current_human_index(), Ok(None)) {
        return o(this);
    }
    let r = o(this);
    let n = current_search_has_no_club(this);
    PLACAR.registrar(r == n, || format!("current_search_has_no_club() original={} rust={}", r, n));
    r
}

/// { 0x7f8500 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::is_club_in_session_searches(clube)`, pelas telas
/// (0x482aaf, 0x483040, 0x7b34e1). Alguma das 10 buscas da sessão do técnico humano da vez tem o clube (+0x191 = +0
/// do clube): 1. Sem técnico ou sem sessão: 0; índice fora: a caixa (linha 0x480; a original) e 0.
unsafe extern "thiscall" fn is_club_in_session_searches(this: usize, clube: usize) -> u32 {
    let i = match current_human_index() {
        Err(()) => return 0,
        Ok(None) => return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, clube), // a caixa
        Ok(Some(i)) => i,
    };
    let s = cqc12_d(this + 0xc + i * 4);
    if s == 0 {
        return 0;
    }
    let id = cqc12_d(clube);
    // ponytail: a original testa o contador contra 0xff (nunca) para usar a busca atual (+0); fica o contador
    (0..10).any(|k| cqc12_d(search_record(s, k) + 0x191) == id) as u32
}

/// Comparador: pelo valor (só lê); com a caixa, só a original.
unsafe extern "thiscall" fn comparar_is_club_in_session_searches(this: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    if matches!(current_human_index(), Ok(None)) {
        return o(this, clube);
    }
    let r = o(this, clube);
    let n = is_club_in_session_searches(this, clube);
    PLACAR.registrar(r == n, || format!("is_club_in_session_searches(clube {:x}) original={} rust={}", clube, r, n));
    r
}

// ---------- a escolha do elenco da seleção (0x76c120), com a correção dos elencos de 23 e 25 ----------

unsafe fn cqc12_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc12_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

/// Os códigos de posição das vagas da original: elenco de 26 (0x76c771..0x76c84c), de 24 (0x76c870..0x76c92c) e o de
/// qualquer outro tamanho (0x76c939..0x76c9de, 22 vagas), com os comuns gravados depois (0x76c860/0x76c865 e as vagas
/// 0, 1, 3, 5, 6 e 21). O código 0xc da vaga 12/11/10 vem do ebp (0xc desde 0x76c2e4/0x76c6a9).
const CQC12_26: [u16; 26] = [1, 1, 1, 0x202, 0x202, 0x204, 0x204, 0x204, 0x84, 0x804, 0x88, 0x808, 0xc, 0x210, 0x210, 0x210,
    0x220, 0x90, 0x810, 0xa0, 0x820, 0x240, 0x240, 0x240, 0x240, 0x240];
const CQC12_24: [u16; 24] = [1, 1, 0x202, 0x202, 0x204, 0x204, 0x204, 0x84, 0x804, 0x88, 0x808, 0xc, 0x210, 0x210, 0x210,
    0x220, 0x90, 0x810, 0xa0, 0x820, 0x240, 0x240, 0x240, 0x240];
const CQC12_22: [u16; 22] = [1, 1, 1, 0x202, 0x204, 0x204, 0x204, 0x204, 0x84, 0x804, 0xc, 0x210, 0x210, 0x220, 0x90, 0x810,
    0xa0, 0x820, 0x240, 0x240, 0x240, 0x240];
/// As vagas que a tabela de 24 tem a mais que a de 22, na ordem em que aparecem nela: 0x202 (líbero/centro, a vaga
/// 3), 0x88 e 0x808 (volantes pelos lados, 9 e 10) e 0x210 (o 3º meia central, 14).
const CQC12_EXTRAS: [u16; 3] = [0x202, 0x88, 0x808];

/// Os códigos de posição das `tam` vagas (o byte +0x42 do registro do tipo). Fiel à original em 26 e 24 e nas 22
/// primeiras vagas dos outros tamanhos (a tabela de 22). Correção do original aprovada (01/11/2026): a original só
/// preenche as vagas de 22, 24 ou 26 jogadores; nos elencos de 23 (as fases finais, 0x76d430: a Copa de 2002, em
/// 29/05/2002) e 25, as vagas a partir da 22ª ficam com a sobra da pilha (local_86c), que vai à escala (0x6e2000) e
/// ao filtro dos escalados (0x5b7d20), e a sobra muda a cada execução (metades de ponteiros da heap). Aqui as vagas
/// além das 22 da tabela da original recebem, em ordem, as que a tabela de 24 acrescenta à de 22 (CQC12_EXTRAS):
/// o elenco de 23 = o de 22 + 0x202 (a primeira vaga que o de 24 acrescenta); o de 25 = o de 22 + 0x202, 0x88 e
/// 0x808. Por que assim: as 22 primeiras vagas continuam as que a original grava nesses tamanhos (a comparação
/// confere todas), e cada vaga a mais vai na direção da tabela maior seguinte. O elenco de 25 parte da tabela de 22
/// e não da de 24 porque é a que a original usa para ele (só 24 e 26 têm tabela própria). Outros tamanhos: como a
/// original nas 22 primeiras; as demais com código 0 (ninguém passa no filtro). ponytail: o jogo só pede 23, 24 e 26.
unsafe fn squad_position_codes(tam: u8) -> [u16; 50] {
    let mut c = [0u16; 50];
    match tam {
        0x1a => c[..26].copy_from_slice(&CQC12_26),
        0x18 => c[..24].copy_from_slice(&CQC12_24),
        _ => c[..22].copy_from_slice(&CQC12_22),
    }
    if tam == 23 || tam == 25 {
        crate::comparacao::ramo(if tam == 23 { "cqc_12 correção do elenco de 23" } else { "cqc_12 correção do elenco de 25" });
        for i in 22..tam as usize {
            c[i] = CQC12_EXTRAS[i - 22]; // cqc12: as vagas da correção
        }
    }
    c
}

/// O registro da nação nas seleções (+4 da tabela, 0x6e bytes pelo id da nação).
unsafe fn squad_nation_record(this: usize, nacao: usize) -> usize {
    cqc12_d(this + 4).wrapping_add(cqc12_d(nacao).wrapping_mul(0x6e))
}

/// As faixas da reputação aceita (+9 da ficha +0x61): (mínimo exclusive, máximo), −1 e 0x2711 fora da Austrália
/// ([0x9cf224]); para ela, pela competição da partida (amistoso 0x5c6710 de [0xae2c74], ou a da Oceania: 0x8ca/0x2134;
/// Copa das Confederações 0xabe/0x222e; eliminatórias da Oceania 0x8ca e 9000, 8000 ou 10001 pelo byte +0x42; Copa
/// do Mundo 0xabe/10000).
unsafe fn reputation_limits(nacao: usize, p: usize) -> (i16, i16) {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    // 0x5c6710: push [p + 0x14]; mov ecx, [0xae2c74] -> thiscall ret 4
    if cqc12_d(nacao) != cqc12_d(0x9cf224) {
        return (-1, 0x2711);
    }
    if p == 0 || (cqc12_d(0xae2c74) != 0 && amistoso(cqc12_d(0xae2c74), cqc12_d(p + 0x14)) != 0) {
        return (0x8ca, 0x2134);
    }
    let k = cqc12_d(cqc12_d(p + 0x14));
    if k == cqc12_d(0x9cf8a0) {
        (0x8ca, 0x2134)
    } else if k == cqc12_d(0x9cf964) {
        (0xabe, 0x222e)
    } else if k == cqc12_d(0x9cf76c) {
        (0x8ca, match cqc12_b(p + 0x42) { 1 => 0x1f40, 2 => 0x2711, _ => 0x2328 })
    } else if k == cqc12_d(0x9cf79c) {
        (0xabe, 0x2710)
    } else {
        (-1, 0x2711)
    }
}

/// O time do tipo (0x53b3d0 a principal nos tipos 0 e 1, 0x53b4f0 o sub-21), a marca de elenco (+0x5e: 8 no tipo
/// 1, 4 nos outros) e o time da tática (1 no tipo 1).
unsafe fn squad_team_for_type(nacao: usize, t: i8) -> (usize, u8, u32) {
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    // 0x53b3d0 e 0x53b4f0: push nacao; add esp, 4 -> cdecl
    match t {
        1 => (principal(nacao), 8, 1),
        0 => (principal(nacao), 4, 0),
        _ => (sub21(nacao), 4, 0),
    }
}

/// Os tipos de reserva percorridos: só o do tipo, fora o sub-21 (2) dos Jogos Olímpicos ([0x9cf7a0]), que olha as
/// reservas 0 e 1; o sub-21 fora deles olha a 1.
unsafe fn reserve_type_range(t: i8, p: usize) -> (usize, usize) {
    if t != 2 {
        (0, 0)
    } else if p != 0 && cqc12_d(p) == cqc12_d(0x9cf7a0) {
        (0, 1)
    } else {
        (1, 1)
    }
}

/// As entradas das reservas do tipo `k`, grupo `g`: (vetor [this + 0xc + 4k] a partir do início, quantas). O início é
/// o dword +8 + 0x15k do registro mais os bytes +0xe + 0x15k + 4j dos grupos anteriores; a quantidade, o byte +0xd +
/// 0x15k + 4g (como a original lê: a soma e a quantidade saem de bytes vizinhos).
unsafe fn reserve_group(this: usize, reg: usize, k: usize, g: usize) -> (usize, i32, i32) {
    let base = reg + 0xd + 0x15 * k;
    let mut ini = cqc12_d(base - 5) as i32;
    for j in 0..g {
        ini = ini.wrapping_add(cqc12_b(base + 1 + 4 * j) as i32);
    }
    let fim = ini.wrapping_add(cqc12_b(base + 4 * g) as i32);
    (cqc12_d(this + 0xc + 4 * k).wrapping_add((ini as usize).wrapping_mul(4)), ini, fim)
}

/// O jogador `j` das reservas entra na lista da escala? (0x76cb84..0x76cf6c)
#[allow(clippy::too_many_arguments)]
unsafe fn is_squad_candidate(this: usize, nacao: usize, t: i8, data: usize, p: usize, elenco: usize, reg: usize, lim: (i16, i16), j: usize) -> bool {
    let tipo_ok: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x53b620usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let bits: unsafe extern "stdcall" fn(usize) -> u8 = core::mem::transmute(0x7a7eb0usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x55d270usize);
    let pode_mudar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x76db00usize);
    // 0x53b620: push byte +0x3d; add esp, 4 -> cdecl; 0x62c400: push j; mov ecx, [0xb63d28] -> thiscall ret 4 (al);
    // 0x7a7eb0: push j; mov ecx, [0xb7434c] -> stdcall ret 4 (al); 0x547d30: push clube; push j; add esp, 0xc (junto
    // com o da 0x53b3d0) -> cdecl; 0x4cf5a0: push j; mov ecx, [0xae1050] -> thiscall ret 4; 0x55d270: push 0; push 1;
    // push [p + 0x14]; push j; mov ecx, [0xae28f0] -> thiscall ret 0x10; 0x76db00: 6 pushes (nação, 0, j, 0, 0, 0);
    // mov ecx, this -> thiscall ret 0x18
    if j == 0 || tipo_ok(cqc12_b(j + 0x3d) as u32) == 0 || lesao(cqc12_d(0xb63d28), j) != 0 {
        return false;
    }
    if cqc12_d(0xb7434c) != 0 && bits(j) != 0 {
        return false;
    }
    if cqc12_d(j + 0x39) == 0 || nao_gosta(j, principal(cqc12_d(j + 0x1a))) != 0 {
        return false;
    }
    let tj = cqc12_d(j + 0x24);
    if tj != 0 && (cqc12_b(j + 0x28) != 0xb || cqc12_d(tj + 0x53) != nacao) {
        return false;
    }
    if p != 0 {
        let k = cqc12_d(p);
        let c = cqc12_d(j + 0x39);
        let outra_nacao = |c: usize| c != 0 && cqc12_d(c + 0x53) != 0 && cqc12_d(cqc12_d(c + 0x53) + 0x71) != cqc12_d(nacao + 0x71);
        let mut contrato_ok = true;
        if k == cqc12_d(0x9cf668) || k == cqc12_d(0x9cf794) || k == cqc12_d(0x9cf798) {
            if outra_nacao(c) {
                // amistoso, B e sub-21 internacionais: clube de outra nação só com reputação (+0x80) até 0.8 da do
                // adversário (o mandante +0x1c é o próprio time: o visitante +0x20); fora dos amistosos, não
                if k != cqc12_d(0x9cf668) {
                    return false;
                }
                let s = cqc12_d(p + 0x1c);
                let outro = if s == elenco { cqc12_d(p + 0x20) } else { s };
                if cqc12_w(c + 0x80) as f64 * 0.8 > cqc12_w(outro + 0x80) as f64 {
                    return false;
                }
            }
            // clube com jogo (a tabela +0x24 pelo id do clube) e contrato com +0x3d acima de −10: não
            if cqc12_b(cqc12_d(this + 0x24).wrapping_add(cqc12_d(c))) != 0 {
                let x = contrato(cqc12_d(0xae1050), j);
                if x != 0 && (cqc12_b(contrato(cqc12_d(0xae1050), j) + 0x3d) as i8) > -10 {
                    contrato_ok = false;
                }
            }
            if !contrato_ok {
                return false;
            }
        } else if outra_nacao(c) && cqc12_d(c + 0x57) != 0
            && (cqc12_w(cqc12_d(p + 0x14) + 0x69) as f64) < cqc12_w(cqc12_d(c + 0x57) + 0x69) as f64 * 0.7
        {
            // nas outras competições, clube de outra nação cuja liga (+0x57) tem +0x69 acima da da competição / 0.7
            return false;
        }
        if suspenso(cqc12_d(0xae28f0), j, cqc12_d(p + 0x14), 1, 0) != 0 {
            return false;
        }
        if t == 0 {
            let m = cqc12_uw(p + 0x4d) & 0x300;
            let tj = cqc12_d(j + 0x24);
            if m != 0x100 && tj == elenco && cqc12_b(j + 0x5e) & 8 != 0 {
                return false;
            }
            if m != 0x200 && tj != 0 && tj != elenco {
                return false;
            }
        }
    }
    if cqc12_d(j + 0x24) == principal(nacao) && cqc12_b(j + 0x5e) & 4 != 0 {
        if pode_mudar(this, nacao, 0, j, 0, 0, 0) == 0 {
            return false;
        }
        if !(p != 0 && cqc12_d(p) == cqc12_d(0x9cf7a0)) && t != 0 {
            return false;
        }
    }
    // com a marca, só depois da data do elenco do tipo (ano +2 sem sinal, dia +0 com sinal)
    let depois = |a: usize| {
        let (ya, da) = (cqc12_uw(data + 2), cqc12_w(data));
        let (yr, dr) = (cqc12_uw(a + 2), cqc12_w(a));
        ya > yr || (ya == yr && da > dr)
    };
    if cqc12_b(j + 0x5e) & 8 != 0 && !depois(reg + 0x4e) {
        return false;
    }
    let tj = cqc12_d(j + 0x24);
    if tj == sub21(nacao) && cqc12_b(j + 0x5e) & 4 != 0 && !depois(reg + 0x62) {
        return false;
    }
    if t == 2 && cqc12_b(j + 0x18) as i8 > 0x17 { // cqc12: a idade do sub-21
        return false;
    }
    let nat = cqc12_d(j + 0x1a);
    if nat != nacao {
        // de outra nação: sem time, sem a marca +0x22, sem o bit 2 do +0xc do registro de 79 bytes, com a segunda
        // nação (+0x1e) nesta; entra se esta tiver +0x8e maior, ou (clube da mesma confederação +0x71 ou sem clube)
        // com +0x85 de 5 para cima
        if tj != 0 || cqc12_b(j + 0x22) != 0 || cqc12_b(cqc12_d(0xae2d0c).wrapping_add(cqc12_d(j).wrapping_mul(0x4f)) + 0xc) & 2 != 0
            || cqc12_d(j + 0x1e) != nacao || nat == 0
        {
            return false;
        }
        let c = cqc12_d(j + 0x39);
        let n2 = if c != 0 { cqc12_d(c + 0x53) } else { 0 };
        if cqc12_w(nacao + 0x8e) <= cqc12_w(nat + 0x8e) {
            if n2 != 0 && cqc12_d(nacao + 0x71) != cqc12_d(n2 + 0x71) {
                return false;
            }
            if (cqc12_b(nacao + 0x85) as i8) < 5 {
                return false;
            }
        }
    }
    let v = cqc12_w(cqc12_d(j + 0x61) + 9);
    v > lim.0 && v <= lim.1
}

/// A lista da escala (0x76cae5..0x76cfe6): as reservas dos tipos da faixa, grupo a grupo; devolve a quantidade (byte,
/// como a original) e deixa em `cursor` o ponteiro da última entrada percorrida mais 4 (a original o lê depois).
#[allow(clippy::too_many_arguments)]
unsafe fn collect_squad_candidates(this: usize, nacao: usize, t: i8, data: usize, p: usize, elenco: usize, reg: usize, lim: (i16, i16), cand: &mut [usize; 200], cursor: &mut usize) -> u8 {
    let mut n = 0u8;
    let (lo, hi) = reserve_type_range(t, p);
    for k in lo..=hi {
        for g in 0..4 {
            let (v, ini, fim) = reserve_group(this, reg, k, g);
            *cursor = v;
            for _ in ini..fim {
                let j = cqc12_d(*cursor);
                if is_squad_candidate(this, nacao, t, data, p, elenco, reg, lim, j) {
                    // ponytail: a original grava em local_32c[(char) n]; de 128 em diante ela escreveria antes do vetor
                    if (n as i8) >= 0 {
                        cand[n as usize] = j;
                    }
                    n = n.wrapping_add(1);
                }
                *cursor += 4;
            }
        }
    }
    n
}

/// { 0x76c120 } (thiscall, ret 0x14, national_teams.cpp, ~4,9 KB, quadro SEH só das datas locais, sem destrutor):
/// `NationalTeams::select_squad(nação, tipo, data, partida, forçado)`, pela convocação (0x76bab0, em Rust: nação +0x53
/// do lado, tipo, 0xae2c90, partida, 0), pela montagem das tabelas (0x76a840: partida 0) e pelas notícias (0x784100:
/// forçado 1). Tipo 0 principal, 1 B/olímpico (marca 8, time da tática 1), 2 sub-21 (0x53b4f0). Sem `forçado`: nas
/// fases finais (+0x14 da partida numa das 9 chaves) um item (time, competição, data do jogo − 10, 1000) entra na lista
/// em +0 (0x68d0d0, criada com 0x68d020 se faltar); a data do elenco do tipo (+0x3a do registro: hoje − 1 sem partida,
/// a do jogo nos tipos 0 e acima de 2, a do jogo − 1 nos tipos 1 e 2, que antes marcam as duas nações no vetor
/// [this + 0x36] nos Jogos Olímpicos ou dispensam o sub-21 marcado, 0x76d880) e a de hoje (+0x32); as idades e o
/// tamanho (0x76d430 em +0x42/+0x43). Com técnico humano que decide (0x600a50, 0x5fd6b0; ou pela seleção principal
/// de uma seleção B sem técnico): os machucados por mais de 10 dias (0x62c400 = 2, 0x62ed70 +0x2e) saem do elenco; se
/// ainda for dele, o pedido ao técnico (0x76f1a0) e fim. Senão: os códigos das vagas (squad_position_codes, com a correção
/// de 23 e 25); saem do elenco os de outro time, os com a marca e, no tipo 0 com o bit 1 de +0x4e da partida, os sem
/// ela (0x896c60 e +0x5e/+0x24/+0x28); a lista das reservas (collect_squad_candidates) vai à escala (0x6e2000, modo 0x2100914
/// no tipo 1, 0x2000814 no 0 e 0x814 no 2); cada escalado que serve na vaga (0x5b7d20) entra no elenco: sai do time
/// anterior (vaga, 0x76db00, 0x896c60) ou das vagas da tática do próprio (0x8968f0/0x896e90, 0x76db00), ganha a marca
/// (0x540980), o time e +0x28 (0xc de 5 ou 0xc, senão 0xb) e o +9 do registro de 79 bytes vai a 0..100. Por fim o
/// elenco é ordenado (0x945333 com 0x778ef0), a notícia (0x76e0b0) e as vagas da tática com quem não é do time saem.
/// As caixas de erro (linhas 0x748 e 0x76b) como a original.
unsafe extern "thiscall" fn select_squad(this: usize, nacao: usize, tipo: u32, data: usize, p: usize, forcado: u32) {
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let inserir: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    let dispensar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x76d880usize);
    let idades: unsafe extern "stdcall" fn(usize, u32, usize, usize) = core::mem::transmute(0x76d430usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x5fd6b0usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let dias_de_lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x896c60usize);
    let pedir: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x76f1a0usize);
    let pick: unsafe extern "C" fn(usize, u32, usize, u32, u32, usize, u32, usize, u32, u32, u32, u32, u32, u32, u32) -> u8 =
        core::mem::transmute(0x6e2000usize);
    let mascara: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x5b7d20usize);
    let pode_mudar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x76db00usize);
    let da_vaga: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(0x8968f0usize);
    let limpar_vaga: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x896e90usize);
    let marcar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x540980usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32, u32) = core::mem::transmute(0x76e0b0usize);
    // 0x549ef0: push +0x28; push +0x2a; lea ecx, saida -> thiscall ret 8 (palavras); 0x54c8e0: push n; push saida; mov
    // ecx, data -> thiscall ret 8; 0x417c10: push origem; push saida; lea ecx, destino -> thiscall ret 8; 0x54a110: lea
    // ecx -> thiscall; 0x944cf1 (new): push 0xe; add esp, 4 -> cdecl; 0x68d020: mov ecx, bloco -> thiscall ret 0;
    // 0x68d0d0: push 0x12; push item; mov ecx, [this] -> thiscall ret 8; 0x76d880: push nação; mov ecx, this ->
    // thiscall ret 4; 0x76d430: 4 pushes (partida, tipo, +0x42, +0x43) -> stdcall ret 0x10; 0x600a50: 5 pushes (time,
    // 0, 1, 0, 0); add esp -> cdecl; 0x5fd6b0: push 0; push técnico; mov ecx, [0xb63c98] -> thiscall ret 8 (al);
    // 0x5409b0: push; add esp, 4 -> cdecl; 0x62ed70: push j; mov ecx, [0xb63d28] -> thiscall ret 4; 0x896c60: push 1;
    // push time; push j; push elenco; mov ecx, [0xde1d08] -> thiscall ret 0x10; 0x76f1a0: 3 pushes (elenco, time,
    // partida); mov ecx, this -> ret 0xc; 0x6e2000: 15 pushes; add esp, 0x3c -> cdecl; 0x5b7d20: push +0x61; add esp,
    // 4 -> cdecl (ax); 0x8968f0 e 0x896e90: push time; push vaga; push elenco; mov ecx, [0xde1d08] -> thiscall ret 0xc;
    // 0x540980: push marca; push j; add esp, 8 -> cdecl; 0x945333 (qsort): 4 pushes; add esp, 0x10 -> cdecl; 0x76e0b0:
    // 7 pushes; mov ecx, this -> ret 0x1c
    let t = tipo as u8 as i8;
    let reg = squad_nation_record(this, nacao);
    let lim = reputation_limits(nacao, p);
    let (elenco, marca, time) = squad_team_for_type(nacao, t);
    let chaves = [0x9cf79cusize, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964];
    let finais = p != 0 && cqc12_d(p + 0x14) != 0 && chaves.iter().any(|&k| cqc12_d(cqc12_d(p + 0x14)) == cqc12_d(k));
    let r = reg.wrapping_add((t as i32 * 0x14) as usize);
    let tatica = || cqc12_d(0xde1d08);
    if forcado == 0 {
        let (mut d88c, mut d87c, mut d8a0) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        let (p88c, p87c, p8a0) = (d88c.as_mut_ptr() as usize, d87c.as_mut_ptr() as usize, d8a0.as_mut_ptr() as usize);
        if finais {
            crate::comparacao::ramo("cqc_12 elenco das fases finais");
            let d = data_nova(p87c, cqc12_uw(p + 0x2a) as u32, cqc12_uw(p + 0x28) as u32);
            menos(d, p88c, 10);
            let comp = cqc12_d(p + 0x14);
            if cqc12_d(this) == 0 {
                let m = novo(0xe);
                cqc12_pd(this, if m != 0 { lista_nova(m) } else { 0 });
            }
            let mut item = [0u8; 0x12];
            let pi = item.as_mut_ptr() as usize;
            padrao(pi + 8);
            cqc12_pd(pi + 4, cqc12_d(comp));
            cqc12_pd(pi, cqc12_d(elenco));
            copia(pi + 8, p87c, p88c);
            core::ptr::write_unaligned((pi + 0x10) as *mut u16, 1000);
            inserir(cqc12_d(this), pi, 0x12);
        }
        if p == 0 {
            let x = menos(data, p88c, 1);
            copia(r + 0x3a, p87c, x);
        } else if t != 1 && t != 2 {
            let x = data_nova(p88c, cqc12_uw(p + 0x2a) as u32, cqc12_uw(p + 0x28) as u32);
            copia(r + 0x3a, p87c, x);
        } else {
            // a nação de cada lado (+0x53 do time) e a casa dela no vetor [this + 0x36]
            let nac = |o: usize| cqc12_d(cqc12_d(p + o) + 0x53);
            let casa = |o: usize| cqc12_d(nac(o)).wrapping_add(cqc12_d(this + 0x36));
            if cqc12_d(p) == cqc12_d(0x9cf7a0) {
                cqc12_pb(casa(0x1c), 1);
                cqc12_pb(casa(0x20), 1);
            } else {
                if cqc12_b(casa(0x1c)) != 0 {
                    dispensar(this, nac(0x1c));
                }
                if cqc12_b(casa(0x20)) != 0 {
                    dispensar(this, nac(0x20));
                }
                cqc12_pb(casa(0x1c), 0);
                cqc12_pb(casa(0x20), 0);
            }
            let x = data_nova(p88c, cqc12_uw(p + 0x2a) as u32, cqc12_uw(p + 0x28) as u32);
            let y = menos(x, p87c, 1);
            copia(r + 0x3a, p8a0, y);
        }
        copia(r + 0x32, p88c, data);
        idades(p, tipo, r + 0x42, r + 0x43);
        let tecnico = cqc12_d(principal(nacao) + 0xcf);
        let decide = || {
            (humano(elenco, 0, 1, 0, 0) != 0 && (t == 0 || t9a06(cqc12_d(0xb63c98), tecnico, 0) != 0))
                || (segunda(elenco) != 0 && cqc12_d(elenco + 0xcf) == 0
                    && humano(principal(cqc12_d(elenco + 0x53)), 0, 1, 0, 0) != 0
                    && t9a06(cqc12_d(0xb63c98), tecnico, 0) != 0)
        };
        if decide() {
            for k in 0..50 {
                let s = elenco + 0xd7 + 4 * k;
                let j = cqc12_d(s);
                if j == 0 || lesao(cqc12_d(0xb63d28), j) != 2 || cqc12_b(j + 0x5e) & marca == 0 {
                    continue;
                }
                let l = dias_de_lesao(cqc12_d(0xb63d28), j);
                if l != 0 && cqc12_w(l + 0x2e) > 10 {
                    tirar(tatica(), elenco, j, time, 1);
                    cqc12_pb(j + 0x5e, cqc12_b(j + 0x5e) & 3);
                    cqc12_pd(j + 0x24, 0);
                    cqc12_pb(j + 0x28, if cqc12_b(j + 0x28) == 0xc { 5 } else { 0 });
                    cqc12_pd(s, 0);
                }
            }
        }
        if decide() {
            crate::comparacao::ramo("cqc_12 elenco pelo técnico humano");
            cqc12_pb(r + 0x44, 0);
            pedir(this, elenco, time, p);
            return;
        }
    }
    let tam = r + 0x42;
    let codigos = squad_position_codes(cqc12_b(tam));
    // saem do elenco: os de outro time; os com a marca; no tipo 0 com o bit 1 de +0x4e da partida, os sem ela
    for k in 0..50 {
        let s = elenco + 0xd7 + 4 * k;
        let j = cqc12_d(s);
        if j == 0 {
            continue;
        }
        if cqc12_d(j + 0x24) != elenco {
            tirar(tatica(), elenco, j, time, 1);
        } else {
            let antigo = if cqc12_b(j + 0x5e) & marca != 0 {
                5
            } else if t == 0 && cqc12_b(p + 0x4e) & 1 != 0 {
                0xb
            } else {
                continue;
            };
            tirar(tatica(), elenco, j, time, 1);
            cqc12_pb(j + 0x5e, cqc12_b(j + 0x5e) & 3);
            cqc12_pd(j + 0x24, 0);
            cqc12_pb(j + 0x28, if cqc12_b(j + 0x28) == 0xc { antigo } else { 0 });
        }
        cqc12_pd(s, 0); // cqc12: a vaga do elenco fica livre
    }
    let mut cand = [0usize; 200];
    let mut cursor = 0usize;
    let n = collect_squad_candidates(this, nacao, t, data, p, elenco, reg, lim, &mut cand, &mut cursor);
    let modo: u32 = if t == 1 { 0x2100914 } else if t != 2 { 0x2000814 } else { 0x814 };
    let mut saida = [0usize; 50];
    crate::comparacao::ramo("cqc_12 escala do elenco");
    pick(cand.as_ptr() as usize, n as u32, codigos.as_ptr() as usize, (modo & !0xff) | cqc12_b(tam) as u32, 0,
        saida.as_mut_ptr() as usize, modo, elenco, 0, 0, 0, 0, 0, 0, 0);
    let mut i = 0usize;
    // ponytail: a original não limita a 50 (o jogo pede até 26)
    while (i as i32) < cqc12_b(tam) as i8 as i32 && i < 50 {
        let j = saida[i];
        if j != 0 && codigos[i] & mascara(cqc12_d(j + 0x61)) != 0 {
            if let Some(k) = (0..50).find(|&k| cqc12_d(elenco + 0xd7 + 4 * k) == 0) {
                cqc12_pd(elenco + 0xd7 + 4 * k, j);
            }
            let tj = cqc12_d(j + 0x24);
            if tj != elenco && tj != 0 {
                // de outro time: sai da vaga de lá (com a permissão, a marca de convocado do tipo dele zera) e da tática
                let v = ((cqc12_b(j + 0x5e) as i8 as i32 & 8) >> 3) as u32;
                let c3: u8 = if segunda(tj) == 0 { (v != 0) as u8 } else if v != 0 { 1 } else { 2 };
                let mut c = cqc12_d(j + 0x24) + 0xd7;
                for _ in 0..50 {
                    if cqc12_d(c) == j {
                        cqc12_pd(c, 0);
                        if pode_mudar(this, cqc12_d(cqc12_d(j + 0x24) + 0x53), c3 as u32, 0, 0, 0, 0) != 0 {
                            cqc12_pb(reg + 0x43 + c3 as usize * 0x14, 0);
                            break;
                        }
                        show_error_at(0xa5b2a4, 0x748);
                    }
                    c += 4;
                }
                cursor = c;
                tirar(tatica(), cqc12_d(j + 0x24), j, v, 1);
            } else if tj == elenco && cqc12_b(j + 0x5e) & marca == 0 {
                // já do time sem a marca: sai das vagas da tática do time (time da tática invertido)
                let bz = (time == 0) as u32;
                let c3: u8 = if segunda(elenco) == 0 { (bz != 0) as u8 } else if bz != 0 { 1 } else { 2 };
                for v in 0..20 {
                    if da_vaga(tatica(), cqc12_d(j + 0x24), v, bz) != j {
                        continue;
                    }
                    limpar_vaga(tatica(), cqc12_d(j + 0x24), v, bz);
                    // a original passa como jogador o dword do último ponteiro de percurso (cursor): reproduzido
                    if c3 != 0 && pode_mudar(this, cqc12_d(cqc12_d(j + 0x24) + 0x53), c3 as u32, cqc12_d(cursor), 0, 0, 0) != 0 {
                        cqc12_pb(reg + 0x43 + c3 as usize * 0x14, 0);
                        break;
                    }
                    show_error_at(0xa5b2a4, 0x76b);
                }
            }
            marcar(j, marca as u32);
            cqc12_pd(j + 0x24, elenco); // cqc12: o convocado ganha o time
            let x = cqc12_b(j + 0x28);
            cqc12_pb(j + 0x28, if x == 5 || x == 0xc { 0xc } else { 0xb });
            let e = cqc12_d(0xae2d0c).wrapping_add(cqc12_d(j).wrapping_mul(0x4f)) + 9;
            cqc12_pb(e, (cqc12_b(e) as i8).clamp(0, 100) as u8);
        }
        i += 1;
    }
    ordenar(elenco + 0xd7, 0x32, 4, 0x778ef0);
    let chave = if p == 0 { u32::MAX } else { cqc12_d(cqc12_d(p + 0x14)) as u32 };
    noticia(this, elenco, time, p, chave, finais as u32, 0, 0);
    for v in 0..20 {
        let x = da_vaga(tatica(), elenco, v, time);
        if x != 0 && cqc12_d(x + 0x24) != elenco {
            limpar_vaga(tatica(), elenco, v, time);
        }
    }
}

/// O que a escala da 0x76c120 lê e a original muda depois dela: o registro da nação (0x6e bytes) e, de cada jogador
/// das reservas percorridas, +0x24..+0x28 (time, estado) e +0x5e (marcas de elenco).
unsafe fn squad_selection_regions(this: usize, nacao: usize, t: i8, p: usize) -> Vec<(usize, usize)> {
    let reg = squad_nation_record(this, nacao);
    let mut v = vec![(reg, 0x6e)];
    let (lo, hi) = reserve_type_range(t, p);
    for k in lo..=hi {
        for g in 0..4 {
            let (a, ini, fim) = reserve_group(this, reg, k, g);
            for i in 0..(fim - ini).max(0) as usize {
                let j = cqc12_d(a + 4 * i);
                if j != 0 {
                    v.push((j + 0x24, 5));
                    v.push((j + 0x5e, 1));
                }
            }
        }
    }
    v
}

/// Comparador: a original aloca (0x68d020/0x68d0d0), mexe nos elencos, nas vagas da tática e manda notícias (0x76e0b0,
/// 0x76f1a0): não pode rodar duas vezes. Só ela roda, com uma gravação da fita aberta nas regiões de squad_selection_regions:
/// a escala (0x6e2000) entra nela como unidade com os vetores de entrada (a lista e os códigos) e o estado das
/// regiões logo antes. Depois, com esse estado de volta, a cópia Rust monta a lista (collect_squad_candidates) e os códigos
/// (squad_position_codes), que têm de bater com os da original, junto com a quantidade, o modo e o time; o estado de depois
/// da original volta no fim. Nos elencos de 23 e 25 a original passa a sobra da pilha nas vagas a partir da 22ª: elas
/// ficam fora (correção conhecida, o ramo); as 22 primeiras e o resto conferem. Sem escala (técnico humano), nada a
/// comparar. Dentro de uma gravação de fora, só a original.
unsafe extern "thiscall" fn comparar_select_squad(this: usize, nacao: usize, tipo: u32, data: usize, p: usize, forcado: u32) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32)>(%%I3%%);
    if is_recording() {
        return o(this, nacao, tipo, data, p, forcado);
    }
    let t = tipo as u8 as i8;
    let regioes = squad_selection_regions(this, nacao, t, p);
    let externa = tape_lock().take();
    *tape_lock() = Some((regioes.clone(), Vec::new()));
    let nivel = DENTRO.swap(0, Relaxed);
    o(this, nacao, tipo, data, p, forcado);
    let gravadas = tape_lock().take().map_or(Vec::new(), |f| f.1);
    DENTRO.store(nivel, Relaxed);
    *tape_lock() = externa;
    let Some((_, _, a, _, antes, _, (ent, _))) = gravadas.iter().find(|c| c.0 == 0x6e2000) else {
        return;
    };
    let depois = crate::comparacao::Memoria::ler(&regioes);
    let sorteio = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    antes.escrever();
    let reg = squad_nation_record(this, nacao);
    let r = reg.wrapping_add((t as i32 * 0x14) as usize);
    let tam = cqc12_b(r + 0x42);
    let (elenco, _, _) = squad_team_for_type(nacao, t);
    let modo: u32 = if t == 1 { 0x2100914 } else if t != 2 { 0x2000814 } else { 0x814 };
    let mut cand = [0usize; 200];
    let mut cursor = 0usize;
    let (n, codigos) = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| {
        let n = collect_squad_candidates(this, nacao, t, data, p, elenco, reg, reputation_limits(nacao, p), &mut cand, &mut cursor);
        (n, squad_position_codes(tam))
    }));
    sorteio.escrever();
    depois.escrever();
    let n1 = (a[1] as u8 as i8).max(0) as usize;
    let dw = |k: usize| u32::from_le_bytes([ent[k], ent[k + 1], ent[k + 2], ent[k + 3]]) as usize;
    let lista_o: Vec<usize> = (0..n1.min(ent.len() / 4)).map(|k| dw(4 * k)).collect();
    let cod_o: Vec<u16> = ent.get(4 * n1..).unwrap_or(&[]).chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    let corrigido = tam == 23 || tam == 25;
    let lim = if corrigido { 22.min(cod_o.len()) } else { cod_o.len() };
    let igual = a[1] as u8 == n && a[2] == modo && a[3] as usize == elenco && cod_o.len() == tam as usize
        && lista_o[..] == cand[..n1.min(200)] && cod_o[..lim] == codigos[..lim];
    PLACAR.registrar(igual, || format!(
        "select_squad(nação {:x}, tipo {}, tam {}) original: n {} modo {:#x} time {:x} lista {:x?} códigos {:x?}; rust: n {} modo {:#x} time {:x} lista {:x?} códigos {:x?}",
        nacao, t, tam, a[1] as u8, a[2], a[3], lista_o, cod_o, n, modo, elenco, &cand[..(n as i8).max(0) as usize], &codigos[..tam as usize]));
}
