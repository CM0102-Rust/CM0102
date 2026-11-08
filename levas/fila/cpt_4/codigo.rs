// ---------- %%LEVA%%a leva: a nação-chave da competição e a ordem do ranking da FIFA: o índice da nação-chave da competição, a ordem das seleções no ranking e a comparação das pontuações (key_nation.cpp, fifa_rankings.cpp) ----------

unsafe fn cpt4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x669470 } (cdecl, key_nation.cpp): `key_nation_index_of_comp(competição)` (pelo alcance das regras da partida
/// 0x8d2fa0, pela liga do dia 0x6847c0, pelos registros da copa 0x519890 e pela nação-chave do jogo 0x48dd70 da cpt_3):
/// competição de nação (+0x5d): a linha dela na tabela das 35 nações-chave (0xb63d60, 72 bytes), ou 4. Sem nação: Copa
/// do Mundo e eliminatórias, 0; torneio de seleções (0x669ae0), 1; Copa do Mundo de Clubes e Mundial de Clubes da FIFA,
/// 2; senão a linha de continente (nação 0) com o continente da competição (+0x59), ou 3. A original lê +0x5d antes de
/// conferir a competição nula (o caso de erro nunca chega à caixa). TapeCall a cada partida: sem contador.
unsafe extern "C" fn key_nation_index_of_comp(comp: usize) -> u8 {
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

unsafe extern "C" fn comparar_key_nation_index_of_comp(comp: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I0%%)(comp);
    let n = key_nation_index_of_comp(comp);
    PLACAR.registrar(r == n, || format!("key_nation_index_of_comp(competição {}) original={} rust={}", cpt4_d(comp) as i32, r, n));
    r
}

/// { 0x58d610 } (thiscall, ret 4, fifa_rankings.cpp): `FIFA_RANKINGS::collect_ranked_order(&conta)` (pela atualização do
/// dia 18 0x58d460 e pela tela do ranking): um vetor (0x944e46) de [0xae2350] ponteiros; sem ele, a caixa "Error"
/// (linha 0x1e3) e 0, sem gravar a conta. As entradas da tabela (0x2c bytes) das nações com seleção (+0x71 da nação,
/// registros de 0x122 bytes em [0xae23a8]), ordenadas pela pontuação (qsort 0x945333 com 0x58e240); a conta em `conta`.
unsafe extern "thiscall" fn collect_ranked_order(this: usize, conta: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let v = alocar((cpt4_d(0xae2350) as u32).wrapping_mul(4));
    if v == 0 {
        show_error_at(0x9c74f4, 0x1e3);
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
unsafe extern "thiscall" fn comparar_collect_ranked_order(this: usize, conta: *mut i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *mut i32) -> usize>(%%I1%%)(this, conta);
    if r == 0 {
        return r;
    }
    let mut c = 0i32;
    let v = collect_ranked_order(this, &mut c);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let igual = v != 0 && c == *conta && (0..c as usize).all(|k| cpt4_d(r + k * 4) == cpt4_d(v + k * 4));
    PLACAR.registrar(igual, || format!("collect_ranked_order conta original={} rust={}", *conta, c));
    liberar(v);
    r
}

/// { 0x58e240 } (cdecl, fifa_rankings.cpp): `compare_ranking_score_desc(a, b)`, a ordem do qsort da 0x58d610: as
/// pontuações (f32 +0xc das entradas apontadas): a menor que b (ou sem ordem), 1; iguais, 0; maior, −1.
unsafe extern "C" fn compare_ranking_score_desc(a: usize, b: usize) -> i32 {
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

unsafe extern "C" fn comparar_compare_ranking_score_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = compare_ranking_score_desc(a, b);
    PLACAR.registrar(r == n, || format!("compare_ranking_score_desc original={} rust={}", r, n));
    r
}
