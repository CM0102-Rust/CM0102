use super::*;

// The comparers of the routines in ../comp/comp_util.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/comp_util.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `comp_util_sort_clubs_by_strength` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: ordena no lugar (aloca e libera o temporário): o esperado sai do núcleo sobre uma cópia (chaves num
/// Vec) antes da original; depois, o vetor tem de ser igual.
pub(crate) unsafe extern "C" fn comparar_comp_util_sort_clubs_by_strength(v: usize, n: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32)>(IDX_COMP_UTIL_SORT_CLUBS_BY_STRENGTH);
    if v == 0 || n <= 1 {
        return o(v, n);
    }
    let mut expected = core::slice::from_raw_parts(v as *const u32, n as usize).to_vec();
    let mut k = vec![0i32; n as usize];
    sort_clubs_by_strength(expected.as_mut_ptr(), k.as_mut_ptr(), n);
    o(v, n);
    let obtido = core::slice::from_raw_parts(v as *const u32, n as usize);
    PLACAR.registrar(obtido == &expected[..], || format!("comp_util_sort_clubs_by_strength(n {}) original={:x?} rust={:x?}", n, obtido, expected));
}

/// Comparer for `set_club_final_position` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: acha ou acrescenta o registro (não roda duas vezes): o registro visto antes (se havia) tem de ser o
/// devolvido, com a posição em +0x72.
pub(crate) unsafe extern "thiscall" fn comparar_set_club_final_position(this: usize, clube: usize, pos: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_SET_CLUB_FINAL_POSITION);
    if clube == 0 {
        return o(this, clube, pos); // simplification: a caixa, só a original
    }
    let search: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let before = search(this, clube);
    let r = o(this, clube, pos);
    let same = r != 0 && (before == 0 || r == before) && *((r + 0x72) as *const u8) == pos as u8;
    PLACAR.registrar(same, || format!("set_club_final_position(clube {:x}, {}) antes={:x} original={:x}", clube, pos as u8, before, r));
    r
}

/// Comparer for `comp_util_get_fixture_venue_side` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pura (consulta 0x53b900, sem efeitos): pelo lado.
pub(crate) unsafe extern "C" fn comparar_comp_util_get_fixture_venue_side(clube: usize, jogo: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_COMP_UTIL_GET_FIXTURE_VENUE_SIDE);
    let r = o(clube, jogo);
    let n = comp_util_get_fixture_venue_side(clube, jogo);
    PLACAR.registrar(r == n, || format!("comp_util_get_fixture_venue_side(clube {:x}, jogo {:x}) original={} rust={}", clube, jogo, r, n));
    r
}

/// Comparer for `compare_nation_coefficients_desc` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_compare_nation_coefficients_desc(a: usize, b: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_NATION_COEFFICIENTS_DESC);
    let r = o(a, b);
    let n = compare_nation_coefficients_desc(a, b);
    PLACAR.registrar(r == n, || format!("compare_nation_coefficients_desc original={} rust={}", r, n));
    r
}

/// Comparer for `comp_util_shuffle_pointer_array` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sorteia (0x90cfc0): lado a lado pelo estado do sorteio e pelo vetor, a cópia em sem_contar.
pub(crate) unsafe extern "C" fn comparar_comp_util_shuffle_pointer_array(itens: usize, n: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_COMP_UTIL_SHUFFLE_POINTER_ARRAY);
    let k = (n as u16 as i16).max(0) as usize;
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if k > 0 {
        reg.push((itens, 4 * k));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(itens, n),
        || crate::aleatorio::sem_contar(|| comp_util_shuffle_pointer_array(itens, n)));
    PLACAR.registrar(igual, || format!("comp_util_shuffle_pointer_array({}) {:x?} x {:x?}", k, mo, mr));
}

