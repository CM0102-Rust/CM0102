// ---------- 256a leva: o ranking da FIFA no mês: os jogos de uma seleção no período, a atualização do dia 18 e a ordem dos resultados por pontos (fifa_rankings.cpp) ----------


unsafe fn cpt1_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}


/// counts the results of the national side's history between two months (year × 12 + month), from the end, for the work
/// permit
/// Original: `FIFA_RANKINGS::count_results_in_period` (0x58d710, fifa_rankings.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x58d710 } (thiscall, ret 0x14, fifa_rankings.cpp): `FIFA_RANKINGS::count_results_in_period(nação, mês_de, ano_de,
/// mês_até, ano_até)` (pela permissão de trabalho 0x8c7950, com os dois anos até hoje): na entrada da nação (id [nação],
/// 0x2c bytes na tabela [this]), o histórico (+4, entradas de 5 bytes: pontos, mês +2, ano +3; conta +0xa) do fim para o
/// começo; ano × 12 + mês antes do começo do período para a busca, até o fim conta um. Em 16 bits. Chamada a cada
/// permissão que chega aos jogos da seleção: sem contador.
unsafe extern "thiscall" fn jogos_da_selecao_no_periodo(this: usize, nacao: usize, mes_de: u32, ano_de: u32, mes_ate: u32, ano_ate: u32) -> u16 {
    let de = (ano_de & 0xffff) as i32 * 12 + mes_de as u8 as i8 as i32;
    let ate = (ano_ate & 0xffff) as i32 * 12 + mes_ate as u8 as i8 as i32;
    let e = read_usize(this).wrapping_add((read_usize(nacao) as u32).wrapping_mul(0x2c) as usize);
    let hist = read_usize(e + 4);
    let mut j = cpt1_h(e + 0xa).wrapping_sub(1);
    let mut n = 0u16;
    while j >= 0 {
        let p = hist.wrapping_add(j as isize as usize * 5);
        let v = read_u16(p + 3) as i32 * 12 + *((p + 2) as *const u8) as i32;
        if v < de {
            break;
        }
        if v <= ate { // cpt1: ate o mes final
            n = n.wrapping_add(1); // cpt1: mais um jogo
        }
        j = j.wrapping_sub(1);
    }
    n // cpt1: os jogos no periodo
}

unsafe extern "thiscall" fn comparar_jogos_da_selecao_no_periodo(this: usize, nacao: usize, mes_de: u32, ano_de: u32, mes_ate: u32, ano_ate: u32) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u16>(1155)(this, nacao, mes_de, ano_de, mes_ate, ano_ate);
    let n = jogos_da_selecao_no_periodo(this, nacao, mes_de, ano_de, mes_ate, ano_ate);
    PLACAR.registrar(r == n, || format!("jogos_da_selecao_no_periodo(nação {}, {}/{} a {}/{}) original={} rust={}", read_usize(nacao), mes_de as u8, ano_de as u16, mes_ate as u8, ano_ate as u16, r, n));
    r
}

