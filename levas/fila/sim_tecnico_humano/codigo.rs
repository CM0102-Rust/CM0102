// ---------- %%LEVA%%a leva: o registro do técnico humano (hora das férias e escalação guardada) e o técnico da busca (human_manager.cpp, player_search.cpp) ----------

/// O registro de 42735 bytes do técnico humano `pessoa` (nula: o jogador de rede da vaga do humano da
/// vez, [0xb74c26 + 0x300 × vaga]). Os humanos são as 16 últimas pessoas: índice = id − ([0xae2364] − 16),
/// válido de 0 a [gerente+4] − 1. `None` quando a original dá a caixa de erro.
unsafe fn human_manager_record(gerente: usize, pessoa: usize) -> Option<usize> {
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
unsafe extern "thiscall" fn get_holiday_stamp(gerente: usize, pessoa: usize) -> u32 {
    match human_manager_record(gerente, pessoa) {
        Some(r) => core::ptr::read_unaligned((r + 0x9a65) as *const u32),
        None => original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(gerente, pessoa),
    }
}

/// { 0x5fed50 } (thiscall, ret 4, human_manager.cpp): o endereço da escalação guardada do técnico
/// humano (16 pares jogador e posição, +0x9b28 do registro). Fora da faixa: a original (caixa de erro, 0).
unsafe extern "thiscall" fn get_saved_lineup(gerente: usize, pessoa: usize) -> usize {
    match human_manager_record(gerente, pessoa) {
        Some(r) => r + 0x9b28,
        None => original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(gerente, pessoa),
    }
}

/// { 0x7ae570 } (thiscall, ret 4, player_search.cpp): grava na busca (+0) o id do técnico humano
/// `pessoa` e devolve 1. Pessoa nula ou que não é humana (id abaixo de [0xae2364] − 16): a original
/// (caixa de erro, 0).
unsafe extern "thiscall" fn set_manager(busca: usize, pessoa: usize) -> u32 {
    if pessoa == 0 || *(pessoa as *const i32) < *(0xae2364 as *const i32) - 0x10 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(busca, pessoa);
    }
    *(busca as *mut i32) = *(pessoa as *const i32);
    1
}

unsafe extern "thiscall" fn comparar_get_holiday_stamp(gerente: usize, pessoa: usize) -> u32 {
    if human_manager_record(gerente, pessoa).is_none() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(gerente, pessoa);
    }
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(gerente, pessoa);
    let n = get_holiday_stamp(gerente, pessoa);
    PLACAR.record(r == n, || format!("get_holiday_stamp original={} rust={}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_get_saved_lineup(gerente: usize, pessoa: usize) -> usize {
    if human_manager_record(gerente, pessoa).is_none() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(gerente, pessoa);
    }
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(gerente, pessoa);
    let n = get_saved_lineup(gerente, pessoa);
    PLACAR.record(r == n, || format!("get_saved_lineup original={:#x} rust={:#x}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_set_manager(busca: usize, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    if pessoa == 0 || *(pessoa as *const i32) < *(0xae2364 as *const i32) - 0x10 {
        return o(busca, pessoa);
    }
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&[(busca, 4)], || o(busca, pessoa), || set_manager(busca, pessoa));
    PLACAR.record(igual && ro == rr, || format!("set_manager original={} rust={} {:x?} x {:x?}", ro, rr, mo, mr));
    ro
}
