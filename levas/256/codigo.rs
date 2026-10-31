// ---------- 256a leva: o ranking da FIFA no mês: os jogos de uma seleção no período, a atualização do dia 18 e a ordem dos resultados por pontos (fifa_rankings.cpp) ----------

unsafe fn cpt1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpt1_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpt1_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// { 0x58d710 } (thiscall, ret 0x14, fifa_rankings.cpp): `FIFA_RANKINGS::count_results_in_period(nação, mês_de, ano_de,
/// mês_até, ano_até)` (pela permissão de trabalho 0x8c7950, com os dois anos até hoje): na entrada da nação (id [nação],
/// 0x2c bytes na tabela [this]), o histórico (+4, entradas de 5 bytes: pontos, mês +2, ano +3; conta +0xa) do fim para o
/// começo; ano × 12 + mês antes do começo do período para a busca, até o fim conta um. Em 16 bits. Chamada a cada
/// permissão que chega aos jogos da seleção: sem contador.
unsafe extern "thiscall" fn jogos_da_selecao_no_periodo(this: usize, nacao: usize, mes_de: u32, ano_de: u32, mes_ate: u32, ano_ate: u32) -> u16 {
    let de = (ano_de & 0xffff) as i32 * 12 + mes_de as u8 as i8 as i32;
    let ate = (ano_ate & 0xffff) as i32 * 12 + mes_ate as u8 as i8 as i32;
    let e = cpt1_d(this).wrapping_add((cpt1_d(nacao) as u32).wrapping_mul(0x2c) as usize);
    let hist = cpt1_d(e + 4);
    let mut j = cpt1_h(e + 0xa).wrapping_sub(1);
    let mut n = 0u16;
    while j >= 0 {
        let p = hist.wrapping_add(j as isize as usize * 5);
        let v = cpt1_w(p + 3) as i32 * 12 + *((p + 2) as *const u8) as i32;
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
    PLACAR.registrar(r == n, || format!("jogos_da_selecao_no_periodo(nação {}, {}/{} a {}/{}) original={} rust={}", cpt1_d(nacao), mes_de as u8, ano_de as u16, mes_ate as u8, ano_ate as u16, r, n));
    r
}

/// { 0x58d460 } (thiscall, ret 4, fifa_rankings.cpp): `FIFA_RANKINGS::update_rankings(data)` (pelo dia das seleções
/// 0x76b100): só no dia 18 (0x54cf70). Guarda a data em +5 (0x417c10) e, em cada nação ([0xae2350] entradas), tira do
/// histórico os resultados de mais de um ano (ano + 1 antes do de hoje, ou o mesmo ano com o mês antes do de hoje,
/// 0x54cfb0), pondo o último no lugar; o laço vai do byte baixo da conta − 1 até 0, como na original. Depois ordena o
/// histórico pelos pontos (qsort 0x945333 com 0x58e220). No mês guardado em +4, com o ano diferente do de [0xdd6ec8],
/// os totais dos anos andam uma casa (+0x14 → +0x28) e +0x14 zera. Por fim as pontuações (0x58dfc0) e as posições:
/// a ordem (0x58d610) dá a posição + 1 em +0x10, com a anterior em +0x12; o vetor da ordem é liberado (0x9452ca).
unsafe extern "thiscall" fn atualizar_ranking_da_fifa(this: usize, data: usize) {
    let dia: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x54cf70usize);
    let mes_de: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x54cfb0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let pontuar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x58dfc0usize);
    let ordem: unsafe extern "thiscall" fn(usize, *mut i32) -> usize = core::mem::transmute(0x58d610usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if dia(data) != 0x12 {
        return;
    }
    let mes = mes_de(data);
    let mut guardada = [0u8; 8];
    copiar(this + 5, guardada.as_mut_ptr() as usize, data);
    let ano = cpt1_w(data + 2) as i32;
    let mut i = 0i32;
    while i < cpt1_d(0xae2350) as i32 {
        let e = cpt1_d(this) + i as usize * 0x2c;
        let mut k = (*((e + 0xa) as *const u8)).wrapping_sub(1) as i8 as i32;
        while k >= 0 {
            let hist = cpt1_d(e + 4);
            let p = hist + k as usize * 5;
            let y = cpt1_w(p + 3) as i32 + 1;
            if y < ano || (y == ano && *((p + 2) as *const u8) < mes) {
                let ultimo = hist + cpt1_h(e + 0xa) as isize as usize * 5 - 5;
                core::ptr::copy(ultimo as *const u8, p as *mut u8, 5);
                *((e + 0xa) as *mut i16) = cpt1_h(e + 0xa).wrapping_sub(1);
            }
            k -= 1;
        }
        ordenar(cpt1_d(e + 4), cpt1_h(e + 0xa) as i32 as u32, 5, 0x58e220);
        i += 1;
    }
    if mes == *((this + 4) as *const u8) && cpt1_w(data + 2) != cpt1_w(0xdd6eca) {
        let mut i = 0i32;
        while i < cpt1_d(0xae2350) as i32 {
            let e = cpt1_d(this) + i as usize * 0x2c;
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
        let r = cpt1_d(v + k as usize * 4);
        *((r + 0x12) as *mut u16) = cpt1_w(r + 0x10);
        *((r + 0x10) as *mut u16) = (k + 1) as u16;
        k += 1;
    }
    liberar(v);
}

/// Comparador: lado a lado pelo objeto (+0..+0xd), pela tabela das nações e por cada histórico (capacidade +8 × 5);
/// a ordem é alocada e liberada pelas duas.
unsafe extern "thiscall" fn comparar_atualizar_ranking_da_fifa(this: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1156);
    let t = cpt1_d(this);
    let n = cpt1_d(0xae2350);
    let mut reg = vec![(this, 0xd), (t, n * 0x2c)];
    for i in 0..n {
        let e = t + i * 0x2c;
        let h = cpt1_d(e + 4);
        let cap = cpt1_h(e + 8) as i32;
        if h != 0 && cap > 0 {
            reg.push((h, cap as usize * 5));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, data), || atualizar_ranking_da_fifa(this, data));
    PLACAR.registrar(igual, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).flat_map(|(a, b)| (0..a.1.len()).filter(move |&i| a.1[i] != b.1[i]).map(move |i| a.0 + i)).take(16).collect();
        format!("atualizar_ranking_da_fifa(dia {}) diferenças em {:x?}", cpt1_w(data), dif)
    });
}

/// { 0x58e220 } (cdecl, fifa_rankings.cpp): `compare_result_points_desc(a, b)`, a ordem do qsort dos históricos na
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

unsafe fn cpt2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpt2_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpt2_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// A entrada da nação `nacao` (id em [nacao]) na tabela do ranking [this] (0x2c bytes cada).
unsafe fn cpt2_entrada(this: usize, nacao: usize) -> usize {
    cpt2_d(this).wrapping_add((cpt2_d(nacao) as u32).wrapping_mul(0x2c) as usize)
}

/// O mês e o ano do jogo `partida`: a data (0x549ef0) do dia do ano (+0x2a) e do ano (+0x28), o mês dela (0x54cfb0).
unsafe fn cpt2_mes_e_ano(partida: usize) -> (u8, u16) {
    let data_nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mes_de: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x54cfb0usize);
    let mut d = [0u8; 8];
    let p = data_nova(d.as_mut_ptr(), cpt2_w(partida + 0x2a) as u32, cpt2_w(partida + 0x28) as u32);
    (mes_de(p), cpt2_w(partida + 0x28))
}

/// { 0x58dea0 } (thiscall, ret 0xc, fifa_rankings.cpp, SEH só da data local, de destrutor vazio):
/// `FIFA_RANKINGS::append_result(nação, pontos, partida)` (pelo resultado internacional 0x58cf70, para cada seleção):
/// com a conta (+0xa) igual à capacidade (+8), a capacidade cresce 5 e o histórico (+4) é realocado (0x945667) ou
/// alocado (0x944e46) com capacidade × 5 bytes. A entrada nova, no lugar da conta antiga: os pontos (16 bits), o mês do
/// jogo (+2) e o ano (+3).
unsafe extern "thiscall" fn resultado_no_historico(this: usize, nacao: usize, pontos: u32, partida: usize) {
    let e = cpt2_entrada(this, nacao);
    if cpt2_w(e + 0xa) == cpt2_w(e + 8) {
        *((e + 8) as *mut u16) = cpt2_w(e + 8).wrapping_add(5);
        let h = cpt2_d(e + 4);
        let n = cpt2_h(e + 8) as i32 * 5;
        let novo = if h != 0 {
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
            realocar(h, n as u32)
        } else {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
            alocar(n as u32)
        };
        *((e + 4) as *mut usize) = novo;
    }
    let c = cpt2_h(e + 0xa);
    *((e + 0xa) as *mut i16) = c.wrapping_add(1); // cpt2: mais um resultado
    let p = cpt2_d(e + 4).wrapping_add(c as isize as usize * 5);
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
    let p = cpt2_d(e + 4).wrapping_add(conta as isize as usize * 5);
    let cap_n = if conta == cap { cap.wrapping_add(5) } else { cap };
    let lido = (cpt2_h(e + 0xa), cpt2_h(e + 8), cpt2_w(p), *((p + 2) as *const u8), cpt2_w(p + 3));
    let plano = (conta.wrapping_add(1), cap_n, pontos as u16, mes, ano);
    PLACAR.registrar(lido == plano, || format!("resultado_no_historico(nação {}, pontos {}) original={:?} plano={:?}", cpt2_d(nacao), pontos as u16, lido, plano));
}

/// { 0x58e190 } (thiscall, ret 4, fifa_rankings.cpp): `FIFA_RANKINGS::get_nation_strength_tier(nação)` (duas vezes por
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
    PLACAR.registrar(r == n, || format!("faixa_da_selecao(nação {}) original={} rust={}", cpt2_d(nacao), r, n));
    r
}

/// { 0x58dfc0 } (thiscall, fifa_rankings.cpp): `FIFA_RANKINGS::recalculate_scores()` (pela atualização do dia 18
/// 0x58d460): em cada nação, a soma dos pontos dos oito primeiros resultados (o histórico vem ordenado) e a de todos;
/// com mais de oito, o total do ano (+0x14) é (todos × 8 / conta + oito) / 2, senão os oito. Se a data de [0xdd6ec8]
/// passa da de hoje menos 365 dias (0x54c8e0), com menos de nove resultados o total é (+0x18 + oito) / (conta + 8) × 8
/// (sem resultados, o +0x18). A pontuação (+0xc, f32): (+0x14 + 5/6 de +0x18 + 4/6 de +0x1c + 3/6 de +0x20 + 2/6 de
/// +0x24 + 1/6 de +0x28, cada parte truncada) × [0x96a350].
unsafe extern "thiscall" fn pontuacoes_do_ranking(this: usize) {
    let datar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let fator = core::ptr::read_unaligned(0x96a350 as *const f64);
    let mut i = 0i32;
    if cpt2_d(0xae2350) as i32 <= 0 {
        return;
    }
    loop {
        let e = cpt2_d(this) + i as usize * 0x2c;
        let conta = cpt2_h(e + 0xa);
        let ponto = |k: i8| cpt2_h(cpt2_d(e + 4).wrapping_add(k as isize as usize * 5)) as i32;
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
        let (by, ry) = (u16::from_le_bytes([base[2], base[3]]), cpt2_w(r + 2));
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
        if i >= cpt2_d(0xae2350) as i32 {
            break;
        }
    }
}

unsafe extern "thiscall" fn comparar_pontuacoes_do_ranking(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1160);
    let reg = [(cpt2_d(this), cpt2_d(0xae2350) * 0x2c)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || pontuacoes_do_ranking(this));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).collect();
        format!("pontuacoes_do_ranking diferenças nos deslocamentos {:x?}", dif)
    });
}

// ---------- 256a leva: a nação-chave do jogo: a nação-chave ativa de uma competição ou jogo, se a competição é a Copa do Mundo e se é um torneio internacional (comp.cpp, key_nation.cpp) ----------

unsafe fn cpt3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x48dd70 } (cdecl, comp.cpp): `comp_active_key_nation_of_fixture(competição, partida)`, a chave das listas de jogos
/// do FIXTURE_MANAGER (add_fixture 0x5aa680, find_fixtures 0x5acbe0, ...), da simulação rápida (0x6b66a0) e do peso do
/// ranking da FIFA (0x58cf70). Amistoso (0x5c6710 sobre [0xae2c74]) ou reservas ([0x9cf684]) com a partida dada e a
/// competição dela amistosa ([0xae2c74]+4) ou de reservas: com uma das duas seleções (0x53b900 dos clubes +0x1c/+0x20),
/// 1; nações (+0x53) diferentes ou fora da escolha (+0x11c), 3; a nação na tabela das 35 nações-chave (0xb63d60, 72
/// bytes cada, escolhida em +0x11c), o byte +0x14 da linha; senão 0 com o mandante seleção e 3 sem. Senão, competição de
/// nação (+0x5d): o byte +0x14 da linha dela, ou 3. Sem nação: Copa do Mundo e eliminatórias (0x669a10), 0; torneio
/// internacional (0x669ae0), 1; senão o índice da nação-chave (0x669470). Chamada a cada jogo procurado: sem contador.
unsafe extern "C" fn nacao_chave_do_jogo(comp: usize, partida: usize) -> u8 {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let linha = |nacao: usize| (0..0x23usize).find(|&k| {
        let n = cpt3_d(0xb63d60 + 72 * k);
        n != 0 && *((n + 0x11c) as *const u8) != 0 && n == nacao
    });
    let nacao = cpt3_d(comp + 0x5d);
    let reservas = cpt3_d(0x9cf684);
    if (amistoso(cpt3_d(0xae2c74), comp) != 0 || cpt3_d(comp) == reservas) && partida != 0 {
        let pc = cpt3_d(partida + 0x14);
        if pc == cpt3_d(cpt3_d(0xae2c74) + 4) || cpt3_d(pc) == reservas {
            if selecao(cpt3_d(partida + 0x1c)) != 0 || selecao(cpt3_d(partida + 0x20)) != 0 {
                return 1;
            }
            let casa = cpt3_d(partida + 0x1c);
            let a = cpt3_d(casa + 0x53);
            if a != cpt3_d(cpt3_d(partida + 0x20) + 0x53) || *((a + 0x11c) as *const u8) == 0 {
                return 3;
            }
            if let Some(k) = linha(a) {
                return *((0xb63d74 + 72 * k) as *const u8); // cpt3: a nacao do amistoso
            }
            return if selecao(casa) != 0 { 0 } else { 3 };
        }
    }
    if cpt3_d(comp + 0x5d) != 0 {
        return match linha(nacao) {
            Some(k) => *((0xb63d74 + 72 * k) as *const u8), // cpt3: a nacao-chave da competicao
            None => 3,
        };
    }
    let copa: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669a10usize);
    let internacional: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669ae0usize);
    let indice: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669470usize);
    if copa(comp) != 0 {
        return 0; // cpt3: Copa do Mundo e eliminatorias
    }
    if internacional(comp) != 0 {
        return 1; // cpt3: torneio internacional
    }
    indice(comp)
}