/// Comparer for `comp_util_sort_and_shuffle_top_clubs` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: ordena (qsort, determinístico) e sorteia: lado a lado pelo estado do sorteio e pelos `n` clubes, a cópia
/// em sem_contar. Caixa (`n` negativo ou menor que `escolhas`): só a original.
pub(crate) unsafe extern "C" fn comparar_comp_util_sort_and_shuffle_top_clubs(clubs: usize, n: u32, escolhas: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32)>(IDX_COMP_UTIL_SORT_AND_SHUFFLE_TOP_CLUBS);
    let (nn, e) = (n as u16 as i16, escolhas as u16 as i16);
    if nn < 0 || nn < e {
        return o(clubs, n, escolhas);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if nn > 0 {
        reg.push((clubs, 4 * nn as usize));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(clubs, n, escolhas),
        || crate::aleatorio::sem_contar(|| comp_util_sort_and_shuffle_top_clubs(clubs, n, escolhas)));
    PLACAR.registrar(igual, || format!("comp_util_sort_and_shuffle_top_clubs({}, {}) {:x?} x {:x?}", nn, e, mo, mr));
}

/// Comparer for `comp_util_get_league_clash_type` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_comp_util_get_league_clash_type(a: usize, b: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_COMP_UTIL_GET_LEAGUE_CLASH_TYPE)(a, b);
    let n = comp_util_get_league_clash_type(a, b);
    PLACAR.registrar(r == n, || format!("comp_util_get_league_clash_type({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `comp_util_get_club_strength_score` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_comp_util_get_club_strength_score(clube: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> i32>(IDX_COMP_UTIL_GET_CLUB_STRENGTH_SCORE)(clube);
    let n = comp_util_get_club_strength_score(clube);
    PLACAR.registrar(r == n, || format!("comp_util_get_club_strength_score({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `comp_util_get_latest_placed_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_comp_util_get_latest_placed_club(comp: usize, place: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(IDX_COMP_UTIL_GET_LATEST_PLACED_CLUB);
    if place as u8 > 2 {
        return o(comp, place);
    }
    let n = (*(G_CLUB_COMP_HISTORY_COUNT as *const i32)).max(0) as usize;
    let reg = [(*(G_CLUB_COMP_HISTORY_TABLE as *const usize), 0x1a * n)];
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(comp, place), || comp_util_get_latest_placed_club(comp, place));
    PLACAR.registrar(igual && r == x, || format!("comp_util_get_latest_placed_club({:x}, {}) {:x}/{:x} {:x?} x {:x?}", comp, place as u8, r, x, mo, mr));
    r
}

/// Comparer for `comp_util_read_fixture_from_news_slots` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_comp_util_read_fixture_from_news_slots(noticia: usize, partida: usize, cursor: *mut u8) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, *mut u8)>(IDX_COMP_UTIL_READ_FIXTURE_FROM_NEWS_SLOTS);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(partida, 0x4d), (cursor as usize, 1)],
        || o(noticia, partida, cursor), || comp_util_read_fixture_from_news_slots(noticia, partida, cursor));
    PLACAR.registrar(igual, || format!("comp_util_read_fixture_from_news_slots {:x?} x {:x?}", mo, mr));
}

/// Comparer for `comp_util_classify_tie_result_for_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_comp_util_classify_tie_result_for_news(clube: usize, partida: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_COMP_UTIL_CLASSIFY_TIE_RESULT_FOR_NEWS)(clube, partida);
    let n = comp_util_classify_tie_result_for_news(clube, partida);
    PLACAR.registrar(r == n, || format!("comp_util_classify_tie_result_for_news original={} rust={}", r, n));
    r
}

/// Comparer for `comp_util_write_fixture_to_news_slots` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_comp_util_write_fixture_to_news_slots(jogo: usize, noticia: usize, cursor: *mut u8) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, *mut u8) -> u32>(IDX_COMP_UTIL_WRITE_FIXTURE_TO_NEWS_SLOTS);
    let k = *cursor as i8;
    // índice fora de 0..49 em algum dos 18: a 0x785790 abre a caixa de erro; só a original
    if !(0..0x32 - 17).contains(&k) {
        return o(jogo, noticia, cursor);
    }
    let reg = [(noticia + 5, 200), (cursor as usize, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(jogo, noticia, cursor), || comp_util_write_fixture_to_news_slots(jogo, noticia, cursor));
    PLACAR.registrar(igual && r == n, || format!("comp_util_write_fixture_to_news_slots(cursor {}) original={} rust={} {:x?} x {:x?}", k, r, n, mo, mr));
    r
}

