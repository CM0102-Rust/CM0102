use super::*;

// The comparers of the routines in ../comp/fifa_rankings.rs.
// [pt] Os comparadores das rotinas de ../comp/fifa_rankings.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `count_results_in_period`.
pub(crate) unsafe extern "thiscall" fn compare_count_results_in_period(this: usize, nation: usize, month_of: u32, year_of: u32, month_until: u32, year_until: u32) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u16>(IDX_COUNT_RESULTS_IN_PERIOD)(this, nation, month_of, year_of, month_until, year_until);
    let n = count_results_in_period(this, nation, month_of, year_of, month_until, year_until);
    SCOREBOARD.record(r == n, || format!("count_results_in_period(nação {}, {}/{} a {}/{}) original={} rust={}", read_usize(nation), month_of as u8, year_of as u16, month_until as u8, year_until as u16, r, n));
    r
}

/// Comparer for `update_rankings`.
///
/// [pt] Comparador: lado a lado pelo objeto (+0..+0xd), pela tabela das nações e por cada histórico (capacidade +8 × 5);
/// a ordem é alocada e liberada pelas duas.
pub(crate) unsafe extern "thiscall" fn compare_update_rankings(this: usize, data: usize) {
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
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, data), || update_rankings(this, data));
    SCOREBOARD.record(igual, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).flat_map(|(a, b)| (0..a.1.len()).filter(move |&i| a.1[i] != b.1[i]).map(move |i| a.0 + i)).take(16).collect();
        format!("update_rankings(dia {}) diferenças em {:x?}", read_u16(data), dif)
    });
}

/// Comparer for `compare_result_points_desc`.
pub(crate) unsafe extern "C" fn compare_compare_result_points_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_RESULT_POINTS_DESC)(a, b);
    let n = compare_result_points_desc(a, b);
    SCOREBOARD.record(r == n, || format!("compare_result_points_desc({}, {}) original={} rust={}", read_i16(a), read_i16(b), r, n));
    r
}

/// Comparer for `append_result`.
pub(crate) unsafe extern "thiscall" fn compare_append_result(this: usize, nation: usize, points: u32, partida: usize) {
    let e = ranking_entry(this, nation);
    let (conta, cap) = (read_i16(e + 0xa), read_i16(e + 8));
    let (mes, ano) = match_month_year(Fixture(partida));
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_APPEND_RESULT)(this, nation, points, partida);
    let p = read_usize(e + 4).wrapping_add(conta as isize as usize * 5);
    let cap_n = if conta == cap { cap.wrapping_add(5) } else { cap };
    let lido = (read_i16(e + 0xa), read_i16(e + 8), read_u16(p), *((p + 2) as *const u8), read_u16(p + 3));
    let plan = (conta.wrapping_add(1), cap_n, points as u16, mes, ano);
    SCOREBOARD.record(lido == plan, || format!("append_result(nação {}, pontos {}) original={:?} plano={:?}", read_usize(nation), points as u16, lido, plan));
}

/// Comparer for `get_nation_strength_tier`.
pub(crate) unsafe extern "thiscall" fn compare_get_nation_strength_tier(this: usize, nation: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_NATION_STRENGTH_TIER)(this, nation);
    let n = get_nation_strength_tier(this, nation);
    SCOREBOARD.record(r == n, || format!("get_nation_strength_tier(nação {}) original={} rust={}", read_usize(nation), r, n));
    r
}

/// Comparer for `recalculate_scores`.
pub(crate) unsafe extern "thiscall" fn compare_recalculate_scores(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RECALCULATE_SCORES);
    let reg = [(read_usize(this), read_usize(G_NATION_COUNT) * 0x2c)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || recalculate_scores(this));
    SCOREBOARD.record(igual, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).collect();
        format!("recalculate_scores diferenças nos deslocamentos {:x?}", dif)
    });
}

/// Comparer for `collect_ranked_order`.
pub(crate) unsafe extern "thiscall" fn compare_collect_ranked_order(this: usize, count: *mut i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *mut i32) -> usize>(IDX_COLLECT_RANKED_ORDER)(this, count);
    if r == 0 {
        return r;
    }
    let mut c = 0i32;
    let v = collect_ranked_order(this, &mut c);
    let same = v != 0 && c == *count && (0..c as usize).all(|k| read_usize(r + k * 4) == read_usize(v + k * 4));
    SCOREBOARD.record(same, || format!("collect_ranked_order conta original={} rust={}", *count, c));
    exe::crt_free(v);
    r
}

/// Comparer for `compare_ranking_score_desc`.
pub(crate) unsafe extern "C" fn compare_compare_ranking_score_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_RANKING_SCORE_DESC)(a, b);
    let n = compare_ranking_score_desc(a, b);
    SCOREBOARD.record(r == n, || format!("compare_ranking_score_desc original={} rust={}", r, n));
    r
}

/// Comparer for `create_default_table`.
///
/// [pt] Lado a lado à mão: a original no objeto de quem chama (ela aloca a tabela em +0), a cópia num objeto de rascunho
/// (aloca a dela); iguais as duas tabelas inteiras (a do rascunho é solta).
pub(crate) unsafe extern "thiscall" fn compare_create_default_table(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_DEFAULT_TABLE);
    o(this);
    let mut s = [0usize; 1];
    create_default_table(s.as_mut_ptr() as usize);
    let (ta, tb) = (read_u32(this) as usize, s[0]);
    let n = (read_u32(G_NATION_COUNT) as i32).max(0) as usize * 0x2c;
    let same = if ta != 0 && tb != 0 {
        core::slice::from_raw_parts(ta as *const u8, n) == core::slice::from_raw_parts(tb as *const u8, n)
    } else {
        ta == tb
    };
    let dif = if same || ta == 0 || tb == 0 {
        None
    } else {
        (0..n).find(|&i| *((ta + i) as *const u8) != *((tb + i) as *const u8))
    };
    if tb != 0 {
        exe::crt_free(tb);
    }
    SCOREBOARD.record(same, || format!("create_default_table: a primeira diferença na nação {:?}", dif.map(|i| (i / 0x2c, i % 0x2c))));
}