unsafe extern "C" fn comparar_nacao_chave_do_jogo(comp: usize, partida: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u8>(1161)(comp, partida);
    let n = nacao_chave_do_jogo(comp, partida);
    PLACAR.registrar(r == n, || format!("nacao_chave_do_jogo(competição {}, partida {:x}) original={} rust={}", cpt3_d(comp) as i32, partida, r, n));
    r
}

/// Os ids das competições da Copa do Mundo: a final e as seis eliminatórias (Europa [0x9cf780], África, Ásia,
/// CONCACAF, Oceania, América do Sul).
const CPT3_COPA: [usize; 7] = [0x9cf79c, 0x9cf77c, 0x9cf774, 0x9cf770, 0x9cf780, 0x9cf76c, 0x9cf778];

/// Os ids dos torneios de seleções fora da Copa do Mundo: Olimpíada, Eurocopa, eliminatórias da Eurocopa, Copa
/// Africana [0x9cf7ac], Copa América, seleção B, sub-21, Copa da Oceania, Copa das Confederações e Copa da Ásia.
const CPT3_INTERNACIONAIS: [usize; 10] = [0x9cf7a0, 0x9cf7a4, 0x9cf784, 0x9cf7ac, 0x9cf7a8, 0x9cf794, 0x9cf798, 0x9cf8a0, 0x9cf964, 0x9cf790];

/// { 0x669a10 } (cdecl, key_nation.cpp): `key_nation_is_world_cup_comp(competição)`: o id (+0) é a Copa do Mundo ou uma
/// das eliminatórias, 1; senão 0. Competição nula: a caixa "Error" (linha 0x3d7) e 0.
unsafe extern "C" fn e_copa_do_mundo(comp: usize) -> u32 {
    if comp == 0 {
        aviso_em(0x9eaff4, 0x3d7);
        return 0;
    }
    let id = cpt3_d(comp);
    CPT3_COPA.iter().any(|&g| cpt3_d(g) == id) as u32
}

unsafe extern "C" fn comparar_e_copa_do_mundo(comp: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(1162);
    if comp == 0 {
        return o(comp);
    }
    let r = o(comp);
    let n = e_copa_do_mundo(comp);
    PLACAR.registrar(r == n, || format!("e_copa_do_mundo(competição {}) original={} rust={}", cpt3_d(comp) as i32, r, n));
    r
}

/// { 0x669ae0 } (cdecl, key_nation.cpp): `key_nation_is_international_comp(competição)`: o id é um dos dez torneios de
/// seleções fora da Copa do Mundo, 1; senão 0. Competição nula: a caixa (linha 0x3ef) e 0.
unsafe extern "C" fn e_torneio_internacional(comp: usize) -> u32 {
    if comp == 0 {
        aviso_em(0x9eaff4, 0x3ef);
        return 0;
    }
    let id = cpt3_d(comp);
    CPT3_INTERNACIONAIS.iter().any(|&g| cpt3_d(g) == id) as u32
}

unsafe extern "C" fn comparar_e_torneio_internacional(comp: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(1163);
    if comp == 0 {
        return o(comp);
    }
    let r = o(comp);
    let n = e_torneio_internacional(comp);
    PLACAR.registrar(r == n, || format!("e_torneio_internacional(competição {}) original={} rust={}", cpt3_d(comp) as i32, r, n));
    r
}

// ---------- 256a leva: a nação-chave da competição e a ordem do ranking da FIFA: o índice da nação-chave da competição, a ordem das seleções no ranking e a comparação das pontuações (key_nation.cpp, fifa_rankings.cpp) ----------

unsafe fn cpt4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x669470 } (cdecl, key_nation.cpp): `key_nation_index_of_comp(competição)` (pelo alcance das regras da partida
/// 0x8d2fa0, pela liga do dia 0x6847c0, pelos registros da copa 0x519890 e pela nação-chave do jogo 0x48dd70 da cpt_3):
/// competição de nação (+0x5d): a linha dela na tabela das 35 nações-chave (0xb63d60, 72 bytes), ou 4. Sem nação: Copa
/// do Mundo e eliminatórias, 0; torneio de seleções (0x669ae0), 1; Copa do Mundo de Clubes e Mundial de Clubes da FIFA,
/// 2; senão a linha de continente (nação 0) com o continente da competição (+0x59), ou 3. A original lê +0x5d antes de
/// conferir a competição nula (o caso de erro nunca chega à caixa). Chamada a cada partida: sem contador.
unsafe extern "C" fn indice_da_nacao_chave(comp: usize) -> u8 {
    let nacao = cpt4_d(comp + 0x5d);
    if nacao != 0 {
        let linha = (0..0x23usize).find(|&k| cpt4_d(0xb63d60 + 72 * k) == nacao); // cpt4: a busca da nacao
        return match linha {
            Some(k) => k as u8, // cpt4: a linha da nacao
            None => 4,
        };
    }
    let id = cpt4_d(comp);
    let copa = [0x9cf79c, 0x9cf77c, 0x9cf774, 0x9cf770, 0x9cf780, 0x9cf76c, 0x9cf778]; // cpt4: a Copa e as eliminatorias
    if copa.iter().any(|&g| cpt4_d(g) == id) {
        return 0;
    }
    let internacional: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669ae0usize);
    if internacional(comp) != 0 {
        return 1;
    }
    if id == cpt4_d(0x9cf6e4) || id == cpt4_d(0x9cf7bc) {
        return 2;
    }
    for k in 0..0x23usize {
        let c = cpt4_d(comp + 0x59);
        if cpt4_d(0xb63d60 + 72 * k) == 0 && c != 0 && c == cpt4_d(0xb63d64 + 72 * k) {
            return k as u8; // cpt4: a linha do continente
        }
    }
    3
}

unsafe extern "C" fn comparar_indice_da_nacao_chave(comp: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(1164)(comp);
    let n = indice_da_nacao_chave(comp);
    PLACAR.registrar(r == n, || format!("indice_da_nacao_chave(competição {}) original={} rust={}", cpt4_d(comp) as i32, r, n));
    r
}

/// { 0x58d610 } (thiscall, ret 4, fifa_rankings.cpp): `FIFA_RANKINGS::collect_ranked_order(&conta)` (pela atualização do
/// dia 18 0x58d460 e pela tela do ranking): um vetor (0x944e46) de [0xae2350] ponteiros; sem ele, a caixa "Error"
/// (linha 0x1e3) e 0, sem gravar a conta. As entradas da tabela (0x2c bytes) das nações com seleção (+0x71 da nação,
/// registros de 0x122 bytes em [0xae23a8]), ordenadas pela pontuação (qsort 0x945333 com 0x58e240); a conta em `conta`.
unsafe extern "thiscall" fn ordem_do_ranking(this: usize, conta: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let v = alocar((cpt4_d(0xae2350) as u32).wrapping_mul(4));
    if v == 0 {
        aviso_em(0x9c74f4, 0x1e3);
        return 0;
    }
    let nacoes = cpt4_d(0xae23a8);
    let mut n = 0usize;
    let mut i = 0usize;
    while (i as i32) < cpt4_d(0xae2350) as i32 {
        if cpt4_d(nacoes + i * 0x122 + 0x71) != 0 {
            *((v + n * 4) as *mut usize) = cpt4_d(this) + i * 0x2c;
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
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let igual = v != 0 && c == *conta && (0..c as usize).all(|k| cpt4_d(r + k * 4) == cpt4_d(v + k * 4));
    PLACAR.registrar(igual, || format!("ordem_do_ranking conta original={} rust={}", *conta, c));
    liberar(v);
    r
}

/// { 0x58e240 } (cdecl, fifa_rankings.cpp): `compare_ranking_score_desc(a, b)`, a ordem do qsort da 0x58d610: as
/// pontuações (f32 +0xc das entradas apontadas): a menor que b (ou sem ordem), 1; iguais, 0; maior, −1.
unsafe extern "C" fn ordem_das_pontuacoes(a: usize, b: usize) -> i32 {
    let fa = *((cpt4_d(a) + 0xc) as *const f32);
    let fb = *((cpt4_d(b) + 0xc) as *const f32);
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

// ---------- 256a leva: o tempo das cidades: o tempo do dia, a região do weather.cfg e o período da região (weather.cpp) ----------

unsafe fn cpt5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpt5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A linha seguinte do arquivo (0x90cba0: FILE*, buffer), 0 no fim.
unsafe fn cpt5_ler_linha(f: usize, buf: *mut u8) -> i32 {
    let ler: unsafe extern "C" fn(usize, *mut u8) -> i32 = core::mem::transmute(0x90cba0usize);
    ler(f, buf)
}

/// { 0x922ee0 } (thiscall, weather.cpp): `Weather::update_city_weather()`, todo dia pelo laço do dia do jogo (0x5cd9aa,
/// [0xde2f74]). Sem a tabela das cidades (+4, 4 bytes por cidade): nada. Cada cidade ([0xae2354], relida a cada volta):
/// as três medidas (+0, +1, +2) viram (medida + sorteio(100)) × 0,5 truncado, presas em 0..100; sorteio(10) zerado dá
/// o vento novo (+3) = sorteio(3).
unsafe extern "thiscall" fn tempo_das_cidades(this: usize) {
    if cpt5_d(this + 4) == 0 || cpt5_d(0xae2354) as i32 <= 0 {
        return;
    }
    crate::comparacao::ramo("cpt_5 tempo do dia");
    let mut i = 0usize;
    loop {
        for k in 0..3usize { // cpt5: as três medidas
            let p = cpt5_d(this + 4) + i * 4 + k;
            let r = cpt5_sortear(100);
            *(p as *mut u8) = ftol((r as i16 as i32 + *(p as *const i8) as i32) as f64 * f64_em(0x968968)) as u8;
            let p = cpt5_d(this + 4) + i * 4 + k;
            let v = *(p as *const i8);
            if v < 0 {
                *(p as *mut u8) = 0;
            } else if v > 0x64 {
                *(p as *mut u8) = 0x64;
            }
        }
        if cpt5_sortear(10) == 0 { // cpt5: um em dez muda o vento
            crate::comparacao::ramo("cpt_5 vento novo");
            let r = cpt5_sortear(3); // cpt5: o vento novo
            *((cpt5_d(this + 4) + i * 4 + 3) as *mut u8) = r as u8;
        }
        i += 1;
        if i as i32 >= cpt5_d(0xae2354) as i32 {
            break;
        }
    }
}

// ponytail: sorteia 3 ou 4 vezes por cidade: só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_tempo_das_cidades(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1167)(this)
}

/// { 0x922c40 } (thiscall, ret 8, weather.cpp): `Weather::read_region(f, regiao)`: uma linha do weather.cfg (0x90cba0,
/// buffer de 2000) com "%ld %s" (0xad4e24) no id (+0) e no nome (+4) da região (0x6e bytes); depois os quatro
/// períodos (0x922d20) em +0x1e, +0x31, +0x44 e +0x57. Devolve 1 se leu tudo (só o al, como a original).
unsafe extern "thiscall" fn ler_regiao_do_clima(this: usize, f: usize, regiao: usize) -> u8 {
    let ler: unsafe extern "C" fn(*const u8, usize, ...) -> i32 = core::mem::transmute(0x946fd8usize);
    let periodo: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x922d20usize);
    crate::comparacao::ramo("cpt_5 regiao do clima");
    let mut linha = [0u8; 2000];
    if cpt5_ler_linha(f, linha.as_mut_ptr()) == 0 {
        return 0;
    }
    if ler(linha.as_ptr(), 0xad4e24, regiao, regiao + 4) != 2 {
        return 0;
    }
    for k in [0x1e, 0x31, 0x44, 0x57] {
        if periodo(this, f, regiao + k) == 0 {
            return 0;
        }
    }
    1
}

// ponytail: lê do arquivo (não dá para ler duas vezes): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_ler_regiao_do_clima(this: usize, f: usize, regiao: usize) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(1168)(this, f, regiao)
}

