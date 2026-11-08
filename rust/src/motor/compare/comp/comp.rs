use super::*;

// The comparers of the routines in ../comp/comp.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/comp.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `send_prize_money_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_send_prize_money_news(this: usize, clube: usize, lo: u32, hi: i32, position: u32, round: u32, partida: usize, kind: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, u32, usize, u32)>(IDX_SEND_PRIZE_MONEY_NEWS)(this, clube, lo, hi, position, round, partida, kind)
}

/// Comparer for `send_tv_rights_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_send_tv_rights_news(this: usize, clube: usize, value: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_SEND_TV_RIGHTS_NEWS)(this, clube, value)
}

/// Comparer for `all_stages_complete` (comparison mode: see the header of motor.rs).
///
/// [pt] Só a original (o método +0x10 marca a liga completa, grava recordes e paga prêmios: não roda duas vezes): a cópia
/// leva os ramos.
pub(crate) unsafe extern "thiscall" fn comparar_all_stages_complete(this: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_ALL_STAGES_COMPLETE)(this)
}

/// Comparer for `comp_collect_club_list` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado: a original com a saída de quem chama, a cópia com saída própria, a partir do mesmo [0xb67a34]; a conta
/// e os clubes das duas listas iguais (a da cópia é liberada).
pub(crate) unsafe extern "thiscall" fn comparar_comp_collect_club_list(this: usize, list: *mut usize, x: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16>(IDX_COMP_COLLECT_CLUB_LIST);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let mut l2 = 0usize;
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, list, x), || comp_collect_club_list(this, &mut l2, x));
    let mesmas = a == b && (*list == 0) == (l2 == 0) && (*list == 0 || core::slice::from_raw_parts(*list as *const u32, a as usize) == core::slice::from_raw_parts(l2 as *const u32, b as usize));
    if l2 != 0 {
        soltar(l2);
    }
    PLACAR.registrar(igual && mesmas, || format!("comp_collect_club_list: conta original={} rust={}", a, b));
    a
}

/// Comparer for `find_club_progress` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado nas três saídas (a original e a cópia com saídas próprias, começando dos valores de quem chama).
pub(crate) unsafe extern "thiscall" fn comparar_find_club_progress(this: usize, clube: usize, stage_id: *mut i16, round: *mut i16, status: *mut u8) {
    let (mut f2, mut r2, mut s2) = (*stage_id, *round, *status);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i16, *mut i16, *mut u8)>(IDX_FIND_CLUB_PROGRESS)(this, clube, stage_id, round, status);
    find_club_progress(this, clube, &mut f2, &mut r2, &mut s2);
    let ok = f2 == *stage_id && r2 == *round && s2 == *status;
    PLACAR.registrar(ok, || format!("find_club_progress: original ({}, {}, {}) rust ({}, {}, {})", *stage_id, *round, *status, f2, r2, s2));
}

/// Comparer for `get_first_stage_of_group` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da primeira fase do grupo: com o índice válido (−1..+0x30), as duas (só leem) e o mesmo índice; fora
/// dele só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn comparar_get_first_stage_of_group(this: usize, k: i32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> i32>(IDX_GET_FIRST_STAGE_OF_GROUP);
    if k < -1 || k > read_u32_as_usize(this + 0x30) as i32 {
        return o(this, k);
    }
    let r = o(this, k);
    let n = get_first_stage_of_group(this, k);
    PLACAR.registrar(r == n, || format!("get_first_stage_of_group({}): original={} rust={}", k, r, n));
    r
}

/// Comparer for `get_current_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da fase atual: só a original (simplification: o método 0x10 das fases pode refazer a tabela; não roda duas
/// vezes); a cópia se prova no modo Rust pelas telas.
pub(crate) unsafe extern "thiscall" fn comparar_get_current_stage(this: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_GET_CURRENT_STAGE)(this)
}

/// Comparador: rotina pura; compara o byte devolvido.
pub(crate) unsafe extern "C" fn comparar_comp_active_key_nation_of_club(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_COMP_ACTIVE_KEY_NATION_OF_CLUB)(clube);
    let n = comp_active_key_nation_of_club(clube);
    PLACAR.registrar(r == n, || format!("comp_active_key_nation_of_club({:#x}) original={} rust={}", clube, r, n));
    r
}

