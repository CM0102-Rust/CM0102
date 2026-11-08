use super::*;

// The comparers of the routines in ../comp/fifa_rankings.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/fifa_rankings.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `count_results_in_period` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_count_results_in_period(this: usize, nacao: usize, mes_de: u32, ano_de: u32, mes_ate: u32, ano_ate: u32) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u16>(IDX_COUNT_RESULTS_IN_PERIOD)(this, nacao, mes_de, ano_de, mes_ate, ano_ate);
    let n = count_results_in_period(this, nacao, mes_de, ano_de, mes_ate, ano_ate);
    PLACAR.registrar(r == n, || format!("count_results_in_period(nação {}, {}/{} a {}/{}) original={} rust={}", read_usize(nacao), mes_de as u8, ano_de as u16, mes_ate as u8, ano_ate as u16, r, n));
    r
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

/// Comparer for `compare_result_points_desc` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_compare_result_points_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_RESULT_POINTS_DESC)(a, b);
    let n = compare_result_points_desc(a, b);
    PLACAR.registrar(r == n, || format!("compare_result_points_desc({}, {}) original={} rust={}", read_i16(a), read_i16(b), r, n));
    r
}

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

/// Comparer for `get_nation_strength_tier` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_nation_strength_tier(this: usize, nacao: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_NATION_STRENGTH_TIER)(this, nacao);
    let n = get_nation_strength_tier(this, nacao);
    PLACAR.registrar(r == n, || format!("get_nation_strength_tier(nação {}) original={} rust={}", read_usize(nacao), r, n));
    r
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

/// Comparer for `compare_ranking_score_desc` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_compare_ranking_score_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_RANKING_SCORE_DESC)(a, b);
    let n = compare_ranking_score_desc(a, b);
    PLACAR.registrar(r == n, || format!("compare_ranking_score_desc original={} rust={}", r, n));
    r
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
