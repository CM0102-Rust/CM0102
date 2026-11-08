// ---------- %%LEVA%%a leva: contratos e suspensão: as cláusulas de rebaixamento do clube rebaixado, tirar a oferta de contrato da pessoa ao clube e se o jogador está suspenso para a partida ----------

unsafe fn cpk3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O contrato (registro de 80 bytes em [this], pelo índice +0 do registro de 79 bytes em [0xae2d0c]) da pessoa `p`
/// com o clube `clube` (+4 = id do clube) e a cláusula de rebaixamento ainda por acionar (+0x1f = 1); senão 0.
unsafe fn contract_with_relegation_clause(this: usize, clube: usize, p: usize) -> usize {
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
unsafe fn club_and_b_team(clube: usize) -> [usize; 2] {
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
unsafe extern "thiscall" fn trigger_relegation_clauses(this: usize, clube: usize, liga: usize) {
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize) -> u32 = core::mem::transmute(0x8e3db0usize);
    for c in club_and_b_team(clube) {
        if c == 0 {
            continue;
        }
        for k in 0..50 {
            let p = cpk3_d(c + 0xd7 + k * 4);
            let contrato = contract_with_relegation_clause(this, c, p);
            if contrato != 0 {
                crate::comparison::branch("cpk_3 clausula de rebaixamento");
                *((contrato + 0x1f) as *mut u8) = 2; // a cláusula acionada
                noticia(cpk3_d(0xde1f54), p, liga, 3, contrato);
            }
        }
    }
}

/// A primeira oferta de contrato da lista `l` (sem mexer no cursor) da pessoa `p` (+0) ao clube `c` (+4).
unsafe fn first_offer_for_staff_club(l: usize, p: usize, c: usize) -> Option<usize> {
    list_items_capped(l).into_iter().find(|&e| cpk3_d(e) == p && cpk3_d(e + 4) == c)
}

/// { 0x4cca70 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract_offer(pessoa, clube)`: a
/// primeira oferta da lista (+0x34) da pessoa ao clube (ids): devolve a reserva de registro (+0x63 ≠ −1: 0x7a81a0
/// sobre [0xb7434c] com +0x67, +0x6b), tira a marca de contrato acertado se ela estava acertada (+0x50 = 2: 0x8c5430),
/// sai da lista (0x68d1f0) e as pendentes da pessoa (+8 do registro de 79 bytes em [0xae2d0c]) descem uma.
unsafe extern "thiscall" fn remove_contract_offer(this: usize, pessoa: *const u32, clube: *const u32) {
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
    crate::comparison::branch("cpk_3 tirar oferta de contrato");
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
unsafe extern "thiscall" fn is_player_suspended(this: usize, jogador: usize, comp: *const i32, extra: u32, partida: usize) -> u32 {
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
    crate::comparison::branch("cpk_3 suspenso pela competicao");
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
unsafe extern "thiscall" fn comparar_trigger_relegation_clauses(this: usize, clube: usize, liga: usize) {
    let mut alvos = Vec::new();
    for c in club_and_b_team(clube) {
        if c != 0 {
            alvos.extend((0..50).map(|k| contract_with_relegation_clause(this, c, cpk3_d(c + 0xd7 + k * 4))).filter(|&x| x != 0));
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(this, clube, liga);
    let fora: Vec<usize> = alvos.iter().copied().filter(|&x| *((x + 0x1f) as *const u8) != 2).collect();
    SCOREBOARD.record(fora.is_empty(), || format!("trigger_relegation_clauses({:x}) {} contratos, sem acionar {:x?}", clube, alvos.len(), fora));
}

/// Comparador: a lista esperada (sem a primeira oferta da pessoa ao clube) e as pendentes antes; a original tira (e
/// libera o item: nada dele é lido depois); confere os itens e o byte.
unsafe extern "thiscall" fn comparar_remove_contract_offer(this: usize, pessoa: *const u32, clube: *const u32) {
    let l = cpk3_d(this + 0x34);
    if l == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, *const u32, *const u32)>(%%I1%%)(this, pessoa, clube);
    }
    let n = (cpk3_d(0xae2d0c).wrapping_add((*pessoa as usize).wrapping_mul(79)) + 8) as *const u8;
    let alvo = first_offer_for_staff_club(l, *pessoa as usize, *clube as usize);
    let esperado: Vec<usize> = list_items_capped(l).into_iter().filter(|&e| Some(e) != alvo).collect();
    let pendentes = if alvo.is_some() { (*n).wrapping_sub(1) } else { *n };
    original_fn::<unsafe extern "thiscall" fn(usize, *const u32, *const u32)>(%%I1%%)(this, pessoa, clube);
    let depois = list_items_capped(l);
    SCOREBOARD.record(depois == esperado && *n == pendentes, || format!("remove_contract_offer({}, {}) itens {} (esperava {}), pendentes {} (esperava {})", *pessoa, *clube, depois.len(), esperado.len(), *n, pendentes));
}

/// Comparador: lado a lado pelas 20 vagas (+0x1e0) do clube do jogador (a 0x7e85b0 e a 0x600a50 só leem).
unsafe extern "thiscall" fn comparar_is_player_suspended(this: usize, jogador: usize, comp: *const i32, extra: u32, partida: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32, usize) -> u32>(%%I2%%);
    let clube = if jogador != 0 { cpk3_d(jogador + 0x39) } else { 0 };
    let regioes: &[(usize, usize)] = if clube != 0 { &[(clube + 0x1e0, 80)] } else { &[] };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(regioes, || o(this, jogador, comp, extra, partida), || is_player_suspended(this, jogador, comp, extra, partida));
    SCOREBOARD.record(igual && r == n, || format!("is_player_suspended({:x}, {}) original={} rust={} {:x?} x {:x?}", jogador, *comp, r, n, mo, mr));
    r
}