/// on the 18th: drops the results older than a year, sorts the histories, rolls the yearly totals and redoes scores and
/// positions
/// Original: `FIFA_RANKINGS::update_rankings` (0x58d460, fifa_rankings.cpp, thiscall, ret 4).
///
/// [pt] { 0x58d460 } (thiscall, ret 4, fifa_rankings.cpp): `FIFA_RANKINGS::update_rankings(data)` (pelo dia das seleções
/// 0x76b100): só no dia 18 (0x54cf70). Guarda a data em +5 (0x417c10) e, em cada nação ([0xae2350] entradas), tira do
/// histórico os resultados de mais de um ano (ano + 1 antes do de hoje, ou o mesmo ano com o mês antes do de hoje,
/// 0x54cfb0), pondo o último no lugar; o laço vai do byte baixo da conta − 1 até 0, como na original. Depois ordena o
/// histórico pelos pontos (qsort 0x945333 com 0x58e220). No mês guardado em +4, com o ano diferente do de [0xdd6ec8],
/// os totais dos anos andam uma casa (+0x14 → +0x28) e +0x14 zera. Por fim as pontuações (0x58dfc0) e as posições:
/// a ordem (0x58d610) dá a posição + 1 em +0x10, com a anterior em +0x12; o vetor da ordem é liberado (0x9452ca).
unsafe extern "thiscall" fn atualizar_ranking_da_fifa(this: usize, data: usize) {
    let dia: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_DATE_GET_DAY_OF_MONTH);
    let mes_de: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_DATE_GET_MONTH);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let pontuar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FIFA_RANKINGS_RECALCULATE_SCORES);
    let ordem: unsafe extern "thiscall" fn(usize, *mut i32) -> usize = core::mem::transmute(FN_FIFA_RANKINGS_COLLECT_RANKED_ORDER);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    if dia(data) != 0x12 {
        return;
    }
    let mes = mes_de(data);
    let mut guardada = [0u8; 8];
    copiar(this + 5, guardada.as_mut_ptr() as usize, data);
    let ano = read_u16(data + 2) as i32;
    let mut i = 0i32;
    while i < read_usize(0xae2350) as i32 {
        let e = read_usize(this) + i as usize * 0x2c;
        let mut k = (*((e + 0xa) as *const u8)).wrapping_sub(1) as i8 as i32;
        while k >= 0 {
            let hist = read_usize(e + 4);
            let p = hist + k as usize * 5;
            let y = read_u16(p + 3) as i32 + 1;
            if y < ano || (y == ano && *((p + 2) as *const u8) < mes) {
                let ultimo = hist + cpt1_h(e + 0xa) as isize as usize * 5 - 5;
                core::ptr::copy(ultimo as *const u8, p as *mut u8, 5);
                *((e + 0xa) as *mut i16) = cpt1_h(e + 0xa).wrapping_sub(1);
            }
            k -= 1;
        }
        ordenar(read_usize(e + 4), cpt1_h(e + 0xa) as i32 as u32, 5, 0x58e220);
        i += 1;
    }
    if mes == *((this + 4) as *const u8) && read_u16(data + 2) != read_u16(0xdd6eca) {
        let mut i = 0i32;
        while i < read_usize(0xae2350) as i32 {
            let e = read_usize(this) + i as usize * 0x2c;
            for o in [0x24usize, 0x20, 0x1c, 0x18, 0x14] {
                *((e + o + 4) as *mut u32) = *((e + o) as *const u32);
            }
            *((e + 0x14) as *mut u32) = 0;
            i += 1;
        }
    }
    pontuar(this);
    // a original guarda a conta no lugar do argumento (a data): sem o vetor, a conta fica sendo o ponteiro
    let mut n = data as i32;
    let v = ordem(this, &mut n);
    let mut k = 0i32;
    while k < n {
        let r = read_usize(v + k as usize * 4);
        *((r + 0x12) as *mut u16) = read_u16(r + 0x10);
        *((r + 0x10) as *mut u16) = (k + 1) as u16;
        k += 1;
    }
    liberar(v);
}

/// Comparador: lado a lado pelo objeto (+0..+0xd), pela tabela das nações e por cada histórico (capacidade +8 × 5);
/// a ordem é alocada e liberada pelas duas.
unsafe extern "thiscall" fn comparar_atualizar_ranking_da_fifa(this: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1156);
    let t = read_usize(this);
    let n = read_usize(0xae2350);
    let mut reg = vec![(this, 0xd), (t, n * 0x2c)];
    for i in 0..n {
        let e = t + i * 0x2c;
        let h = read_usize(e + 4);
        let cap = cpt1_h(e + 8) as i32;
        if h != 0 && cap > 0 {
            reg.push((h, cap as usize * 5));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, data), || atualizar_ranking_da_fifa(this, data));
    PLACAR.registrar(igual, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).flat_map(|(a, b)| (0..a.1.len()).filter(move |&i| a.1[i] != b.1[i]).map(move |i| a.0 + i)).take(16).collect();
        format!("atualizar_ranking_da_fifa(dia {}) diferenças em {:x?}", read_u16(data), dif)
    });
}

/// qsort order of the histories: the points of b minus those of a (highest first)
/// Original: `compare_result_points_desc` (0x58e220, fifa_rankings.cpp, cdecl).
///
/// [pt] { 0x58e220 } (cdecl, fifa_rankings.cpp): `compare_result_points_desc(a, b)`, a ordem do qsort dos históricos na
/// atualização do ranking: os pontos (palavra +0, com sinal) de `b` menos os de `a`.
unsafe extern "C" fn ordem_dos_resultados_por_pontos(a: usize, b: usize) -> i32 {
    cpt1_h(b) as i32 - cpt1_h(a) as i32
}

