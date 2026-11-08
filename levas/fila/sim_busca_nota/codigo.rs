// ---------- %%LEVA%%a leva: a última nota do jogador e a vaga e o tipo da busca de jogadores (player_stats.cpp, player_search.cpp) ----------

/// { 0x7c1a60 } (thiscall, ret 4, player_stats.cpp): a nota da última partida do jogador `*id` (os 4
/// bits baixos do byte +0x26 do registro de 39 bytes em [estatisticas+0x30]); sem nota, 6,5
/// ([0x96e2b8]). Jogador nulo: a original (caixa de erro). Devolve em st0.
unsafe extern "thiscall" fn get_last_match_rating(estatisticas: usize, id: *const i32) -> f32 {
    if id.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> f32>(%%I0%%)(estatisticas, id);
    }
    let registro = (*((estatisticas + 0x30) as *const usize) as isize + *id as isize * 39) as usize;
    let nota = *((registro + 0x26) as *const u8) & 0xf;
    if nota != 0 { nota as f32 } else { *(0x96e2b8 as *const f32) }
}

/// { 0x7ae640 } (thiscall, ret 4, player_search.cpp): grava a vaga `vaga` (0 a 9) da busca em +4 e
/// devolve 1. Fora da faixa: a original (caixa de erro, devolve 0).
unsafe extern "thiscall" fn set_slot_index(busca: usize, vaga: i8) -> u32 {
    if !(0..=9).contains(&vaga) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(%%I1%%)(busca, vaga);
    }
    *((busca + 4) as *mut i8) = vaga;
    1
}

/// { 0x7ae6e0 } (thiscall, ret 4, player_search.cpp): grava o tipo de lista `tipo` (1 a 4) da busca em
/// +0xa e devolve 1. Fora da faixa: a original (caixa de erro, devolve 0).
unsafe extern "thiscall" fn set_list_type(busca: usize, tipo: i8) -> u32 {
    if !(1..=4).contains(&tipo) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(%%I2%%)(busca, tipo);
    }
    *((busca + 0xa) as *mut i8) = tipo;
    1
}

unsafe extern "thiscall" fn comparar_get_last_match_rating(estatisticas: usize, id: *const i32) -> f32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> f32>(%%I0%%)(estatisticas, id);
    if id.is_null() {
        return r;
    }
    let n = get_last_match_rating(estatisticas, id);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("get_last_match_rating({}) original={} rust={}", *id, r, n));
    r
}

unsafe extern "thiscall" fn comparar_set_slot_index(busca: usize, vaga: i8) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(%%I1%%);
    if !(0..=9).contains(&vaga) {
        return o(busca, vaga);
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(busca + 4, 1)], || o(busca, vaga), || set_slot_index(busca, vaga));
    PLACAR.registrar(igual && ro == rr, || format!("set_slot_index({}) original={} rust={} {:x?} x {:x?}", vaga, ro, rr, mo, mr));
    ro
}

unsafe extern "thiscall" fn comparar_set_list_type(busca: usize, tipo: i8) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(%%I2%%);
    if !(1..=4).contains(&tipo) {
        return o(busca, tipo);
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(busca + 0xa, 1)], || o(busca, tipo), || set_list_type(busca, tipo));
    PLACAR.registrar(igual && ro == rr, || format!("set_list_type({}) original={} rust={} {:x?} x {:x?}", tipo, ro, rr, mo, mr));
    ro
}