/// Comparador: rotina pura; compara o byte devolvido.
pub(crate) unsafe extern "C" fn comparar_comp_active_key_nation_of_stadium(estadio: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_COMP_ACTIVE_KEY_NATION_OF_STADIUM)(estadio);
    let n = comp_active_key_nation_of_stadium(estadio);
    PLACAR.registrar(r == n, || format!("comp_active_key_nation_of_stadium({:#x}) original={} rust={}", estadio, r, n));
    r
}

/// Comparer for `comp_key_nation_entry_from_active_index` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: rotina pura; compara a entrada devolvida. Sem entrada (a caixa): só a original.
pub(crate) unsafe extern "C" fn comparar_comp_key_nation_entry_from_active_index(active: i8) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(i8) -> i32>(IDX_COMP_KEY_NATION_ENTRY_FROM_ACTIVE_INDEX);
    if key_nation_by_active_index(active).is_none() {
        return o(active); // simplification: the box does not run twice
    }
    let r = o(active);
    let n = comp_key_nation_entry_from_active_index(active);
    PLACAR.registrar(r == n, || format!("comp_key_nation_entry_from_active_index({}) original={} rust={}", active, r, n));
    r
}

/// Comparer for `get_fixture_match_importance` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (a virtual da fase só lê; roda duas vezes).
pub(crate) unsafe extern "thiscall" fn comparar_get_fixture_match_importance(this: usize, p: usize, a: u32, b: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_GET_FIXTURE_MATCH_IMPORTANCE)(this, p, a, b);
    let n = get_fixture_match_importance(this, p, a, b);
    PLACAR.registrar(r == n, || format!("get_fixture_match_importance({:x}, partida {:x}, {}, {}) original={} rust={}", this, p, a, b, r, n));
    r
}

/// Comparer for `get_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê). A caixa: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_get_stage(this: usize, k: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(IDX_GET_STAGE);
    if k < -1 || k > read_usize(this + 0x30) as i32 {
        return o(this, k); // simplification: the box, original only
    }
    let r = o(this, k);
    let n = get_stage(this, k);
    PLACAR.registrar(r == n, || format!("get_stage({:x}, {}) original={:x} rust={:x}", this, k, r, n));
    r
}

/// Comparer for `comp_active_key_nation_of_fixture` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_comp_active_key_nation_of_fixture(comp: usize, partida: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_COMP_ACTIVE_KEY_NATION_OF_FIXTURE)(comp, partida);
    let n = comp_active_key_nation_of_fixture(comp, partida);
    PLACAR.registrar(r == n, || format!("comp_active_key_nation_of_fixture(competição {}, partida {:x}) original={} rust={}", read_usize(comp) as i32, partida, r, n));
    r
}

/// Comparer for `are_round_fixtures_known` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê). A caixa: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_are_round_fixtures_known(this: usize, stage: i32, round: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(IDX_ARE_ROUND_FIXTURES_KNOWN);
    if stage < -1 && stage <= read_usize(this + 0x30) as i32 {
        return o(this, stage, round); // simplification: the box, original only
    }
    let r = o(this, stage, round);
    let n = are_round_fixtures_known(this, stage, round);
    PLACAR.registrar(r == n, || format!("are_round_fixtures_known({:x}, {}, {}) original={} rust={}", this, stage, round, r, n));
    r
}

/// Comparer for `counts_for_fixture_clash` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_counts_for_fixture_clash(this: usize, jogo: usize, other: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_COUNTS_FOR_FIXTURE_CLASH)(this, jogo, other);
    let n = counts_for_fixture_clash(this, jogo, other);
    PLACAR.registrar(r == n, || format!("counts_for_fixture_clash(rodada {}) original={} rust={}", read_i16(jogo + 0x30), r, n));
    r
}

/// Comparer for `special_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_special_club(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_SPECIAL_CLUB)(clube);
    let v = special_club(clube);
    PLACAR.registrar(r == v, || format!("special_club({:#x}) original={} rust={}", clube, r, v));
    r
}