/// { 0x922d20 } (thiscall, ret 8, weather.cpp): `Weather::read_region_period(f, periodo)`: arquivo nulo é a caixa
/// "Error" (weather.cpp, 0xea) e 0. Uma linha (0x90cba0) com o formato de 0xad4e2c (o nome e 18 inteiros: o dia,
/// cinco W, cinco P e sete H); com os 19 lidos, o período (0x13 bytes) recebe o dia (palavra em +0) e os 17 bytes
/// seguintes (+2..+0x12). Devolve 1 se leu (só o al).
unsafe extern "thiscall" fn ler_periodo_do_clima(_this: usize, f: usize, periodo: usize) -> u8 {
    let ler: unsafe extern "C" fn(*const u8, usize, ...) -> i32 = core::mem::transmute(0x946fd8usize);
    if f == 0 {
        aviso_em(0xad4dec, 0xea);
        return 0;
    }
    crate::comparacao::ramo("cpt_5 periodo do clima");
    let mut linha = [0u8; 2000];
    if cpt5_ler_linha(f, linha.as_mut_ptr()) == 0 {
        return 0;
    }
    // ponytail: o nome tem 200 bytes na original; aqui 2000 para o %s não estourar (com os nomes do arquivo, igual).
    let mut nome = [0u8; 2000];
    let mut v = [0i32; 18];
    let p = v.as_mut_ptr();
    let n = ler(linha.as_ptr(), 0xad4e2c, nome.as_mut_ptr(), p, p.add(1), p.add(2), p.add(3), p.add(4), p.add(5),
                p.add(6), p.add(7), p.add(8), p.add(9), p.add(10), p.add(11), p.add(12), p.add(13), p.add(14),
                p.add(15), p.add(16), p.add(17));
    if n != 0x13 {
        return 0;
    }
    core::ptr::write_unaligned(periodo as *mut u16, v[0] as u16);
    for k in 1..18usize {
        *((periodo + 1 + k) as *mut u8) = v[k] as u8;
    }
    1
}

// ponytail: lê do arquivo: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_ler_periodo_do_clima(this: usize, f: usize, periodo: usize) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(1169)(this, f, periodo)
}

// ---------- 256a leva: os coeficientes das nações (o bônus de dezembro e a história de cinco anos), o dia das aposentadorias e a soma presa de dois bytes (game.cpp) ----------

unsafe fn cpt6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpt6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpt6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpt6_f(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

unsafe fn cpt6_pf(a: usize, v: f64) {
    core::ptr::write_unaligned(a as *mut f64, v);
}

/// A história do coeficiente da nação: +0xb0, +0xb8, +0xc0, +0xc8, +0xd0 (cinco anos) e +0xd8 (o ano corrente).
unsafe fn cpt6_historia(na: usize) -> [f64; 6] {
    let mut h = [0f64; 6];
    for (k, x) in h.iter_mut().enumerate() {
        *x = cpt6_f(na + 0xb0 + k * 8);
    }
    h
}

/// O mês da data do jogo (0x54cfb0 sobre 0xae2c90, o al com sinal).
unsafe fn cpt6_mes() -> i32 {
    let mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x54cfb0usize);
    mes(0xae2c90) as u8 as i8 as i32
}

/// Dezembro na história `h` da nação `na` (`marca` = +0x121): com marca < 12, o ano corrente ganha
/// reputação (+0x8e) × 0,05 × (12 − marca) × 3; a história anda um ano, o ano corrente e a marca zeram.
unsafe fn cpt6_dezembro(na: usize, h: &mut [f64; 6], marca: &mut u8) {
    crate::comparacao::ramo("cpt_6 dezembro");
    if (*marca as i8) < 0xc {
        crate::comparacao::ramo("cpt_6 bonus de dezembro");
        h[5] = cpt6_w(na + 0x8e) as i32 as f64 * f64_em(0x967888) * (0xc - *marca as i8 as i32) as f64 * f64_em(0x968930) + h[5]; // cpt6: o bônus de dezembro
    }
    h[0] = h[1];
    h[1] = h[2];
    h[2] = h[3];
    h[3] = h[4];
    h[4] = h[5];
    h[5] = 0.0; // cpt6: o ano corrente zerado
    *marca = 0;
}

/// { 0x5d5f20 } (cdecl, game.cpp): `game_update_nation_coefficients()`, todo dia pelo laço do dia do jogo (0x5cda10).
/// Na primeira vez ([0x9cd218]), a história de cada nação (+0xb0..+0xd0) recebe o coeficiente de +0xa8. No dia do
/// mês da tabela 0x9cd200 (palavra pelo mês 0x54cfb0): um vetor (malloc de 12 bytes por nação: a nação e a soma
/// 0,6·c0 + 0,4·b8 + 0,2·b0 + 0,8·c8 + d8 + d0), ordenado (qsort 0x945333 com 0x4afba0) e solto; no dia de dezembro
/// ([0x9cd216]) cada nação passa também pelo bônus e pelo deslocamento da história (`cpt6_dezembro`).
unsafe extern "C" fn coeficientes_das_nacoes() {
    let malloc: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944e46usize);
    let qsort: unsafe extern "C" fn(usize, usize, usize, usize) = core::mem::transmute(0x945333usize);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if cpt6_b(0x9cd218) != 0 {
        if (cpt6_d(0xae2350) as i32) > 0 {
            let mut p = cpt6_d(0xae23a8) + 0xd0;
            let mut i = 0i16;
            loop {
                let v = cpt6_f(p - 0x28);
                for k in 0..5usize {
                    cpt6_pf(p - k * 8, v);
                }
                i = i.wrapping_add(1);
                p += 0x122;
                if i as i32 >= cpt6_d(0xae2350) as i32 {
                    break;
                }
            }
        }
        *(0x9cd218 as *mut u8) = 0;
    }
    let hoje = cpt6_d(0xae2c90) as i16;
    if cpt6_w(0x9cd200 + (cpt6_mes() * 2) as usize) != hoje {
        return;
    }
    let vetor = malloc(cpt6_d(0xae2350).wrapping_mul(0xc));
    if vetor == 0 {
        aviso_em(0x9cd2a4, 0x156d);
        return;
    }
    let dez = cpt6_w(0x9cd216) == cpt6_w(0xae2c90); // cpt6: o dia de dezembro
    let mut n = cpt6_d(0xae2350) as i32;
    if n > 0 {
        let mut na = cpt6_d(0xae23a8);
        let mut i = 0i16;
        loop {
            let e = vetor + i as i32 as usize * 0xc;
            *(e as *mut usize) = na;
            let s = cpt6_f(na + 0xc0) * f64_em(0x968e48) + cpt6_f(na + 0xb8) * f64_em(0x968e00) + cpt6_f(na + 0xb0) * f64_em(0x968918)
                + cpt6_f(na + 0xc8) * f64_em(0x9689b0) + cpt6_f(na + 0xd8) + cpt6_f(na + 0xd0);
            cpt6_pf(e + 4, s);
            if dez {
                let mut h = cpt6_historia(na);
                let mut marca = cpt6_b(na + 0x121);
                cpt6_dezembro(na, &mut h, &mut marca);
                for (k, x) in h.iter().enumerate() {
                    cpt6_pf(na + 0xb0 + k * 8, *x);
                }
                *((na + 0x121) as *mut u8) = marca;
            }
            i = i.wrapping_add(1);
            na += 0x122;
            n = cpt6_d(0xae2350) as i32;
            if i as i32 >= n {
                break;
            }
        }
    }
    qsort(vetor, n as usize, 0xc, 0x4afba0);
    free(vetor);
}

/// Comparador: a original aloca e solta o vetor (e o ordena), então roda só ela, conferida contra o plano feito antes
/// pelas mesmas contas da cópia: a história (+0xb0..+0xd8) e a marca (+0x121) de cada nação e [0x9cd218] zerado.
unsafe extern "C" fn comparar_coeficientes_das_nacoes() {
    let o = orig::<unsafe extern "C" fn()>(1170);
    let iniciar = cpt6_b(0x9cd218) != 0;
    let hoje = cpt6_d(0xae2c90) as i16;
    let dez = cpt6_w(0x9cd200 + (cpt6_mes() * 2) as usize) == hoje && cpt6_w(0x9cd216) == hoje;
    let n = cpt6_d(0xae2350) as i32;
    let base = cpt6_d(0xae23a8);
    let mut plano = Vec::new();
    for k in 0..n.max(0) as usize {
        let na = base + k * 0x122;
        let mut h = cpt6_historia(na);
        if iniciar {
            let v = cpt6_f(na + 0xa8);
            for x in h.iter_mut().take(5) {
                *x = v;
            }
        }
        let mut marca = cpt6_b(na + 0x121);
        if dez {
            cpt6_dezembro(na, &mut h, &mut marca);
        }
        plano.push((h, marca));
    }
    o();
    let mut erro = None;
    for (k, (h, marca)) in plano.iter().enumerate() {
        let na = base + k * 0x122;
        let g = cpt6_historia(na);
        if (0..6).any(|j| g[j].to_bits() != h[j].to_bits()) || cpt6_b(na + 0x121) != *marca {
            erro = Some(format!("nação {} ({:x}): {:?}/{} x plano {:?}/{}", k, na, g, cpt6_b(na + 0x121), h, marca));
            break;
        }
    }
    if cpt6_b(0x9cd218) != 0 {
        erro = Some(format!("[0x9cd218] = {}", cpt6_b(0x9cd218)));
    }
    PLACAR.registrar(erro.is_none(), || format!("coeficientes_das_nacoes(dezembro {}): {}", dez, erro.clone().unwrap_or_default()));
}

/// { 0x5d6250 } (cdecl, game.cpp): `is_player_regen_day(dia)`: 1 nos dias 0, 80, 173 e 250 do ano (a busca das
/// aposentadorias), senão 0. Pelo laço do dia do jogo (0x5cda97, com a data inteira: o dia é a palavra baixa).
/// A original só devolve o al (o ah fica com o byte alto do dia).
unsafe extern "C" fn dia_das_aposentadorias(dia: u32) -> u32 {
    let d = dia as u16;
    (d == 0 || d == 0x50 || d == 0xad || d == 0xfa) as u32
}

/// Comparador: pelo al (o único byte que os chamadores leem).
unsafe extern "C" fn comparar_dia_das_aposentadorias(dia: u32) -> u32 {
    let r = orig::<unsafe extern "C" fn(u32) -> u32>(1171)(dia);
    let n = dia_das_aposentadorias(dia);
    PLACAR.registrar(r as u8 == n as u8, || format!("dia_das_aposentadorias({}) {} x {}", dia as u16, r as u8, n as u8));
    r
}

/// { 0x5d61d0 } (cdecl, game.cpp): `clamp_char_sum(a, b, min, max)`: a soma dos dois bytes com sinal (16 bits),
/// presa entre os outros dois. Devolve o ax da original: `min` ou `max` estendidos, ou o byte alto do `max`
/// estendido com a soma no al (os 26 chamadores, no diário dos jogadores, só gravam o al).
unsafe extern "C" fn soma_presa(a: u32, b: u32, min: u32, max: u32) -> u32 {
    let s = (a as u8 as i8 as i16).wrapping_add(b as u8 as i8 as i16);
    let lo = min as u8 as i8 as i16;
    let hi = max as u8 as i8 as i16;
    if lo > s {
        lo as u16 as u32
    } else if hi < s {
        hi as u16 as u32
    } else {
        ((hi as u16 & 0xff00) | (s as u16 & 0xff)) as u32
    }
}

/// Comparador: pelo ax (os 16 bits altos do eax da original são os do chamador).
unsafe extern "C" fn comparar_soma_presa(a: u32, b: u32, min: u32, max: u32) -> u32 {
    let r = orig::<unsafe extern "C" fn(u32, u32, u32, u32) -> u32>(1172)(a, b, min, max);
    let n = soma_presa(a, b, min, max);
    PLACAR.registrar(r as u16 == n as u16, || format!("soma_presa({}, {}, {}, {}) {:x} x {:x}", a as u8 as i8, b as u8 as i8, min as u8 as i8, max as u8 as i8, r as u16, n as u16));
    r
}

// ---------- 256a leva: o estádio neutro e o clima inicial: a nota do estádio para o jogo, o fator da distância no público e a leitura do weather.cfg (stadium.cpp, weather.cpp) ----------

