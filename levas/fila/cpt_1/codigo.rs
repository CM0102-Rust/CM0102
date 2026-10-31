// ---------- %%LEVA%%a leva: o ranking da FIFA no mês: os jogos de uma seleção no período, a atualização do dia 18 e a ordem dos resultados por pontos (fifa_rankings.cpp) ----------

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
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u16>(%%I0%%)(this, nacao, mes_de, ano_de, mes_ate, ano_ate);
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
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
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
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = ordem_dos_resultados_por_pontos(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_resultados_por_pontos({}, {}) original={} rust={}", cpt1_h(a), cpt1_h(b), r, n));
    r
}
