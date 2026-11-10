use super::*;

// The comparers of the routines in ../comp/comp.rs.
// [pt] Os comparadores das rotinas de ../comp/comp.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `send_prize_money_news`.
pub(crate) unsafe extern "thiscall" fn compare_send_prize_money_news(this: usize, club: usize, lo: u32, hi: i32, position: u32, round: u32, partida: usize, kind: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, u32, usize, u32)>(IDX_SEND_PRIZE_MONEY_NEWS)(this, club, lo, hi, position, round, partida, kind)
}

/// Comparer for `send_tv_rights_news`.
pub(crate) unsafe extern "thiscall" fn compare_send_tv_rights_news(this: usize, club: usize, value: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_SEND_TV_RIGHTS_NEWS)(this, club, value)
}

/// Comparer for `all_stages_complete`.
///
/// [pt] Só a original (o método +0x10 marca a liga completa, grava recordes e paga prêmios: não roda duas vezes): a cópia
/// leva os ramos.
pub(crate) unsafe extern "thiscall" fn compare_all_stages_complete(this: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_ALL_STAGES_COMPLETE)(this)
}

/// Comparer for `comp_collect_club_list`.
///
/// [pt] Lado a lado: a original com a saída de quem chama, a cópia com saída própria, a partir do mesmo [0xb67a34]; a conta
/// e os clubes das duas listas iguais (a da cópia é liberada).
pub(crate) unsafe extern "thiscall" fn compare_comp_collect_club_list(this: usize, list: *mut usize, x: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16>(IDX_COMP_COLLECT_CLUB_LIST);
    let mut l2 = 0usize;
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this, list, x), || comp_collect_club_list(this, &mut l2, x));
    let mesmas = a == b && (*list == 0) == (l2 == 0) && (*list == 0 || core::slice::from_raw_parts(*list as *const u32, a as usize) == core::slice::from_raw_parts(l2 as *const u32, b as usize));
    if l2 != 0 {
        exe::crt_free(l2);
    }
    SCOREBOARD.record(igual && mesmas, || format!("comp_collect_club_list: count original={} rust={}", a, b));
    a
}

/// Comparer for `find_club_progress`.
///
/// [pt] Lado a lado nas três saídas (a original e a cópia com saídas próprias, começando dos valores de quem chama).
pub(crate) unsafe extern "thiscall" fn compare_find_club_progress(this: usize, club: usize, stage_id: *mut i16, round: *mut i16, status: *mut u8) {
    let (mut f2, mut r2, mut s2) = (*stage_id, *round, *status);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i16, *mut i16, *mut u8)>(IDX_FIND_CLUB_PROGRESS)(this, club, stage_id, round, status);
    find_club_progress(this, club, &mut f2, &mut r2, &mut s2);
    let ok = f2 == *stage_id && r2 == *round && s2 == *status;
    SCOREBOARD.record(ok, || format!("find_club_progress: original ({}, {}, {}) rust ({}, {}, {})", *stage_id, *round, *status, f2, r2, s2));
}

/// Comparer for `get_first_stage_of_group`.
///
/// [pt] O comparador da primeira fase do grupo: com o índice válido (−1..+0x30), as duas (só leem) e o mesmo índice; fora
/// dele só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn compare_get_first_stage_of_group(this: usize, k: i32) -> i32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> i32>(IDX_GET_FIRST_STAGE_OF_GROUP);
    if k < -1 || k > this.current_stage() as i32 {
        return o(this.addr(), k);
    }
    let r = o(this.addr(), k);
    let n = get_first_stage_of_group(this.addr(), k);
    SCOREBOARD.record(r == n, || format!("get_first_stage_of_group({}): original={} rust={}", k, r, n));
    r
}

/// Comparer for `get_current_stage`.
///
/// [pt] O comparador da fase atual: só a original (simplification: o método 0x10 das fases pode refazer a tabela; não roda duas
/// vezes); a cópia se prova no modo Rust pelas telas.
pub(crate) unsafe extern "thiscall" fn compare_get_current_stage(this: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_GET_CURRENT_STAGE)(this)
}

/// Comparador: rotina pura; compara o byte devolvido.
pub(crate) unsafe extern "C" fn compare_comp_active_key_nation_of_club(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_COMP_ACTIVE_KEY_NATION_OF_CLUB)(club);
    let n = comp_active_key_nation_of_club(club);
    SCOREBOARD.record(r == n, || format!("comp_active_key_nation_of_club({:#x}) original={} rust={}", club, r, n));
    r
}