unsafe fn cpt7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpt7_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpt7_f(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

unsafe fn cpt7_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A distância entre duas cidades (x em +0x23, y em +0x2b, doubles) × 44,44, truncada (_ftol).
unsafe fn cpt7_distancia(a: usize, b: usize) -> i32 {
    let dx = (cpt7_f(a + 0x23) - cpt7_f(b + 0x23)).abs();
    let dy = (cpt7_f(a + 0x2b) - cpt7_f(b + 0x2b)).abs();
    ftol((dy * dy + dx * dx).sqrt() * f64_em(0x96fe40))
}

/// { 0x85da00 } (thiscall, ret 0x14, stadium.cpp): `Stadium::score_venue_for_fixture(estadio, marcas, casa, fora,
/// jogo)`, pela busca do estádio neutro 0x85c800 (com as marcas 0x20 nos jogos da carreira). O estádio nacional da
/// Inglaterra (nação [0x9cf2e4] +0x80) antes de 2004: 0. A nota parte de 100000:
/// - marcas & 8, com as cidades dos dois clubes (+0x69 → +0x38) e a do estádio (+0x38) e mais de 10000 lugares (+0x3c):
///   pelas distâncias d1 (casa-fora), d2 (casa-estádio), d3 (fora-estádio): com d1 < 30, 200000/180000/140000 se d2 e
///   d3 < 15/30/50 e 50000 se uma passa de 100; senão 200000 se as duas < 0,588·d1 e 50000 se uma ≥ 0,8333·d1;
/// - marcas & 0x20: o público esperado (10000; seleção da casa 60000; senão a soma dos públicos +0x73 dos dois clubes,
///   no mínimo 5000 e, acima de 40000, a metade sem ficar abaixo de nenhum dos dois); lugares acima dele: × 1,2; acima
///   de 1,5× dele: / ((lugares − 1,5·esperado)·8,33e-5 + 1); abaixo de 0,3/0,5/0,7/0,9 dele: × 0,1/0,3/0,7/0,9;
/// - senão, com marcas & 4: a nota é sorteio(lugares).
/// No fim, nota × (sorteio(100) × 0,004 + 0,8), truncada.
unsafe extern "thiscall" fn nota_do_estadio_neutro(_this: usize, est: usize, marcas: u32, casa: usize, fora: usize, jogo: usize) -> i32 {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let inglaterra = cpt7_d(0xae23a8).wrapping_add(cpt7_d(0x9cf2e4).wrapping_mul(0x122));
    if est == cpt7_d(inglaterra + 0x80) && core::ptr::read_unaligned(0xae2c92 as *const u16) < 0x7d4 {
        return 0;
    }
    let m = marcas as u8;
    let mut nota: i32 = 100000;
    if m & 8 != 0 {
        let cc = cpt7_d(casa + 0x69);
        let cf = cpt7_d(fora + 0x69);
        let (cidc, cidf) = (if cc != 0 { cpt7_d(cc + 0x38) } else { 0 }, if cf != 0 { cpt7_d(cf + 0x38) } else { 0 });
        let cide = cpt7_d(est + 0x38);
        if cc != 0 && cidc != 0 && cf != 0 && cidf != 0 && cide != 0 && cpt7_i(est + 0x3c) > 0x2710 {
            let d1 = cpt7_distancia(cidc, cidf);
            let d2 = cpt7_distancia(cidc, cide);
            let d3 = cpt7_distancia(cidf, cide);
            if d1 < 0x1e {
                if d2 < 0xf && d3 < 0xf {
                    nota = 200000;
                } else if d2 < 0x1e && d3 < 0x1e {
                    nota = 180000;
                } else if d2 < 0x32 && d3 < 0x32 {
                    nota = 140000;
                } else if d2 >= 0x64 || d3 >= 0x64 {
                    nota = 50000;
                }
            } else {
                let (f1, f2, f3) = (d1 as f64, d2 as f64, d3 as f64);
                let perto = f1 * f64_em(0x96fe38);
                let longe = f1 * f64_em(0x969088);
                if f2 < perto && f3 < perto {
                    nota = 200000;
                } else if !(f2 < longe && f3 < longe) {
                    nota = 50000;
                }
            }
        }
    }
    let lugares = cpt7_i(est + 0x3c);
    if m & 0x20 != 0 {
        let mut esperado: i32 = 0x2710;
        if jogo != 0 {
            if e_selecao(cpt7_d(jogo + 0x1c)) != 0 {
                esperado = 0xea60;
            } else {
                let a = cpt7_i(cpt7_d(jogo + 0x20) + 0x73);
                let c = cpt7_i(cpt7_d(jogo + 0x1c) + 0x73);
                let s = c.wrapping_add(a);
                if s < 0x1388 {
                    esperado = 0x1388;
                } else if s > 0x9c40 {
                    let mut h = s / 2;
                    if h <= c {
                        h = c;
                    }
                    if h <= a {
                        h = a;
                    }
                    esperado = h;
                } else {
                    esperado = s;
                }
            }
        }
        if lugares > esperado {
            nota = ftol(nota as f64 * f64_em(0x968978));
        }
        let (l, e) = (lugares as f64, esperado as f64);
        let t = e * f64_em(0x968940);
        if l > t {
            nota = ftol(nota as f64 / ((l - t) * f64_em(0x96fe30) + f64_em(0x967890)));
        } else if l < e * f64_em(0x968e80) {
            nota = ftol(nota as f64 * f64_em(0x967880));
        } else if l < e * f64_em(0x968968) {
            nota = ftol(nota as f64 * f64_em(0x968e80));
        } else if l < e * f64_em(0x968e40) {
            nota = ftol(nota as f64 * f64_em(0x968e40));
        } else if l < e * f64_em(0x9689d8) {
            nota = ftol(nota as f64 * f64_em(0x9689d8));
        }
    } else if m & 4 != 0 {
        nota = cpt7_sortear(lugares);
    }
    crate::comparacao::ramo("cpt_7 nota do estadio");
    let r = cpt7_sortear(100); // cpt7: o sorteio da nota
    ftol((r as f64 * f64_em(0x968d60) + f64_em(0x9689b0)) * nota as f64)
}

// ponytail: sorteia (e, com a marca 4, sorteia os lugares): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_nota_do_estadio_neutro(this: usize, est: usize, marcas: u32, casa: usize, fora: usize, jogo: usize) -> i32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, usize) -> i32>(1173)(this, est, marcas, casa, fora, jogo)
}

/// { 0x85d8a0 } (thiscall, ret 0x14, stadium.cpp): `Stadium::get_travel_distance_factor(jogo, cidade_do_estadio,
/// cidade_do_clube, base)`, no público do jogo (0x85cba0, pelo motor 0x6b7f20). D = distância entre as cidades
/// (pela metade com o jogo +0x30 = 0xa0 e +0x3a = 5); F = 1 − D·[0x96afe0], ao quadrado sem +0x3c; com D e a
/// reputação R (byte +0x7e da nação +0x53 do clube de fora +0x20) acima de [0x9689a0]: F / (R + 1)². Com base acima de
/// [0x969558], o mínimo 1 − ([0x968ea8] − base)·[0x968f20]; com +0x3a = 10, o mínimo 0,2; com os dois clubes da mesma
/// nação, 0,05; e sempre [0x96d2c8]. Devolve o double no st0.
unsafe extern "thiscall" fn fator_da_distancia(_this: usize, jogo: usize, cid_est: usize, cid_clube: usize, base: f64) -> f64 {
    let nacao = cpt7_d(cpt7_d(jogo + 0x20) + 0x53);
    let r = *((nacao + 0x7e) as *const i8) as i32 as f64;
    let dy = cpt7_f(cid_est + 0x2b) - cpt7_f(cid_clube + 0x2b);
    let dx = cpt7_f(cid_est + 0x23) - cpt7_f(cid_clube + 0x23);
    let mut d = (dx * dx + dy * dy).sqrt();
    let tipo = *((jogo + 0x3a) as *const u8);
    if core::ptr::read_unaligned((jogo + 0x30) as *const u16) == 0xa0 && tipo == 5 {
        d *= f64_em(0x968968);
    }
    let mut f = f64_em(0x967890) - d * f64_em(0x96afe0);
    if *((jogo + 0x3c) as *const u8) == 0 {
        f *= f;
    }
    if d > f64_em(0x9689a0) && r > f64_em(0x9689a0) {
        let q = r + f64_em(0x967890);
        f /= q * q;
    }
    if base > f64_em(0x969558) {
        let minimo = f64_em(0x967890) - (f64_em(0x968ea8) - base) * f64_em(0x968f20);
        if f < minimo {
            return minimo;
        }
    }
    if tipo == 0xa && f < f64_em(0x968918) {
        return f64_em(0x968918);
    }
    if f < f64_em(0x967888) && cpt7_d(cpt7_d(jogo + 0x1c) + 0x53) == nacao {
        return f64_em(0x967888);
    }
    if f < f64_em(0x96d2c8) {
        return f64_em(0x96d2c8);
    }
    f
}

/// Comparador: pelo valor (os bits do double).
unsafe extern "thiscall" fn comparar_fator_da_distancia(this: usize, jogo: usize, cid_est: usize, cid_clube: usize, base: f64) -> f64 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, f64) -> f64>(1174)(this, jogo, cid_est, cid_clube, base);
    let n = fator_da_distancia(this, jogo, cid_est, cid_clube, base);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("fator_da_distancia(jogo {:x}, base {}) {} x {}", jogo, base, r, n));
    r
}

/// { 0x922810 } (thiscall, weather.cpp): `Weather::load_config()`, na criação do jogo (0x9226c0, Setup.cpp). Abre o
/// CM3_DATA\weather.cfg (0x90bde0, fopen 0x945b00 "r"); a primeira linha (0x90cba0, buffer de 1000) dá quantas regiões
/// ("%ld" em +8); as regiões (malloc de 0x6e bytes cada em +0) e a tabela das cidades (malloc de 4 bytes por cidade
/// [0xae2354] em +4); cada região pela 0x922c40 (a primeira que falha para o jogo; as outras só encerram a leitura);
/// cada cidade sorteia as três medidas ((sorteio(100) + sorteio(100)) × 0,5) e o vento (sorteio(3)). Fecha o arquivo
/// (0x945a09). Os erros (arquivo, linha, contagem, memória) são a caixa "Error" de weather.cpp e o jogo para.
unsafe extern "thiscall" fn ler_o_clima(this: usize) {
    let pasta: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let ler_linha: unsafe extern "C" fn(usize, *mut u8) -> i32 = core::mem::transmute(0x90cba0usize);
    let ler: unsafe extern "C" fn(*const u8, usize, ...) -> i32 = core::mem::transmute(0x946fd8usize);
    let malloc: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944e46usize);
    let regiao: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x922c40usize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let f = abrir(pasta(0x9b5348, 0xad4e18), 0x9899fc);
    if f == 0 {
        erro_em(0xad4dec, 0x68);
    }
    let mut linha = [0u8; 1000];
    if ler_linha(f, linha.as_mut_ptr()) == 0 {
        erro_em(0xad4dec, 0x6f);
    }
    if ler(linha.as_ptr(), 0x98c11c, this + 8) != 1 {
        erro_em(0xad4dec, 0x75);
    }
    let r = malloc(cpt7_d(this + 8).wrapping_mul(0x6e));
    *(this as *mut usize) = r;
    if r == 0 {
        erro_em(0xad4dec, 0x7e);
    }
    let c = malloc(cpt7_d(0xae2354).wrapping_mul(4));
    *((this + 4) as *mut usize) = c;
    if c == 0 {
        erro_em(0xad4dec, 0x86);
    }
    let mut i = 0i32;
    while i < cpt7_i(this + 8) {
        if regiao(this, f, cpt7_d(this) + i as usize * 0x6e) == 0 {
            if i == 0 {
                erro_em(0xad4dec, 0x92);
            }
            break;
        }
        i += 1;
    }
    let mut k = 0usize;
    while (k as i32) < cpt7_i(0xae2354) {
        crate::comparacao::ramo("cpt_7 clima inicial");
        for j in 0..3usize {
            let a = cpt7_sortear(100);
            let b = cpt7_sortear(100); // cpt7: o segundo sorteio da medida
            *((cpt7_d(this + 4) + k * 4 + j) as *mut u8) = ftol(a.wrapping_add(b) as f64 * f64_em(0x968968)) as u8;
        }
        let v = cpt7_sortear(3); // cpt7: o vento inicial
        *((cpt7_d(this + 4) + k * 4 + 3) as *mut u8) = v as u8;
        k += 1;
    }
    fechar(f);
}

// ponytail: abre o arquivo, aloca e sorteia: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_ler_o_clima(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1175)(this)
}

// ---------- 256a leva: a força dos treinadores no treino: o fator do treinador por categoria, a comissão de treino do técnico humano e a carga do programa de treino (training_manager.cpp) ----------

unsafe fn cpt8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpt8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// Os seis divisores da média dos treinadores (floats imediatos da 0x8b41d0: 1; 1,45; 1,7; 1,9; 2,05; 2,2), pela
/// quantidade de treinadores (1 a 6).
fn cpt8_divisor(n: i8) -> f64 {
    const T: [u32; 6] = [0x3f800000, 0x3fb9999a, 0x3fd9999a, 0x3ff33333, 0x40033333, 0x400ccccd];
    f32::from_bits(T[(n - 1) as usize]) as f64
}

/// { 0x8b41d0 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_coaching_factor(clube, categoria)`,
/// pela TRAINING_MANAGER::update_player_training (0x8b1750, cinco vezes por jogador em cada atualização do treino; o
/// valor entra como (fator − 100) / 5 no peso do sorteio do progresso). Clube nulo: 0. Sem técnico (+0xcf) ou com técnico
/// do computador (id abaixo de [0xae2364] − 16): 100 (o eax leva os 16 bits altos do ponteiro do técnico, como na
/// original). Técnico humano: a comissão dele (0x8b4cd0, seis vagas de 9 bytes: id −1 é vaga vazia, a nota da categoria
/// em +4 + categoria); com n treinadores de nota, a soma (16 bits) dividida pelo n-ésimo divisor e truncada (_ftol
/// 0x945640), × 40/7 (0x970400) e truncada; abaixo de 50, 50. Quente no ramo do computador: sem contador ali.
unsafe extern "thiscall" fn fator_do_treinador(this: usize, clube: usize, categoria: i8) -> u32 {
    let _ = this;
    if clube == 0 {
        return 0;
    }
    let t = cpt8_d(clube + 0xcf);
    if t == 0 {
        crate::comparacao::ramo("cpt_8 sem tecnico");
        return 100; // cpt8: sem técnico
    }
    if cpt8_i(t) < cpt8_i(0xae2364).wrapping_sub(0x10) {
        return (t as u32 & 0xffff_0000) | 100; // cpt8: técnico do computador
    }
    let comissao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8b4cd0usize);
    let s = comissao(this, t);
    let (mut n, mut soma) = (0i8, 0i16);
    for k in 0..6usize {
        let e = s + 9 * k;
        if cpt8_i(e) == -1 {
            continue;
        }
        let nota = *((e as isize + 4 + categoria as isize) as *const i8);
        if nota != 0 {
            n += 1;
            soma = soma.wrapping_add(nota as i16);
        }
    }
    if n != 0 {
        crate::comparacao::ramo("cpt_8 treinadores");
        soma = ftol(soma as f64 / cpt8_divisor(n)) as i16; // cpt8: a média dos treinadores
    }
    let r = ftol(soma as f64 * f64_em(0x970400));
    if (r as i16) < 0x32 {
        return 0x32;
    }
    r as u32
}

/// Comparador: a devolução inteira (a consulta só lê; a comissão do humano é lida pelas duas).
unsafe extern "thiscall" fn comparar_fator_do_treinador(this: usize, clube: usize, categoria: i8) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, i8) -> u32>(1176)(this, clube, categoria);
    let n = fator_do_treinador(this, clube, categoria);
    PLACAR.registrar(r == n, || format!("fator_do_treinador(clube {:x}, categoria {}) original={:x} rust={:x}", clube, categoria, r, n));
    r
}

