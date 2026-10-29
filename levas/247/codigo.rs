// ---------- 247a leva: os recordes do clube: achar ou criar os recordes de sempre e os da temporada e guardar a notícia de recorde batido ----------

unsafe fn cpk1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpk1_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A entrada do clube (0x6a bytes, pelo id +0 do clube) na tabela [this+0x1c]: +0 o índice dos recordes de sempre,
/// +4 o dos da temporada (−1 = ainda sem).
unsafe fn cpk1_entrada(this: usize, clube: usize) -> usize {
    cpk1_d(this + 0x1c).wrapping_add(cpk1_d(clube).wrapping_mul(0x6a))
}

/// A falta de memória das tabelas dos recordes (club_records.cpp, linha `linha`): caixa e fim, como a original.
unsafe fn cpk1_sem_memoria(linha: u32) -> ! {
    aviso_em(0x98b6a0, linha);
    erro_fatal()
}

/// { 0x452710 } (thiscall, ret 8, club_records.cpp): `Club_Records::get_all_time_record(clube, criar)`: os recordes
/// de sempre do clube: o índice +0 da entrada dele ([this+0x1c] + id × 0x6a) no vetor [this+4] de 0x38b bytes. Sem
/// índice (−1) e com `criar`: o índice novo é a conta (+0xc, que sobe), o vetor cresce de 50 em 50 (capacidade +8,
/// realloc 0x945667; sem memória, caixa 0x1694 e fim) e o registro é reiniciado (0x452fe0). Devolve o registro, ou 0
/// sem índice. Clube nulo: a original (caixa 0x1680, 0).
unsafe extern "thiscall" fn recordes_de_sempre(this: usize, clube: usize, criar: u32) -> usize {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(883)(this, clube, criar);
    }
    let e = cpk1_entrada(this, clube);
    if cpk1_d(e) as i32 == -1 && criar != 0 {
        crate::comparacao::ramo("cpk_1 recordes de sempre novos");
        if cpk1_d(this + 0xc) == cpk1_d(this + 8) {
            let cap = cpk1_d(this + 8).wrapping_add(50);
            cpk1_por(this + 8, cap);
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
            let p = realocar(cpk1_d(this + 4), cap.wrapping_mul(0x38b) as u32);
            cpk1_por(this + 4, p);
            if p == 0 {
                cpk1_sem_memoria(0x1694);
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
unsafe extern "thiscall" fn recordes_da_temporada(this: usize, clube: usize, criar: u32) -> usize {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(884)(this, clube, criar);
    }
    let e = cpk1_entrada(this, clube) + 4;
    if cpk1_d(e) as i32 == -1 && criar != 0 {
        crate::comparacao::ramo("cpk_1 recordes da temporada novos");
        if cpk1_d(this + 0x18) == cpk1_d(this + 0x14) {
            let cap = cpk1_d(this + 0x14).wrapping_add(50);
            cpk1_por(this + 0x14, cap);
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
            let p = realocar(cpk1_d(this + 0x10), cap.wrapping_mul(0x38b) as u32);
            cpk1_por(this + 0x10, p);
            if p == 0 {
                cpk1_sem_memoria(0x16c9);
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
unsafe extern "thiscall" fn guardar_noticia_de_recorde(this: usize, id: u32, tipo: u32) -> u32 {
    crate::comparacao::ramo("cpk_1 noticia de recorde");
    if cpk1_d(this + 0x2c) == cpk1_d(this + 0x28) {
        let cap = cpk1_d(this + 0x28).wrapping_add(5);
        cpk1_por(this + 0x28, cap);
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let p = realocar(cpk1_d(this + 0x24), cap.wrapping_mul(6) as u32);
        cpk1_por(this + 0x24, p);
        if p == 0 {
            aviso_em(0x98b6a0, 0x174f);
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
unsafe fn cpk1_plano(e: usize, conta: usize, criar: u32) -> (i32, usize, bool) {
    let i = cpk1_d(e) as i32;
    let n = cpk1_d(conta);
    if i == -1 && criar != 0 { (n as i32, n.wrapping_add(1), true) } else { (i, n, false) }
}

/// Confere o resultado da original com o plano: o índice da entrada, a conta, o registro devolvido e, se novo, as
/// marcas da 0x452fe0 (−1.0f em +0x1f1, −1 em +0x313).
unsafe fn cpk1_conferir(nome: &str, clube: usize, e: usize, conta: usize, vetor: usize, plano: (i32, usize, bool), r: usize) {
    let (i, n, novo) = plano;
    let esperado = if i <= -1 { 0 } else { cpk1_d(vetor).wrapping_add((i as usize).wrapping_mul(0x38b)) };
    let reiniciado = !novo || (r != 0 && cpk1_d(r + 0x1f1) == 0xbf800000 && cpk1_d(r + 0x313) as i32 == -1);
    PLACAR.registrar(r == esperado && cpk1_d(e) as i32 == i && cpk1_d(conta) == n && reiniciado, || {
        format!("{}(clube {}) índice {} (esperava {}), conta {} (esperava {}), registro {:x} (esperava {:x}), novo={} reiniciado={}",
            nome, cpk1_d(clube), cpk1_d(e) as i32, i, cpk1_d(conta), n, r, esperado, novo, reiniciado)
    });
}

/// Comparador: a criação realoca e não pode rodar duas vezes: o plano da cópia antes, a original roda, confere.
unsafe extern "thiscall" fn comparar_recordes_de_sempre(this: usize, clube: usize, criar: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(883);
    if clube == 0 {
        return o(this, clube, criar);
    }
    let e = cpk1_entrada(this, clube);
    let plano = cpk1_plano(e, this + 0xc, criar);
    let r = o(this, clube, criar);
    cpk1_conferir("recordes_de_sempre", clube, e, this + 0xc, this + 4, plano, r);
    r
}

/// Comparador: o mesmo pelos recordes da temporada.
unsafe extern "thiscall" fn comparar_recordes_da_temporada(this: usize, clube: usize, criar: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(884);
    if clube == 0 {
        return o(this, clube, criar);
    }
    let e = cpk1_entrada(this, clube) + 4;
    let plano = cpk1_plano(e, this + 0x18, criar);
    let r = o(this, clube, criar);
    cpk1_conferir("recordes_da_temporada", clube, e, this + 0x18, this + 0x10, plano, r);
    r
}

/// Comparador: a original acrescenta (pode realocar); confere a conta, o item novo (id, tipo) e a devolução.
unsafe extern "thiscall" fn comparar_guardar_noticia_de_recorde(this: usize, id: u32, tipo: u32) -> u32 {
    let n = cpk1_d(this + 0x2c);
    let r = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(885)(this, id, tipo);
    if r == 0 {
        return r; // sem memória: a caixa já saiu
    }
    let item = cpk1_d(this + 0x24).wrapping_add(n.wrapping_mul(6));
    let (gid, gtipo) = (core::ptr::read_unaligned(item as *const u32), *((item + 4) as *const u8));
    PLACAR.registrar(r == 1 && cpk1_d(this + 0x2c) == n.wrapping_add(1) && gid == id && gtipo == tipo as u8, || {
        format!("guardar_noticia_de_recorde({}, {}) devolveu {}, conta {} -> {}, item ({}, {})", id, tipo as u8, r, n, cpk1_d(this + 0x2c), gid, gtipo)
    });
    r
}

// ---------- 247a leva: as listas do regen: a reserva de técnico, a aposentadoria pendente e a vaga para o regen ----------

unsafe fn cpk2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpk2_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpk2_por16(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

unsafe fn cpk2_realocar(p: usize, tam: usize) -> usize {
    let f: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    f(p, tam as u32)
}

/// A falta de memória das listas do regen (player_regen.cpp, linha `linha`): caixa e fim, como a original.
unsafe fn cpk2_sem_memoria(linha: u32) -> ! {
    aviso_em(0xa6bf9c, linha);
    erro_fatal()
}

/// { 0x7a81a0 } (thiscall, ret 0xc, player_regen.cpp): `PLAYER_REGEN::add_spare_non_player_record(id, nação,
/// segunda)`: acrescenta (id +0, nação +4, segunda nação +8) à lista de 16 bytes das reservas de não jogador
/// ([this+0x1c], conta +0x20, capacidade +0x24; tirada pela 0x7a82b0). O original decide crescer pelo banco de
/// jogadores livres (conta +0x40 = capacidade +0x44 − 1), não por esta lista: fiel. Cresce 250 (realloc 0x945667;
/// sem memória, caixa 0x9c4 e fim).
unsafe extern "thiscall" fn acrescentar_reserva_de_tecnico(this: usize, id: u32, nacao: u32, segunda: u32) {
    crate::comparacao::ramo("cpk_2 reserva de tecnico");
    if cpk2_d(this + 0x40) == cpk2_d(this + 0x44).wrapping_sub(1) {
        let p = cpk2_realocar(cpk2_d(this + 0x1c), cpk2_d(this + 0x24).wrapping_add(250) << 4);
        cpk2_por(this + 0x1c, p);
        cpk2_por(this + 0x24, cpk2_d(this + 0x24).wrapping_add(250));
        if p == 0 {
            cpk2_sem_memoria(0x9c4);
        }
    }
    let n = cpk2_d(this + 0x20);
    let item = cpk2_d(this + 0x1c).wrapping_add(n << 4);
    cpk2_por(item, id as usize);
    cpk2_por(item + 4, nacao as usize);
    cpk2_por(item + 8, segunda as usize);
    cpk2_por(this + 0x20, n.wrapping_add(1)); // mais uma reserva
}

/// { 0x7a8510 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::add_retirement(registro)`: acrescenta à tabela de
/// 26 bytes das aposentadorias pendentes ([this+4], conta +8, capacidade +0xc) os campos +0, +4 (dwords), +0x18
/// (byte), +8, +0xa, +0x10, +0x12 (palavras) e +0x19 (byte) do registro, nessa ordem (+0xc..+0x10 e +0x14..+0x18
/// ficam como estão). Conta = capacidade − 1: cresce 250 (realloc 0x945667; sem memória, caixa 0xa53 e fim).
unsafe extern "thiscall" fn acrescentar_aposentadoria(this: usize, r: usize) {
    crate::comparacao::ramo("cpk_2 aposentadoria");
    let cap = cpk2_d(this + 0xc);
    if cpk2_d(this + 8) == cap.wrapping_sub(1) {
        let p = cpk2_realocar(cpk2_d(this + 4), cap.wrapping_add(250).wrapping_mul(26));
        cpk2_por(this + 4, p);
        cpk2_por(this + 0xc, cpk2_d(this + 0xc).wrapping_add(250));
        if p == 0 {
            cpk2_sem_memoria(0xa53);
        }
    }
    let item = || cpk2_d(this + 4).wrapping_add(cpk2_d(this + 8).wrapping_mul(26));
    cpk2_por(item(), cpk2_d(r));
    cpk2_por(item() + 4, cpk2_d(r + 4));
    *((item() + 0x18) as *mut u8) = *((r + 0x18) as *const u8);
    for o in [8usize, 0xa, 0x10, 0x12] {
        cpk2_por16(item() + o, core::ptr::read_unaligned((r + o) as *const u16));
    }
    *((item() + 0x19) as *mut u8) = *((r + 0x19) as *const u8);
    cpk2_por(this + 8, cpk2_d(this + 8).wrapping_add(1)); // mais uma aposentadoria
}

/// { 0x7a87d0 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::queue_staff_for_regen(entrada)`: a pessoa de id
/// +0 da entrada (registro de 0x6e bytes em [0xae23bc]) entra na fila do regen (12 bytes em [this+0x10], conta
/// +0x14, capacidade +0x18: id +0, palavras +4 e +6 da entrada). Com dados de não jogador (+0x69): na lista de 16
/// bytes [this+0x1c] (conta +0x20, capacidade +0x24) o id deles +0, a nação (+0x1a; sem nação, um sorteio
/// 0x90cfc0 entre as [0xae2350]) +4 e o id da pessoa +0xc. Com dados de jogador (+0x61): no banco de jogadores
/// livres [this+0x3c] (conta +0x40, capacidade +0x44) o id deles +0, o da nação +4 (sem conferir nula, como a
/// original) e o da pessoa +0xc. Cada lista cresce 250 quando conta = capacidade − 1 (realloc 0x945667; caixas 0xaac,
/// 0xac5, 0xae9 e fim; na terceira o original confere a conta, não o ponteiro: fiel). Id fora de 0..=[0xae2364]: a
/// original (caixa 0xa9d).
unsafe extern "thiscall" fn guardar_vaga_para_regen(this: usize, entrada: usize) {
    let id = cpk2_d(entrada) as i32;
    if id < 0 || id > *(0xae2364 as *const i32) {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(888)(this, entrada);
    }
    crate::comparacao::ramo("cpk_2 vaga para o regen");
    let pessoa = cpk2_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    let cap = cpk2_d(this + 0x18);
    if cpk2_d(this + 0x14) == cap.wrapping_sub(1) {
        let p = cpk2_realocar(cpk2_d(this + 0x10), cap.wrapping_add(250).wrapping_mul(12));
        cpk2_por(this + 0x10, p);
        cpk2_por(this + 0x18, cpk2_d(this + 0x18).wrapping_add(250));
        if p == 0 {
            cpk2_sem_memoria(0xaac);
        }
    }
    let fila = cpk2_d(this + 0x10).wrapping_add(cpk2_d(this + 0x14).wrapping_mul(12));
    cpk2_por(fila, id as usize);
    cpk2_por16(fila + 4, core::ptr::read_unaligned((entrada + 4) as *const u16));
    cpk2_por16(fila + 6, core::ptr::read_unaligned((entrada + 6) as *const u16));
    cpk2_por(this + 0x14, cpk2_d(this + 0x14).wrapping_add(1)); // mais uma na fila do regen
    if cpk2_d(pessoa + 0x69) != 0 {
        if cpk2_d(this + 0x20) == cpk2_d(this + 0x24).wrapping_sub(1) {
            let p = cpk2_realocar(cpk2_d(this + 0x1c), cpk2_d(this + 0x24).wrapping_add(250) << 4);
            cpk2_por(this + 0x1c, p);
            cpk2_por(this + 0x24, cpk2_d(this + 0x24).wrapping_add(250));
            if p == 0 {
                cpk2_sem_memoria(0xac5);
            }
        }
        let item = || cpk2_d(this + 0x1c).wrapping_add(cpk2_d(this + 0x20) << 4);
        cpk2_por(item(), cpk2_d(cpk2_d(pessoa + 0x69)));
        let nacao = cpk2_d(pessoa + 0x1a);
        if nacao != 0 {
            cpk2_por(item() + 4, cpk2_d(nacao));
        } else {
            let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
            let s = sorteio(*(0xae2350 as *const i32));
            cpk2_por(item() + 4, s as usize);
        }
        cpk2_por(item() + 0xc, cpk2_d(pessoa));
        cpk2_por(this + 0x20, cpk2_d(this + 0x20).wrapping_add(1));
    }
    if cpk2_d(pessoa + 0x61) != 0 {
        let cap = cpk2_d(this + 0x44);
        if cpk2_d(this + 0x40) == cap.wrapping_sub(1) {
            let p = cpk2_realocar(cpk2_d(this + 0x3c), cap.wrapping_add(250) << 4);
            cpk2_por(this + 0x3c, p);
            cpk2_por(this + 0x44, cpk2_d(this + 0x44).wrapping_add(250));
            if cpk2_d(this + 0x40) == 0 {
                cpk2_sem_memoria(0xae9); // a original confere a conta (+0x40), não o ponteiro
            }
        }
        let item = || cpk2_d(this + 0x3c).wrapping_add(cpk2_d(this + 0x40) << 4);
        cpk2_por(item(), cpk2_d(cpk2_d(pessoa + 0x61)));
        cpk2_por(item() + 4, cpk2_d(cpk2_d(pessoa + 0x1a)));
        cpk2_por(item() + 0xc, cpk2_d(pessoa));
        cpk2_por(this + 0x40, cpk2_d(this + 0x40).wrapping_add(1));
    }
}

/// Comparador: a original acrescenta (pode realocar); confere a conta e o item novo.
unsafe extern "thiscall" fn comparar_acrescentar_reserva_de_tecnico(this: usize, id: u32, nacao: u32, segunda: u32) {
    let n = cpk2_d(this + 0x20);
    orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(886)(this, id, nacao, segunda);
    let item = cpk2_d(this + 0x1c).wrapping_add(n << 4);
    let g = [cpk2_d(item) as u32, cpk2_d(item + 4) as u32, cpk2_d(item + 8) as u32];
    PLACAR.registrar(cpk2_d(this + 0x20) == n.wrapping_add(1) && g == [id, nacao, segunda], || {
        format!("acrescentar_reserva_de_tecnico({}, {}, {}) conta {} -> {}, item {:?}", id, nacao, segunda, n, cpk2_d(this + 0x20), g)
    });
}

/// Os 26 bytes que a 0x7a8510 copia do registro (os outros, zerados).
unsafe fn cpk2_campos_da_aposentadoria(r: usize) -> [u8; 26] {
    let mut c = [0u8; 26];
    for (o, n) in [(0usize, 4usize), (4, 4), (8, 2), (0xa, 2), (0x10, 2), (0x12, 2), (0x18, 1), (0x19, 1)] {
        c[o..o + n].copy_from_slice(core::slice::from_raw_parts((r + o) as *const u8, n));
    }
    c
}

/// Comparador: o registro copiado antes; a original acrescenta (pode realocar); confere a conta e os campos.
unsafe extern "thiscall" fn comparar_acrescentar_aposentadoria(this: usize, r: usize) {
    let n = cpk2_d(this + 8);
    let esperado = cpk2_campos_da_aposentadoria(r);
    orig::<unsafe extern "thiscall" fn(usize, usize)>(887)(this, r);
    let item = cpk2_d(this + 4).wrapping_add(n.wrapping_mul(26));
    let g = cpk2_campos_da_aposentadoria(item);
    PLACAR.registrar(cpk2_d(this + 8) == n.wrapping_add(1) && g == esperado, || {
        format!("acrescentar_aposentadoria conta {} -> {}, item {:x?} (esperava {:x?})", n, cpk2_d(this + 8), g, esperado)
    });
}

/// Comparador: a original sorteia e realoca (não pode rodar duas vezes): o plano antes (o que entra em cada lista);
/// confere as contas e os itens (a nação sorteada só pela faixa).
unsafe extern "thiscall" fn comparar_guardar_vaga_para_regen(this: usize, entrada: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(888);
    let id = cpk2_d(entrada) as i32;
    if id < 0 || id > *(0xae2364 as *const i32) {
        return o(this, entrada);
    }
    let pessoa = cpk2_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    let (n1, n2, n3) = (cpk2_d(this + 0x14), cpk2_d(this + 0x20), cpk2_d(this + 0x40));
    let datas = cpk2_d(entrada + 4) as u32;
    let np = cpk2_d(pessoa + 0x69);
    let jp = cpk2_d(pessoa + 0x61);
    let nac = cpk2_d(pessoa + 0x1a);
    let reserva = (np != 0).then(|| (cpk2_d(np) as u32, (nac != 0).then(|| cpk2_d(nac) as u32), cpk2_d(pessoa) as u32));
    let banco = (jp != 0).then(|| (cpk2_d(jp) as u32, cpk2_d(nac) as u32, cpk2_d(pessoa) as u32));
    o(this, entrada);
    let fila = cpk2_d(this + 0x10).wrapping_add(n1.wrapping_mul(12));
    let mut ok = cpk2_d(this + 0x14) == n1 + 1 && cpk2_d(fila) as i32 == id && cpk2_d(fila + 4) as u32 == datas;
    ok &= cpk2_d(this + 0x20) == n2 + reserva.is_some() as usize;
    if let Some((rid, rnac, rp)) = reserva {
        let i = cpk2_d(this + 0x1c).wrapping_add(n2 << 4);
        let gn = cpk2_d(i + 4) as u32;
        ok &= cpk2_d(i) as u32 == rid && cpk2_d(i + 0xc) as u32 == rp
            && rnac.map_or(gn < (*(0xae2350 as *const i32)).max(1) as u32, |v| gn == v);
    }
    ok &= cpk2_d(this + 0x40) == n3 + banco.is_some() as usize;
    if let Some((bid, bnac, bp)) = banco {
        let i = cpk2_d(this + 0x3c).wrapping_add(n3 << 4);
        ok &= cpk2_d(i) as u32 == bid && cpk2_d(i + 4) as u32 == bnac && cpk2_d(i + 0xc) as u32 == bp;
    }
    PLACAR.registrar(ok, || {
        format!("guardar_vaga_para_regen({}) contas {} {} {} -> {} {} {}, reserva {:?}, banco {:?}", id, n1, n2, n3,
            cpk2_d(this + 0x14), cpk2_d(this + 0x20), cpk2_d(this + 0x40), reserva, banco)
    });
}

// ---------- 247a leva: contratos e suspensão: as cláusulas de rebaixamento do clube rebaixado, tirar a oferta de contrato da pessoa ao clube e se o jogador está suspenso para a partida ----------

unsafe fn cpk3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O contrato (registro de 80 bytes em [this], pelo índice +0 do registro de 79 bytes em [0xae2d0c]) da pessoa `p`
/// com o clube `clube` (+4 = id do clube) e a cláusula de rebaixamento ainda por acionar (+0x1f = 1); senão 0.
unsafe fn cpk3_clausula(this: usize, clube: usize, p: usize) -> usize {
    if p == 0 || cpk3_d(p) as i32 >= cpk3_d(0xae2364) as i32 {
        return 0;
    }
    let i = cpk3_d(cpk3_d(0xae2d0c).wrapping_add(cpk3_d(p).wrapping_mul(79))) as i32;
    if i < 0 {
        return 0;
    }
    let c = (i as usize).wrapping_mul(80).wrapping_add(cpk3_d(this));
    if c == 0 || cpk3_d(c + 4) != cpk3_d(clube) || *((c + 0x1f) as *const u8) != 1 {
        return 0;
    }
    c
}

/// O clube e o parceiro (time A/B, 0x540a50 ativo): o clube B não aciona (o A cuida dos dois). Clube nulo ou B:
/// nenhum.
unsafe fn cpk3_clubes(clube: usize) -> [usize; 2] {
    if clube == 0 {
        return [0, 0];
    }
    let mut eh_a = 0u32;
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let p = parceiro(clube, &mut eh_a, 1);
    if p != 0 && eh_a == 0 {
        return [0, 0];
    }
    [clube, p]
}

/// { 0x4cd030 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::trigger_relegation_clauses(clube,
/// liga)`: no clube rebaixado e no parceiro (time A/B), cada um dos 50 da comissão e elenco (+0xd7) com a cláusula de
/// rebaixamento por acionar no contrato com o clube (+0x1f = 1) passa a 2 e sai a notícia (0x8e3db0 sobre
/// [0xde1f54] com o tipo 3).
unsafe extern "thiscall" fn acionar_clausulas_de_rebaixamento(this: usize, clube: usize, liga: usize) {
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize) -> u32 = core::mem::transmute(0x8e3db0usize);
    for c in cpk3_clubes(clube) {
        if c == 0 {
            continue;
        }
        for k in 0..50 {
            let p = cpk3_d(c + 0xd7 + k * 4);
            let contrato = cpk3_clausula(this, c, p);
            if contrato != 0 {
                crate::comparacao::ramo("cpk_3 clausula de rebaixamento");
                *((contrato + 0x1f) as *mut u8) = 2; // a cláusula acionada
                noticia(cpk3_d(0xde1f54), p, liga, 3, contrato);
            }
        }
    }
}

/// A primeira oferta de contrato da lista `l` (sem mexer no cursor) da pessoa `p` (+0) ao clube `c` (+4).
unsafe fn cpk3_oferta(l: usize, p: usize, c: usize) -> Option<usize> {
    cpd1_itens(l).into_iter().find(|&e| cpk3_d(e) == p && cpk3_d(e + 4) == c)
}

/// { 0x4cca70 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract_offer(pessoa, clube)`: a
/// primeira oferta da lista (+0x34) da pessoa ao clube (ids): devolve a reserva de registro (+0x63 ≠ −1: 0x7a81a0
/// sobre [0xb7434c] com +0x67, +0x6b), tira a marca de contrato acertado se ela estava acertada (+0x50 = 2: 0x8c5430),
/// sai da lista (0x68d1f0) e as pendentes da pessoa (+8 do registro de 79 bytes em [0xae2d0c]) descem uma.
unsafe extern "thiscall" fn tirar_oferta_de_contrato(this: usize, pessoa: *const u32, clube: *const u32) {
    let (c, p) = (*clube as usize, *pessoa as usize);
    let l = cpk3_d(this + 0x34);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpk3_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut e = proximo(l);
    while e != 0 && !(cpk3_d(e) == p && cpk3_d(e + 4) == c) {
        e = proximo(l);
    }
    if e == 0 {
        return;
    }
    crate::comparacao::ramo("cpk_3 tirar oferta de contrato");
    let reserva = cpk3_d(e + 0x63);
    if reserva as i32 != -1 {
        let devolver: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x7a81a0usize);
        devolver(cpk3_d(0xb7434c), reserva, cpk3_d(e + 0x67), cpk3_d(e + 0x6b));
    }
    if *((e + 0x50) as *const u8) == 2 {
        let desmarcar: unsafe extern "thiscall" fn(usize, *const u32) = core::mem::transmute(0x8c5430usize);
        desmarcar(cpk3_d(0xde1f54), pessoa);
    }
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    tirar(l, e);
    let n = (cpk3_d(0xae2d0c).wrapping_add((*pessoa as usize).wrapping_mul(79)) + 8) as *mut u8;
    *n = (*n).wrapping_sub(1); // uma pendente a menos
}

/// { 0x55d270 } (thiscall, ret 0x10, discipline.cpp): `Discipline::is_player_suspended(jogador, competição, extra,
/// partida)`: jogador fora das pessoas ([0xae2364]) ou competição (id em `*comp`, vetor [0xadadfc]) inexistente: 0.
/// Suspenso pelo tribunal geral ([[this]], 0x7e85b0 com 1): 1. Senão, pelo tribunal da competição (+0x50, ≠ −1, com
/// 0): 1, e se o clube dele (+0x39) é de humano (0x600a50 com 0, 1, 0, 0), ele sai da vaga que ocupa entre as 20 de
/// +0x1e0. Senão 0.
unsafe extern "thiscall" fn esta_suspenso(this: usize, jogador: usize, comp: *const i32, extra: u32, partida: usize) -> u32 {
    if jogador != 0 && cpk3_d(jogador) as i32 >= cpk3_d(0xae2364) as i32 {
        return 0;
    }
    let v = cpk3_d(0xadadfc);
    if v == 0 {
        return 0;
    }
    let c = cpk3_d(v.wrapping_add((*comp as usize).wrapping_mul(4)));
    if c == 0 {
        return 0;
    }
    let julgar: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize) -> u32 = core::mem::transmute(0x7e85b0usize);
    let tribunais = cpk3_d(this);
    if julgar(cpk3_d(tribunais), jogador, 1, extra, partida) != 0 {
        return 1; // a suspensão geral
    }
    let t = *((c + 0x50) as *const i8);
    if t == -1 {
        return 0;
    }
    let b = cpk3_d(tribunais.wrapping_add((t as isize as usize).wrapping_mul(4)));
    if b == 0 || julgar(b, jogador, 0, extra, partida) == 0 { // sem a suspensão da competição
        return 0;
    }
    crate::comparacao::ramo("cpk_3 suspenso pela competicao");
    let clube = cpk3_d(jogador + 0x39);
    if clube != 0 {
        let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
        if humano(clube, 0, 1, 0, 0) != 0 {
            if let Some(s) = (0..20).map(|k| clube + 0x1e0 + k * 4).find(|&s| cpk3_d(s) == jogador) {
                *(s as *mut usize) = 0; // sai da vaga
            }
        }
    }
    1
}

/// Comparador: os contratos que a cópia acionaria (antes); a original aciona e dá as notícias (não roda duas vezes);
/// confere que todos ficaram com 2.
unsafe extern "thiscall" fn comparar_acionar_clausulas_de_rebaixamento(this: usize, clube: usize, liga: usize) {
    let mut alvos = Vec::new();
    for c in cpk3_clubes(clube) {
        if c != 0 {
            alvos.extend((0..50).map(|k| cpk3_clausula(this, c, cpk3_d(c + 0xd7 + k * 4))).filter(|&x| x != 0));
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(889)(this, clube, liga);
    let fora: Vec<usize> = alvos.iter().copied().filter(|&x| *((x + 0x1f) as *const u8) != 2).collect();
    PLACAR.registrar(fora.is_empty(), || format!("acionar_clausulas_de_rebaixamento({:x}) {} contratos, sem acionar {:x?}", clube, alvos.len(), fora));
}

/// Comparador: a lista esperada (sem a primeira oferta da pessoa ao clube) e as pendentes antes; a original tira (e
/// libera o item: nada dele é lido depois); confere os itens e o byte.
unsafe extern "thiscall" fn comparar_tirar_oferta_de_contrato(this: usize, pessoa: *const u32, clube: *const u32) {
    let l = cpk3_d(this + 0x34);
    if l == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, *const u32, *const u32)>(890)(this, pessoa, clube);
    }
    let n = (cpk3_d(0xae2d0c).wrapping_add((*pessoa as usize).wrapping_mul(79)) + 8) as *const u8;
    let alvo = cpk3_oferta(l, *pessoa as usize, *clube as usize);
    let esperado: Vec<usize> = cpd1_itens(l).into_iter().filter(|&e| Some(e) != alvo).collect();
    let pendentes = if alvo.is_some() { (*n).wrapping_sub(1) } else { *n };
    orig::<unsafe extern "thiscall" fn(usize, *const u32, *const u32)>(890)(this, pessoa, clube);
    let depois = cpd1_itens(l);
    PLACAR.registrar(depois == esperado && *n == pendentes, || format!("tirar_oferta_de_contrato({}, {}) itens {} (esperava {}), pendentes {} (esperava {})", *pessoa, *clube, depois.len(), esperado.len(), *n, pendentes));
}

/// Comparador: lado a lado pelas 20 vagas (+0x1e0) do clube do jogador (a 0x7e85b0 e a 0x600a50 só leem).
unsafe extern "thiscall" fn comparar_esta_suspenso(this: usize, jogador: usize, comp: *const i32, extra: u32, partida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32, usize) -> u32>(891);
    let clube = if jogador != 0 { cpk3_d(jogador + 0x39) } else { 0 };
    let regioes: &[(usize, usize)] = if clube != 0 { &[(clube + 0x1e0, 80)] } else { &[] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(regioes, || o(this, jogador, comp, extra, partida), || esta_suspenso(this, jogador, comp, extra, partida));
    PLACAR.registrar(igual && r == n, || format!("esta_suspenso({:x}, {}) original={} rust={} {:x?} x {:x?}", jogador, *comp, r, n, mo, mr));
    r
}

// ---------- 247a leva: as regras da partida e o amistoso: o alcance das regras de escalação da partida, o motivo de um jogador não poder jogar e a vaga do amistoso marcado ----------

unsafe fn cpk4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x8d2fa0 } (thiscall, ret 0x10, transfer_manager.cpp): `TRANSFER_MANAGER::get_fixture_rules_scope(partida, tipo,
/// nação, continente)`: competição da partida (+0x14) de reservas ([0x9cf684]): a nação do mandante (+0x1c, +0x53), o
/// continente dela (+0x71, ou 0) e tipo 1. Amistoso (0x5c6710 sobre [0xae2c74]): 0, 0 e 0xff. Senão, pela nação-chave
/// da competição (0x669470, com sinal): a nação da tabela 0xb63d60 (72 bytes cada), o continente da competição (+0x59)
/// e tipo 4 (chaves 0 e 1), 2 (3 a 7) ou 1.
unsafe extern "thiscall" fn escopo_das_regras_da_partida(_this: usize, partida: usize, tipo: *mut u8, nacao: *mut usize, continente: *mut usize) {
    let comp = cpk4_d(partida + 0x14);
    if cpk4_d(comp) == cpk4_d(0x9cf684) {
        let n = cpk4_d(cpk4_d(partida + 0x1c) + 0x53);
        *nacao = n;
        *continente = if n != 0 { cpk4_d(n + 0x71) } else { 0 };
        *tipo = 1;
        return;
    }
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    if amistoso(cpk4_d(0xae2c74), comp) != 0 {
        *nacao = 0;
        *continente = 0;
        *tipo = 0xff;
        return;
    }
    let chave: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669470usize);
    let k = chave(comp) as i8;
    *nacao = cpk4_d((0xb63d60isize + k as isize * 72) as usize);
    *continente = cpk4_d(cpk4_d(partida + 0x14) + 0x59);
    *tipo = match k {
        0 | 1 => 4,
        3..=7 => 2,
        _ => {
            crate::comparacao::ramo("cpk_4 escopo da nacao");
            1 // a nação da competição
        }
    };
}

/// { 0x8cf5e0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::get_ineligibility_code(jogador, partida)`:
/// mandante (+0x1c) que não é seleção (0x53b900) e jogador ausente sem licença (AWOL: 0x418f60 sobre [0xad9c6c], com 0,
/// que tira a marca de quem está sem clube): 9. Senão, pelo alcance das regras (0x8d2fa0; 0xff e 4: 0): a primeira das
/// 28 regras de +0x8ac do continente (+0xe, tipo 2) ou da nação (+0xa) com o mesmo tipo (+0x12) responde (virtual
/// +0x18 com jogador e partida). Nenhuma: 0.
unsafe extern "thiscall" fn codigo_de_inelegibilidade(this: usize, jogador: usize, partida: usize) -> u8 {
    if partida != 0 {
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        if selecao(cpk4_d(partida + 0x1c)) == 0 {
            let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x418f60usize);
            if ausente(cpk4_d(0xad9c6c), jogador, 0) != 0 {
                return 9;
            }
        }
    }
    let (mut tipo, mut nacao, mut continente) = (0u8, 0usize, 0usize);
    let escopo: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut usize, *mut usize) = core::mem::transmute(0x8d2fa0usize);
    escopo(this, partida, &mut tipo, &mut nacao, &mut continente);
    if tipo == 0xff || tipo == 4 {
        return 0;
    }
    for k in 0..28 {
        let r = cpk4_d(this + 0x8ac + k * 4);
        let casa = if tipo == 2 {
            cpk4_d(r + 0xe) == continente && *((r + 0x12) as *const u8) == 2
        } else {
            cpk4_d(r + 0xa) == nacao && *((r + 0x12) as *const u8) == tipo // a regra da nação
        };
        if casa {
            crate::comparacao::ramo("cpk_4 regra da partida");
            let responder: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(cpk4_d(cpk4_d(r) + 0x18));
            return responder(r, jogador, partida);
        }
    }
    0
}

/// { 0x5c6740 } (thiscall, ret 0x10, friendly.cpp): `Friendly::find_scheduled_friendly_slot(dia, ano, clube,
/// adversário)`: na lista dos amistosos (+0x34cc; o cursor volta ao começo), o primeiro registro do clube (+0) com uma
/// vaga (25 bytes a partir de +9, [+8] delas, com sinal) do dia (+0), ano (+2) e adversário (+0x10): a vaga. Senão 0.
unsafe extern "thiscall" fn vaga_do_amistoso(this: usize, dia: u32, ano: u32, clube: usize, adversario: usize) -> usize {
    // ponytail: a Data local da original (0x549ef0) só guarda o dia e o ano em 16 bits (o bissexto não é lido)
    let (dia, ano) = (dia as u16, ano as u16);
    let l = cpk4_d(this + 0x34cc);
    if l == 0 {
        return 0;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpk4_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut e = proximo(l);
    while e != 0 {
        if cpk4_d(e) == clube {
            for i in 0..(*((e + 8) as *const i8)).max(0) as usize {
                let v = e + 25 * i + 9;
                if *((v + 2) as *const u16) == ano && *(v as *const u16) == dia && cpk4_d(v + 0x10) == adversario {
                    crate::comparacao::ramo("cpk_4 vaga do amistoso");
                    return e + 25 * i + 9; // a vaga i
                }
            }
        }
        e = proximo(l);
    }
    0
}

/// Comparador: lado a lado pelas três saídas (a 0x669470 e a 0x5c6710 só leem).
unsafe extern "thiscall" fn comparar_escopo_das_regras_da_partida(this: usize, partida: usize, tipo: *mut u8, nacao: *mut usize, continente: *mut usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut usize, *mut usize)>(892);
    let regioes = [(tipo as usize, 1), (nacao as usize, 4), (continente as usize, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, partida, tipo, nacao, continente), || escopo_das_regras_da_partida(this, partida, tipo, nacao, continente));
    PLACAR.registrar(igual, || format!("escopo_das_regras_da_partida({:x}) {:x?} x {:x?}", partida, mo, mr));
}

/// Comparador: pelo valor. Jogador com a marca de ausente (0x418f60 com 1, que só lê): só a original (com 0 ela tira
/// a marca de quem está sem clube e a segunda chamada veria outro estado).
unsafe extern "thiscall" fn comparar_codigo_de_inelegibilidade(this: usize, jogador: usize, partida: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(893);
    let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x418f60usize);
    if ausente(cpk4_d(0xad9c6c), jogador, 1) != 0 {
        return o(this, jogador, partida);
    }
    let r = o(this, jogador, partida);
    let n = codigo_de_inelegibilidade(this, jogador, partida);
    PLACAR.registrar(r == n, || format!("codigo_de_inelegibilidade({:x}, {:x}) original={} rust={}", jogador, partida, r, n));
    r
}

/// Comparador: pelo valor (as duas voltam o cursor da lista ao começo e param no mesmo registro).
unsafe extern "thiscall" fn comparar_vaga_do_amistoso(this: usize, dia: u32, ano: u32, clube: usize, adversario: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize>(894)(this, dia, ano, clube, adversario);
    let n = vaga_do_amistoso(this, dia, ano, clube, adversario);
    PLACAR.registrar(r == n, || format!("vaga_do_amistoso({}/{}, {:x}, {:x}) original={:#x} rust={:#x}", dia as u16, ano as u16, clube, adversario, r, n));
    r
}

// ---------- 247a leva: as notícias para os técnicos humanos de um continente e da região da competição, e se a notícia é importante para a pessoa ----------

unsafe fn cpk5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x786640 } (thiscall, ret 8, news.cpp): `News::send_to_continent_managers(noticia, continente)`: alcance do
/// continente (+0xde = 3) e a entrega (0x785d40 com 0) a cada técnico humano ativo cujo clube (+0x39) tem a nação
/// (+0x53) no `continente` (+0x71) ou cuja nação (+0x1a) está nele. Devolve 1, ou 0 na primeira recusada. Notícia ou
/// continente nulos: a original (caixa "Error").
unsafe extern "thiscall" fn noticia_para_os_tecnicos_do_continente(this: usize, noticia: usize, continente: usize) -> u32 {
    if noticia == 0 || continente == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(895)(this, noticia, continente); // caixa: só a original
    }
    cpj3_entregar(this, noticia, 3, |p| {
        let c = cpk5_d(p + 0x39);
        if c != 0 {
            let n = cpk5_d(c + 0x53);
            if n != 0 && cpk5_d(n + 0x71) == continente { // o continente da nação do clube
                crate::comparacao::ramo("cpk_5 continente pelo clube");
                return true;
            }
        }
        let s = cpk5_d(p + 0x1a);
        s != 0 && cpk5_d(s + 0x71) == continente // o continente da nação da pessoa
    }) as u32
}

