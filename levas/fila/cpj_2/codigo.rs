// ---------- %%LEVA%%a leva: as listas de pendências: soltar as suspensões guardadas, tirar as ofertas pendentes da pessoa e marcar a decisão depois da transferência acertada ----------

unsafe fn cpj2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpj2_proximo(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    f(l)
}

unsafe fn cpj2_tirar(l: usize, e: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    f(l, e);
}

/// O contador de suspensões guardadas (+0x1c) do registro de 29 bytes do id `id` em [this+0x14].
unsafe fn cpj2_guardadas(this: usize, id: usize) -> *mut i8 {
    (cpj2_d(this + 0x14).wrapping_add(id.wrapping_mul(29)) + 0x1c) as *mut i8
}

/// { 0x5567d0 } (thiscall, ret 4, discipline.cpp): `Discipline::drop_held_bans(pessoa)`: da lista das suspensões
/// guardadas (+0x1c; o cursor +0xa volta ao começo +2), tira (0x68d1f0) cada item do id `*pessoa` enquanto o contador
/// do registro dele (+0x1c, com sinal) for positivo, descontando um a cada item.
unsafe extern "thiscall" fn soltar_suspensoes_guardadas(this: usize, pessoa: *const u32) {
    let l = this + 0x1c;
    let n = cpj2_guardadas(this, *pessoa as usize);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpj2_d(l + 2));
    let mut e = cpj2_proximo(l);
    while e != 0 && *n > 0 {
        if cpj2_d(e) as u32 == *pessoa {
            cpj2_tirar(l, e);
            *n = (*n).wrapping_sub(1); // uma a menos
        }
        e = cpj2_proximo(l);
    }
}

/// O byte de ofertas pendentes (+8) do registro de 79 bytes da pessoa em [0xae2d0c].
unsafe fn cpj2_pendentes(id: usize) -> *mut u8 {
    (cpj2_d(0xae2d0c).wrapping_add(id.wrapping_mul(79)) + 8) as *mut u8
}

/// { 0x4ccbf0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_pending_offers_for_staff(pessoa)`:
/// com a lista das ofertas (+0x34) e ofertas pendentes no registro da pessoa (+8 do de 79 bytes em [0xae2d0c]): tira
/// a marca de contrato acertado (0x8c5430 sobre [0xde1f54]) e, do começo da lista, cada oferta da pessoa (0x68d1f0),
/// descontando o byte, até ele zerar.
unsafe extern "thiscall" fn tirar_ofertas_pendentes(this: usize, pessoa: *const u32) {
    let l = cpj2_d(this + 0x34);
    if l == 0 || *cpj2_pendentes(*pessoa as usize) == 0 {
        return;
    }
    let desmarcar: unsafe extern "thiscall" fn(usize, *const u32) -> u32 = core::mem::transmute(0x8c5430usize);
    desmarcar(cpj2_d(0xde1f54), pessoa);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpj2_d(l + 2));
    let mut e = cpj2_proximo(l);
    while e != 0 {
        if cpj2_d(e) as u32 == *pessoa {
            cpj2_tirar(l, e);
            let p = cpj2_pendentes(*pessoa as usize);
            *p = (*p).wrapping_sub(1);
            if *p == 0 {
                return; // a última pendente
            }
        }
        e = cpj2_proximo(l);
    }
}

/// A primeira oferta da lista `l` (sem mexer no cursor) da pessoa (+0) e do clube (+4) da oferta de transferência e
/// ainda sem decisão (+0x62 = 0).
unsafe fn cpj2_oferta_a_marcar(l: usize, oferta: usize) -> Option<usize> {
    cpd1_itens(l).into_iter().find(|&e| cpj2_d(e) == cpj2_d(oferta + 0xc) && cpj2_d(e + 4) == cpj2_d(oferta + 8) && *((e + 0x62) as *const u8) == 0)
}

/// Habilidade (palavra +0xb do registro de jogador +0x61 da pessoa +0xc) / 500 acima da reputação (+0x80) do clube
/// (+8) da oferta?
unsafe fn cpj2_acima_do_clube(oferta: usize) -> u32 {
    let pessoa = cpj2_d(0xae23bc).wrapping_add(cpj2_d(oferta + 0xc).wrapping_mul(0x6e));
    let hab = *((cpj2_d(pessoa + 0x61) + 0xb) as *const i16) as i32 / 500;
    let clube = cpj2_d(0xae23b4).wrapping_add(cpj2_d(oferta + 8).wrapping_mul(0x245));
    (hab > *((clube + 0x80) as *const i16) as i32) as u32
}

