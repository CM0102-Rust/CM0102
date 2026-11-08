use super::*;




/// counts the results of the national side's history between two months (year × 12 + month), from the end, for the work
/// permit
/// Original: `FIFA_RANKINGS::count_results_in_period` (0x58d710, fifa_rankings.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x58d710 } (thiscall, ret 0x14, fifa_rankings.cpp): `FIFA_RANKINGS::count_results_in_period(nação, mês_de, ano_de,
/// mês_até, ano_até)` (pela permissão de trabalho 0x8c7950, com os dois anos até hoje): na entrada da nação (id [nação],
/// 0x2c bytes na tabela [this]), o histórico (+4, entradas de 5 bytes: pontos, mês +2, ano +3; conta +0xa) do fim para o
/// começo; ano × 12 + mês antes do começo do período para a busca, até o fim conta um. Em 16 bits. TapeCall a cada
/// permissão que chega aos jogos da seleção: sem contador.
pub(crate) unsafe extern "thiscall" fn count_results_in_period(this: usize, nacao: usize, mes_de: u32, ano_de: u32, mes_ate: u32, ano_ate: u32) -> u16 {
    let de = (ano_de & 0xffff) as i32 * 12 + mes_de as u8 as i8 as i32;
    let ate = (ano_ate & 0xffff) as i32 * 12 + mes_ate as u8 as i8 as i32;
    let e = read_usize(this).wrapping_add((read_usize(nacao) as u32).wrapping_mul(0x2c) as usize);
    let hist = read_usize(e + 4);
    let mut j = read_i16(e + 0xa).wrapping_sub(1);
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

/// Comparer for `count_results_in_period` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_count_results_in_period(this: usize, nacao: usize, mes_de: u32, ano_de: u32, mes_ate: u32, ano_ate: u32) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u16>(IDX_COUNT_RESULTS_IN_PERIOD)(this, nacao, mes_de, ano_de, mes_ate, ano_ate);
    let n = count_results_in_period(this, nacao, mes_de, ano_de, mes_ate, ano_ate);
    PLACAR.registrar(r == n, || format!("count_results_in_period(nação {}, {}/{} a {}/{}) original={} rust={}", read_usize(nacao), mes_de as u8, ano_de as u16, mes_ate as u8, ano_ate as u16, r, n));
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
pub(crate) unsafe extern "thiscall" fn update_rankings(this: usize, data: usize) {
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
    while i < read_usize(G_NATION_COUNT) as i32 {
        let e = read_usize(this) + i as usize * 0x2c;
        let mut k = (*((e + 0xa) as *const u8)).wrapping_sub(1) as i8 as i32;
        while k >= 0 {
            let hist = read_usize(e + 4);
            let p = hist + k as usize * 5;
            let y = read_u16(p + 3) as i32 + 1;
            if y < ano || (y == ano && *((p + 2) as *const u8) < mes) {
                let ultimo = hist + read_i16(e + 0xa) as isize as usize * 5 - 5;
                core::ptr::copy(ultimo as *const u8, p as *mut u8, 5);
                *((e + 0xa) as *mut i16) = read_i16(e + 0xa).wrapping_sub(1);
            }
            k -= 1;
        }
        ordenar(read_usize(e + 4), read_i16(e + 0xa) as i32 as u32, 5, 0x58e220);
        i += 1;
    }
    if mes == *((this + 4) as *const u8) && read_u16(data + 2) != read_u16(G_GAME_START_DATE_YEAR) {
        let mut i = 0i32;
        while i < read_usize(G_NATION_COUNT) as i32 {
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

/// Comparer for `update_rankings` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo objeto (+0..+0xd), pela tabela das nações e por cada histórico (capacidade +8 × 5);
/// a ordem é alocada e liberada pelas duas.
pub(crate) unsafe extern "thiscall" fn comparar_update_rankings(this: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UPDATE_RANKINGS);
    let t = read_usize(this);
    let n = read_usize(G_NATION_COUNT);
    let mut reg = vec![(this, 0xd), (t, n * 0x2c)];
    for i in 0..n {
        let e = t + i * 0x2c;
        let h = read_usize(e + 4);
        let cap = read_i16(e + 8) as i32;
        if h != 0 && cap > 0 {
            reg.push((h, cap as usize * 5));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, data), || update_rankings(this, data));
    PLACAR.registrar(igual, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).flat_map(|(a, b)| (0..a.1.len()).filter(move |&i| a.1[i] != b.1[i]).map(move |i| a.0 + i)).take(16).collect();
        format!("update_rankings(dia {}) diferenças em {:x?}", read_u16(data), dif)
    });
}

/// qsort order of the histories: the points of b minus those of a (highest first)
/// Original: `compare_result_points_desc` (0x58e220, fifa_rankings.cpp, cdecl).
///
/// [pt] { 0x58e220 } (cdecl, fifa_rankings.cpp): `compare_result_points_desc(a, b)`, a ordem do qsort dos históricos na
/// atualização do ranking: os pontos (palavra +0, com sinal) de `b` menos os de `a`.
pub(crate) unsafe extern "C" fn compare_result_points_desc(a: usize, b: usize) -> i32 {
    read_i16(b) as i32 - read_i16(a) as i32
}

/// Comparer for `compare_result_points_desc` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_compare_result_points_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_RESULT_POINTS_DESC)(a, b);
    let n = compare_result_points_desc(a, b);
    PLACAR.registrar(r == n, || format!("compare_result_points_desc({}, {}) original={} rust={}", read_i16(a), read_i16(b), r, n));
    r
}