/// { 0x786860 } (thiscall, ret 8, news.cpp): `News::send_to_comp_region(noticia, comp)`: com a nação da competição
/// (+0x5d), para os técnicos da nação (0x786520); senão, com o continente (+0x59), para os do continente (0x786640);
/// senão para todos (0x786780). Devolve o que a entrega devolve. Notícia ou competição nulas: a original (caixa
/// "Error").
unsafe extern "thiscall" fn noticia_para_a_regiao_da_competicao(this: usize, noticia: usize, comp: usize) -> u32 {
    if noticia == 0 || comp == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(896)(this, noticia, comp); // caixa: só a original
    }
    let nacao = cpk5_d(comp + 0x5d);
    if nacao != 0 {
        crate::comparacao::ramo("cpk_5 noticia da nacao da competicao");
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x786520usize);
        return f(this, noticia, nacao);
    }
    let continente = cpk5_d(comp + 0x59);
    if continente != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x786640usize);
        return f(this, noticia, continente);
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
    f(this, noticia)
}

/// O técnico (+0xcf) do clube cujo id está no dado da notícia em `o` ([0xae23b4], 0x245 bytes por clube).
unsafe fn cpk5_tecnico_do_clube(noticia: usize, o: usize) -> usize {
    cpk5_d(cpk5_d(0xae23b4).wrapping_add(cpk5_d(noticia + o).wrapping_mul(0x245)) + 0xcf)
}

