// ---------- %%LEVA%%a leva: as listas de pendências: soltar as suspensões guardadas, tirar as ofertas pendentes da pessoa e marcar a decisão depois da transferência acertada ----------

unsafe fn cpj2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn contract_manager_list_next(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    f(l)
}

unsafe fn list_remove(l: usize, e: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    f(l, e);
}

/// O contador de suspensões guardadas (+0x1c) do registro de 29 bytes do id `id` em [this+0x14].
unsafe fn stored_suspension_counter(this: usize, id: usize) -> *mut i8 {
    (cpj2_d(this + 0x14).wrapping_add(id.wrapping_mul(29)) + 0x1c) as *mut i8
}

/// { 0x5567d0 } (thiscall, ret 4, discipline.cpp): `Discipline::drop_held_bans(pessoa)`: da lista das suspensões
/// guardadas (+0x1c; o cursor +0xa volta ao começo +2), tira (0x68d1f0) cada item do id `*pessoa` enquanto o contador
/// do registro dele (+0x1c, com sinal) for positivo, descontando um a cada item.
unsafe extern "thiscall" fn drop_held_bans(this: usize, pessoa: *const u32) {
    let l = this + 0x1c;
    let n = stored_suspension_counter(this, *pessoa as usize);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpj2_d(l + 2));
    let mut e = contract_manager_list_next(l);
    while e != 0 && *n > 0 {
        if cpj2_d(e) as u32 == *pessoa {
            list_remove(l, e);
            *n = (*n).wrapping_sub(1); // uma a menos
        }
        e = contract_manager_list_next(l);
    }
}

/// O byte de ofertas pendentes (+8) do registro de 79 bytes da pessoa em [0xae2d0c].
unsafe fn pending_offers_byte(id: usize) -> *mut u8 {
    (cpj2_d(0xae2d0c).wrapping_add(id.wrapping_mul(79)) + 8) as *mut u8
}

/// { 0x4ccbf0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_pending_offers_for_staff(pessoa)`:
/// com a lista das ofertas (+0x34) e ofertas pendentes no registro da pessoa (+8 do de 79 bytes em [0xae2d0c]): tira
/// a marca de contrato acertado (0x8c5430 sobre [0xde1f54]) e, do começo da lista, cada oferta da pessoa (0x68d1f0),
/// descontando o byte, até ele zerar.
unsafe extern "thiscall" fn remove_pending_offers_for_staff(this: usize, pessoa: *const u32) {
    let l = cpj2_d(this + 0x34);
    if l == 0 || *pending_offers_byte(*pessoa as usize) == 0 {
        return;
    }
    let desmarcar: unsafe extern "thiscall" fn(usize, *const u32) -> u32 = core::mem::transmute(0x8c5430usize);
    desmarcar(cpj2_d(0xde1f54), pessoa);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpj2_d(l + 2));
    let mut e = contract_manager_list_next(l);
    while e != 0 {
        if cpj2_d(e) as u32 == *pessoa {
            list_remove(l, e);
            let p = pending_offers_byte(*pessoa as usize);
            *p = (*p).wrapping_sub(1);
            if *p == 0 {
                return; // a última pendente
            }
        }
        e = contract_manager_list_next(l);
    }
}

/// A primeira oferta da lista `l` (sem mexer no cursor) da pessoa (+0) e do clube (+4) da oferta de transferência e
/// ainda sem decisão (+0x62 = 0).
unsafe fn offer_to_schedule(l: usize, oferta: usize) -> Option<usize> {
    list_items_capped(l).into_iter().find(|&e| cpj2_d(e) == cpj2_d(oferta + 0xc) && cpj2_d(e + 4) == cpj2_d(oferta + 8) && *((e + 0x62) as *const u8) == 0)
}

/// Habilidade (palavra +0xb do registro de jogador +0x61 da pessoa +0xc) / 500 acima da reputação (+0x80) do clube
/// (+8) da oferta?
unsafe fn ability_above_club_reputation(oferta: usize) -> u32 {
    let pessoa = cpj2_d(0xae23bc).wrapping_add(cpj2_d(oferta + 0xc).wrapping_mul(0x6e));
    let hab = *((cpj2_d(pessoa + 0x61) + 0xb) as *const i16) as i32 / 500;
    let clube = cpj2_d(0xae23b4).wrapping_add(cpj2_d(oferta + 8).wrapping_mul(0x245));
    (hab > *((clube + 0x80) as *const i16) as i32) as u32
}