/// { 0x8b4cd0 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::get_coach_assignments(tecnico)`: a comissão
/// de treino (seis vagas de 9 bytes) de um dos 16 técnicos humanos, em this + 0x652d + 54 × ([0xae2364] − id − 1).
/// Técnico nulo: a caixa "Error" (linha 0xa20) e 0; fora dos 16 humanos: a caixa (linha 0xa2a) e 0.
unsafe extern "thiscall" fn comissao_do_humano(this: usize, tecnico: usize) -> usize {
    if tecnico == 0 {
        aviso_em(0xa8ccd0, 0xa20);
        return 0;
    }
    let v = cpt8_i(0xae2364).wrapping_sub(cpt8_i(tecnico)).wrapping_sub(1);
    if !(0..0x10).contains(&v) {
        aviso_em(0xa8ccd0, 0xa2a);
        return 0;
    }
    this + 0x652d + 54 * v as usize
}

/// Comparador: a devolução, sem os casos de erro (só a original).
unsafe extern "thiscall" fn comparar_comissao_do_humano(this: usize, tecnico: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1177);
    if tecnico == 0 || !(0..0x10).contains(&cpt8_i(0xae2364).wrapping_sub(cpt8_i(tecnico)).wrapping_sub(1)) {
        return o(this, tecnico);
    }
    let r = o(this, tecnico);
    let n = comissao_do_humano(this, tecnico);
    PLACAR.registrar(r == n, || format!("comissao_do_humano(tecnico {:x}) original={:x} rust={:x}", tecnico, r, n));
    r
}

/// { 0x8b4f60 } (thiscall, ret 0, training_manager.cpp): `TRAINING_SCHEDULE::get_training_load()`: a carga do
/// programa pelos cinco níveis (+4..+8, com sinal): trunc(+8 × 0,5) − trunc(+4 × −1,3) − trunc(+7 × −0,5) −
/// trunc(+6 × −0,5) − trunc(+5 × −0,5) (constantes 0x968968, 0x96f878, 0x969540; _ftol 0x945640), em 16 bits; de 100
/// para cima, 100. Pela update_player_training (0x8b1750, o jogador do clube humano), pela set_player_schedule
/// (0x8b2250) e pela reassign_squad_schedules (0x8b2470); vai para a ficha de forma (Injury::set_training_load 0x62c660).
unsafe extern "thiscall" fn carga_do_programa(this: usize) -> i32 {
    let nivel = |k: usize| *((this + k) as *const i8) as f64;
    let a = ftol(nivel(4) * f64_em(0x96f878)) as i16;
    let mut s = ftol(nivel(8) * f64_em(0x968968)) as i16;
    s = s.wrapping_sub(a);
    s = s.wrapping_sub(ftol(nivel(7) * f64_em(0x969540)) as i16);
    s = s.wrapping_sub(ftol(nivel(6) * f64_em(0x969540)) as i16);
    s = s.wrapping_sub(ftol(nivel(5) * f64_em(0x969540)) as i16);
    if s >= 0x64 {
        return 0x64;
    }
    s as i32
}

/// Comparador: a devolução.
unsafe extern "thiscall" fn comparar_carga_do_programa(this: usize) -> i32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1178)(this);
    let n = carga_do_programa(this);
    PLACAR.registrar(r == n, || format!("carga_do_programa({:x}) original={} rust={}", this, r, n));
    r
}

// ---------- 256a leva: o progresso do treino nos atributos: aplicar o progresso das cinco categorias, achar o treino de posição do jogador e se ele aprende uma posição depressa (training_manager.cpp) ----------

unsafe fn cpt9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpt9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpt9_b(a: usize) -> i8 {
    *(a as *const i8)
}

/// O atributo de 1 a 20 em `a` mais `v`, entre 1 e 20.
unsafe fn cpt9_soma20(a: usize, v: i32) {
    *(a as *mut i8) = (cpt9_b(a) as i32 + v).clamp(1, 20) as i8;
}

/// O atributo de −125 a 125 em `a` mais `v`, entre −125 e 125.
unsafe fn cpt9_soma125(a: usize, v: i32) {
    *(a as *mut i8) = (cpt9_b(a) as i32 + v).clamp(-125, 125) as i8;
}

/// Quanto o atributo anda com o progresso `d` sobre o meio byte guardado `n` (com sinal): (d + n) / 2 − n / 2, as
/// divisões truncadas para o zero e a conta em 8 bits.
fn cpt9_passo(d: i8, n: i32) -> i32 {
    (((d as i32 + n) / 2) as i8).wrapping_sub((n / 2) as i8) as i32
}

/// O meio byte alto de `a` mais `d` (o baixo fica).
unsafe fn cpt9_alto(a: usize, d: i8) {
    let v = *(a as *const u8);
    *(a as *mut u8) = ((v & 0xf0).wrapping_add((d as u8) << 4)) | (v & 0x0f);
}

/// O meio byte baixo de `a` mais `d` (o alto fica).
unsafe fn cpt9_baixo(a: usize, d: i8) {
    let v = *(a as *const u8);
    *(a as *mut u8) = (v & 0xf0) | (v.wrapping_add(d as u8) & 0x0f);
}

/// { 0x8b2be0 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::apply_training_progress(pessoa, d)`, pela
/// update_player_training (0x8b1750, cada jogador em cada atualização do treino) e pelas 0x8b3640 e 0x8b37c0. Pessoa
/// nula: a caixa "Error" (linha 0x4b3); sem a ficha (+0x61): a caixa (linha 0x4ba); id a partir de [0xae2364]: nada.
/// Com a entrada de 6 bytes do jogador (this +0x651d + 6 × id da ficha), cada categoria com progresso `d[k]` diferente
/// de zero anda o passo (cpt9_passo sobre o meio byte guardado) nos seus atributos da ficha (os de 1 a 20 o passo, os de
/// −125 a 125 seis vezes o passo) e guarda o meio byte mais `d[k]`: a 3 (+4 alto), a 0 (+3 baixo), a 2 (+4 baixo), a 1
/// (+3 alto) e a 4 (+5 baixo). Depois o treino de posição (0x8b4b20): com a entrada de aprendizado de 15 para cima, a
/// posição ou o lado da máscara vai a 20; abaixo, volta a 1 se ainda sobrar mais de uma posição (+0xf..+0x15) ou lado
/// (+0x17..+0x19) de 15 para cima. Quente: sem contador.
unsafe extern "thiscall" fn aplicar_o_progresso(this: usize, pessoa: usize, d: usize) {
    if pessoa == 0 {
        aviso_em(0xa8ccd0, 0x4b3);
        return;
    }
    let f = cpt9_d(pessoa + 0x61);
    if f == 0 {
        aviso_em(0xa8ccd0, 0x4ba);
        return;
    }
    if cpt9_i(pessoa) >= cpt9_i(0xae2364) {
        return;
    }
    let e = cpt9_d(this + 0x651d) + 6 * cpt9_i(f) as usize;
    // a categoria 3 (o meio byte alto de +4)
    let k = cpt9_b(d + 3);
    if k != 0 {
        let dd = cpt9_passo(k, (cpt9_b(e + 4) >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6;
            cpt9_soma20(f + 0x22, dd);
            cpt9_soma125(f + 0x23, seis);
            cpt9_soma125(f + 0x26, seis);
            cpt9_soma20(f + 0x29, dd);
            cpt9_soma125(f + 0x2b, seis);
            cpt9_soma125(f + 0x37, seis);
            cpt9_soma125(f + 0x3e, seis);
            cpt9_soma20(f + 0x40, dd);
            cpt9_soma125(f + 0x41, seis);
        }
        cpt9_alto(e + 4, k); // cpt9: guarda o progresso da categoria 3
    }
    // a categoria 0 (o meio byte baixo de +3)
    let k = cpt9_b(d);
    if k != 0 {
        let dd = cpt9_passo(k, (cpt9_b(e + 3) << 4 >> 4) as i32);
        if dd != 0 {
            cpt9_soma20(f + 0x1b, dd); // cpt9: o primeiro atributo da categoria 0
            cpt9_soma20(f + 0x1d, dd);
            cpt9_soma20(f + 0x2e, dd);
            cpt9_soma20(f + 0x34, dd);
            cpt9_soma20(f + 0x36, dd);
            cpt9_soma125(f + 0x3a, dd * 6);
            cpt9_soma20(f + 0x3c, dd);
            cpt9_soma20(f + 0x3d, dd);
        }
        cpt9_baixo(e + 3, k);
    }
    // a categoria 2 (o meio byte baixo de +4)
    let k = cpt9_b(d + 2);
    if k != 0 {
        let dd = cpt9_passo(k, (cpt9_b(e + 4) << 4 >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6;
            cpt9_soma125(f + 0x27, seis);
            cpt9_soma125(f + 0x31, seis);
            cpt9_soma125(f + 0x38, seis);
        }
        cpt9_baixo(e + 4, k);
    }
    // a categoria 1 (o meio byte alto de +3)
    let k = cpt9_b(d + 1);
    if k != 0 {
        let dd = cpt9_passo(k, (cpt9_b(e + 3) >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6; // cpt9: a categoria 1, seis vezes o passo
            cpt9_soma125(f + 0x24, seis);
            cpt9_soma125(f + 0x32, seis);
            cpt9_soma125(f + 0x33, seis);
            cpt9_soma20(f + 0x3f, dd);
            cpt9_soma125(f + 0x39, seis);
        }
        cpt9_alto(e + 3, k);
    }
    // a categoria 4 (o meio byte baixo de +5)
    let k = cpt9_b(d + 4);
    if k != 0 {
        let dd = cpt9_passo(k, (cpt9_b(e + 5) << 4 >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6;
            cpt9_soma125(f + 0x2a, seis);
            cpt9_soma125(f + 0x35, seis);
        }
        cpt9_baixo(e + 5, k);
    }
    // o treino de posição
    let achar: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32 = core::mem::transmute(0x8b4b20usize);
    let mut p = 0usize;
    let n = achar(this, pessoa, &mut p);
    if n <= 0 {
        return;
    }
    let de15 = |o: usize| (cpt9_b(f + o) >= 0xf) as i8;
    let mut posicoes = de15(0xf) + de15(0x10) + de15(0x11) + de15(0x12) + de15(0x13) + de15(0x14) + de15(0x15);
    let mut lados = de15(0x18) + de15(0x19) + de15(0x17);
    for j in 0..n as usize {
        let item = p + 8 * j;
        let campo = match *((item + 4) as *const i16) as i32 {
            1 => 0xf,
            2 => 0x10,
            4 => 0x11,
            8 => 0x12,
            0x10 => 0x13,
            0x20 => 0x14,
            0x40 => 0x15,
            0x80 => 0x18,
            0x200 => 0x19,
            0x800 => 0x17,
            _ => continue,
        };
        if cpt9_b(item + 6) >= 0xf {
            *((f + campo) as *mut u8) = 0x14;
        } else if campo == 0x18 || campo == 0x19 || campo == 0x17 {
            if lados > 1 {
                *((f + campo) as *mut u8) = 1;
                lados -= 1;
            }
        } else if posicoes > 1 {
            *((f + campo) as *mut u8) = 1;
            posicoes -= 1;
        }
    }
}

/// Comparador: lado a lado na ficha (+0xf..+0x41) e na entrada de treino do jogador (6 bytes), sem os casos de erro e
/// sem as pessoas que não são jogadores (só a original).
unsafe extern "thiscall" fn comparar_aplicar_o_progresso(this: usize, pessoa: usize, d: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1179);
    if pessoa == 0 || cpt9_d(pessoa + 0x61) == 0 || cpt9_i(pessoa) >= cpt9_i(0xae2364) {
        return o(this, pessoa, d);
    }
    let f = cpt9_d(pessoa + 0x61);
    let e = cpt9_d(this + 0x651d) + 6 * cpt9_i(f) as usize;
    let regioes = [(f + 0xf, 0x33), (e, 6)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, pessoa, d), || aplicar_o_progresso(this, pessoa, d));
    PLACAR.registrar(igual, || format!("aplicar_o_progresso(pessoa {:x}, d {:x?}) {:x?} x {:x?}", pessoa, core::slice::from_raw_parts(d as *const i8, 5), mo.0, mr.0));
}

/// { 0x8b4b20 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::find_position_training(pessoa, &primeira)`:
/// pessoa nula, a caixa "Error" (linha 0x995); saída nula, a caixa (linha 0x99c); sem a ficha (+0x61), a caixa (linha
/// 0x9a3): 0. Sem a marca 0x10 em +5 da entrada de treino do jogador (this +0x651d + 6 × id da ficha): 0. Senão a busca
/// binária (0x946464, ordem 0x8b4e00) do id da pessoa na lista do treino de posição (this +0x6521, conta +0x6525, itens
/// de 8 bytes); não achou: 0; achou: recua até o primeiro item da pessoa, grava-o na saída e devolve quantos seguidos são
/// dela.
unsafe extern "thiscall" fn achar_o_treino_de_posicao(this: usize, pessoa: usize, saida: *mut usize) -> i32 {
    if pessoa == 0 {
        aviso_em(0xa8ccd0, 0x995);
        return 0;
    }
    if saida.is_null() {
        aviso_em(0xa8ccd0, 0x99c);
        return 0;
    }
    let f = cpt9_d(pessoa + 0x61);
    if f == 0 {
        aviso_em(0xa8ccd0, 0x9a3);
        return 0;
    }
    if *((cpt9_d(this + 0x651d) + 6 * cpt9_i(f) as usize + 5) as *const u8) & 0x10 == 0 {
        return 0;
    }
    let chave = cpt9_i(pessoa);
    let busca: unsafe extern "C" fn(*const i32, usize, usize, usize, usize) -> usize = core::mem::transmute(0x946464usize);
    let base = cpt9_d(this + 0x6521);
    let mut a = busca(&chave, base, cpt9_d(this + 0x6525), 8, 0x8b4e00);
    if a == 0 {
        return 0;
    }
    while a > base {
        if cpt9_i(a - 8) != chave {
            break;
        }
        a -= 8;
    }
    *saida = a;
    let ultimo = cpt9_d(this + 0x6521) + cpt9_d(this + 0x6525) * 8 - 8;
    let mut n = 0i32;
    while a <= ultimo && cpt9_i(a) == chave {
        a += 8;
        n += 1;
    }
    n
}

/// Comparador: a devolução e a saída (reposta entre as duas), sem os casos de erro (só a original).
unsafe extern "thiscall" fn comparar_achar_o_treino_de_posicao(this: usize, pessoa: usize, saida: *mut usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32>(1180);
    if pessoa == 0 || saida.is_null() || cpt9_d(pessoa + 0x61) == 0 {
        return o(this, pessoa, saida);
    }
    let antes = *saida;
    let r = o(this, pessoa, saida);
    let so = *saida;
    *saida = antes;
    let n = achar_o_treino_de_posicao(this, pessoa, saida);
    let sr = *saida;
    *saida = so;
    PLACAR.registrar(r == n && so == sr, || format!("achar_o_treino_de_posicao(pessoa {:x}) original={} {:x} rust={} {:x}", pessoa, r, so, n, sr));
    r
}

/// { 0x8b4db0 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::has_fast_position_training(pessoa)`, pelo
/// diário das pessoas (game_daily_staff_update 0x5ce2d0): 1 se algum item do treino de posição da pessoa (0x8b4b20) tem
/// aprendizado (+7) acima de 50; senão 0.
unsafe extern "thiscall" fn aprende_depressa(this: usize, pessoa: usize) -> u32 {
    let achar: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32 = core::mem::transmute(0x8b4b20usize);
    let mut p = 0usize;
    let n = achar(this, pessoa, &mut p);
    for j in 0..n.max(0) as usize {
        if cpt9_b(p + 8 * j + 7) > 0x32 {
            return 1;
        }
    }
    0
}

/// Comparador: a devolução (as duas só leem; a busca roda pelas duas).
unsafe extern "thiscall" fn comparar_aprende_depressa(this: usize, pessoa: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1181)(this, pessoa);
    let n = aprende_depressa(this, pessoa);
    PLACAR.registrar(r == n, || format!("aprende_depressa(pessoa {:x}) original={} rust={}", pessoa, r, n));
    r
}

// ---------- 256a leva: a semana do treino, a reserva de ficha de não jogador e a lesão que volta (training_manager.cpp, player_regen.cpp, injury.cpp) ----------

unsafe fn cpt10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

/// { 0x8b1730 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::advance_week_counter(&dia)`, todo dia pelo
/// laço do dia (game_main_day_loop 0x5cccd0, com o dia do ano da data do jogo 0xae2c90): no dia múltiplo de 7 (resto
/// com sinal), a semana do treino (+0x688d, 16 bits) sobe 1; a update_player_training (0x8b1750) põe em dia as semanas
/// que faltam de cada jogador. Devolve dia / 7 (o chamador não lê).
unsafe extern "thiscall" fn avancar_a_semana(this: usize, dia: *const i16) -> i32 {
    let v = *dia as i32;
    let resto = v % 7; // cpt10: a semana de 7 dias
    if resto == 0 { // cpt10: o sétimo dia
        crate::comparacao::ramo("cpt_10 semana");
        let s = (this + 0x688d) as *mut u16;
        *s = (*s).wrapping_add(1); // cpt10: mais uma semana
    }
    v / 7
}

/// Comparador: lado a lado na semana do treino, com a devolução.
unsafe extern "thiscall" fn comparar_avancar_a_semana(this: usize, dia: *const i16) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i16) -> i32>(1182);
    let regioes = [(this + 0x688d, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, dia), || avancar_a_semana(this, dia));
    PLACAR.registrar(igual && r == n, || format!("avancar_a_semana(dia {}) original={} rust={} {:x?} x {:x?}", *dia, r, n, mo.0, mr.0));
    r
}