/// Comparer for `comp_util_format_stage_round_name` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x4bffc0: lado a lado na saída (até o tamanho, no máximo 0x400 bytes) e na devolução.
pub(crate) unsafe extern "C" fn comparar_comp_util_format_stage_round_name(stage: u32, round: u32, out: *mut u8, size: i32, estilo: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32, u32, *mut u8, i32, u32) -> i32>(IDX_COMP_UTIL_FORMAT_STAGE_ROUND_NAME);
    if out.is_null() || size < 0xc8 {
        return o(stage, round, out, size, estilo); // a caixa: só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(out as usize, (size as usize).min(0x400))],
        || o(stage, round, out, size, estilo), || comp_util_format_stage_round_name(stage, round, out, size, estilo));
    PLACAR.registrar(igual && r == n, || format!("comp_util_format_stage_round_name({}, {}) {} x {} {:x?} x {:x?}", stage as i16, round as i16, r, n, mo, mr));
    r
}

/// Comparer for `comp_util_scale_by_round` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pura (pelo eax inteiro). Fora da faixa (a caixa de erro) só a original.
pub(crate) unsafe extern "C" fn comparar_comp_util_scale_by_round(low: u32, high: u32, round: u32, rounds: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, u32, u32, u32) -> u32>(IDX_COMP_UTIL_SCALE_BY_ROUND);
    if !(0..=100).contains(&interpolate_by_round(low, high, round, rounds).1) {
        return o(low, high, round, rounds);
    }
    let r = o(low, high, round, rounds);
    let n = comp_util_scale_by_round(low, high, round, rounds);
    PLACAR.registrar(r == n, || format!("comp_util_scale_by_round({}, {}, {}, {}) original={:x} rust={:x}",
        low as u8 as i8, high as u8 as i8, round as u16 as i16, rounds as u16 as i16, r, n));
    r
}

/// Comparer for `comp_util_add_history_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_comp_util_add_history_record(league: usize, a: u32, b: u32, c: u32, d: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32) -> usize>(IDX_COMP_UTIL_ADD_HISTORY_RECORD);
    if free_comp_history_row() == 0 {
        return o(league, a, b, c, d); // cresce (realloc): só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&comp_history_regions(), || o(league, a, b, c, d), || comp_util_add_history_record(league, a, b, c, d));
    PLACAR.registrar(igual && r == n, || format!("comp_util_add_history_record({:x}, {:x}, {:x}, {:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", league, a, b, c, d, r, n, mo, mr));
    r
}

/// Comparer for `comp_util_get_parent_stage_id` (comparison mode: see the header of motor.rs).
///
/// [pt] Pura: a devolução.
pub(crate) unsafe extern "C" fn comparar_comp_util_get_parent_stage_id(id: i32) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(i32) -> i32>(IDX_COMP_UTIL_GET_PARENT_STAGE_ID)(id);
    let n = comp_util_get_parent_stage_id(id);
    PLACAR.registrar(r == n, || format!("comp_util_get_parent_stage_id({:#x}): original={:#x} rust={:#x}", id, r, n));
    r
}

/// Comparer for `units_digit` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_units_digit(n: i32) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(i32) -> u8>(IDX_UNITS_DIGIT)(n);
    let v = units_digit(n);
    PLACAR.registrar(r == v, || format!("units_digit({}) original={} rust={}", n, r, v));
    r
}

/// Comparer for `is_national_team` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_is_national_team(clube: usize) -> u32 {
    if is_recording_post_match() {
        return call_recording(FN_DATABASE_IS_NATIONAL_TEAM, 0, [clube as u32, 0, 0, 0, 0], || original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_IS_NATIONAL_TEAM)(clube));
    }
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_IS_NATIONAL_TEAM)(clube);
    let v = is_national_team(clube);
    PLACAR.registrar(r == v, || format!("is_national_team original={} rust={}", r, v));
    r
}

/// Comparer for `competition_counts` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_competition_counts(id: i32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(i32) -> u32>(IDX_COMPETITION_COUNTS)(id);
    let v = competition_counts(id);
    PLACAR.registrar(r == v, || format!("competition_counts({}) original={} rust={}", id, r, v));
    r
}