unsafe extern "C" fn comparar_ordem_dos_resultados_por_pontos(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1157)(a, b);
    let n = ordem_dos_resultados_por_pontos(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_resultados_por_pontos({}, {}) original={} rust={}", cpt1_h(a), cpt1_h(b), r, n));
    r
}

// ---------- 256a leva: o ranking da FIFA no jogo: o resultado no histórico da seleção, a faixa de força da seleção e as pontuações do ranking (fifa_rankings.cpp) ----------


unsafe fn cpt2_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}


/// A entrada da nação `nacao` (id em [nacao]) na tabela do ranking [this] (0x2c bytes cada).
unsafe fn cpt2_entrada(this: usize, nacao: usize) -> usize {
    read_usize(this).wrapping_add((read_usize(nacao) as u32).wrapping_mul(0x2c) as usize)
}

/// O mês e o ano do jogo `partida`: a data (0x549ef0) do dia do ano (+0x2a) e do ano (+0x28), o mês dela (0x54cfb0).
unsafe fn cpt2_mes_e_ano(partida: usize) -> (u8, u16) {
    let data_nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let mes_de: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_DATE_GET_MONTH);
    let mut d = [0u8; 8];
    let p = data_nova(d.as_mut_ptr(), read_u16(partida + 0x2a) as u32, read_u16(partida + 0x28) as u32);
    (mes_de(p), read_u16(partida + 0x28))
}

/// appends the points, month and year of the match to the national side's history, growing the array by 5 entries
/// Original: `FIFA_RANKINGS::append_result` (0x58dea0, fifa_rankings.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x58dea0 } (thiscall, ret 0xc, fifa_rankings.cpp, SEH só da data local, de destrutor vazio):
/// `FIFA_RANKINGS::append_result(nação, pontos, partida)` (pelo resultado internacional 0x58cf70, para cada seleção):
/// com a conta (+0xa) igual à capacidade (+8), a capacidade cresce 5 e o histórico (+4) é realocado (0x945667) ou
/// alocado (0x944e46) com capacidade × 5 bytes. A entrada nova, no lugar da conta antiga: os pontos (16 bits), o mês do
/// jogo (+2) e o ano (+3).
unsafe extern "thiscall" fn resultado_no_historico(this: usize, nacao: usize, pontos: u32, partida: usize) {
    let e = cpt2_entrada(this, nacao);
    if read_u16(e + 0xa) == read_u16(e + 8) {
        *((e + 8) as *mut u16) = read_u16(e + 8).wrapping_add(5);
        let h = read_usize(e + 4);
        let n = cpt2_h(e + 8) as i32 * 5;
        let novo = if h != 0 {
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
            realocar(h, n as u32)
        } else {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
            alocar(n as u32)
        };
        *((e + 4) as *mut usize) = novo;
    }
    let c = cpt2_h(e + 0xa);
    *((e + 0xa) as *mut i16) = c.wrapping_add(1); // cpt2: mais um resultado
    let p = read_usize(e + 4).wrapping_add(c as isize as usize * 5);
    *(p as *mut u16) = pontos as u16;
    let (mes, ano) = cpt2_mes_e_ano(partida);
    *((p + 2) as *mut u8) = mes; // cpt2: o mes do jogo
    core::ptr::write_unaligned((p + 3) as *mut u16, ano); // cpt2: o ano do jogo
    crate::comparacao::ramo("cpt_2 resultado");
}

