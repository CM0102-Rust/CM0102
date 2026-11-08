// ---------- %%LEVA%%a leva: os recordes do clube: achar ou criar os recordes de sempre e os da temporada e guardar a notícia de recorde batido ----------

unsafe fn cpk1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpk1_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A entrada do clube (0x6a bytes, pelo id +0 do clube) na tabela [this+0x1c]: +0 o índice dos recordes de sempre,
/// +4 o dos da temporada (−1 = ainda sem).
unsafe fn club_records_entry(this: usize, clube: usize) -> usize {
    cpk1_d(this + 0x1c).wrapping_add(cpk1_d(clube).wrapping_mul(0x6a))
}

/// A falta de memória das tabelas dos recordes (club_records.cpp, linha `linha`): caixa e fim, como a original.
unsafe fn records_out_of_memory(linha: u32) -> ! {
    show_error_at(0x98b6a0, linha);
    fatal_exit()
}

/// { 0x452710 } (thiscall, ret 8, club_records.cpp): `Club_Records::get_all_time_record(clube, criar)`: os recordes
/// de sempre do clube: o índice +0 da entrada dele ([this+0x1c] + id × 0x6a) no vetor [this+4] de 0x38b bytes. Sem
/// índice (−1) e com `criar`: o índice novo é a conta (+0xc, que sobe), o vetor cresce de 50 em 50 (capacidade +8,
/// realloc 0x945667; sem memória, caixa 0x1694 e fim) e o registro é reiniciado (0x452fe0). Devolve o registro, ou 0
/// sem índice. Clube nulo: a original (caixa 0x1680, 0).
unsafe extern "thiscall" fn get_all_time_record(this: usize, clube: usize, criar: u32) -> usize {
    if clube == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I0%%)(this, clube, criar);
    }
    let e = club_records_entry(this, clube);
    if cpk1_d(e) as i32 == -1 && criar != 0 {
        crate::comparacao::ramo("cpk_1 recordes de sempre novos");
        if cpk1_d(this + 0xc) == cpk1_d(this + 8) {
            let cap = cpk1_d(this + 8).wrapping_add(50);
            cpk1_por(this + 8, cap);
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
            let p = realocar(cpk1_d(this + 4), cap.wrapping_mul(0x38b) as u32);
            cpk1_por(this + 4, p);
            if p == 0 {
                records_out_of_memory(0x1694);
            }
        }
        let n = cpk1_d(this + 0xc);
        cpk1_por(e, n);
        cpk1_por(this + 0xc, n.wrapping_add(1)); // a conta dos recordes de sempre
        let reiniciar: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x452fe0usize);
        reiniciar(cpk1_d(this + 4).wrapping_add(cpk1_d(e).wrapping_mul(0x38b)));
    }
    let i = cpk1_d(e) as i32;
    if i <= -1 {
        return 0;
    }
    cpk1_d(this + 4).wrapping_add((i as usize).wrapping_mul(0x38b))
}

/// { 0x4528d0 } (thiscall, ret 8, club_records.cpp): `Club_Records::get_season_record(clube, criar)`: o mesmo para os
/// recordes da temporada: índice +4 da entrada, vetor [this+0x10], capacidade +0x14, conta +0x18 (sem memória, caixa
/// 0x16c9 e fim). Clube nulo: a original (caixa 0x16b5, 0).
unsafe extern "thiscall" fn get_season_record(this: usize, clube: usize, criar: u32) -> usize {
    if clube == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I1%%)(this, clube, criar);
    }
    let e = club_records_entry(this, clube) + 4;
    if cpk1_d(e) as i32 == -1 && criar != 0 {
        crate::comparacao::ramo("cpk_1 recordes da temporada novos");
        if cpk1_d(this + 0x18) == cpk1_d(this + 0x14) {
            let cap = cpk1_d(this + 0x14).wrapping_add(50);
            cpk1_por(this + 0x14, cap);
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
            let p = realocar(cpk1_d(this + 0x10), cap.wrapping_mul(0x38b) as u32);
            cpk1_por(this + 0x10, p);
            if p == 0 {
                records_out_of_memory(0x16c9);
            }
        }
        let n = cpk1_d(this + 0x18);
        cpk1_por(e, n);
        cpk1_por(this + 0x18, n.wrapping_add(1)); // a conta dos recordes da temporada
        let reiniciar: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x452fe0usize);
        reiniciar(cpk1_d(this + 0x10).wrapping_add(cpk1_d(e).wrapping_mul(0x38b)));
    }
    let i = cpk1_d(e) as i32;
    if i <= -1 {
        return 0;
    }
    cpk1_d(this + 0x10).wrapping_add((i as usize).wrapping_mul(0x38b))
}

