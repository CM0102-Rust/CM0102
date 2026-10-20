// ---------- %%LEVA%%a leva: o registro do técnico humano (hora das férias e escalação guardada) e o técnico da busca (human_manager.cpp, player_search.cpp) ----------

/// O registro de 42735 bytes do técnico humano `pessoa` (nula: o jogador de rede da vaga do humano da
/// vez, [0xb74c26 + 0x300 × vaga]). Os humanos são as 16 últimas pessoas: índice = id − ([0xae2364] − 16),
/// válido de 0 a [gerente+4] − 1. `None` quando a original dá a caixa de erro.
unsafe fn registro_do_tecnico_humano(gerente: usize, pessoa: usize) -> Option<usize> {
    let pessoa = if pessoa != 0 { pessoa } else { *((0xb74c26 + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize) };
    if pessoa == 0 {
        return None;
    }
    let d = (*(pessoa as *const i32)).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
    if d < 0 || d > *((gerente + 4) as *const i32) - 1 {
        return None;
    }
    Some((*(gerente as *const usize) as isize + d as isize * 42735) as usize)
}

/// { 0x5fdc30 } (thiscall, ret 4, human_manager.cpp): a hora (relógio em ms) guardada quando o técnico
/// humano saiu de férias (+0x9a65 do registro). Fora da faixa: a original (caixa de erro, 0).
unsafe extern "thiscall" fn hora_das_ferias(gerente: usize, pessoa: usize) -> u32 {
    match registro_do_tecnico_humano(gerente, pessoa) {
        Some(r) => core::ptr::read_unaligned((r + 0x9a65) as *const u32),
        None => orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(gerente, pessoa),
    }
}

/// { 0x5fed50 } (thiscall, ret 4, human_manager.cpp): o endereço da escalação guardada do técnico
/// humano (16 pares jogador e posição, +0x9b28 do registro). Fora da faixa: a original (caixa de erro, 0).
unsafe extern "thiscall" fn escalacao_guardada(gerente: usize, pessoa: usize) -> usize {
    match registro_do_tecnico_humano(gerente, pessoa) {
        Some(r) => r + 0x9b28,
        None => orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(gerente, pessoa),
    }
}

/// { 0x7ae570 } (thiscall, ret 4, player_search.cpp): grava na busca (+0) o id do técnico humano
/// `pessoa` e devolve 1. Pessoa nula ou que não é humana (id abaixo de [0xae2364] − 16): a original
/// (caixa de erro, 0).
unsafe extern "thiscall" fn tecnico_da_busca(busca: usize, pessoa: usize) -> u32 {
    if pessoa == 0 || *(pessoa as *const i32) < *(0xae2364 as *const i32) - 0x10 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(busca, pessoa);
    }
    *(busca as *mut i32) = *(pessoa as *const i32);
    1
}

unsafe extern "thiscall" fn comparar_hora_das_ferias(gerente: usize, pessoa: usize) -> u32 {
    if registro_do_tecnico_humano(gerente, pessoa).is_none() {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(gerente, pessoa);
    }
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(gerente, pessoa);
    let n = hora_das_ferias(gerente, pessoa);
    PLACAR.registrar(r == n, || format!("hora_das_ferias original={} rust={}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_escalacao_guardada(gerente: usize, pessoa: usize) -> usize {
    if registro_do_tecnico_humano(gerente, pessoa).is_none() {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(gerente, pessoa);
    }
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(gerente, pessoa);
    let n = escalacao_guardada(gerente, pessoa);
    PLACAR.registrar(r == n, || format!("escalacao_guardada original={:#x} rust={:#x}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_tecnico_da_busca(busca: usize, pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    if pessoa == 0 || *(pessoa as *const i32) < *(0xae2364 as *const i32) - 0x10 {
        return o(busca, pessoa);
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(busca, 4)], || o(busca, pessoa), || tecnico_da_busca(busca, pessoa));
    PLACAR.registrar(igual && ro == rr, || format!("tecnico_da_busca original={} rust={} {:x?} x {:x?}", ro, rr, mo, mr));
    ro
}