// ponytail: a original pode realocar o histórico (não roda duas vezes): o plano antes (conta, capacidade, pontos, mês e
// ano), a original roda e a entrada nova é conferida; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_resultado_no_historico(this: usize, nacao: usize, pontos: u32, partida: usize) {
    let e = cpt2_entrada(this, nacao);
    let (conta, cap) = (cpt2_h(e + 0xa), cpt2_h(e + 8));
    let (mes, ano) = cpt2_mes_e_ano(partida);
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(1158)(this, nacao, pontos, partida);
    let p = read_usize(e + 4).wrapping_add(conta as isize as usize * 5);
    let cap_n = if conta == cap { cap.wrapping_add(5) } else { cap };
    let lido = (cpt2_h(e + 0xa), cpt2_h(e + 8), read_u16(p), *((p + 2) as *const u8), read_u16(p + 3));
    let plano = (conta.wrapping_add(1), cap_n, pontos as u16, mes, ano);
    PLACAR.registrar(lido == plano, || format!("resultado_no_historico(nação {}, pontos {}) original={:?} plano={:?}", read_usize(nacao), pontos as u16, lido, plano));
}

/// the national side's strength tier from 1 to 4, by reputation and ranking position
/// Original: `FIFA_RANKINGS::get_nation_strength_tier` (0x58e190, fifa_rankings.cpp, thiscall, ret 4).
///
/// [pt] { 0x58e190 } (thiscall, ret 4, fifa_rankings.cpp): `FIFA_RANKINGS::get_nation_strength_tier(nação)` (duas vezes por
/// resultado internacional 0x58cf70): reputação (+0x8e) abaixo de 3000 e posição (+0x10 da entrada) abaixo de 50: 3;
/// reputação abaixo de 6000 e posição abaixo de 15: 2; senão pela posição: abaixo de 21, 1; de 61, 2; de 101, 3; 4.
unsafe extern "thiscall" fn faixa_da_selecao(this: usize, nacao: usize) -> u8 {
    let rep = cpt2_h(nacao + 0x8e);
    let pos = || cpt2_h(cpt2_entrada(this, nacao) + 0x10);
    if rep < 0xbb8 && pos() < 0x32 {
        return 3;
    }
    if rep < 0x1770 && pos() < 0xf {
        return 2;
    }
    let p = pos();
    if p < 0x15 {
        1
    } else if p < 0x3d {
        2
    } else if p >= 0x65 {
        4
    } else {
        3
    }
}

unsafe extern "thiscall" fn comparar_faixa_da_selecao(this: usize, nacao: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1159)(this, nacao);
    let n = faixa_da_selecao(this, nacao);
    PLACAR.registrar(r == n, || format!("faixa_da_selecao(nação {}) original={} rust={}", read_usize(nacao), r, n));
    r
}

/// the year total from the best eight results and the score from the totals of this year and the five before
/// Original: `FIFA_RANKINGS::recalculate_scores` (0x58dfc0, fifa_rankings.cpp, thiscall).
///
/// [pt] { 0x58dfc0 } (thiscall, fifa_rankings.cpp): `FIFA_RANKINGS::recalculate_scores()` (pela atualização do dia 18
/// 0x58d460): em cada nação, a soma dos pontos dos oito primeiros resultados (o histórico vem ordenado) e a de todos;
/// com mais de oito, o total do ano (+0x14) é (todos × 8 / conta + oito) / 2, senão os oito. Se a data de [0xdd6ec8]
/// passa da de hoje menos 365 dias (0x54c8e0), com menos de nove resultados o total é (+0x18 + oito) / (conta + 8) × 8
/// (sem resultados, o +0x18). A pontuação (+0xc, f32): (+0x14 + 5/6 de +0x18 + 4/6 de +0x1c + 3/6 de +0x20 + 2/6 de
/// +0x24 + 1/6 de +0x28, cada parte truncada) × [0x96a350].
unsafe extern "thiscall" fn pontuacoes_do_ranking(this: usize) {
    let datar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let fator = core::ptr::read_unaligned(0x96a350 as *const f64);
    let mut i = 0i32;
    if read_usize(0xae2350) as i32 <= 0 {
        return;
    }
    loop {
        let e = read_usize(this) + i as usize * 0x2c;
        let conta = cpt2_h(e + 0xa);
        let ponto = |k: i8| cpt2_h(read_usize(e + 4).wrapping_add(k as isize as usize * 5)) as i32;
        let mut oito = 0i32;
        let mut k = 0i8;
        if conta > 0 {
            loop {
                if k >= 8 {
                    break;
                }
                oito = oito.wrapping_add(ponto(k));
                k = k.wrapping_add(1);
                if k as i16 >= conta {
                    break;
                }
            }
        }
        let mut todos = oito;
        while (k as i16) < conta {
            todos = todos.wrapping_add(ponto(k));
            k = k.wrapping_add(1);
        }
        let ano = if conta > 8 { (todos.wrapping_mul(8) / conta as i32 + oito) / 2 } else { oito };
        *((e + 0x14) as *mut i32) = ano;
        let base = core::ptr::read_unaligned(0xdd6ec8 as *const [u8; 8]);
        let mut d = [0u8; 8];
        let r = datar(0xae2c90, &mut d, 0x16d);
        let (by, ry) = (u16::from_le_bytes([base[2], base[3]]), read_u16(r + 2));
        let depois = by > ry || (by == ry && i16::from_le_bytes([base[0], base[1]]) > cpt2_h(r));
        if depois && cpt2_h(e + 0xa) < 9 {
            let c = cpt2_h(e + 0xa);
            let v18 = *((e + 0x18) as *const i32);
            *((e + 0x14) as *mut i32) = if c != 0 { (v18.wrapping_add(oito) / (c as i32 + 8)).wrapping_shl(3) } else { v18 };
        }
        let t = |o: usize| *((e + o) as *const i32);
        let soma = (t(0x24).wrapping_mul(2) / 6)
            .wrapping_add(t(0x20).wrapping_mul(3) / 6)
            .wrapping_add(t(0x1c).wrapping_mul(4) / 6)
            .wrapping_add(t(0x18).wrapping_mul(5) / 6)
            .wrapping_add(t(0x28) / 6)
            .wrapping_add(t(0x14));
        *((e + 0xc) as *mut f32) = (soma as f64 * fator) as f32;
        i += 1;
        if i >= read_usize(0xae2350) as i32 {
            break;
        }
    }
}