/// Nation's entry in the FIFA ranking table [this] (0x2c bytes each).
///
/// [pt] A entrada da nação `nacao` (id em [nacao]) na tabela do ranking [this] (0x2c bytes cada).
pub(crate) unsafe fn ranking_entry(this: usize, nacao: usize) -> usize {
    read_usize(this).wrapping_add((read_usize(nacao) as u32).wrapping_mul(0x2c) as usize)
}

/// Month and year of a match: Date from day of year (+0x2a) and year (+0x28), then its month.
///
/// [pt] O mês e o ano do jogo `partida`: a data (0x549ef0) do dia do ano (+0x2a) e do ano (+0x28), o mês dela (0x54cfb0).
pub(crate) unsafe fn match_month_year(partida: usize) -> (u8, u16) {
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
pub(crate) unsafe extern "thiscall" fn append_result(this: usize, nacao: usize, pontos: u32, partida: usize) {
    let e = ranking_entry(this, nacao);
    if read_u16(e + 0xa) == read_u16(e + 8) {
        *((e + 8) as *mut u16) = read_u16(e + 8).wrapping_add(5);
        let h = read_usize(e + 4);
        let n = read_i16(e + 8) as i32 * 5;
        let novo = if h != 0 {
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
            realocar(h, n as u32)
        } else {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
            alocar(n as u32)
        };
        *((e + 4) as *mut usize) = novo;
    }
    let c = read_i16(e + 0xa);
    *((e + 0xa) as *mut i16) = c.wrapping_add(1); // cpt2: mais um resultado
    let p = read_usize(e + 4).wrapping_add(c as isize as usize * 5);
    *(p as *mut u16) = pontos as u16;
    let (mes, ano) = match_month_year(partida);
    *((p + 2) as *mut u8) = mes; // cpt2: o mes do jogo
    core::ptr::write_unaligned((p + 3) as *mut u16, ano); // cpt2: o ano do jogo
    crate::comparacao::ramo("cpt_2 resultado");
}

// ponytail: a original pode realocar o histórico (não roda duas vezes): o plano antes (conta, capacidade, pontos, mês e
// ano), a original roda e a entrada nova é conferida; a cópia leva o ramo.
/// Comparer for `append_result` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_append_result(this: usize, nacao: usize, pontos: u32, partida: usize) {
    let e = ranking_entry(this, nacao);
    let (conta, cap) = (read_i16(e + 0xa), read_i16(e + 8));
    let (mes, ano) = match_month_year(partida);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_APPEND_RESULT)(this, nacao, pontos, partida);
    let p = read_usize(e + 4).wrapping_add(conta as isize as usize * 5);
    let cap_n = if conta == cap { cap.wrapping_add(5) } else { cap };
    let lido = (read_i16(e + 0xa), read_i16(e + 8), read_u16(p), *((p + 2) as *const u8), read_u16(p + 3));
    let plano = (conta.wrapping_add(1), cap_n, pontos as u16, mes, ano);
    PLACAR.registrar(lido == plano, || format!("append_result(nação {}, pontos {}) original={:?} plano={:?}", read_usize(nacao), pontos as u16, lido, plano));
}

