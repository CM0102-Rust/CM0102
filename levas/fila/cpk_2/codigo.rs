// ---------- %%LEVA%%a leva: as listas do regen: a reserva de técnico, a aposentadoria pendente e a vaga para o regen ----------

unsafe fn cpk2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpk2_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpk2_por16(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

unsafe fn crt_realloc(p: usize, tam: usize) -> usize {
    let f: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    f(p, tam as u32)
}

/// A falta de memória das listas do regen (player_regen.cpp, linha `linha`): caixa e fim, como a original.
unsafe fn regen_out_of_memory(linha: u32) -> ! {
    show_error_at(0xa6bf9c, linha);
    fatal_exit()
}

/// { 0x7a81a0 } (thiscall, ret 0xc, player_regen.cpp): `PLAYER_REGEN::add_spare_non_player_record(id, nação,
/// segunda)`: acrescenta (id +0, nação +4, segunda nação +8) à lista de 16 bytes das reservas de não jogador
/// ([this+0x1c], conta +0x20, capacidade +0x24; tirada pela 0x7a82b0). O original decide crescer pelo banco de
/// jogadores livres (conta +0x40 = capacidade +0x44 − 1), não por esta lista: fiel. Cresce 250 (realloc 0x945667;
/// sem memória, caixa 0x9c4 e fim).
unsafe extern "thiscall" fn add_spare_non_player_record(this: usize, id: u32, nacao: u32, segunda: u32) {
    crate::comparison::branch("cpk_2 reserva de tecnico");
    if cpk2_d(this + 0x40) == cpk2_d(this + 0x44).wrapping_sub(1) {
        let p = crt_realloc(cpk2_d(this + 0x1c), cpk2_d(this + 0x24).wrapping_add(250) << 4);
        cpk2_por(this + 0x1c, p);
        cpk2_por(this + 0x24, cpk2_d(this + 0x24).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0x9c4);
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
unsafe extern "thiscall" fn add_retirement(this: usize, r: usize) {
    crate::comparison::branch("cpk_2 aposentadoria");
    let cap = cpk2_d(this + 0xc);
    if cpk2_d(this + 8) == cap.wrapping_sub(1) {
        let p = crt_realloc(cpk2_d(this + 4), cap.wrapping_add(250).wrapping_mul(26));
        cpk2_por(this + 4, p);
        cpk2_por(this + 0xc, cpk2_d(this + 0xc).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0xa53);
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
unsafe extern "thiscall" fn queue_staff_for_regen(this: usize, entrada: usize) {
    let id = cpk2_d(entrada) as i32;
    if id < 0 || id > *(0xae2364 as *const i32) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, entrada);
    }
    crate::comparison::branch("cpk_2 vaga para o regen");
    let pessoa = cpk2_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    let cap = cpk2_d(this + 0x18);
    if cpk2_d(this + 0x14) == cap.wrapping_sub(1) {
        let p = crt_realloc(cpk2_d(this + 0x10), cap.wrapping_add(250).wrapping_mul(12));
        cpk2_por(this + 0x10, p);
        cpk2_por(this + 0x18, cpk2_d(this + 0x18).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0xaac);
        }
    }
    let fila = cpk2_d(this + 0x10).wrapping_add(cpk2_d(this + 0x14).wrapping_mul(12));
    cpk2_por(fila, id as usize);
    cpk2_por16(fila + 4, core::ptr::read_unaligned((entrada + 4) as *const u16));
    cpk2_por16(fila + 6, core::ptr::read_unaligned((entrada + 6) as *const u16));
    cpk2_por(this + 0x14, cpk2_d(this + 0x14).wrapping_add(1)); // mais uma na fila do regen
    if cpk2_d(pessoa + 0x69) != 0 {
        if cpk2_d(this + 0x20) == cpk2_d(this + 0x24).wrapping_sub(1) {
            let p = crt_realloc(cpk2_d(this + 0x1c), cpk2_d(this + 0x24).wrapping_add(250) << 4);
            cpk2_por(this + 0x1c, p);
            cpk2_por(this + 0x24, cpk2_d(this + 0x24).wrapping_add(250));
            if p == 0 {
                regen_out_of_memory(0xac5);
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
            let p = crt_realloc(cpk2_d(this + 0x3c), cap.wrapping_add(250) << 4);
            cpk2_por(this + 0x3c, p);
            cpk2_por(this + 0x44, cpk2_d(this + 0x44).wrapping_add(250));
            if cpk2_d(this + 0x40) == 0 {
                regen_out_of_memory(0xae9); // a original confere a conta (+0x40), não o ponteiro
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
unsafe extern "thiscall" fn comparar_add_spare_non_player_record(this: usize, id: u32, nacao: u32, segunda: u32) {
    let n = cpk2_d(this + 0x20);
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(%%I0%%)(this, id, nacao, segunda);
    let item = cpk2_d(this + 0x1c).wrapping_add(n << 4);
    let g = [cpk2_d(item) as u32, cpk2_d(item + 4) as u32, cpk2_d(item + 8) as u32];
    SCOREBOARD.record(cpk2_d(this + 0x20) == n.wrapping_add(1) && g == [id, nacao, segunda], || {
        format!("add_spare_non_player_record({}, {}, {}) conta {} -> {}, item {:?}", id, nacao, segunda, n, cpk2_d(this + 0x20), g)
    });
}

/// Os 26 bytes que a 0x7a8510 copia do registro (os outros, zerados).
unsafe fn retirement_entry_fields(r: usize) -> [u8; 26] {
    let mut c = [0u8; 26];
    for (o, n) in [(0usize, 4usize), (4, 4), (8, 2), (0xa, 2), (0x10, 2), (0x12, 2), (0x18, 1), (0x19, 1)] {
        c[o..o + n].copy_from_slice(core::slice::from_raw_parts((r + o) as *const u8, n));
    }
    c
}

/// Comparador: o registro copiado antes; a original acrescenta (pode realocar); confere a conta e os campos.
unsafe extern "thiscall" fn comparar_add_retirement(this: usize, r: usize) {
    let n = cpk2_d(this + 8);
    let esperado = retirement_entry_fields(r);
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, r);
    let item = cpk2_d(this + 4).wrapping_add(n.wrapping_mul(26));
    let g = retirement_entry_fields(item);
    SCOREBOARD.record(cpk2_d(this + 8) == n.wrapping_add(1) && g == esperado, || {
        format!("add_retirement conta {} -> {}, item {:x?} (esperava {:x?})", n, cpk2_d(this + 8), g, esperado)
    });
}

/// Comparador: a original sorteia e realoca (não pode rodar duas vezes): o plano antes (o que entra em cada lista);
/// confere as contas e os itens (a nação sorteada só pela faixa).
unsafe extern "thiscall" fn comparar_queue_staff_for_regen(this: usize, entrada: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
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
    SCOREBOARD.record(ok, || {
        format!("queue_staff_for_regen({}) contas {} {} {} -> {} {} {}, reserva {:?}, banco {:?}", id, n1, n2, n3,
            cpk2_d(this + 0x14), cpk2_d(this + 0x20), cpk2_d(this + 0x40), reserva, banco)
    });
}