unsafe extern "thiscall" fn comparar_pontuacoes_do_ranking(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1160);
    let reg = [(read_usize(this), read_usize(0xae2350) * 0x2c)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || pontuacoes_do_ranking(this));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).collect();
        format!("pontuacoes_do_ranking diferenças nos deslocamentos {:x?}", dif)
    });
}

// ---------- 256a leva: a nação-chave da competição e a ordem do ranking da FIFA: o índice da nação-chave da competição, a ordem das seleções no ranking e a comparação das pontuações (key_nation.cpp, fifa_rankings.cpp) ----------


/// the competition's key nation row: the nation's, 0 for the World Cup, 1 for national side tournaments, 2 for the club
/// world cups, the continent's or 3
/// Original: `key_nation_index_of_comp` (0x669470, key_nation.cpp, cdecl).
///
/// [pt] { 0x669470 } (cdecl, key_nation.cpp): `key_nation_index_of_comp(competição)` (pelo alcance das regras da partida
/// 0x8d2fa0, pela liga do dia 0x6847c0, pelos registros da copa 0x519890 e pela nação-chave do jogo 0x48dd70 da cpt_3):
/// competição de nação (+0x5d): a linha dela na tabela das 35 nações-chave (0xb63d60, 72 bytes), ou 4. Sem nação: Copa
/// do Mundo e eliminatórias, 0; torneio de seleções (0x669ae0), 1; Copa do Mundo de Clubes e Mundial de Clubes da FIFA,
/// 2; senão a linha de continente (nação 0) com o continente da competição (+0x59), ou 3. A original lê +0x5d antes de
/// conferir a competição nula (o caso de erro nunca chega à caixa). Chamada a cada partida: sem contador.
unsafe extern "C" fn indice_da_nacao_chave(comp: usize) -> u8 {
    let nacao = read_usize(comp + 0x5d);
    if nacao != 0 {
        let linha = (0..0x23usize).find(|&k| read_usize(0xb63d60 + 72 * k) == nacao); // cpt4: a busca da nacao
        return match linha {
            Some(k) => k as u8, // cpt4: a linha da nacao
            None => 4,
        };
    }
    let id = read_usize(comp);
    let copa = [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT]; // cpt4: a Copa e as eliminatorias
    if copa.iter().any(|&g| read_usize(g) == id) {
        return 0;
    }
    let internacional: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_IS_INTERNATIONAL_COMP);
    if internacional(comp) != 0 {
        return 1;
    }
    if id == read_usize(KEY_CLUB_COMP_WORLD_CLUB_CUP) || id == read_usize(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP) {
        return 2;
    }
    for k in 0..0x23usize {
        let c = read_usize(comp + 0x59);
        if read_usize(0xb63d60 + 72 * k) == 0 && c != 0 && c == read_usize(0xb63d64 + 72 * k) {
            return k as u8; // cpt4: a linha do continente
        }
    }
    3
}