/// { 0x7a6e30 } (thiscall, ret 0xc, player_regen.cpp): `PLAYER_REGEN::take_spare_non_player_record(&id, &nacao,
/// &segunda)`, pela contract_manager (0x4ce4e0, duas chamadas): copia o primeiro item da reserva de fichas de não
/// jogador (this +0x1c: id, nação e segunda nação) para as três saídas e tira-o da reserva
/// (PLAYER_REGEN::remove_spare_non_player_record 0x7a82b0 com o id lido da saída); devolve o eax dela.
unsafe extern "thiscall" fn tirar_a_ficha_reservada(this: usize, id: *mut u32, nacao: *mut u32, segunda: *mut u32) -> u32 {
    crate::comparacao::ramo("cpt_10 ficha reservada");
    *id = cpt10_d(cpt10_d(this + 0x1c)) as u32;
    *nacao = cpt10_d(cpt10_d(this + 0x1c) + 4) as u32;
    *segunda = cpt10_d(cpt10_d(this + 0x1c) + 8) as u32;
    let tirar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x7a82b0usize);
    tirar(this, *id)
}

// ponytail: a reserva perde o item (não roda duas vezes): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_tirar_a_ficha_reservada(this: usize, id: *mut u32, nacao: *mut u32, segunda: *mut u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u32) -> u32>(1183)(this, id, nacao, segunda)
}

/// { 0x62ed30 } (thiscall, ret 4, injury.cpp): `Injury::get_recurring_injury_type(pessoa)`, pelas notícias (0x77bc80),
/// pela avaliação do fisioterapeuta (0x7a2650) e pelas telas: o tipo da lesão que volta (+0x12 da ficha de forma de 31
/// bytes da pessoa, em [this]); pessoa nula, id a partir de [0xae2364] ou sem a ficha de jogador (+0x61): 0xff. Os
/// chamadores leem só o al.
unsafe extern "thiscall" fn lesao_que_volta(this: usize, pessoa: usize) -> u8 {
    if pessoa == 0 || cpt10_d(pessoa) as i32 >= cpt10_d(0xae2364) as i32 || cpt10_d(pessoa + 0x61) == 0 {
        return 0xff;
    }
    *((cpt10_d(this) + 31 * cpt10_d(pessoa)) as *const u8).add(0x12)
}

/// Comparador: o al devolvido.
unsafe extern "thiscall" fn comparar_lesao_que_volta(this: usize, pessoa: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1184)(this, pessoa);
    let n = lesao_que_volta(this, pessoa);
    PLACAR.registrar(r == n, || format!("lesao_que_volta(pessoa {:x}) original={:x} rust={:x}", pessoa, r, n));
    r
}

// ---------- 256a leva: a pré-temporada dos amistosos: o dia dos amistosos, o plano da pré-temporada da IA e as janelas da pré-temporada (friendly.cpp) ----------

unsafe fn cpt11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpt11_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cpt11_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32);
}

/// A data `a` é depois da `b` (+2 o ano, sem sinal; +0 o dia, com sinal), como as comparações da original.
unsafe fn cpt11_depois(a: usize, b: usize) -> bool {
    let (aa, ab) = (cpt11_w(a + 2), cpt11_w(b + 2));
    aa > ab || (aa == ab && cpt11_w(a) as i16 > cpt11_w(b) as i16)
}

/// { 0x5c2460 } (thiscall, ret 8, friendly.cpp): `Friendly::update_for_date(hoje, ativo)`: com `ativo`, nos dois dias
/// da pré-temporada (+0x34ea, +0x34e8) e com o início do jogo [0xdd6ec8] até hoje + 8 semanas (0x54c770; depois disso,
/// nada mais no dia), guarda hoje em +0xa7, planeja a pré-temporada da IA (0x5c27a0) e cancela (0x5c8260) os torneios
/// em uso (+0x65) com data anterior a um ano atrás (0x54cab0, 0x456770). Todo dia: cria os amistosos planejados
/// (0x5c2ae0), atualiza os amistosos das seleções (0x5cbb00), tira e responde as respostas dos torneios (+0x34d0,
/// 0x5c5820) e responde as dos amistosos (+0x34d4, 0x5c5f40), tira os pedidos sem clubes ou vencidos (+0x34c8), cancela
/// os torneios fora de uso cuja data − 2 dias (0x54c8e0) já passou e zera os convites vencidos (+0x34cc, 10 de 0x19
/// bytes por item, a data em +9, a marca em +0x15). Devolve 1.
unsafe extern "thiscall" fn atualizar_amistosos_do_dia(this: usize, hoje: usize, ativo: u32) -> u32 {
    if ativo == 0 {
        return 1;
    }
    let semanas: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c770usize);
    let menos_semanas: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54cab0usize);
    let menos_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let anterior: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let cancelar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5c8260usize);
    let planejar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5c27a0usize);
    let criar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5c2ae0usize);
    let selecoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5cbb00usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let resposta_do_torneio: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x5c5820usize);
    let resposta_do_amistoso: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5c5f40usize);
    let dia = cpt11_w(hoje) as i16 as i32;
    if dia == cpt11_w(this + 0x34ea) as i32 || dia == cpt11_w(this + 0x34e8) as i32 {
        let mut t = [0u8; 8];
        let limite = semanas(hoje, &mut t, 8);
        if cpt11_depois(0xdd6ec8, limite) {
            return 1;
        }
        copiar(this + 0xa7, &mut t, hoje);
        crate::comparacao::ramo("cpt_11 planejou a pré-temporada");
        planejar(this); // cpt11: planejar
        let mut ha_um_ano = [0u8; 8];
        menos_semanas(0xae2c90, &mut ha_um_ano, 0x34);
        let lo = u32::from_le_bytes([ha_um_ano[0], ha_um_ano[1], ha_um_ano[2], ha_um_ano[3]]);
        let hi = u32::from_le_bytes([ha_um_ano[4], ha_um_ano[5], ha_um_ano[6], ha_um_ano[7]]);
        for k in 0..0x7f {
            let e = this + 0xb1 + k * 0x69;
            if cpt11_d(e) != 0 && anterior(e + 4, lo, hi) != 0 && cpt11_d(e + 0x65) != 0 {
                cancelar(this, e, 0);
            }
        }
    }
    criar(this);
    selecoes(this);
    let l = cpt11_d(this + 0x34d0);
    if l != 0 {
        let conta = cpt11_w(l) as i16;
        cpt11_pd(l + 0xa, cpt11_d(l + 2));
        if conta > 0 {
            let mut n = conta as i32;
            loop {
                let item = proximo(cpt11_d(this + 0x34d0));
                if item != 0 {
                    let r = cpt11_d(item);
                    tirar(cpt11_d(this + 0x34d0), item);
                    resposta_do_torneio(this, r, 1, 0);
                }
                let l = cpt11_d(this + 0x34d0);
                n -= 1;
                cpt11_pd(l + 0xa, cpt11_d(l + 2));
                if n == 0 {
                    break;
                }
            }
        }
    }
    let l = cpt11_d(this + 0x34d4);
    if l != 0 {
        cpt11_pd(l + 0xa, cpt11_d(l + 2));
        let mut item = proximo(cpt11_d(this + 0x34d4));
        while item != 0 {
            resposta_do_amistoso(this, cpt11_d(item), 0);
            item = proximo(cpt11_d(this + 0x34d4));
        }
    }
    let l = cpt11_d(this + 0x34c8);
    if l != 0 {
        cpt11_pd(l + 0xa, cpt11_d(l + 2));
        let mut p = proximo(cpt11_d(this + 0x34c8));
        while p != 0 {
            if cpt11_d(p + 0x10) == 0 || cpt11_d(p + 0x14) == 0 {
                tirar(cpt11_d(this + 0x34c8), p);
            }
            // ponytail: como a original, a data do pedido é olhada mesmo depois de tirá-lo (o bloco já tirado)
            if cpt11_depois(hoje, p) {
                tirar(cpt11_d(this + 0x34c8), p);
            }
            p = proximo(cpt11_d(this + 0x34c8));
        }
    }
    for k in 0..0x7f {
        let e = this + 0xb1 + k * 0x69;
        if cpt11_d(e) != 0 && cpt11_d(e + 0x65) == 0 {
            let mut t = [0u8; 8];
            let fim = menos_dias(e + 4, &mut t, 2);
            if cpt11_depois(hoje, fim) {
                cancelar(this, e, 0);
            }
        }
    }
    let l = cpt11_d(this + 0x34cc);
    if l != 0 {
        cpt11_pd(l + 0xa, cpt11_d(l + 2));
        let mut item = proximo(cpt11_d(this + 0x34cc));
        while item != 0 {
            for k in 0..10 {
                let s = item + 0x15 + k * 0x19;
                let mut t = [0u8; 8];
                let fim = menos_dias(s - 0xc, &mut t, 2);
                if cpt11_depois(hoje, fim) {
                    cpt11_pd(s, 0);
                }
            }
            item = proximo(cpt11_d(this + 0x34cc));
        }
    }
    // ponytail: a original faz `mov al, 1` e deixa sobra em cima; quem chama testa o byte
    1
}

