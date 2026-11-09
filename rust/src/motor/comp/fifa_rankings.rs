use super::*;

/// Counts the results in the national side's history between two months (year × 12 + month), from the end, for the work
/// permit.
///
/// Ret 0x14: `FIFA_RANKINGS::count_results_in_period(nation, month_from, year_from, month_to, year_to)` (used by the
/// work permit `is_work_permit_granted`, with the two years up to today): in the nation's entry (id `[nation]`, 0x2c bytes in table
/// `[this]`), the history (+4, 5-byte entries: points, month +2, year +3; count +0xa) from the end to the start; a year
/// × 12 + month before the start of the period stops the search, up to the end of the period each one counts. In 16
/// bits. TapeCall on every permit that reaches the national side matches: no counter.
/// Original: `FIFA_RANKINGS::count_results_in_period` (0x58d710, fifa_rankings.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x58d710 } (thiscall, ret 0x14, fifa_rankings.cpp): `FIFA_RANKINGS::count_results_in_period(nação, mês_de, ano_de,
/// mês_até, ano_até)` (pela permissão de trabalho 0x8c7950, com os dois anos até hoje): na entrada da nação (id [nação],
/// 0x2c bytes na tabela [this]), o histórico (+4, entradas de 5 bytes: pontos, mês +2, ano +3; conta +0xa) do fim para o
/// começo; ano × 12 + mês antes do começo do período para a busca, até o fim conta um. Em 16 bits. TapeCall a cada
/// permissão que chega aos jogos da seleção: sem contador.
pub(crate) unsafe extern "thiscall" fn count_results_in_period(this: usize, nation: usize, month_of: u32, year_of: u32, month_until: u32, year_until: u32) -> u16 {
    let de = (year_of & 0xffff) as i32 * 12 + month_of as u8 as i8 as i32;
    let ate = (year_until & 0xffff) as i32 * 12 + month_until as u8 as i8 as i32;
    let e = read_usize(this).wrapping_add((read_usize(nation) as u32).wrapping_mul(0x2c) as usize);
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
            n = n.wrapping_add(1); // cpt1: one more match
        }
        j = j.wrapping_sub(1);
    }
    n // cpt1: the matches in the period
}

/// Updates the FIFA rankings on the 18th: drops the results older than a year, sorts the histories, rolls the yearly
/// totals and redoes scores and positions.
///
/// Ret 4: `FIFA_RANKINGS::update_rankings(date)` (from the national sides' day `national_teams_daily_update`): only on the 18th (`exe::date_get_day_of_month`).
/// Stores the date in +5 (`exe::date_operator_3d`) and, in each nation (`[0xae2350]` entries), removes from the history the results
/// older than a year (year + 1 before today's, or the same year with the month before today's, `exe::date_get_month`), putting the
/// last one in its place; the loop goes from the low byte of the count − 1 down to 0, as in the original. Then sorts
/// the history by points (qsort `exe::crt_qsort` with `compare_result_points_desc`). In the month stored in +4, with the year different from that
/// of `[0xdd6ec8]`, the yearly totals move one slot (+0x14 → +0x28) and +0x14 is zeroed. Finally the scores (`recalculate_scores`)
/// and the positions: the order (`collect_ranked_order`) gives position + 1 in +0x10, with the previous one in +0x12; the order
/// vector is freed (`exe::crt_free`).
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
    if (exe::date_get_day_of_month(data) as u16) != 0x12 {
        return;
    }
    let month = exe::date_get_month(data);
    let mut stored = [0u8; 8];
    exe::date_operator_3d(this + 5, stored.as_mut_ptr() as usize, data);
    let year = read_u16(data + 2) as i32;
    let mut i = 0i32;
    while i < read_usize(G_NATION_COUNT) as i32 {
        let e = read_usize(this) + i as usize * 0x2c;
        let mut k = (*((e + 0xa) as *const u8)).wrapping_sub(1) as i8 as i32;
        while k >= 0 {
            let hist = read_usize(e + 4);
            let p = hist + k as usize * 5;
            let y = read_u16(p + 3) as i32 + 1;
            if y < year || (y == year && *((p + 2) as *const u8) < month) {
                let last = hist + read_i16(e + 0xa) as isize as usize * 5 - 5;
                core::ptr::copy(last as *const u8, p as *mut u8, 5);
                *((e + 0xa) as *mut i16) = read_i16(e + 0xa).wrapping_sub(1);
            }
            k -= 1;
        }
        exe::crt_qsort(read_usize(e + 4), read_i16(e + 0xa) as i32 as u32, 5, 0x58e220);
        i += 1;
    }
    if month == *((this + 4) as *const u8) && read_u16(data + 2) != read_u16(G_GAME_START_DATE_YEAR) {
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
    exe::fifa_rankings_recalculate_scores(this);
    // the original keeps the count in the argument’s place (the date): without the vector, the count stays as the pointer
    let mut n = data as i32;
    let v = exe::fifa_rankings_collect_ranked_order(this, &mut n);
    let mut k = 0i32;
    while k < n {
        let r = read_usize(v + k as usize * 4);
        *((r + 0x12) as *mut u16) = read_u16(r + 0x10);
        *((r + 0x10) as *mut u16) = (k + 1) as u16;
        k += 1;
    }
    exe::crt_free(v);
}

