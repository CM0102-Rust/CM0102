// ---------- %%LEVA%%a leva: a partida assistida por algum humano, o apelido dado pelo humano e a cópia da aposentadoria pendente (human_manager.cpp, player_regen.cpp) ----------

/// { 0x5fe420 } (thiscall, ret 4, human_manager.cpp): 1 se algum dos 16 técnicos humanos ativos
/// (registros de 110 bytes em [0xae23bc], a partir de [0xae2364] − 16, +0x5f não nulo) escolheu
/// assistir a `partida` (0x5fe290); senão 0. Partida nula: a original (caixa de erro e 0).
unsafe extern "thiscall" fn is_fixture_watched(this: usize, partida: usize) -> u32 {
    if partida == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, partida);
    }
    let assiste: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x5fe290usize);
    for k in 0..16i32 {
        let i = k.wrapping_add(*(0xae2364 as *const i32)).wrapping_sub(0x10);
        let h = (*(0xae23bc as *const usize)).wrapping_add((i as isize * 110) as usize);
        if *((h + 0x5f) as *const u8) != 0 && assiste(this, partida, h) != 0 {
            return 1;
        }
    }
    0 // ninguém assiste
}

/// O índice (0..[this+4]) do técnico humano `humano` (ou o da vez, [0xb74c26 + 0x300 × [0xb77c7a]])
/// para a 0x600740: None se não há humano; Some(None) fora da faixa (caso de erro da original).
unsafe fn human_index_for_nickname(this: usize, humano: usize) -> Option<Option<usize>> {
    let h = if humano != 0 {
        humano
    } else {
        *((0xb74c26 + 0x300 * *(0xb77c7a as *const i16) as isize) as *const usize)
    };
    if h == 0 {
        return None;
    }
    let i = (*(h as *const i32)).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
    if i < 0 || i > (*((this + 4) as *const i32)).wrapping_sub(1) {
        return Some(None);
    }
    Some(Some(i as usize))
}

/// { 0x600740 } (thiscall, ret 8, human_manager.cpp): 1 se o técnico humano `humano` (0 = o da vez)
/// deu apelido à pessoa de id [`pessoa`]: busca binária (0x946464 com 0x601a10, itens de 0x1e bytes)
/// na lista +0xa6e3 (quantos em +0xa6eb) do registro dele (0xa6ef bytes em [this]). Pessoa ou humano
/// nulos, ou lista vazia: 0. Humano fora da faixa: a original (caixa de erro e 0).
unsafe extern "thiscall" fn has_nickname(this: usize, pessoa: usize, humano: usize) -> u32 {
    if pessoa == 0 {
        return 0;
    }
    let i = match human_index_for_nickname(this, humano) {
        None => return 0,
        Some(None) => return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%)(this, pessoa, humano),
        Some(Some(i)) => i,
    };
    let buscar: unsafe extern "C" fn(*const u32, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let r = (*(this as *const usize)).wrapping_add(i.wrapping_mul(0xa6ef));
    let lista = core::ptr::read_unaligned((r + 0xa6e3) as *const usize);
    // a chave: o id e um byte 0 (a 0x601a10 só compara o id)
    let chave = [*(pessoa as *const u32), 0u32];
    let achou = if lista == 0 { 0 } else { (buscar(chave.as_ptr(), lista, core::ptr::read_unaligned((r + 0xa6eb) as *const u32), 0x1e, 0x601a10) != 0) as u32 };
    achou // apelido
}

/// { 0x7a8780 } (thiscall, ret 4, player_regen.cpp): copia a aposentadoria pendente (0x1a bytes) de
/// `origem`: o id da pessoa (+0) e do clube (+4), as datas em +8 e +0x10 (0x417c10, com o retorno numa
/// local), o motivo (+0x18) e a marca de anunciada (+0x19). Devolve `this`.
unsafe extern "thiscall" fn retire_record_assign(this: usize, origem: usize) -> usize {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    *(this as *mut u32) = *(origem as *const u32);
    *((this + 4) as *mut u32) = *((origem + 4) as *const u32);
    copiar(this + 8, &mut local, origem + 8);
    copiar(this + 0x10, &mut local, origem + 0x10);
    *((this + 0x18) as *mut u8) = *((origem + 0x18) as *const u8); // motivo
    *((this + 0x19) as *mut u8) = *((origem + 0x19) as *const u8);
    this
}

unsafe extern "thiscall" fn comparar_is_fixture_watched(this: usize, partida: usize) -> u32 {
    // a 0x5fe290 só lê: pode rodar nos dois lados
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    if partida == 0 {
        return o(this, partida);
    }
    let r = o(this, partida);
    let n = is_fixture_watched(this, partida);
    PLACAR.record(r == n, || format!("is_fixture_watched original={} rust={}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_has_nickname(this: usize, pessoa: usize, humano: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%);
    if pessoa != 0 && matches!(human_index_for_nickname(this, humano), Some(None)) {
        return o(this, pessoa, humano);
    }
    let r = o(this, pessoa, humano);
    let n = has_nickname(this, pessoa, humano);
    PLACAR.record(r == n, || format!("has_nickname original={} rust={}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_retire_record_assign(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x1a)], || o(this, origem), || retire_record_assign(this, origem));
    PLACAR.record(igual && r == n, || format!("retire_record_assign {:x?} x {:x?}", mo, mr));
    r
}