/// { 0x784eb0 } (thiscall, ret 4, news.cpp): `NewsItem::is_important_for(pessoa)`: a notícia é importante para a
/// pessoa (o byte +4 que a entrega 0x785d40 grava)? Pela categoria (0x785670) e o tipo (+0): categoria 2, os tipos 7 a
/// 0xb, 0x27 e 0x28; categoria 4, 0xbb8, 0xbb9 e 0xbc4; categoria 5, uma lista fixa, 0xfa1 se a pessoa é o técnico do
/// clube do dado 5 (+0x19) e 0xfb6 se é o do dado 4 (+0x15) ou do 5; categoria 6, 0x1770 se o dado 0 (+5) é um dos
/// 16 técnicos humanos; categoria 1, 0x1388; o resto, 0. A original chama a 0x785670 até seis vezes (só lê): a cópia,
/// uma. Pessoa nula ou tipo fora de 1..0x1790 (caixa da 0x785670): a original.
unsafe extern "thiscall" fn noticia_importante_para_a_pessoa(noticia: usize, pessoa: usize) -> u8 {
    let t = cpk5_d(noticia) as i32;
    if pessoa == 0 || !(1..=0x1790).contains(&t) {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(897)(noticia, pessoa); // caixa: só a original
    }
    let categoria: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x785670usize);
    let r = match categoria(noticia) as u8 {
        2 => matches!(t, 7..=0xb | 0x27 | 0x28),
        4 => matches!(t, 0xbb8 | 0xbb9 | 0xbc4),
        5 => match t {
            0xfa0 | 0xfa4 | 0xfad | 0xfae | 0xfaf | 0xfb0 | 0xfb2 | 0xfb4 | 0xfb7 | 0xfb8 | 0xfbb | 0xfbf | 0xfc0 => true,
            0xfa1 => {
                crate::comparacao::ramo("cpk_5 importante oferta pelo clube");
                cpk5_tecnico_do_clube(noticia, 0x19) == pessoa // o técnico do clube da oferta
            }
            0xfb6 => cpk5_tecnico_do_clube(noticia, 0x15) == pessoa || cpk5_tecnico_do_clube(noticia, 0x19) == pessoa,
            _ => false,
        },
        6 => t == 0x1770 && cpk5_d(noticia + 5) as i32 >= (cpk5_d(0xae2364) as i32).wrapping_sub(0x10),
        1 => t == 0x1388,
        _ => false, // 3 e 7: nenhum
    };
    r as u8
}