/// qsort order of the histories: the points of b minus those of a (highest first)
/// Original: `compare_result_points_desc` (0x58e220, fifa_rankings.cpp, cdecl).
///
/// [pt] { 0x58e220 } (cdecl, fifa_rankings.cpp): `compare_result_points_desc(a, b)`, a ordem do qsort dos históricos na
/// atualização do ranking: os pontos (palavra +0, com sinal) de `b` menos os de `a`.
pub(crate) unsafe extern "C" fn compare_result_points_desc(a: usize, b: usize) -> i32 {
    read_i16(b) as i32 - read_i16(a) as i32
}

/// Nation's entry in the FIFA ranking table [this] (0x2c bytes each).
///
/// [pt] A entrada da nação `nacao` (id em [nacao]) na tabela do ranking [this] (0x2c bytes cada).
pub(crate) unsafe fn ranking_entry(this: usize, nation: usize) -> usize {
    read_usize(this).wrapping_add((read_usize(nation) as u32).wrapping_mul(0x2c) as usize)
}

/// Month and year of a match: Date from day of year (+0x2a) and year (+0x28), then its month.
///
/// [pt] O mês e o ano do jogo `partida`: a data (0x549ef0) do dia do ano (+0x2a) e do ano (+0x28), o mês dela (0x54cfb0).
pub(crate) unsafe fn match_month_year(partida: Fixture) -> (u8, u16) {
    let mut d = [0u8; 8];
    let p = exe::new_date((d.as_mut_ptr()) as *mut u8 as usize, partida.day() as u32, partida.year() as u32);
    (exe::date_get_month(p), partida.year())
}

/// Appends the points, month and year of the match to the national side's history, growing the array by 5 entries when
/// full.
///
/// Ret 0xc; SEH only for the local date, with an empty destructor: `FIFA_RANKINGS::append_result(nation, points,
/// fixture)` (from the international result `exe::fifa_rankings_record_match_result`, for each national side): with the count (+0xa) equal to the
/// capacity (+8), the capacity grows by 5 and the history (+4) is reallocated (`exe::crt_realloc`) or allocated (`exe::crt_malloc`) with
/// capacity × 5 bytes. The new entry, in the place of the old count: the points (16 bits), the match month (+2) and the
/// year (+3).
/// Original: `FIFA_RANKINGS::append_result` (0x58dea0, fifa_rankings.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x58dea0 } (thiscall, ret 0xc, fifa_rankings.cpp, SEH só da data local, de destrutor vazio):
/// `FIFA_RANKINGS::append_result(nação, pontos, partida)` (pelo resultado internacional 0x58cf70, para cada seleção):
/// com a conta (+0xa) igual à capacidade (+8), a capacidade cresce 5 e o histórico (+4) é realocado (0x945667) ou
/// alocado (0x944e46) com capacidade × 5 bytes. A entrada nova, no lugar da conta antiga: os pontos (16 bits), o mês do
/// jogo (+2) e o ano (+3).
pub(crate) unsafe extern "thiscall" fn append_result(this: usize, nation: usize, points: u32, partida: usize) {
    let e = ranking_entry(this, nation);
    if read_u16(e + 0xa) == read_u16(e + 8) {
        *((e + 8) as *mut u16) = read_u16(e + 8).wrapping_add(5);
        let h = read_usize(e + 4);
        let n = read_i16(e + 8) as i32 * 5;
        let new = if h != 0 {
            exe::crt_realloc(h, n as u32)
        } else {
            exe::crt_malloc(n as u32)
        };
        *((e + 4) as *mut usize) = new;
    }
    let c = read_i16(e + 0xa);
    *((e + 0xa) as *mut i16) = c.wrapping_add(1); // cpt2: mais um resultado
    let p = read_usize(e + 4).wrapping_add(c as isize as usize * 5);
    *(p as *mut u16) = points as u16;
    let (mes, ano) = match_month_year(Fixture(partida));
    *((p + 2) as *mut u8) = mes; // cpt2: the match month
    core::ptr::write_unaligned((p + 3) as *mut u16, ano); // cpt2: the match year
    crate::comparison::branch("cpt_2 result");
}