/// { 0x4cc930 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::schedule_decision_for_transfer(oferta)`:
/// a primeira oferta de contrato da lista (+0x34) da pessoa (+0xc) e do clube (+8) da oferta de transferência ainda
/// sem decisão (+0x62) fica marcada (1) e a decisão da pessoa é agendada (0x4cd230 com a vaga +0x20). Devolve se a
/// habilidade da pessoa / 500 passa a reputação do clube.
unsafe extern "thiscall" fn schedule_decision_for_transfer(this: usize, oferta: usize) -> u32 {
    let l = cpj2_d(this + 0x34);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpj2_d(l + 2));
    let mut e = contract_manager_list_next(l);
    while e != 0 {
        if cpj2_d(e) == cpj2_d(oferta + 0xc) && cpj2_d(e + 4) == cpj2_d(oferta + 8) && *((e + 0x62) as *const u8) == 0 {
            *((e + 0x62) as *mut u8) = 1; // a oferta vale
            let agendar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x4cd230usize);
            agendar(this, cpj2_d(oferta + 0xc) as u32, cpj2_d(oferta + 0x20) as u32);
            break;
        }
        e = contract_manager_list_next(l);
    }
    ability_above_club_reputation(oferta)
}

/// Comparador: o plano da cópia (os itens do id que saem até o contador zerar) antes; a original tira; confere os
/// itens que ficaram e o contador.
unsafe extern "thiscall" fn compare_drop_held_bans(this: usize, pessoa: *const u32) {
    let l = this + 0x1c;
    let n = stored_suspension_counter(this, *pessoa as usize);
    let antes = list_items_capped(l);
    let mut resta = *n;
    let mut esperado = Vec::new();
    for &e in &antes {
        if resta > 0 && cpj2_d(e) as u32 == *pessoa {
            resta -= 1;
        } else {
            esperado.push(e);
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, *const u32)>(%%I0%%)(this, pessoa);
    let depois = list_items_capped(l);
    SCOREBOARD.record(depois == esperado && *n == resta, || format!("drop_held_bans({}) itens {} -> {} (esperava {}), contador {} (esperava {})", *pessoa, antes.len(), depois.len(), esperado.len(), *n, resta));
}

/// Comparador: o mesmo plano pelo byte de pendentes; a original tira (e desmarca); confere os itens e o byte.
unsafe extern "thiscall" fn compare_remove_pending_offers_for_staff(this: usize, pessoa: *const u32) {
    let l = cpj2_d(this + 0x34);
    if l == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, *const u32)>(%%I1%%)(this, pessoa);
    }
    let p = pending_offers_byte(*pessoa as usize);
    let antes = list_items_capped(l);
    let mut resta = *p;
    let mut esperado = Vec::new();
    for &e in &antes {
        if resta != 0 && cpj2_d(e) as u32 == *pessoa {
            resta = resta.wrapping_sub(1);
        } else {
            esperado.push(e);
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, *const u32)>(%%I1%%)(this, pessoa);
    let depois = list_items_capped(l);
    SCOREBOARD.record(depois == esperado && *p == resta, || format!("remove_pending_offers_for_staff({}) itens {} -> {} (esperava {}), pendentes {} (esperava {})", *pessoa, antes.len(), depois.len(), esperado.len(), *p, resta));
}

/// Comparador: a oferta que a cópia marcaria antes; a original marca e agenda (a 0x4cd230 pode acrescentar na lista
/// das decisões); confere a marca e o valor.
unsafe extern "thiscall" fn compare_schedule_decision_for_transfer(this: usize, oferta: usize) -> u32 {
    let alvo = offer_to_schedule(cpj2_d(this + 0x34), oferta);
    let n = ability_above_club_reputation(oferta);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, oferta);
    let marcada = alvo.map_or(true, |e| *((e + 0x62) as *const u8) == 1);
    SCOREBOARD.record(marcada && r == n, || format!("schedule_decision_for_transfer({:x}) alvo {:x?} marcada={} original={} rust={}", oferta, alvo, marcada, r, n));
    r
}