/// Comparadores: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
unsafe extern "thiscall" fn comparar_noticia_para_os_tecnicos_do_continente(this: usize, noticia: usize, continente: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_os_tecnicos_do_continente");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(895)(this, noticia, continente)
}

unsafe extern "thiscall" fn comparar_noticia_para_a_regiao_da_competicao(this: usize, noticia: usize, comp: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_a_regiao_da_competicao");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(896)(this, noticia, comp)
}

/// Comparador: pelo valor (a 0x785670 só lê).
unsafe extern "thiscall" fn comparar_noticia_importante_para_a_pessoa(noticia: usize, pessoa: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(897)(noticia, pessoa);
    let t = cpk5_d(noticia) as i32;
    if pessoa == 0 || !(1..=0x1790).contains(&t) {
        return r;
    }
    let n = noticia_importante_para_a_pessoa(noticia, pessoa);
    PLACAR.registrar(r == n, || format!("noticia_importante_para_a_pessoa(tipo {:#x}, {:x}) original={} rust={}", t, pessoa, r, n));
    r
}

// ---------- 247a leva: o texto de um dado da notícia, a notícia do clube em apuro financeiro e se os olheiros do clube cobrem a nação ----------

unsafe fn cpk6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpk6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x785a20 } (thiscall, ret 0xc, news.cpp): `NewsItem::get_slot_string(dado, destino, n)`: copia o texto guardado a
/// partir do dado `dado` (+5 + 4 × dado) para `destino` como o strncpy (0x9466d0: até o zero, completando com zeros
/// até `n`) e zera o último byte (`destino[n − 1]`). Devolve 1. Dado fora de 0..49, destino nulo ou `n` < 1: a
/// original (caixa "Error" e 0).
unsafe extern "thiscall" fn texto_do_dado_da_noticia(noticia: usize, dado: u32, destino: *mut u8, n: i32) -> u32 {
    let k = dado as u8 as i8;
    if !(0..0x32).contains(&k) || destino.is_null() || n < 1 {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32>(898)(noticia, dado, destino, n); // caixa: só a original
    }
    crate::comparacao::ramo("cpk_6 texto do dado");
    let origem = (noticia + 5 + 4 * k as usize) as *const u8; // o dado
    let mut fim = false;
    for i in 0..n as usize {
        let c = if fim { 0 } else { *origem.add(i) };
        fim |= c == 0;
        *destino.add(i) = c;
    }
    *destino.add(n as usize - 1) = 0;
    1
}