/// the national side's strength tier from 1 to 4, by reputation and ranking position
/// Original: `FIFA_RANKINGS::get_nation_strength_tier` (0x58e190, fifa_rankings.cpp, thiscall, ret 4).
///
/// [pt] { 0x58e190 } (thiscall, ret 4, fifa_rankings.cpp): `FIFA_RANKINGS::get_nation_strength_tier(nação)` (duas vezes por
/// resultado internacional 0x58cf70): reputação (+0x8e) abaixo de 3000 e posição (+0x10 da entrada) abaixo de 50: 3;
/// reputação abaixo de 6000 e posição abaixo de 15: 2; senão pela posição: abaixo de 21, 1; de 61, 2; de 101, 3; 4.
pub(crate) unsafe extern "thiscall" fn get_nation_strength_tier(this: usize, nacao: usize) -> u8 {
    let rep = read_i16(nacao + nation::REPUTATION);
    let pos = || read_i16(ranking_entry(this, nacao) + 0x10);
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

/// Comparer for `get_nation_strength_tier` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_nation_strength_tier(this: usize, nacao: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_NATION_STRENGTH_TIER)(this, nacao);
    let n = get_nation_strength_tier(this, nacao);
    PLACAR.registrar(r == n, || format!("get_nation_strength_tier(nação {}) original={} rust={}", read_usize(nacao), r, n));
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
pub(crate) unsafe extern "thiscall" fn recalculate_scores(this: usize) {
    let datar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let fator = core::ptr::read_unaligned(0x96a350 as *const f64);
    let mut i = 0i32;
    if read_usize(G_NATION_COUNT) as i32 <= 0 {
        return;
    }
    loop {
        let e = read_usize(this) + i as usize * 0x2c;
        let conta = read_i16(e + 0xa);
        let ponto = |k: i8| read_i16(read_usize(e + 4).wrapping_add(k as isize as usize * 5)) as i32;
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
        let base = core::ptr::read_unaligned(G_GAME_START_DATE as *const [u8; 8]);
        let mut d = [0u8; 8];
        let r = datar(G_CURRENT_DATE, &mut d, 0x16d);
        let (by, ry) = (u16::from_le_bytes([base[2], base[3]]), read_u16(r + 2));
        let depois = by > ry || (by == ry && i16::from_le_bytes([base[0], base[1]]) > read_i16(r));
        if depois && read_i16(e + 0xa) < 9 {
            let c = read_i16(e + 0xa);
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
        if i >= read_usize(G_NATION_COUNT) as i32 {
            break;
        }
    }
}

/// Comparer for `recalculate_scores` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_recalculate_scores(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RECALCULATE_SCORES);
    let reg = [(read_usize(this), read_usize(G_NATION_COUNT) * 0x2c)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || recalculate_scores(this));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).collect();
        format!("recalculate_scores diferenças nos deslocamentos {:x?}", dif)
    });
}




