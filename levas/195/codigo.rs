// ---------- 195a leva: a última nota do jogador e a vaga e o tipo da busca de jogadores (player_stats.cpp, player_search.cpp) ----------

/// { 0x7c1a60 } (thiscall, ret 4, player_stats.cpp): a nota da última partida do jogador `*id` (os 4
/// bits baixos do byte +0x26 do registro de 39 bytes em [estatisticas+0x30]); sem nota, 6,5
/// ([0x96e2b8]). Jogador nulo: a original (caixa de erro). Devolve em st0.
unsafe extern "thiscall" fn ultima_nota_do_jogador(estatisticas: usize, id: *const i32) -> f32 {
    if id.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, *const i32) -> f32>(457)(estatisticas, id);
    }
    let registro = (*((estatisticas + 0x30) as *const usize) as isize + *id as isize * 39) as usize;
    let nota = *((registro + 0x26) as *const u8) & 0xf;
    if nota != 0 { nota as f32 } else { *(0x96e2b8 as *const f32) }
}

/// { 0x7ae640 } (thiscall, ret 4, player_search.cpp): grava a vaga `vaga` (0 a 9) da busca em +4 e
/// devolve 1. Fora da faixa: a original (caixa de erro, devolve 0).
unsafe extern "thiscall" fn vaga_da_busca(busca: usize, vaga: i8) -> u32 {
    if !(0..=9).contains(&vaga) {
        return orig::<unsafe extern "thiscall" fn(usize, i8) -> u32>(458)(busca, vaga);
    }
    *((busca + 4) as *mut i8) = vaga;
    1
}

/// { 0x7ae6e0 } (thiscall, ret 4, player_search.cpp): grava o tipo de lista `tipo` (1 a 4) da busca em
/// +0xa e devolve 1. Fora da faixa: a original (caixa de erro, devolve 0).
unsafe extern "thiscall" fn tipo_da_lista_da_busca(busca: usize, tipo: i8) -> u32 {
    if !(1..=4).contains(&tipo) {
        return orig::<unsafe extern "thiscall" fn(usize, i8) -> u32>(459)(busca, tipo);
    }
    *((busca + 0xa) as *mut i8) = tipo;
    1
}

unsafe extern "thiscall" fn comparar_ultima_nota_do_jogador(estatisticas: usize, id: *const i32) -> f32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, *const i32) -> f32>(457)(estatisticas, id);
    if id.is_null() {
        return r;
    }
    let n = ultima_nota_do_jogador(estatisticas, id);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("ultima_nota_do_jogador({}) original={} rust={}", *id, r, n));
    r
}

unsafe extern "thiscall" fn comparar_vaga_da_busca(busca: usize, vaga: i8) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8) -> u32>(458);
    if !(0..=9).contains(&vaga) {
        return o(busca, vaga);
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(busca + 4, 1)], || o(busca, vaga), || vaga_da_busca(busca, vaga));
    PLACAR.registrar(igual && ro == rr, || format!("vaga_da_busca({}) original={} rust={} {:x?} x {:x?}", vaga, ro, rr, mo, mr));
    ro
}

unsafe extern "thiscall" fn comparar_tipo_da_lista_da_busca(busca: usize, tipo: i8) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8) -> u32>(459);
    if !(1..=4).contains(&tipo) {
        return o(busca, tipo);
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(busca + 0xa, 1)], || o(busca, tipo), || tipo_da_lista_da_busca(busca, tipo));
    PLACAR.registrar(igual && ro == rr, || format!("tipo_da_lista_da_busca({}) original={} rust={} {:x?} x {:x?}", tipo, ro, rr, mo, mr));
    ro
}