// simplification: the original may reallocate the history (does not run twice): the plan beforehand (count, capacity, points, month and
// year), the original runs and the new entry is checked; the copy takes the branch.

/// Returns the national side's strength tier from 1 to 4, by reputation and ranking position.
///
/// Ret 4: `FIFA_RANKINGS::get_nation_strength_tier(nation)` (twice per international result `exe::fifa_rankings_record_match_result`): reputation
/// (+0x8e) below 3000 and position (+0x10 of the entry) below 50: 3; reputation below 6000 and position below 15: 2;
/// otherwise by position: below 21, 1; below 61, 2; below 101, 3; else 4.
/// Original: `FIFA_RANKINGS::get_nation_strength_tier` (0x58e190, fifa_rankings.cpp, thiscall, ret 4).
///
/// [pt] { 0x58e190 } (thiscall, ret 4, fifa_rankings.cpp): `FIFA_RANKINGS::get_nation_strength_tier(nação)` (duas vezes por
/// resultado internacional 0x58cf70): reputação (+0x8e) abaixo de 3000 e posição (+0x10 da entrada) abaixo de 50: 3;
/// reputação abaixo de 6000 e posição abaixo de 15: 2; senão pela posição: abaixo de 21, 1; de 61, 2; de 101, 3; 4.
pub(crate) unsafe extern "thiscall" fn get_nation_strength_tier(this: usize, nation: usize) -> u8 {
    let nation = Nation(nation);
    let rep = nation.reputation();
    let pos = || read_i16(ranking_entry(this, nation.addr()) + 0x10);
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

/// Recalculates the scores: the year total from the best eight results and the score from the totals of this year and
/// the five before.
///
/// `FIFA_RANKINGS::recalculate_scores()` (from the update of the 18th `update_rankings`): in each nation, the sum of the points
/// of the first eight results (the history comes sorted) and that of all; with more than eight, the year total (+0x14)
/// is (all × 8 / count + eight) / 2, otherwise the eight. If the date of `[0xdd6ec8]` is after today minus 365 days
/// (0x54c8e0), with fewer than nine results the total is (+0x18 + eight) / (count + 8) × 8 (with no results, +0x18).
/// The score (+0xc, f32): (+0x14 + 5/6 of +0x18 + 4/6 of +0x1c + 3/6 of +0x20 + 2/6 of +0x24 + 1/6 of +0x28, each part
/// truncated) × `[0x96a350]`.
/// Original: `FIFA_RANKINGS::recalculate_scores` (0x58dfc0, fifa_rankings.cpp, thiscall).
///
/// [pt] { 0x58dfc0 } (thiscall, fifa_rankings.cpp): `FIFA_RANKINGS::recalculate_scores()` (pela atualização do dia 18
/// 0x58d460): em cada nação, a soma dos pontos dos oito primeiros resultados (o histórico vem ordenado) e a de todos;
/// com mais de oito, o total do ano (+0x14) é (todos × 8 / conta + oito) / 2, senão os oito. Se a data de [0xdd6ec8]
/// passa da de hoje menos 365 dias (0x54c8e0), com menos de nove resultados o total é (+0x18 + oito) / (conta + 8) × 8
/// (sem resultados, o +0x18). A pontuação (+0xc, f32): (+0x14 + 5/6 de +0x18 + 4/6 de +0x1c + 3/6 de +0x20 + 2/6 de
/// +0x24 + 1/6 de +0x28, cada parte truncada) × [0x96a350].
pub(crate) unsafe extern "thiscall" fn recalculate_scores(this: usize) {
    let factor = core::ptr::read_unaligned(0x96a350 as *const f64);
    let mut i = 0i32;
    if read_usize(G_NATION_COUNT) as i32 <= 0 {
        return;
    }
    loop {
        let e = read_usize(this) + i as usize * 0x2c;
        let count = read_i16(e + 0xa);
        let point = |k: i8| read_i16(read_usize(e + 4).wrapping_add(k as isize as usize * 5)) as i32;
        let mut oito = 0i32;
        let mut k = 0i8;
        if count > 0 {
            loop {
                if k >= 8 {
                    break;
                }
                oito = oito.wrapping_add(point(k));
                k = k.wrapping_add(1);
                if k as i16 >= count {
                    break;
                }
            }
        }
        let mut all = oito;
        while (k as i16) < count {
            all = all.wrapping_add(point(k));
            k = k.wrapping_add(1);
        }
        let year = if count > 8 { (all.wrapping_mul(8) / count as i32 + oito) / 2 } else { oito };
        *((e + 0x14) as *mut i32) = year;
        let base = core::ptr::read_unaligned(G_GAME_START_DATE as *const [u8; 8]);
        let mut d = [0u8; 8];
        let r = exe::date_sub_days(G_CURRENT_DATE, (&mut d) as *mut [u8; 8] as usize, 0x16d);
        let (by, ry) = (u16::from_le_bytes([base[2], base[3]]), read_u16(r + 2));
        let after = by > ry || (by == ry && i16::from_le_bytes([base[0], base[1]]) > read_i16(r));
        if after && read_i16(e + 0xa) < 9 {
            let c = read_i16(e + 0xa);
            let v18 = *((e + 0x18) as *const i32);
            *((e + 0x14) as *mut i32) = if c != 0 { (v18.wrapping_add(oito) / (c as i32 + 8)).wrapping_shl(3) } else { v18 };
        }
        let t = |o: usize| *((e + o) as *const i32);
        let sum = (t(0x24).wrapping_mul(2) / 6)
            .wrapping_add(t(0x20).wrapping_mul(3) / 6)
            .wrapping_add(t(0x1c).wrapping_mul(4) / 6)
            .wrapping_add(t(0x18).wrapping_mul(5) / 6)
            .wrapping_add(t(0x28) / 6)
            .wrapping_add(t(0x14));
        *((e + 0xc) as *mut f32) = (sum as f64 * factor) as f32;
        i += 1;
        if i >= read_usize(G_NATION_COUNT) as i32 {
            break;
        }
    }
}

/// Returns a new array with the ranking entries of the nations that have a national side, sorted by score, and the
/// count.
///
/// Ret 4: `FIFA_RANKINGS::collect_ranked_order(&count)` (from the update of the 18th `update_rankings` and from the ranking
/// screen): a vector (`exe::crt_malloc`) of `[0xae2350]` pointers; without it, the "Error" box (line 0x1e3) and 0, without
/// storing the count. The table entries (0x2c bytes) of the nations with a national side (+0x71 of the nation,
/// 0x122-byte records in `[0xae23a8]`), sorted by score (qsort `exe::crt_qsort` with `compare_ranking_score_desc`); the count goes in `count`.
/// Original: `FIFA_RANKINGS::collect_ranked_order` (0x58d610, fifa_rankings.cpp, thiscall, ret 4).
///
/// [pt] { 0x58d610 } (thiscall, ret 4, fifa_rankings.cpp): `FIFA_RANKINGS::collect_ranked_order(&conta)` (pela atualização do
/// dia 18 0x58d460 e pela tela do ranking): um vetor (0x944e46) de [0xae2350] ponteiros; sem ele, a caixa "Error"
/// (linha 0x1e3) e 0, sem gravar a conta. As entradas da tabela (0x2c bytes) das nações com seleção (+0x71 da nação,
/// registros de 0x122 bytes em [0xae23a8]), ordenadas pela pontuação (qsort 0x945333 com 0x58e240); a conta em `conta`.
pub(crate) unsafe extern "thiscall" fn collect_ranked_order(this: usize, count: *mut i32) -> usize {
    let v = exe::crt_malloc((read_usize(G_NATION_COUNT) as u32).wrapping_mul(4));
    if v == 0 {
        show_error_at(0x9c74f4, 0x1e3);
        return 0;
    }
    let nations = read_usize(G_NATION_TABLE);
    let mut n = 0usize;
    let mut i = 0usize;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        if read_usize(nations + i * nation::SIZE + 0x71) != 0 {
            *((v + n * 4) as *mut usize) = read_usize(this) + i * 0x2c;
            n += 1;
        }
        i += 1;
    }
    exe::crt_qsort(v, n as u32, 4, 0x58e240);
    *count = n as i32;
    v
}