/// { 0x74ae60 } (thiscall, ret 4, media.cpp, quadro SEH sem destrutor): `Media::post_financial_trouble_news(clube)`:
/// com o clube, a nação (+0x53), a ficha (+0x69, com +0x38) e a divisão (+0x57), monta a notícia 0x1790 numa local
/// (0x77bbe0 com 0) com os dados 0 = id do clube, 1 = reputação (+0x80) e 2 = sorteio(2) (0x90cfc0); clube de
/// reputação 9000 ou mais numa divisão com +0x69 acima de 12: para todos os técnicos (0x786780); senão para os da
/// nação (0x7863f0). A notícia local tem 0xe0 bytes (a entrega copia só até +0xde).
unsafe extern "thiscall" fn noticia_de_apuro_financeiro(_this: usize, clube: usize) {
    if clube == 0 || cpk6_d(clube + 0x53) == 0 {
        return;
    }
    let ficha = cpk6_d(clube + 0x69);
    if ficha == 0 || cpk6_d(ficha + 0x38) == 0 || cpk6_d(clube + 0x57) == 0 {
        return;
    }
    crate::comparacao::ramo("cpk_6 apuro financeiro");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    construir(n, 0x1790, 0);
    gravar(n, 0, cpk6_d(clube) as u32);
    gravar(n, 1, cpk6_w(clube + 0x80) as i32 as u32);
    let v = sorteio(2); // o sorteio da notícia
    gravar(n, 2, v as u32);
    let noticias = cpk6_d(0xb7431c);
    let divisao = cpk6_d(clube + 0x57);
    if cpk6_w(clube + 0x80) >= 9000 && divisao != 0 && cpk6_w(divisao + 0x69) > 12 {
        let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
        todos(noticias, n);
    } else {
        let pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
        pais(noticias, n, cpk6_d(clube + 0x53));
    }
}