/// { 0x5c27a0 } (thiscall, ret 0, friendly.cpp): `Friendly::plan_ai_preseason_friendlies()`: monta um candidato de
/// 0x16 bytes (+0 o id do clube, +4 zero, +5 a faixa = reputação +0x80 / 500, +6 o id da nação, +0xa e +0x12 listas
/// nulas, +0xe/+0x10 a janela −1) para cada clube ([0xae23b4], 0x245 bytes, [0xae235c] deles) que não é seleção
/// (0x53b900) e tem nação; ordena os candidatos pela faixa (qsort 0x945333 com 0x5c5270) e um vetor de ponteiros pela
/// nação e faixa (0x5c5290); põe as janelas (0x5c2d40), planeja torneios e excursões (0x5c3040) e os amistosos
/// (0x5c4770); libera as listas dos candidatos (0x68d040 + 0x944c94) e os dois vetores (0x9452ca). Sem memória para os
/// candidatos: a caixa (linha 0x18a).
unsafe extern "thiscall" fn planejar_pre_temporada_da_ia(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let janelas: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(0x5c2d40usize);
    let torneios: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(0x5c3040usize);
    let amistosos: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(0x5c4770usize);
    let fim_da_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let v = alocar((cpt11_d(0xae235c) as u32).wrapping_mul(0x16));
    if v == 0 {
        return aviso_em(0x9cd0a0, 0x18a);
    }
    let mut n: i16 = 0;
    let mut clube = cpt11_d(0xae23b4);
    let mut i: i32 = 0;
    if cpt11_d(0xae235c) as i32 > 0 {
        loop {
            if e_selecao(clube) == 0 && cpt11_d(clube + 0x53) != 0 {
                let r = v + n as usize * 0x16;
                let rep = cpt11_w(clube + 0x80) as i16 as i32;
                crate::comparacao::ramo("cpt_11 candidato da pré-temporada");
                let faixa = (rep / 500) as u8; // cpt11: a faixa
                cpt11_pd(r, cpt11_d(clube));
                *((r + 4) as *mut u8) = 0;
                *((r + 5) as *mut u8) = faixa;
                cpt11_pd(r + 6, cpt11_d(cpt11_d(clube + 0x53)));
                cpt11_pd(r + 0xa, 0);
                cpt11_pd(r + 0x12, 0);
                core::ptr::write_unaligned((r + 0xe) as *mut u16, 0xffff);
                core::ptr::write_unaligned((r + 0x10) as *mut u16, 0xffff);
                n += 1;
            }
            clube += 0x245;
            i += 1;
            if i as i16 as i32 >= cpt11_d(0xae235c) as i32 {
                break;
            }
        }
    }
    let ns = n as i32;
    qsort(v, ns as u32, 0x16, 0x5c5270);
    let p = alocar((ns as u32).wrapping_mul(4)); // a original não confere
    for k in 0..ns.max(0) as usize {
        cpt11_pd(p + k * 4, v + k * 0x16);
    }
    qsort(p, ns as u32, 4, 0x5c5290);
    janelas(this, p, ns);
    torneios(this, v, p, ns);
    amistosos(this, v, p, ns);
    for k in 0..ns.max(0) as usize {
        for l in [cpt11_d(v + k * 0x16 + 0xa), cpt11_d(v + k * 0x16 + 0x12)] {
            if l != 0 {
                fim_da_lista(l);
                apagar(l);
            }
        }
    }
    liberar(v);
    liberar(p);
}

/// O trabalho da 0x5c2d40 sobre os `n` ponteiros para candidatos `ptrs` (ordenados pela nação e faixa): para cada uma
/// das 35 entradas de [0xb63d60] (0x48 bytes: +0 a nação, +0x26 a liga, +0x2a o início) com nação em uso (+0x11c):
/// o fim da janela é o início − 1 semana (0x54cab0) em dias desde hoje e o começo, 18 dias antes (no mínimo 3); acha
/// (bsearch 0x946464 com 0x5c5290) o primeiro candidato da nação e grava a janela (+0xe, +0x10) nos seguintes enquanto
/// o clube for da nação (sem o último do vetor, como a original); o primeiro e o último clube da tabela da liga (+0xa7
/// da competição de tipo 1) ganham uma lista nova (0x944cf1 + 0x68d020) em +0x12. Com `gravar` falso só monta a lista
/// (candidato, começo, fim, lista nova). A original calcula a data de início da liga (e + dias, tipo 2) e não usa.
unsafe fn cpt11_janelas(ptrs: usize, n: i32, gravar: bool) -> Vec<(usize, i16, i16, bool)> {
    let menos_semanas: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54cab0usize);
    let data_nova: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mais_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copiar: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let bsearch: unsafe extern "C" fn(*const usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let mut plano = Vec::new();
    for k in 0..0x23 {
        let ent = 0xb63d60 + k * 0x48;
        let nac = cpt11_d(ent);
        if nac == 0 || *((nac + 0x11c) as *const u8) == 0 {
            continue;
        }
        let (mut prim, mut ult) = (u32::MAX as usize, u32::MAX as usize);
        let lg = cpt11_d(ent + 0x26);
        let comp = if lg != 0 { cpt11_d(cpt11_d(0xadadfc) + cpt11_d(lg) * 4) } else { 0 };
        if comp != 0 && *((comp + 0x43) as *const u8) == 1 {
            if gravar {
                // as datas de início da liga que a original monta e não usa (só locais)
                let (mut t1, mut t2, mut t3, mut t4, mut t5, mut t6) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
                let x = cpt11_d(comp + 0xa3);
                let ano = *((x + 5) as *const i8) as i16 as u16 as u32;
                let r = data_nova(&mut t1, cpt11_w(x + 3) as u32, ano);
                copiar(&mut t2, &mut t3, r);
                if *((x + 0x21) as *const u8) == 2 {
                    let r = data_nova(&mut t4, cpt11_w(x + 3) as u32, ano);
                    let r = mais_dias(r, &mut t5, *((x + 0x22) as *const i8) as i16 as u16 as u32);
                    copiar(&mut t6, &mut t3, r);
                }
            }
            let t = cpt11_d(comp + 0xa7);
            let a = cpt11_d(t);
            prim = if a != 0 { cpt11_d(a) } else { u32::MAX as usize };
            let b = cpt11_d(t + 6);
            ult = if b != 0 { cpt11_d(b) } else { u32::MAX as usize };
        }
        let mut l = [0u8; 8];
        menos_semanas(ent + 0x2a, &mut l, 1);
        let dano = u16::from_le_bytes([l[2], l[3]]).wrapping_sub(cpt11_w(0xae2c92));
        let fim = (dano as u32).wrapping_mul(365).wrapping_sub(cpt11_d(0xae2c90) as u32)
            .wrapping_add(u32::from_le_bytes([l[0], l[1], l[2], l[3]])) as u16 as i16;
        let mut ini = fim.wrapping_sub(18); // cpt11: o início da janela
        if ini < 3 {
            ini = 3;
        }
        let mut chave = [0u8; 0x16];
        chave[5] = 0xff;
        chave[6..10].copy_from_slice(&(cpt11_d(nac) as u32).to_le_bytes());
        let pchave = chave.as_ptr() as usize;
        let mut q = bsearch(&pchave, ptrs, n as u32, 4, 0x5c5290);
        if q == 0 {
            continue;
        }
        while q > ptrs {
            q -= 4;
            if cpt11_d(cpt11_d(q) + 6) != cpt11_d(nac) {
                q += 4;
                break;
            }
        }
        let ate = ptrs.wrapping_add((n as usize).wrapping_mul(4)).wrapping_sub(4);
        while q < ate {
            let c = cpt11_d(q);
            let id = cpt11_d(c);
            if cpt11_d(cpt11_d(0xae23b4) + id * 0x245 + 0x53) != nac {
                break;
            }
            let nova = id == prim || id == ult;
            if gravar {
                if nova {
                    let m = novo(0xe);
                    cpt11_pd(c + 0x12, if m != 0 { lista_nova(m) } else { 0 });
                }
                crate::comparacao::ramo("cpt_11 janela gravada");
                core::ptr::write_unaligned((c + 0xe) as *mut i16, ini);
                core::ptr::write_unaligned((c + 0x10) as *mut i16, fim);
            }
            plano.push((c, ini, fim, nova));
            q += 4;
        }
    }
    plano
}

/// { 0x5c2d40 } (thiscall, ret 8, friendly.cpp): `Friendly::set_preseason_windows(candidatos, n)`: a janela da
/// pré-temporada de cada nação nos candidatos dela (ver `cpt11_janelas`); o `this` não é lido. A original constrói e
/// destrói duas datas locais que não usa (0x9463af / 0x9462bb com 0x54a110 / 0x5a53c0).
unsafe extern "thiscall" fn janelas_da_pre_temporada(_this: usize, ptrs: usize, n: i32) {
    cpt11_janelas(ptrs, n, true);
}

/// Comparador: anda o calendário e planeja (aloca, sorteia): só a original.
unsafe extern "thiscall" fn comparar_atualizar_amistosos_do_dia(this: usize, hoje: usize, ativo: u32) -> u32 {
    // ponytail: cancela torneios, tira pedidos e cria amistosos (não roda duas vezes); a cópia se prova pela carreira
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1185)(this, hoje, ativo)
}

/// Comparador: monta e planeja os amistosos da IA (aloca, sorteia): só a original.
unsafe extern "thiscall" fn comparar_planejar_pre_temporada_da_ia(this: usize) {
    // ponytail: os torneios, as excursões e os amistosos sorteiam e alocam; a cópia se prova pela carreira
    orig::<unsafe extern "thiscall" fn(usize)>(1186)(this)
}

/// Comparador: grava as janelas e cria listas (não roda duas vezes): o plano antes (sem gravar), a original grava;
/// cada candidato planejado tem de ter a janela e, quando é a ponta da tabela, uma lista em +0x12.
unsafe extern "thiscall" fn comparar_janelas_da_pre_temporada(this: usize, ptrs: usize, n: i32) {
    let plano = cpt11_janelas(ptrs, n, false);
    orig::<unsafe extern "thiscall" fn(usize, usize, i32)>(1187)(this, ptrs, n);
    let erros: Vec<String> = plano.iter()
        .filter(|&&(c, ini, fim, nova)| {
            core::ptr::read_unaligned((c + 0xe) as *const i16) != ini || core::ptr::read_unaligned((c + 0x10) as *const i16) != fim
                || (nova && cpt11_d(c + 0x12) == 0)
        })
        .map(|&(c, ini, fim, nova)| format!("candidato {:x}: esperado {}..{} lista {} obtido {}..{} lista {:x}", c, ini, fim, nova,
            core::ptr::read_unaligned((c + 0xe) as *const i16), core::ptr::read_unaligned((c + 0x10) as *const i16), cpt11_d(c + 0x12)))
        .collect();
    PLACAR.registrar(erros.is_empty(), || format!("janelas_da_pre_temporada(n {}) {} candidatos: {}", n, plano.len(), erros.join("; ")));
}

// ---------- 256a leva: as excursões da pré-temporada: marcar os jogos da excursão e as duas ordens dos candidatos do amistoso (friendly.cpp) ----------

unsafe fn cpt12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpt12_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// A data `a` (+2 o ano sem sinal, +0 o dia com sinal) comparada à `b`: -1, 0 ou 1.
unsafe fn cpt12_ordem(a: usize, b: usize) -> i32 {
    let (aa, ab) = (cpt12_w(a + 2), cpt12_w(b + 2));
    if aa != ab {
        return if aa < ab { -1 } else { 1 };
    }
    let (da, db) = (cpt12_w(a) as i16, cpt12_w(b) as i16);
    if da < db { -1 } else if da > db { 1 } else { 0 }
}

/// Uma lista nova (0x944cf1 de 0xe bytes + 0x68d020) ou 0 sem memória.
unsafe fn cpt12_lista_nova() -> usize {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let m = novo(0xe);
    if m != 0 { lista_nova(m) } else { 0 }
}

/// { 0x5c4040 } (thiscall, ret 0xc, friendly.cpp): `Friendly::schedule_tour_matches(candidato, adversários, n)`: a
/// janela é a do candidato (+0xe..+0x10) cortada pela do primeiro adversário (se ele tem uma) e encurtada em 2n + 1
/// dias; sem espaço, 0. Até 5 tentativas: o começo anda um sorteio (0x90cfc0) da largura (o começo fica andado entre
/// as tentativas, como na original), o primeiro jogo é o início da temporada (+0xa7) + começo e o último, esse + 2n + 1
/// (0x54c5b0); uma data já guardada na lista do candidato (+0x12) depois do primeiro − 2 dias ou antes do último + 2
/// estraga a tentativa (com qualquer data guardada, todas falham; na quinta, segue assim mesmo). Depois, para cada
/// adversário, com a data livre para ele (0x5c6b70) e o amistoso montado (0x5c6f00, o clube em casa), manda o pedido
/// (0x5c5570) e guarda a data na lista dos dois (criadas se faltam; 0x68d0d0); a data anda 2 dias a cada adversário.
/// Devolve 1.
unsafe extern "thiscall" fn marcar_jogos_da_excursao(this: usize, cand: usize, adv: usize, n: u32) -> u32 {
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let mais_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let menos_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let copiar: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let data_padrao: unsafe extern "thiscall" fn(*mut [u8; 8]) -> usize = core::mem::transmute(0x54a110usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let livre: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize, u32) -> usize = core::mem::transmute(0x5c6b70usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, *mut [u8; 8], usize, u32) -> usize = core::mem::transmute(0x5c6f00usize);
    let pedir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5c5570usize);
    let guardar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let n = n as u8 as i8;
    let mut ini = cpt12_w(cand + 0xe) as i16;
    let mut fim = cpt12_w(cand + 0x10) as i16;
    let a0 = cpt12_d(adv);
    let x = cpt12_w(a0 + 0xe) as i16;
    if x != -1 {
        if x > ini {
            ini = x;
        }
        let y = cpt12_w(a0 + 0x10) as i16;
        if y < fim {
            fim = y;
        }
    }
    let largura = (n as u8).wrapping_mul(2).wrapping_add(1) as i8 as i16; // cpt12: a largura da excursão
    fim = fim.wrapping_sub(largura);
    if ini >= fim {
        return 0;
    }
    crate::comparacao::ramo("cpt_12 janela da excursão");
    let (mut d1, mut d2, mut t, mut u) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    data_padrao(&mut d1);
    data_padrao(&mut d2);
    let mut tentativas: i8 = 0;
    while tentativas < 5 {
        let mut ok = true;
        ini = ini.wrapping_add(sortear(fim as i32 - ini as i32) as i16);
        let r = mais_dias(this + 0xa7, &mut t, ini as u16 as u32);
        copiar(&mut d1, &mut u, r);
        let r = mais_dias(d1.as_ptr() as usize, &mut t, largura as u16 as u32);
        copiar(&mut d2, &mut u, r);
        let l = cpt12_d(cand + 0x12);
        if l == 0 {
            break;
        }
        core::ptr::write_unaligned((l + 0xa) as *mut u32, cpt12_d(l + 2) as u32);
        let mut item = proximo(cpt12_d(cand + 0x12));
        while item != 0 {
            let antes = menos_dias(d1.as_ptr() as usize, &mut t, 2);
            let estraga = if cpt12_ordem(antes, item) < 0 {
                true
            } else {
                let depois = mais_dias(d2.as_ptr() as usize, &mut u, 2);
                cpt12_ordem(depois, item) > 0
            };
            if estraga {
                ok = false;
                break;
            }
            item = proximo(cpt12_d(cand + 0x12));
        }
        tentativas += 1;
        if ok {
            break;
        }
    }
    let clube = cpt12_d(0xae23b4) + cpt12_d(cand) * 0x245;
    let mut p = adv;
    for k in 0..(n as i32).max(0) {
        let outro = cpt12_d(0xae23b4) + cpt12_d(cpt12_d(p)) * 0x245;
        if livre(this, &mut d1, outro, 0) != 0 {
            let m = acrescentar(this, clube, &mut d1, outro, 0); // cpt12: o clube em casa
            if m != 0 {
                crate::comparacao::ramo("cpt_12 jogo da excursão marcado");
                pedir(this, m);
                if cpt12_d(cand + 0x12) == 0 {
                    let l = cpt12_lista_nova();
                    core::ptr::write_unaligned((cand + 0x12) as *mut u32, l as u32);
                }
                if cpt12_d(cpt12_d(p) + 0x12) == 0 {
                    let l = cpt12_lista_nova();
                    core::ptr::write_unaligned((cpt12_d(p) + 0x12) as *mut u32, l as u32);
                }
                guardar(cpt12_d(cand + 0x12), &mut d1, 8);
                guardar(cpt12_d(cpt12_d(p) + 0x12), &mut d1, 8);
            }
        }
        p += 4;
        if k + 1 < n as i32 {
            crate::comparacao::ramo("cpt_12 data do jogo seguinte");
        }
        let r = mais_dias(d1.as_ptr() as usize, &mut t, 2); // cpt12: o próximo jogo
        copiar(&mut d1, &mut u, r);
    }
    1
}