/// Comparador: rotina pura; compara o byte devolvido.
pub(crate) unsafe extern "C" fn compare_comp_active_key_nation_of_stadium(stadium: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_COMP_ACTIVE_KEY_NATION_OF_STADIUM)(stadium);
    let n = comp_active_key_nation_of_stadium(stadium);
    SCOREBOARD.record(r == n, || format!("comp_active_key_nation_of_stadium({:#x}) original={} rust={}", stadium, r, n));
    r
}

/// Comparer for `comp_key_nation_entry_from_active_index`.
///
/// [pt] Comparador: rotina pura; compara a entrada devolvida. Sem entrada (a caixa): só a original.
pub(crate) unsafe extern "C" fn compare_comp_key_nation_entry_from_active_index(active: i8) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(i8) -> i32>(IDX_COMP_KEY_NATION_ENTRY_FROM_ACTIVE_INDEX);
    if key_nation_by_active_index(active).is_none() {
        return o(active); // simplification: the box does not run twice
    }
    let r = o(active);
    let n = comp_key_nation_entry_from_active_index(active);
    SCOREBOARD.record(r == n, || format!("comp_key_nation_entry_from_active_index({}) original={} rust={}", active, r, n));
    r
}

/// Comparer for `get_fixture_match_importance`.
///
/// [pt] Comparador: pelo valor (a virtual da fase só lê; roda duas vezes).
pub(crate) unsafe extern "thiscall" fn compare_get_fixture_match_importance(this: usize, p: usize, a: u32, b: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_GET_FIXTURE_MATCH_IMPORTANCE)(this, p, a, b);
    let n = get_fixture_match_importance(this, p, a, b);
    SCOREBOARD.record(r == n, || format!("get_fixture_match_importance({:x}, match {:x}, {}, {}) original={} rust={}", this, p, a, b, r, n));
    r
}

/// Comparer for `get_stage`.
///
/// [pt] Comparador: pelo valor (só lê). A caixa: só a original.
pub(crate) unsafe extern "thiscall" fn compare_get_stage(this: usize, k: i32) -> usize {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(IDX_GET_STAGE);
    if k < -1 || k > this.current_stage() as i32 {
        return o(this.addr(), k); // simplification: the box, original only
    }
    let r = o(this.addr(), k);
    let n = get_stage(this.addr(), k);
    SCOREBOARD.record(r == n, || format!("get_stage({:x}, {}) original={:x} rust={:x}", this.addr(), k, r, n));
    r
}

/// Comparer for `comp_active_key_nation_of_fixture`.
pub(crate) unsafe extern "C" fn compare_comp_active_key_nation_of_fixture(comp: usize, partida: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_COMP_ACTIVE_KEY_NATION_OF_FIXTURE)(comp, partida);
    let n = comp_active_key_nation_of_fixture(comp, partida);
    SCOREBOARD.record(r == n, || format!("comp_active_key_nation_of_fixture(competition {}, match {:x}) original={} rust={}", read_usize(comp) as i32, partida, r, n));
    r
}

/// Comparer for `are_round_fixtures_known`.
///
/// [pt] Comparador: pelo valor (só lê). A caixa: só a original.
pub(crate) unsafe extern "thiscall" fn compare_are_round_fixtures_known(this: usize, stage: i32, round: i32) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(IDX_ARE_ROUND_FIXTURES_KNOWN);
    if stage < -1 && stage <= this.current_stage() as i32 {
        return o(this.addr(), stage, round); // simplification: the box, original only
    }
    let r = o(this.addr(), stage, round);
    let n = are_round_fixtures_known(this.addr(), stage, round);
    SCOREBOARD.record(r == n, || format!("are_round_fixtures_known({:x}, {}, {}) original={} rust={}", this.addr(), stage, round, r, n));
    r
}

/// Comparer for `counts_for_fixture_clash`.
pub(crate) unsafe extern "thiscall" fn compare_counts_for_fixture_clash(this: usize, game: usize, other: usize) -> u32 {
    let game = Fixture(game);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_COUNTS_FOR_FIXTURE_CLASH)(this, game.addr(), other);
    let n = counts_for_fixture_clash(this, game.addr(), other);
    SCOREBOARD.record(r == n, || format!("counts_for_fixture_clash(round {}) original={} rust={}", game.round_name(), r, n));
    r
}

/// Comparer for `special_club`.
pub(crate) unsafe extern "stdcall" fn compare_special_club(club: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_SPECIAL_CLUB)(club);
    let v = special_club(club);
    SCOREBOARD.record(r == v, || format!("special_club({:#x}) original={} rust={}", club, r, v));
    r
}