/// O clube que a 0x7f9190 usa: o `clube`, ou o do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]] + 0x39).
unsafe fn cpk6_clube(clube: usize) -> usize {
    if clube != 0 {
        return clube;
    }
    cpk6_d(cpk6_d(0xb74c26 + 0x300 * *(0xb77c7a as *const i16) as isize as usize) + 0x39)
}

/// { 0x7f9190 } (thiscall, ret 8, scout_manager.cpp; `this` não é lido): `SCOUT_MANAGER::can_club_scout_nation(nacao,
/// clube)`: o clube (ou o do humano da vez) não cobre a nação só se tem as finanças (0x5a0590 em [0xae2a38]) com saldo
/// de 64 bits abaixo de 1.000.000, reputação (+0x80) abaixo de 5000 e a região (+0x76) da nação dele (+0x53) diferente
/// da região da `nacao`: 0. Senão 1. Sem clube nenhum: a original (caixa "Error" e 0).
unsafe extern "thiscall" fn olheiros_cobrem_a_nacao(this: usize, nacao: usize, clube: usize) -> u32 {
    let c = cpk6_clube(clube);
    if c == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(900)(this, nacao, clube); // caixa: só a original
    }
    let financas_do_clube: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let f = financas_do_clube(cpk6_d(0xae2a38), c);
    if f != 0 && core::ptr::read_unaligned(f as *const i64) < 1_000_000 && cpk6_w(c + 0x80) < 5000 {
        if *((cpk6_d(c + 0x53) + 0x76) as *const u8) != *((nacao + 0x76) as *const u8) { // fora da região
            crate::comparacao::ramo("cpk_6 olheiro fora da regiao");
            return 0;
        }
    }
    1
}