/// a new array with the ranking entries of the nations with a national side, sorted by score, and the count
/// Original: `FIFA_RANKINGS::collect_ranked_order` (0x58d610, fifa_rankings.cpp, thiscall, ret 4).
///
/// [pt] { 0x58d610 } (thiscall, ret 4, fifa_rankings.cpp): `FIFA_RANKINGS::collect_ranked_order(&conta)` (pela atualização do
/// dia 18 0x58d460 e pela tela do ranking): um vetor (0x944e46) de [0xae2350] ponteiros; sem ele, a caixa "Error"
/// (linha 0x1e3) e 0, sem gravar a conta. As entradas da tabela (0x2c bytes) das nações com seleção (+0x71 da nação,
/// registros de 0x122 bytes em [0xae23a8]), ordenadas pela pontuação (qsort 0x945333 com 0x58e240); a conta em `conta`.
pub(crate) unsafe extern "thiscall" fn collect_ranked_order(this: usize, conta: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let v = alocar((read_usize(G_NATION_COUNT) as u32).wrapping_mul(4));
    if v == 0 {
        show_error_at(0x9c74f4, 0x1e3);
        return 0;
    }
    let nacoes = read_usize(G_NATION_TABLE);
    let mut n = 0usize;
    let mut i = 0usize;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        if read_usize(nacoes + i * nation::SIZE + 0x71) != 0 {
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
/// Comparer for `collect_ranked_order` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_collect_ranked_order(this: usize, conta: *mut i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *mut i32) -> usize>(IDX_COLLECT_RANKED_ORDER)(this, conta);
    if r == 0 {
        return r;
    }
    let mut c = 0i32;
    let v = collect_ranked_order(this, &mut c);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let igual = v != 0 && c == *conta && (0..c as usize).all(|k| read_usize(r + k * 4) == read_usize(v + k * 4));
    PLACAR.registrar(igual, || format!("collect_ranked_order conta original={} rust={}", *conta, c));
    liberar(v);
    r
}

/// qsort order of the ranking: the highest score first
/// Original: `compare_ranking_score_desc` (0x58e240, fifa_rankings.cpp, cdecl).
///
/// [pt] { 0x58e240 } (cdecl, fifa_rankings.cpp): `compare_ranking_score_desc(a, b)`, a ordem do qsort da 0x58d610: as
/// pontuações (f32 +0xc das entradas apontadas): a menor que b (ou sem ordem), 1; iguais, 0; maior, −1.
pub(crate) unsafe extern "C" fn compare_ranking_score_desc(a: usize, b: usize) -> i32 {
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

/// Comparer for `compare_ranking_score_desc` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_compare_ranking_score_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_RANKING_SCORE_DESC)(a, b);
    let n = compare_ranking_score_desc(a, b);
    PLACAR.registrar(r == n, || format!("compare_ranking_score_desc original={} rust={}", r, n));
    r
}

/// the initial FIFA ranking rule: each nation's points are its database strength ÷ 35, repeated as the points of each
/// of the 6 years [inference] (× 1000); the position comes from the points order
/// Original: `FIFA_RANKINGS::create_default_table` (0x58dd50, fifa_rankings.cpp, thiscall, ret 0).
///
/// [pt] { 0x58dd50 } (thiscall, ret 0, fifa_rankings.cpp, `FIFA_RANKINGS::create_default_table`): a tabela inicial do
/// ranking da FIFA (registros de 0x2c bytes, um por nação, calloc 0x945501 de [0xae2350] × 0x2c em +0; sem memória, a
/// caixa da linha 0x2da). Cada registro: +0 o índice da nação; a força da nação (+0xa8, double, nações de 0x122 bytes em
/// [0xae23a8]) × 1/35 dá os pontos (+0xc, float) e, × 1000 (truncado, _ftol), os 6 dwords +0x14..+0x28 [inferência: os
/// pontos de cada um dos últimos anos]. Depois a ordem pelos pontos (0x58d610(this, &conta), thiscall, a lista ordenada
/// das nações com continente) dá a posição: +0x12 = a posição de antes (+0x10) e +0x10 = 1, 2, 3...; a lista é solta.
pub(crate) unsafe extern "thiscall" fn create_default_table(this: usize) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let ordem: unsafe extern "thiscall" fn(usize, *mut i32) -> usize = core::mem::transmute(FN_FIFA_RANKINGS_COLLECT_RANKED_ORDER);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let t = calloc(read_u32(G_NATION_COUNT), 0x2c);
    *(this as *mut usize) = t;
    if t == 0 {
        show_error_at(0x9c74f4, 0x2da);
        return;
    }
    let c1 = core::ptr::read_unaligned(0x96a348 as *const f64);
    let c2 = core::ptr::read_unaligned(0x968908 as *const f64);
    let mut k = 0i32;
    while k < read_u32(G_NATION_COUNT) as i32 {
        let r = t + 0x2c * k as usize;
        *(r as *mut i32) = k;
        let a = core::ptr::read_unaligned((read_u32(G_NATION_TABLE) as usize + 0x122 * k as usize + 0xa8) as *const f64) * c1;
        let p = (a * c2) as i32;
        for j in 0..6 {
            *((r + 0x14 + 4 * j) as *mut i32) = p;
        }
        *((r + 0xc) as *mut f32) = a as f32;
        k += 1;
    }
    let mut n = 0i32;
    let l = ordem(this, &mut n);
    for i in 0..n.max(0) as usize {
        let r = read_u32(l + 4 * i) as usize;
        *((r + 0x12) as *mut u16) = *((r + 0x10) as *const u16);
        *((r + 0x10) as *mut u16) = (i + 1) as u16;
    }
    soltar(l);
}

/// Comparer for `create_default_table` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado à mão: a original no objeto de quem chama (ela aloca a tabela em +0), a cópia num objeto de rascunho
/// (aloca a dela); iguais as duas tabelas inteiras (a do rascunho é solta).
pub(crate) unsafe extern "thiscall" fn comparar_create_default_table(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_DEFAULT_TABLE);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    o(this);
    let mut s = [0usize; 1];
    create_default_table(s.as_mut_ptr() as usize);
    let (ta, tb) = (read_u32(this) as usize, s[0]);
    let n = (read_u32(G_NATION_COUNT) as i32).max(0) as usize * 0x2c;
    let igual = if ta != 0 && tb != 0 {
        core::slice::from_raw_parts(ta as *const u8, n) == core::slice::from_raw_parts(tb as *const u8, n)
    } else {
        ta == tb
    };
    let dif = if igual || ta == 0 || tb == 0 {
        None
    } else {
        (0..n).find(|&i| *((ta + i) as *const u8) != *((tb + i) as *const u8))
    };
    if tb != 0 {
        soltar(tb);
    }
    PLACAR.registrar(igual, || format!("create_default_table: a primeira diferença na nação {:?}", dif.map(|i| (i / 0x2c, i % 0x2c))));
}