unsafe extern "C" fn comparar_indice_da_nacao_chave(comp: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(1164)(comp);
    let n = indice_da_nacao_chave(comp);
    PLACAR.registrar(r == n, || format!("indice_da_nacao_chave(competição {}) original={} rust={}", read_usize(comp) as i32, r, n));
    r
}

/// a new array with the ranking entries of the nations with a national side, sorted by score, and the count
/// Original: `FIFA_RANKINGS::collect_ranked_order` (0x58d610, fifa_rankings.cpp, thiscall, ret 4).
///
/// [pt] { 0x58d610 } (thiscall, ret 4, fifa_rankings.cpp): `FIFA_RANKINGS::collect_ranked_order(&conta)` (pela atualização do
/// dia 18 0x58d460 e pela tela do ranking): um vetor (0x944e46) de [0xae2350] ponteiros; sem ele, a caixa "Error"
/// (linha 0x1e3) e 0, sem gravar a conta. As entradas da tabela (0x2c bytes) das nações com seleção (+0x71 da nação,
/// registros de 0x122 bytes em [0xae23a8]), ordenadas pela pontuação (qsort 0x945333 com 0x58e240); a conta em `conta`.
unsafe extern "thiscall" fn ordem_do_ranking(this: usize, conta: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let v = alocar((read_usize(0xae2350) as u32).wrapping_mul(4));
    if v == 0 {
        aviso_em(0x9c74f4, 0x1e3);
        return 0;
    }
    let nacoes = read_usize(0xae23a8);
    let mut n = 0usize;
    let mut i = 0usize;
    while (i as i32) < read_usize(0xae2350) as i32 {
        if read_usize(nacoes + i * 0x122 + 0x71) != 0 {
            *((v + n * 4) as *mut usize) = read_usize(this) + i * 0x2c;
            n += 1;
        }
        i += 1;
    }
    ordenar(v, n as u32, 4, 0x58e240);
    *conta = n as i32;
    v
}

// ponytail: as duas alocam o vetor; a original roda primeiro (o vetor dela volta ao chamador), a cópia depois, e o
// vetor da cópia é liberado (0x9452ca) depois da conferência da conta e dos ponteiros.
unsafe extern "thiscall" fn comparar_ordem_do_ranking(this: usize, conta: *mut i32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, *mut i32) -> usize>(1165)(this, conta);
    if r == 0 {
        return r;
    }
    let mut c = 0i32;
    let v = ordem_do_ranking(this, &mut c);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let igual = v != 0 && c == *conta && (0..c as usize).all(|k| read_usize(r + k * 4) == read_usize(v + k * 4));
    PLACAR.registrar(igual, || format!("ordem_do_ranking conta original={} rust={}", *conta, c));
    liberar(v);
    r
}

/// qsort order of the ranking: the highest score first
/// Original: `compare_ranking_score_desc` (0x58e240, fifa_rankings.cpp, cdecl).
///
/// [pt] { 0x58e240 } (cdecl, fifa_rankings.cpp): `compare_ranking_score_desc(a, b)`, a ordem do qsort da 0x58d610: as
/// pontuações (f32 +0xc das entradas apontadas): a menor que b (ou sem ordem), 1; iguais, 0; maior, −1.
unsafe extern "C" fn ordem_das_pontuacoes(a: usize, b: usize) -> i32 {
    let fa = *((read_usize(a) + 0xc) as *const f32);
    let fb = *((read_usize(b) + 0xc) as *const f32);
    if !(fa >= fb) {
        1
    } else if fa == fb {
        0
    } else {
        -1
    }
}

unsafe extern "C" fn comparar_ordem_das_pontuacoes(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1166)(a, b);
    let n = ordem_das_pontuacoes(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_pontuacoes original={} rust={}", r, n));
    r
}