/// Comparador: lado a lado pelos `n` bytes do destino.
unsafe extern "thiscall" fn comparar_texto_do_dado_da_noticia(noticia: usize, dado: u32, destino: *mut u8, n: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32>(898);
    let k = dado as u8 as i8;
    if !(0..0x32).contains(&k) || destino.is_null() || n < 1 {
        return o(noticia, dado, destino, n);
    }
    let reg = [(destino as usize, n as usize)];
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(noticia, dado, destino, n), || texto_do_dado_da_noticia(noticia, dado, destino, n));
    PLACAR.registrar(igual && r == m, || format!("texto_do_dado_da_noticia({}, {}) {:x?} x {:x?}", k, n, mo, mr));
    r
}

/// Comparador: a notícia sorteia (0x90cfc0) e vai para as caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
unsafe extern "thiscall" fn comparar_noticia_de_apuro_financeiro(this: usize, clube: usize) {
    crate::comparacao::ramo("noticia_de_apuro_financeiro");
    orig::<unsafe extern "thiscall" fn(usize, usize)>(899)(this, clube)
}

/// Comparador: pelo valor (a 0x5a0590 só procura).
unsafe extern "thiscall" fn comparar_olheiros_cobrem_a_nacao(this: usize, nacao: usize, clube: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(900)(this, nacao, clube);
    if cpk6_clube(clube) == 0 {
        return r;
    }
    let n = olheiros_cobrem_a_nacao(this, nacao, clube);
    PLACAR.registrar(r == n, || format!("olheiros_cobrem_a_nacao({:x}, {:x}) original={} rust={}", nacao, clube, r, n));
    r
}

// ---------- 247a leva: a fluência da pessoa num idioma, recomeçar o histórico da pessoa e a posição de uma casa da formação ----------

unsafe fn cpk7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpk7_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpk7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x7c6830 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::get_language_fluency(pessoa, clube, idioma,
/// pelo_contrato)`: quanto a pessoa fala o `idioma` (0 a 10). Técnico humano (id ≥ [0xae2364]): 10 se é o idioma
/// (+0x77) da nação dela (+0x1a), senão 0. Com `pelo_contrato`, o contrato com o clube (0x4cf5f0) de quem joga num
/// clube (+0x39) da mesma nação do `clube` (+0x53): o nível do contrato (+0x4f & 0xf). 10 se a nação (+0x1a) ou a
/// segunda (+0x1e) fala o idioma da nação do `clube` ou o `idioma`. Senão o idioma aprendido: a busca binária (bsearch
/// 0x946464, comparação 0x7c6d40, registros de 6 bytes em [+0x3c], [+0x44] deles) pela chave {id, idioma}, o byte +5;
/// sem a tabela ou sem o registro, 0. Os chamadores só leem o al.
unsafe extern "thiscall" fn fluencia_no_idioma(this: usize, pessoa: usize, clube: usize, idioma: u32, pelo_contrato: u32) -> u8 {
    let idioma = idioma as u8;
    if pessoa != 0 && cpk7_d(pessoa) as i32 >= cpk7_d(0xae2364) as i32 {
        return if cpk7_b(cpk7_d(pessoa + 0x1a) + 0x77) == idioma { 10 } else { 0 }; // o técnico humano
    }
    if pelo_contrato != 0 {
        let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
        let c = contrato(cpk7_d(0xae1050), pessoa, clube);
        if c != 0 {
            let cp = cpk7_d(pessoa + 0x39);
            if cp != 0 && cpk7_d(clube + 0x53) == cpk7_d(cp + 0x53) {
                return cpk7_b(c + 0x4f) & 0xf;
            }
        }
    }
    let fala = |l: u8| {
        if cpk7_b(cpk7_d(pessoa + 0x1a) + 0x77) == l {
            return true;
        }
        let s = cpk7_d(pessoa + 0x1e);
        s != 0 && cpk7_b(s + 0x77) == l // a segunda nação
    };
    if clube != 0 {
        let n = cpk7_d(clube + 0x53);
        if n != 0 && fala(cpk7_b(n + 0x77)) {
            return 10;
        }
    }
    if fala(idioma) {
        return 10;
    }
    let base = cpk7_d(this + 0x3c);
    if base == 0 {
        return 0;
    }
    let mut chave = [0u8; 8];
    chave[..4].copy_from_slice(&(cpk7_d(pessoa) as u32).to_le_bytes());
    chave[4] = idioma;
    let bsearch: unsafe extern "C" fn(*const u8, usize, usize, usize, usize) -> usize = core::mem::transmute(0x946464usize);
    let e = bsearch(chave.as_ptr(), base, cpk7_d(this + 0x44), 6, 0x7c6d40);
    if e == 0 {
        return 0;
    }
    cpk7_b(e + 5)
}

/// { 0x86fb80 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::rewind_staff_history(pessoa)`: a lista do
/// histórico da pessoa (o par de 8 bytes em [+0xc] + 8 × id) fica sem o último (0x58f110 sobre [this]: o segundo
/// deslocamento = −1). Pessoa nula: a original (caixa fatal: o jogo termina).
unsafe extern "thiscall" fn recomecar_historico_da_pessoa(this: usize, pessoa: usize) {
    if pessoa == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(902)(this, pessoa); // caixa fatal: só a original
    }
    crate::comparacao::ramo("cpk_7 recomecar historico");
    let par = cpk7_d(this + 0xc).wrapping_add(cpk7_d(pessoa).wrapping_mul(8));
    let sem_ultimo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x58f110usize);
    sem_ultimo(cpk7_d(this), par, par + 4); // o último deslocamento
}