/// { 0x452cb0 } (thiscall, ret 8, club_records.cpp): `Club_Records::queue_record_news(id, tipo)`: acrescenta (id do
/// clube +0, tipo do recorde +4; o byte +5 fica como está) ao vetor de 6 bytes [this+0x24] das notícias de recorde,
/// lidas pela 0x448e30. Cheio (conta +0x2c = capacidade +0x28): cresce de 5 em 5 (realloc 0x945667); sem memória,
/// caixa 0x174f e devolve 0 (o jogo segue, sem o vetor). Devolve 1.
unsafe extern "thiscall" fn queue_record_news(this: usize, id: u32, tipo: u32) -> u32 {
    crate::comparacao::ramo("cpk_1 noticia de recorde");
    if cpk1_d(this + 0x2c) == cpk1_d(this + 0x28) {
        let cap = cpk1_d(this + 0x28).wrapping_add(5);
        cpk1_por(this + 0x28, cap);
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let p = realocar(cpk1_d(this + 0x24), cap.wrapping_mul(6) as u32);
        cpk1_por(this + 0x24, p);
        if p == 0 {
            show_error_at(0x98b6a0, 0x174f);
            return 0;
        }
    }
    let n = cpk1_d(this + 0x2c);
    let item = cpk1_d(this + 0x24).wrapping_add(n.wrapping_mul(6));
    core::ptr::write_unaligned(item as *mut u32, id);
    *((item + 4) as *mut u8) = tipo as u8;
    cpk1_por(this + 0x2c, n.wrapping_add(1)); // mais uma notícia
    1
}

/// O plano dos recordes (de sempre ou da temporada) antes da original: o índice esperado (o novo é a conta), a conta
/// esperada e se o registro é novo.
unsafe fn record_index_plan(e: usize, conta: usize, criar: u32) -> (i32, usize, bool) {
    let i = cpk1_d(e) as i32;
    let n = cpk1_d(conta);
    if i == -1 && criar != 0 { (n as i32, n.wrapping_add(1), true) } else { (i, n, false) }
}

/// Confere o resultado da original com o plano: o índice da entrada, a conta, o registro devolvido e, se novo, as
/// marcas da 0x452fe0 (−1.0f em +0x1f1, −1 em +0x313).
unsafe fn check_record_plan(nome: &str, clube: usize, e: usize, conta: usize, vetor: usize, plano: (i32, usize, bool), r: usize) {
    let (i, n, novo) = plano;
    let esperado = if i <= -1 { 0 } else { cpk1_d(vetor).wrapping_add((i as usize).wrapping_mul(0x38b)) };
    let reiniciado = !novo || (r != 0 && cpk1_d(r + 0x1f1) == 0xbf800000 && cpk1_d(r + 0x313) as i32 == -1);
    PLACAR.registrar(r == esperado && cpk1_d(e) as i32 == i && cpk1_d(conta) == n && reiniciado, || {
        format!("{}(clube {}) índice {} (esperava {}), conta {} (esperava {}), registro {:x} (esperava {:x}), novo={} reiniciado={}",
            nome, cpk1_d(clube), cpk1_d(e) as i32, i, cpk1_d(conta), n, r, esperado, novo, reiniciado)
    });
}

/// Comparador: a criação realoca e não pode rodar duas vezes: o plano da cópia antes, a original roda, confere.
unsafe extern "thiscall" fn comparar_get_all_time_record(this: usize, clube: usize, criar: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I0%%);
    if clube == 0 {
        return o(this, clube, criar);
    }
    let e = club_records_entry(this, clube);
    let plano = record_index_plan(e, this + 0xc, criar);
    let r = o(this, clube, criar);
    check_record_plan("get_all_time_record", clube, e, this + 0xc, this + 4, plano, r);
    r
}

/// Comparador: o mesmo pelos recordes da temporada.
unsafe extern "thiscall" fn comparar_get_season_record(this: usize, clube: usize, criar: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I1%%);
    if clube == 0 {
        return o(this, clube, criar);
    }
    let e = club_records_entry(this, clube) + 4;
    let plano = record_index_plan(e, this + 0x18, criar);
    let r = o(this, clube, criar);
    check_record_plan("get_season_record", clube, e, this + 0x18, this + 0x10, plano, r);
    r
}

/// Comparador: a original acrescenta (pode realocar); confere a conta, o item novo (id, tipo) e a devolução.
unsafe extern "thiscall" fn comparar_queue_record_news(this: usize, id: u32, tipo: u32) -> u32 {
    let n = cpk1_d(this + 0x2c);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I2%%)(this, id, tipo);
    if r == 0 {
        return r; // sem memória: a caixa já saiu
    }
    let item = cpk1_d(this + 0x24).wrapping_add(n.wrapping_mul(6));
    let (gid, gtipo) = (core::ptr::read_unaligned(item as *const u32), *((item + 4) as *const u8));
    PLACAR.registrar(r == 1 && cpk1_d(this + 0x2c) == n.wrapping_add(1) && gid == id && gtipo == tipo as u8, || {
        format!("queue_record_news({}, {}) devolveu {}, conta {} -> {}, item ({}, {})", id, tipo as u8, r, n, cpk1_d(this + 0x2c), gid, gtipo)
    });
    r
}