// simplification: both allocate the vector; the original runs first (its vector goes back to the caller), the copy after, and the
// the copy’s vector is freed (0x9452ca) after checking the count and the pointers.

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

/// Creates the initial FIFA ranking table. Rule: each nation's points are its database strength ÷ 35, repeated as the
/// points of each of the 6 years [inference] (× 1000); the position comes from the points order.
///
/// Ret 0, `FIFA_RANKINGS::create_default_table`: the initial FIFA ranking table (0x2c-byte records, one per nation,
/// calloc `exe::crt_calloc` of `[0xae2350]` × 0x2c in +0; out of memory: the box at line 0x2da). Each record: +0 the nation
/// index; the nation's strength (+0xa8, double, 0x122-byte nations in `[0xae23a8]`) × 1/35 gives the points (+0xc,
/// float) and, × 1000 (truncated, _ftol), the 6 dwords +0x14..+0x28 [inference: the points of each of the last years].
/// Then the order by points (`collect_ranked_order`(this, &count), thiscall, the sorted list of the nations with a continent) gives
/// the position: +0x12 = the previous position (+0x10) and +0x10 = 1, 2, 3...; the list is freed.
/// Original: `FIFA_RANKINGS::create_default_table` (0x58dd50, fifa_rankings.cpp, thiscall, ret 0).
///
/// [pt] { 0x58dd50 } (thiscall, ret 0, fifa_rankings.cpp, `FIFA_RANKINGS::create_default_table`): a tabela inicial do
/// ranking da FIFA (registros de 0x2c bytes, um por nação, calloc 0x945501 de [0xae2350] × 0x2c em +0; sem memória, a
/// caixa da linha 0x2da). Cada registro: +0 o índice da nação; a força da nação (+0xa8, double, nações de 0x122 bytes em
/// [0xae23a8]) × 1/35 dá os pontos (+0xc, float) e, × 1000 (truncado, _ftol), os 6 dwords +0x14..+0x28 [inferência: os
/// pontos de cada um dos últimos anos]. Depois a ordem pelos pontos (0x58d610(this, &conta), thiscall, a lista ordenada
/// das nações com continente) dá a posição: +0x12 = a posição de antes (+0x10) e +0x10 = 1, 2, 3...; a lista é solta.
pub(crate) unsafe extern "thiscall" fn create_default_table(this: usize) {
    let t = exe::crt_calloc(read_u32(G_NATION_COUNT), 0x2c);
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
    let l = exe::fifa_rankings_collect_ranked_order(this, &mut n);
    for i in 0..n.max(0) as usize {
        let r = read_u32(l + 4 * i) as usize;
        *((r + 0x12) as *mut u16) = *((r + 0x10) as *const u16);
        *((r + 0x10) as *mut u16) = (i + 1) as u16;
    }
    exe::crt_free(l);
}