/// { 0x5b2ba0 } (thiscall, ret 0xc, formation.cpp): `Formation::set_slot_position(casa, posicao, extra)`: sem `extra`,
/// a posição geral da casa (+0x12d, palavras) muda para `posicao` (se mudar: a máscara 0x5b59a0, a marca +0x143 da casa
/// = 1 e a das outras casas com a mesma posição = 0); com ou sem, a posição extra (+0x14e) = `posicao`. Depois, para
/// cada faixa 0..2 e altura 0..3 (a palavra baixa e a alta do argumento da 0x5b7e60), as casas no campo sem e com a
/// bola (+0x164 e +0x194 + 4 × (altura + 4 × faixa + 24 × casa)): com geral = extra, as duas da geral; senão, pelos
/// setores (0x5b73a0): extra de setor abaixo do da geral, sem a bola a extra e com a bola a geral; senão o contrário.
/// Por fim +0x187f = 0. Devolve 1. Casa fora de 0..10: a original (caixa "Error" e 0).
unsafe extern "thiscall" fn posicao_na_formacao(this: usize, casa: u32, posicao: u32, extra: u32) -> u32 {
    let k = casa as u8 as i8;
    if !(0..=10).contains(&k) {
        return orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(903)(this, casa, posicao, extra); // caixa: só a original
    }
    let s = k as usize;
    let p = posicao as u16 as i16;
    let geral = this + 0x12d + 2 * s;
    let extra_da_casa = this + 0x14e + 2 * s;
    if extra == 0 && cpk7_w(geral) != p {
        core::ptr::write_unaligned(geral as *mut i16, p);
        let mascara: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5b59a0usize);
        mascara(this);
        *((this + 0x143 + s) as *mut u8) = 1;
        for j in 0..11usize {
            if j != s && cpk7_w(this + 0x12d + 2 * j) == p {
                *((this + 0x143 + j) as *mut u8) = 0; // a outra casa com a mesma posição
            }
        }
    }
    core::ptr::write_unaligned(extra_da_casa as *mut i16, p);
    // a 0x5b7e60 e a 0x5b73a0 só leem a palavra baixa da máscara (a original empurra o eax com o alto que estiver)
    let casa_no_campo: unsafe extern "C" fn(u32, u32, u32, *mut u32) -> u32 = core::mem::transmute(0x5b7e60usize);
    let separar: unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32 = core::mem::transmute(0x5b73a0usize);
    // as saídas valem de uma volta para a outra, como as locais da original (sem valor só se a 0x5b73a0 falhar
    // logo na primeira: zero aqui)
    let (mut setor_geral, mut setor_extra, mut lado, mut saida) = (0u16, 0u16, 0u16, 0u32);
    if cpk7_w(geral) == cpk7_w(extra_da_casa) {
        crate::comparacao::ramo("cpk_7 formacao mesma posicao");
    } else {
        crate::comparacao::ramo("cpk_7 formacao posicao extra");
    }
    for faixa in 0..3usize {
        for altura in 0..4usize {
            let arg = faixa as u32 | (altura as u32) << 16;
            let i = altura + 4 * (faixa + 6 * s);
            let (sem_bola, com_bola) = (this + 0x164 + 4 * i, this + 0x194 + 4 * i);
            let (g, e) = (cpk7_w(geral) as u16 as u32, cpk7_w(extra_da_casa) as u16 as u32);
            let (a, b) = if g == e {
                (g, g) // a mesma posição com e sem a bola
            } else {
                separar(g, &mut setor_geral, &mut lado);
                separar(e, &mut setor_extra, &mut lado);
                if (setor_extra as i16) < (setor_geral as i16) { // a extra mais atrás
                    (e, g)
                } else {
                    (g, e)
                }
            };
            casa_no_campo(a, 0, arg, &mut saida);
            core::ptr::write_unaligned(sem_bola as *mut u32, saida);
            casa_no_campo(b, 1, arg, &mut saida);
            core::ptr::write_unaligned(com_bola as *mut u32, saida);
        }
    }
    core::ptr::write_unaligned((this + 0x187f) as *mut u32, 0);
    1
}

/// Comparador: pelo valor (a 0x4cf5f0 e a busca só leem).
unsafe extern "thiscall" fn comparar_fluencia_no_idioma(this: usize, pessoa: usize, clube: usize, idioma: u32, pelo_contrato: u32) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8>(901)(this, pessoa, clube, idioma, pelo_contrato);
    let n = fluencia_no_idioma(this, pessoa, clube, idioma, pelo_contrato);
    PLACAR.registrar(r == n, || format!("fluencia_no_idioma({:x}, {:x}, {}, {}) original={} rust={}", pessoa, clube, idioma as u8, pelo_contrato, r, n));
    r
}

/// Comparador: lado a lado pelo par da pessoa (8 bytes; a 0x58f110 só grava o segundo).
unsafe extern "thiscall" fn comparar_recomecar_historico_da_pessoa(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(902);
    if pessoa == 0 {
        return o(this, pessoa);
    }
    let par = cpk7_d(this + 0xc).wrapping_add(cpk7_d(pessoa).wrapping_mul(8));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(par, 8)], || o(this, pessoa), || recomecar_historico_da_pessoa(this, pessoa));
    PLACAR.registrar(igual, || format!("recomecar_historico_da_pessoa({}) {:x?} x {:x?}", cpk7_d(pessoa), mo, mr));
}

/// Comparador: lado a lado pela formação (+0x12d a +0x584: posições, marcas e casas no campo; a máscara +0x187b e
/// +0x187f) e [0xb67a34] (a 0x5b73a0 zera quando falha).
unsafe extern "thiscall" fn comparar_posicao_na_formacao(this: usize, casa: u32, posicao: u32, extra: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(903);
    if !(0..=10).contains(&(casa as u8 as i8)) {
        return o(this, casa, posicao, extra);
    }
    let reg = [(this + 0x12d, 0x584 - 0x12d), (this + 0x187b, 8), (0xb67a34, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, casa, posicao, extra), || posicao_na_formacao(this, casa, posicao, extra));
    PLACAR.registrar(igual && r == n, || format!("posicao_na_formacao({}, {:#x}, {}) {:x?} x {:x?}", casa as u8 as i8, posicao as u16, extra, mo, mr));
    r
}