/// { 0x4cc930 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::schedule_decision_for_transfer(oferta)`:
/// a primeira oferta de contrato da lista (+0x34) da pessoa (+0xc) e do clube (+8) da oferta de transferência ainda
/// sem decisão (+0x62) fica marcada (1) e a decisão da pessoa é agendada (0x4cd230 com a vaga +0x20). Devolve se a
/// habilidade da pessoa / 500 passa a reputação do clube.
unsafe extern "thiscall" fn marcar_decisao_da_transferencia(this: usize, oferta: usize) -> u32 {
    let l = cpj2_d(this + 0x34);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpj2_d(l + 2));
    let mut e = cpj2_proximo(l);
    while e != 0 {
        if cpj2_d(e) == cpj2_d(oferta + 0xc) && cpj2_d(e + 4) == cpj2_d(oferta + 8) && *((e + 0x62) as *const u8) == 0 {
            *((e + 0x62) as *mut u8) = 1; // a oferta vale
            let agendar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x4cd230usize);
            agendar(this, cpj2_d(oferta + 0xc) as u32, cpj2_d(oferta + 0x20) as u32);
            break;
        }
        e = cpj2_proximo(l);
    }
    cpj2_acima_do_clube(oferta)
}

/// Comparador: o plano da cópia (os itens do id que saem até o contador zerar) antes; a original tira; confere os
/// itens que ficaram e o contador.
unsafe extern "thiscall" fn comparar_soltar_suspensoes_guardadas(this: usize, pessoa: *const u32) {
    let l = this + 0x1c;
    let n = cpj2_guardadas(this, *pessoa as usize);
    let antes = cpd1_itens(l);
    let mut resta = *n;
    let mut esperado = Vec::new();
    for &e in &antes {
        if resta > 0 && cpj2_d(e) as u32 == *pessoa {
            resta -= 1;
        } else {
            esperado.push(e);
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, *const u32)>(%%I0%%)(this, pessoa);
    let depois = cpd1_itens(l);
    PLACAR.registrar(depois == esperado && *n == resta, || format!("soltar_suspensoes_guardadas({}) itens {} -> {} (esperava {}), contador {} (esperava {})", *pessoa, antes.len(), depois.len(), esperado.len(), *n, resta));
}

/// Comparador: o mesmo plano pelo byte de pendentes; a original tira (e desmarca); confere os itens e o byte.
unsafe extern "thiscall" fn comparar_tirar_ofertas_pendentes(this: usize, pessoa: *const u32) {
    let l = cpj2_d(this + 0x34);
    if l == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, *const u32)>(%%I1%%)(this, pessoa);
    }
    let p = cpj2_pendentes(*pessoa as usize);
    let antes = cpd1_itens(l);
    let mut resta = *p;
    let mut esperado = Vec::new();
    for &e in &antes {
        if resta != 0 && cpj2_d(e) as u32 == *pessoa {
            resta = resta.wrapping_sub(1);
        } else {
            esperado.push(e);
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, *const u32)>(%%I1%%)(this, pessoa);
    let depois = cpd1_itens(l);
    PLACAR.registrar(depois == esperado && *p == resta, || format!("tirar_ofertas_pendentes({}) itens {} -> {} (esperava {}), pendentes {} (esperava {})", *pessoa, antes.len(), depois.len(), esperado.len(), *p, resta));
}

/// Comparador: a oferta que a cópia marcaria antes; a original marca e agenda (a 0x4cd230 pode acrescentar na lista
/// das decisões); confere a marca e o valor.
unsafe extern "thiscall" fn comparar_marcar_decisao_da_transferencia(this: usize, oferta: usize) -> u32 {
    let alvo = cpj2_oferta_a_marcar(cpj2_d(this + 0x34), oferta);
    let n = cpj2_acima_do_clube(oferta);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, oferta);
    let marcada = alvo.map_or(true, |e| *((e + 0x62) as *const u8) == 1);
    PLACAR.registrar(marcada && r == n, || format!("marcar_decisao_da_transferencia({:x}) alvo {:x?} marcada={} original={} rust={}", oferta, alvo, marcada, r, n));
    r
}