/// { 0x5c5270 } (cdecl, friendly.cpp): o comparador do qsort dos candidatos do amistoso pela faixa (+5, com sinal),
/// a maior primeiro: faixa de b − faixa de a.
unsafe extern "C" fn ordem_pela_faixa(a: usize, b: usize) -> i32 {
    *((b + 5) as *const i8) as i32 - *((a + 5) as *const i8) as i32
}

/// { 0x5c5290 } (cdecl, friendly.cpp): o comparador (qsort e bsearch) dos ponteiros para candidatos: pela nação (+6,
/// a diferença em 32 bits); na mesma nação, pela faixa (+5, a maior primeiro), salvo faixa 0xff em um dos dois (0).
unsafe extern "C" fn ordem_pela_nacao_e_faixa(a: usize, b: usize) -> i32 {
    let (ca, cb) = (cpt12_d(a), cpt12_d(b));
    let dif = (cpt12_d(ca + 6) as u32).wrapping_sub(cpt12_d(cb + 6) as u32) as i32;
    if dif != 0 {
        return dif;
    }
    let (fa, fb) = (*((ca + 5) as *const u8), *((cb + 5) as *const u8));
    if fa == 0xff || fb == 0xff {
        return 0;
    }
    fb as i8 as i32 - fa as i8 as i32
}

/// Comparador: sorteia, monta amistosos e cria listas (não roda duas vezes): só a original.
unsafe extern "thiscall" fn comparar_marcar_jogos_da_excursao(this: usize, cand: usize, adv: usize, n: u32) -> u32 {
    // ponytail: o sorteio e os amistosos novos não rodam duas vezes; a cópia se prova pela carreira
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(1188)(this, cand, adv, n)
}

/// Comparador: rotina pura, pela devolução.
unsafe extern "C" fn comparar_ordem_pela_faixa(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1189)(a, b);
    let n = ordem_pela_faixa(a, b);
    PLACAR.registrar(r == n, || format!("ordem_pela_faixa({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: rotina pura, pela devolução.
unsafe extern "C" fn comparar_ordem_pela_nacao_e_faixa(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1190)(a, b);
    let n = ordem_pela_nacao_e_faixa(a, b);
    PLACAR.registrar(r == n, || format!("ordem_pela_nacao_e_faixa({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 256a leva: as posições previstas da copa: as posições pela rodada de saída, ordenar os clubes pela força e gravar a posição do clube na estatística (Cup.cpp, comp_util.cpp, comp_stats.cpp) ----------

unsafe fn cpt13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpt13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O núcleo da 0x4c08f0 sobre os `n` clubes em `v` e as chaves em `k`: a chave de cada clube é a força
/// (0x4c0720; clube nulo, 0); depois, para cada i, todo j > i com chave maior troca de lugar com i (chave e clube),
/// com sinal.
unsafe fn cpt13_ordenar_com(v: *mut u32, k: *mut i32, n: i32) {
    let forca: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x4c0720usize);
    for i in 0..n.max(0) as usize {
        let c = *v.add(i);
        *k.add(i) = if c != 0 { forca(c as usize) } else { 0 };
    }
    let mut i = 0usize;
    while (i as i32) < n - 1 {
        for j in i + 1..n as usize {
            if *k.add(i) < *k.add(j) { // cpt13: a força maior sobe
                core::ptr::swap(k.add(i), k.add(j));
                core::ptr::swap(v.add(i), v.add(j));
            }
        }
        i += 1;
    }
}

/// { 0x4c08f0 } (cdecl, comp_util.cpp): `comp_util_sort_clubs_by_strength(clubes, n)`: ordena o vetor de `n` clubes
/// pela força (0x4c0720), a maior primeiro, com as chaves num vetor temporário (0x944e46 / 0x9452ca; a original o
/// enche de 0x60 antes, tudo sobrescrito). Sem memória: a caixa (linha 0x292d) e nada.
unsafe extern "C" fn ordenar_clubes_pela_forca(v: usize, n: i32) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let k = alocar((n as u32).wrapping_mul(4));
    if k == 0 {
        return aviso_em(0x99ec20, 0x292d);
    }
    cpt13_ordenar_com(v as *mut u32, k as *mut i32, n);
    if n > 1 {
        crate::comparacao::ramo("cpt_13 clubes ordenados");
    }
    liberar(k);
}

/// { 0x4a2540 } (thiscall, ret 8, comp_stats.cpp): `Comp_Stats::set_club_final_position(clube, posição)`: acha
/// (0x4a4850) ou acrescenta (0x4aceb0) o registro do clube na estatística da competição e grava a posição (byte) em
/// +0x72; devolve o registro. Clube nulo: a caixa (linha 0x680).
unsafe extern "thiscall" fn gravar_posicao_do_clube(this: usize, clube: usize, pos: u32) -> usize {
    if clube == 0 {
        aviso_em(0x99ca94, 0x680);
        return 0; // ponytail: a original devolve o eax da caixa; quem chama não lê
    }
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4aceb0usize);
    let mut e = buscar(this, clube);
    if e == 0 {
        e = acrescentar(this, clube);
    }
    crate::comparacao::ramo("cpt_13 posição gravada");
    core::ptr::write_volatile((e + 0x72) as *mut u8, pos as u8); // cpt13: a posição
    e
}

/// As chamadas da 0x5223a0 à 0x4a2540 (clube, posição), na ordem, com o vetor `v` já ordenado: da penúltima rodada
/// (+0x3c − 2) para trás ([+0xa3] + r × 0x68, a conta de quem sai em +0x1a), até um clube nulo no começo da rodada.
/// Na primeira rodada vista, os que saem nela ganham todos a posição 1 (sem olhar nulos); em cada rodada, os seguintes
/// ganham a posição do contador (que sobe a cada vaga, nula ou não), pulando os nulos. `gravar` = 0 só monta a lista.
unsafe fn cpt13_posicoes(this: usize, v: *const u32, gravar: usize) -> Vec<(usize, u8)> {
    let gravar_posicao: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    let mut chamadas: Vec<(usize, u8)> = Vec::new();
    let mut s: i16 = 0; // o índice no vetor (bp com sinal)
    let mut b: u8 = 1; // o contador de posições
    let mut a: u8 = 1; // a posição empurrada (só o byte de baixo conta; a original empurra a dword com sobra em cima)
    let mut r = cpt13_w(this + 0x3c).wrapping_sub(2) as i32;
    while r >= 0 {
        if *v.offset(s as isize) == 0 {
            break;
        }
        let conta = |r: i32| cpt13_w(cpt13_d(this + 0xa3) + r as usize * 0x68 + 0x1a) as i32;
        if r == cpt13_w(this + 0x3c) as i32 - 2 {
            let mut j = 0;
            while j < conta(r) {
                let c = *v.offset(s as isize) as usize;
                s = s.wrapping_add(1);
                chamadas.push((c, a));
                if gravar != 0 {
                    gravar_posicao(cpt13_d(this + 8), c, a as u32);
                }
                b = b.wrapping_add(1);
                j += 1;
            }
        }
        a = b; // cpt13: a posição da rodada
        let mut j = 0;
        while j < conta(r) {
            let c = *v.offset(s as isize) as usize;
            if c != 0 {
                s = s.wrapping_add(1);
                chamadas.push((c, a));
                if gravar != 0 {
                    crate::comparacao::ramo("cpt_13 posição pela rodada");
                    gravar_posicao(cpt13_d(this + 8), c, a as u32);
                }
            }
            b = b.wrapping_add(1);
            j += 1;
        }
        r -= 1;
    }
    chamadas
}

/// { 0x5223a0 } (thiscall, ret 0, Cup.cpp): `Cup::rank_teams()`: copia os clubes da copa ([+0xa7], registros de 6
/// bytes, +0x3e deles) para um vetor (0x944e46), ordena pela força (0x4c08f0) e dá a cada um a posição prevista pela
/// rodada em que sai (0x4a2540 na estatística [+8]); libera o vetor (0x9452ca). Sem estatística: a caixa (linha
/// 0x1827); sem memória: a original (a caixa da falta de memória e o fim do jogo).
unsafe extern "thiscall" fn posicoes_previstas_da_copa(this: usize) {
    if cpt13_d(this + 8) == 0 {
        return aviso_em(0x9b55ac, 0x1827);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, i32) = core::mem::transmute(0x4c08f0usize);
    let n = cpt13_w(this + 0x3e) as i32;
    let v = alocar((n << 2) as u32);
    if v == 0 {
        return orig::<unsafe extern "thiscall" fn(usize)>(1191)(this);
    }
    for i in 0..n.max(0) as usize {
        *((v + i * 4) as *mut u32) = cpt13_d(cpt13_d(this + 0xa7) + i * 6) as u32;
    }
    ordenar(v, n);
    cpt13_posicoes(this, v as *const u32, 1);
    liberar(v);
}

/// Comparador: grava e pode realocar a estatística (não roda duas vezes): antes da original, as posições são
/// planejadas sobre uma cópia ordenada do vetor (as chaves num Vec); depois, o registro de cada clube planejado tem
/// de ter em +0x72 a última posição dele.
unsafe extern "thiscall" fn comparar_posicoes_previstas_da_copa(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1191);
    let n = cpt13_w(this + 0x3e) as i32;
    if cpt13_d(this + 8) == 0 || n <= 0 {
        return o(this); // ponytail: a caixa ou nada a ordenar, só a original
    }
    let mut v: Vec<u32> = (0..n as usize).map(|i| cpt13_d(cpt13_d(this + 0xa7) + i * 6) as u32).collect();
    let mut k = vec![0i32; n as usize];
    cpt13_ordenar_com(v.as_mut_ptr(), k.as_mut_ptr(), n);
    // ponytail: com mais vagas nas rodadas que clubes, a original lê além do vetor (sobra do bloco); o plano lê 0
    v.extend(core::iter::repeat(0).take(0x100));
    let plano = cpt13_posicoes(this, v.as_ptr(), 0);
    o(this);
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut erros = Vec::new();
    for (i, &(c, p)) in plano.iter().enumerate() {
        if c == 0 || plano[i + 1..].iter().any(|x| x.0 == c) {
            continue;
        }
        let e = buscar(cpt13_d(this + 8), c);
        if e == 0 || *((e + 0x72) as *const u8) != p {
            erros.push(format!("clube {:x} esperado {} obtido {}", c, p, if e == 0 { -1 } else { *((e + 0x72) as *const u8) as i32 }));
        }
    }
    PLACAR.registrar(erros.is_empty(), || format!("posicoes_previstas_da_copa({:x}) {} posições: {}", this, plano.len(), erros.join("; ")));
}

/// Comparador: ordena no lugar (aloca e libera o temporário): o esperado sai do núcleo sobre uma cópia (chaves num
/// Vec) antes da original; depois, o vetor tem de ser igual.
unsafe extern "C" fn comparar_ordenar_clubes_pela_forca(v: usize, n: i32) {
    let o = orig::<unsafe extern "C" fn(usize, i32)>(1192);
    if v == 0 || n <= 1 {
        return o(v, n);
    }
    let mut esperado = core::slice::from_raw_parts(v as *const u32, n as usize).to_vec();
    let mut k = vec![0i32; n as usize];
    cpt13_ordenar_com(esperado.as_mut_ptr(), k.as_mut_ptr(), n);
    o(v, n);
    let obtido = core::slice::from_raw_parts(v as *const u32, n as usize);
    PLACAR.registrar(obtido == &esperado[..], || format!("ordenar_clubes_pela_forca(n {}) original={:x?} rust={:x?}", n, obtido, esperado));
}

/// Comparador: acha ou acrescenta o registro (não roda duas vezes): o registro visto antes (se havia) tem de ser o
/// devolvido, com a posição em +0x72.
unsafe extern "thiscall" fn comparar_gravar_posicao_do_clube(this: usize, clube: usize, pos: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1193);
    if clube == 0 {
        return o(this, clube, pos); // ponytail: a caixa, só a original
    }
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let antes = buscar(this, clube);
    let r = o(this, clube, pos);
    let igual = r != 0 && (antes == 0 || r == antes) && *((r + 0x72) as *const u8) == pos as u8;
    PLACAR.registrar(igual, || format!("gravar_posicao_do_clube(clube {:x}, {}) antes={:x} original={:x}", clube, pos as u8, antes, r));
    r
}
