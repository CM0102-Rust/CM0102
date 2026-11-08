// ---------- %%LEVA%%a leva: a vaga no elenco para a oferta (o limite de 50 jogadores e das vagas da comissão, com a liberação do mais barato), a oferta de empréstimo recusada pelo clube e a cláusula de percentual da próxima venda achada (transfer_manager.cpp) ----------

unsafe fn cqa5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa5_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x8bfa30 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::reject_loan_offer(vaga, marcar, motivo)`,
/// pelo vendedor do computador que recusa todo lance de empréstimo (0x8c8090, oferta com +0x1f = 1) e pela oferta
/// agendada (0x8c63a0). Com `marcar`, +0x2d = 1; estado 2 (recusada) já de cara; o contrato da pessoa vai para
/// [0xae1080] e dá o clube do contrato; os lances do comprador pelo jogador e os do clube do contrato pelos da troca
/// saem (0x84aeb0); a notícia (0x8d5170); a oferta volta em 10 dias (0x8c6220) para sair; sem rival em aberto, a marca
/// de oferta da pessoa sai (+0x86e); as ofertas de contrato (0x4cca70 e, pela vaga +0x20, a 0x4ccb30 de cada jogador
/// da troca, cuja marca +4 zera), com +0x10 = 0 entre elas; por fim a situação na lista de observação do comprador
/// (0x849ce0; achada, 0x843e10 com evento 1 e tipo 0). É a recusa da transferência (0x8bf680) sem o desassossego, sem
/// a transferência futura, sem fechar as ofertas de contrato ligadas e sem a parte dos jogadores da troca no fim: os
/// ajudantes `cpx1_*` (com o ensaio OFFER_RESPONSE_DRY_RUN) servem igual.
unsafe extern "thiscall" fn reject_loan_offer(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = *vaga;
    if marcar != 0 {
        *((o + 0x2d) as *mut u8) = 1;
    }
    *((o + 0x2c) as *mut u8) = 2;
    crate::comparison::branch("cqa_5 loan refused");
    let vendedor = contract_club_stored(cpx1_pessoa(cpx1_d(o + 0xc)));
    clear_offer_and_exchange_markers(this, o, vendedor);
    transfer_manager_post_offer_rejected_news(this, o, motivo);
    schedule_offer_unless_dry_run(this, o, 0xa);
    clear_offer_mark_if_closed(this, o);
    // vendedor 0: sem a parte dos jogadores da troca na lista do comprador, que esta rotina não tem
    withdraw_offer_contract_offers(this, o, 0, 0);
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let comprador = cpx1_clube(cpx1_d(o + 8));
    let p = cpx1_pessoa(cpx1_d(o + 0xc));
    if entrada(shortlist_manager_of(this), comprador, p, (cpx1_b(o + 0x11) != 0xb) as u32, 0) != 0 {
        update_shortlist_status(this, p, comprador, 1, 0);
    }
}

/// Comparador da 0x8bfa30: o da recusa e da retirada (`compare_reject_withdraw`: a cópia em ensaio e a original do mesmo estado;
/// iguais o estado, +0x10, a marca +4 dos jogadores da troca e a marca de oferta da pessoa).
unsafe extern "thiscall" fn compare_reject_loan_offer(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(%%I0%%);
    compare_reject_withdraw("reject_loan_offer", this, vaga, || o(this, vaga, marcar, motivo), || reject_loan_offer(this, vaga, marcar, motivo))
}

/// O ensaio do comparador da 0x8cd450: a cópia roda sem liberar o jogador escolhido (0x86d800 e 0x8c1af0 mexem no
/// resto do jogo) e guarda quem seria em SQUAD_LIMITS_RELEASED_PLAYER.
static mut SQUAD_LIMITS_DRY_RUN: bool = false;
static mut SQUAD_LIMITS_RELEASED_PLAYER: usize = 0;

/// Os tipos 0x53b620 (11 a 15: jogador) e 0x53b610 (não 11), cdecl com o byte.
unsafe fn transfer_manager_is_player_job(t: u8) -> bool {
    let f: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
    f(t as i8) != 0
}

unsafe fn is_not_plain_player_job(t: u8) -> bool {
    let f: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b610usize);
    f(t as i8) != 0
}

/// As quatro contas de vagas da comissão pelo tipo da pessoa: [0] treinadores (8 e 0xf, +0x19f), [1] olheiros (9,
/// +0x1b3), [2] fisioterapeutas (0xa, +0x1cf), [3] auxiliar (6 e 0xd, +0xd3).
unsafe fn staff_role_slot_group(t: u8) -> Option<usize> {
    match t {
        8 | 0xf => Some(0),
        9 => Some(1),
        0xa => Some(2),
        6 | 0xd => Some(3),
        _ => None,
    }
}

/// O id da pessoa no vetor (dword com sinal) abaixo de [0xae2364]: a pessoa ainda existe.
unsafe fn staff_id_in_range(p: usize) -> bool {
    p != 0 && (cqa5_d(p) as i32) < cqa5_d(0xae2364) as i32
}

/// A falha com o motivo: (clube ≠ comprador) + `base` na palavra do motivo, se houver; 0.
unsafe fn fail_with_reason(clube: usize, o: usize, motivo: *mut u16, base: u16) -> u32 {
    if !motivo.is_null() {
        *motivo = (cqa5_d(clube) != cqa5_d(o + 8)) as u16 + base;
    }
    0
}

/// A pessoa está na fila da transferência livre ([0xde1f54] + 0x92c, LLIST de ids; o cursor +0xa volta a +2 e a busca
/// para no achado, como na original).
unsafe fn in_free_transfer_queue(p: usize) -> bool {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let l = cqa5_d(0xde1f54) + 0x92c;
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cqa5_d(l + 2));
    let mut e = proximo(l);
    while e != 0 {
        if cqa5_d(p) == cqa5_d(e) {
            return true;
        }
        e = proximo(l);
    }
    false
}

/// { 0x8cd450 } (thiscall, ret 0x10, transfer_manager.cpp; o mapa a dá como stdcall, mas os dois pontos de chamada na
/// 0x8cfc90 põem a gerência em ecx e ela o usa para a 0x8c1780 e a 0x8c1af0): `TRANSFER_MANAGER::check_squad_limits
/// (clube, oferta, contar_fila, motivo)`, das restrições da oferta (0x8cfc90), pelo comprador e pelo vendedor. Clube
/// nulo: a caixa da linha 0x2c3b e 0. O parceiro do clube (0x540a50 com 0 e 1). Pelo comprador (+8 da oferta = id do
/// clube): a pessoa da oferta já é dele (contrato de tipo 5, de novo pela 0x4cf5a0, com o clube; ou a ficha de
/// empréstimo 0x4cf6d0 com +4 = o clube) conta −1; jogador pelo tipo +0x11 da oferta conta +1; o tipo da comissão
/// pelo mesmo byte marca 1; cada jogador da troca (+4, LLIST de ids) que sai desconta (jogador −1, comissão −1 pelo
/// +0x3d da pessoa). Pelo vendedor: sem troca, 1; o jogador vendido conta −1 e o tipo da comissão −1 (pelo +0x3d); cada
/// nó da troca que chega soma pelo byte +6. Conta positiva: soma os 50 do elenco (+0xd7, ponteiros com id vivo) do clube
/// e do parceiro; acima de 50, menos a fila livre do clube (0x8c1780) se `contar_fila`; ainda acima de 50: fora do
/// comprador do computador sem troca, o motivo (clube ≠ comprador) + 4 e 0; senão o mais barato do elenco ((+0x3d do
/// contrato + 100) × valor +0x52, com +0x3d ≤ 0, sem ficha, sem contrato de tipo 5 em [0xae1080], sem ofertas 0x8c5cf0,
/// trocas 0x8c5e40 ou transferência futura 0x8cca10, fora da fila livre e de tipo 0xb, 0xf ou 0xd) sai de graça
/// (0x86d800 em [0xdd7f00], 0x8c1af0 com −1); nenhum, o motivo + 4 e 0. Depois as vagas da comissão marcadas: 5
/// treinadores (+0x19f), 3 fisioterapeutas (+0x1cf), 7 olheiros (+0x1b3) — os do clube contam por ponteiro, os do
/// parceiro só com id vivo — e o auxiliar (+0xd3 do clube ou do parceiro ocupado); passou do limite, o motivo + 0x22 e 0.
/// Devolve 1.
unsafe extern "thiscall" fn check_squad_limits(this: usize, clube: usize, o: usize, contar_fila: u32, motivo: *mut u16) -> u32 {
    if clube == 0 {
        show_error_at(0xa8d514, 0x2c3b);
        return 0;
    }
    let parceiro_de: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let parceiro = parceiro_de(clube, core::ptr::null_mut(), 1);
    let mut n: i8 = 0;
    let mut com = [0i8; 4];
    if cqa5_d(clube) == cqa5_d(o + 8) {
        let p = cpx1_pessoa(cqa5_d(o + 0xc));
        let c = transfer_manager_get_contract_and_store(p);
        let mut proprio = false;
        if c != 0 && cqa5_b(c + 0x35) & 0x3f == 5 {
            proprio = contract_club_stored(p) == clube;
        }
        if !proprio && ficha(cqa5_d(0xae1050), p) != 0 && cqa5_d(ficha(cqa5_d(0xae1050), p) + 4) == cqa5_d(clube) {
            proprio = true;
        }
        if proprio {
            n = -1;
        }
        let t = cqa5_b(o + 0x11);
        if transfer_manager_is_player_job(t) {
            n = n.wrapping_add(1); // cqa5: o jogador da oferta conta
        }
        if is_not_plain_player_job(t) {
            if let Some(k) = staff_role_slot_group(t) {
                com[k] = 1;
            }
        }
        let l = cqa5_d(o + 4);
        if l != 0 {
            for e in llist_items_from_start(l) {
                let t = cqa5_b(cpx1_pessoa(cqa5_d(e)) + 0x3d);
                if transfer_manager_is_player_job(t) {
                    n = n.wrapping_sub(1);
                }
                if is_not_plain_player_job(t) {
                    if let Some(k) = staff_role_slot_group(t) {
                        com[k] = com[k].wrapping_sub(1);
                    }
                }
            }
        }
    } else {
        if cqa5_d(o + 4) == 0 {
            crate::comparison::branch("cqa_5 seller without exchange");
            return 1; // cqa5: o vendedor sem troca tem vaga
        }
        let t = cqa5_b(cpx1_pessoa(cqa5_d(o + 0xc)) + 0x3d);
        if transfer_manager_is_player_job(t) {
            n = -1;
        }
        if is_not_plain_player_job(t) {
            if let Some(k) = staff_role_slot_group(t) {
                com[k] = -1;
            }
        }
        for e in llist_items_from_start(cqa5_d(o + 4)) {
            let t = cqa5_b(e + 6);
            if transfer_manager_is_player_job(t) {
                n = n.wrapping_add(1);
            }
            if is_not_plain_player_job(t) {
                if let Some(k) = staff_role_slot_group(t) {
                    com[k] = com[k].wrapping_add(1);
                }
            }
        }
    }
    if n > 0 {
        let contar = |base: usize, n: &mut i8| {
            for k in 0..0x32 {
                if staff_id_in_range(cqa5_d(base + 0xd7 + 4 * k)) {
                    *n = n.wrapping_add(1); // cqa5: cada um do elenco conta
                }
            }
        };
        crate::comparison::branch("cqa_5 counts the squad");
        contar(clube, &mut n);
        if parceiro != 0 {
            contar(parceiro, &mut n);
        }
        if n > 0x32 { // cqa5: o limite de 50 no elenco
            if contar_fila != 0 {
                let fila: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x8c1780usize);
                n = n.wrapping_sub(fila(this, clube) as i8);
            }
            if n > 0x32 {
                let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
                if cqa5_d(clube) != cqa5_d(o + 8) || humano(clube, 1, 1, 0, 0) != 0 || cqa5_d(o + 4) != 0 {
                    crate::comparison::branch("cqa_5 squad full");
                    return fail_with_reason(clube, o, motivo, 4);
                }
                let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
                let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
                let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
                let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8cca10usize);
                let mut melhor = 1000000.0f64;
                let mut escolhido = 0usize;
                for k in 0..0x32 {
                    let p = cqa5_d(clube + 0xd7 + 4 * k);
                    if !staff_id_in_range(p) || ficha(cqa5_d(0xae1050), p) != 0 {
                        continue;
                    }
                    // a primeira consulta do contrato não vai para [0xae1080]
                    let c = contrato(cqa5_d(0xae1050), p);
                    if c == 0 {
                        continue;
                    }
                    let s = cqa5_b(c + 0x3d) as i8;
                    let v = (s as f64 + 100.0) * (cqa5_d(p + 0x52) as i32 as f64);
                    if !(v < melhor) || s > 0 {
                        continue;
                    }
                    let c = transfer_manager_get_contract_and_store(p);
                    if c != 0 && cqa5_b(c + 0x35) & 0x3f == 5 {
                        continue;
                    }
                    if ficha(cqa5_d(0xae1050), p) != 0 {
                        continue;
                    }
                    let g = cqa5_d(0xde1f54);
                    if ofertas(g, p, 0, 0) != 0 || trocas(g, p, 0) != 0 || futura(g, p, 0) != 0 {
                        continue;
                    }
                    if in_free_transfer_queue(p) {
                        continue;
                    }
                    if matches!(cqa5_b(p + 0x3d), 0xb | 0xf | 0xd) {
                        escolhido = p;
                        melhor = v;
                    }
                }
                if escolhido == 0 {
                    return fail_with_reason(clube, o, motivo, 4);
                }
                crate::comparison::branch("cqa_5 releases the cheapest");
                if SQUAD_LIMITS_DRY_RUN {
                    SQUAD_LIMITS_RELEASED_PLAYER = escolhido;
                } else {
                    let registrar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x86d800usize);
                    let liberar: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) = core::mem::transmute(0x8c1af0usize);
                    registrar(cqa5_d(0xdd7f00), escolhido);
                    liberar(this, escolhido, clube, 0, -1, 0);
                }
            }
        }
    }
    // as vagas da comissão: (conta, deslocamento, vagas)
    for (k, base, vagas) in [(0usize, 0x19fusize, 5usize), (2, 0x1cf, 3), (1, 0x1b3, 7)] {
        let mut m = com[k];
        if m <= 0 {
            continue;
        }
        for j in 0..vagas {
            if cqa5_d(clube + base + 4 * j) != 0 {
                m = m.wrapping_add(1);
            }
        }
        if parceiro != 0 {
            for j in 0..vagas {
                if staff_id_in_range(cqa5_d(parceiro + base + 4 * j)) {
                    m = m.wrapping_add(1);
                }
            }
        }
        if m > vagas as i8 {
            return fail_with_reason(clube, o, motivo, 0x22);
        }
    }
    if com[3] > 0 && (cqa5_d(clube + 0xd3) != 0 || (parceiro != 0 && cqa5_d(parceiro + 0xd3) != 0)) {
        return fail_with_reason(clube, o, motivo, 0x22);
    }
    1
}

/// Comparador da 0x8cd450: a cópia em ensaio (sem liberar) e depois a original, do mesmo estado ([0xae1080], o motivo,
/// os cursores da fila livre e da troca). Sem liberação: iguais a devolução e essas regiões. Com liberação (a cópia
/// guardou o escolhido): iguais a devolução e o escolhido ter saído do clube (o contrato dele, pela 0x4cf5a0 que só
/// lê, sem o clube); o resto a original refaz ao liberar. Clube nulo: a caixa, só a original.
unsafe extern "thiscall" fn compare_check_squad_limits(this: usize, clube: usize, o: usize, contar_fila: u32, motivo: *mut u16) -> u32 {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u16) -> u32>(%%I1%%);
    if clube == 0 {
        return f(this, clube, o, contar_fila, motivo);
    }
    let mut regioes = vec![(0xae1080usize, 4usize), (cqa5_d(0xde1f54) + 0x936, 4)];
    if !motivo.is_null() {
        regioes.push((motivo as usize, 2));
    }
    if cqa5_d(o + 4) != 0 {
        regioes.push((cqa5_d(o + 4) + 0xa, 4));
    }
    let antes = crate::comparison::MemorySnapshot::read(&regioes);
    SQUAD_LIMITS_DRY_RUN = true;
    SQUAD_LIMITS_RELEASED_PLAYER = 0;
    let n = crate::rng::pause_trace(|| check_squad_limits(this, clube, o, contar_fila, motivo));
    SQUAD_LIMITS_DRY_RUN = false;
    let liberado = SQUAD_LIMITS_RELEASED_PLAYER;
    let mr = crate::comparison::MemorySnapshot::read(&regioes);
    antes.write_back();
    let r = f(this, clube, o, contar_fila, motivo);
    let mo = crate::comparison::MemorySnapshot::read(&regioes);
    let igual = if liberado == 0 {
        mo.0 == mr.0
    } else {
        let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let c = contrato(cqa5_d(0xae1050), liberado);
        c == 0 || cpx1_clube(cqa5_d(c + 4)) != clube
    };
    SCOREBOARD.record(igual && r == n, || {
        format!("check_squad_limits(clube {:x}, oferta {:x}, fila {}) original={} rust={} liberado={:x} {:x?} x {:x?}", clube, o, contar_fila, r, n, liberado, mo, mr)
    });
    r
}

/// { 0x8d0dd0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_next_sale_percentage(pessoa)`, das
/// telas da pessoa (0x884ea2): sem a marca 8 da pessoa ([this+0x86e] + id × 8), 0. O contrato dela (0x4cf5a0, em
/// [0xae1080]) e o clube; sem os dois, 0. A busca binária (0x946464: cdecl(chave, vetor [this+0x880], conta
/// [this+0x888], 0xd, 0x8e54d0)) pela chave com o id do clube (+1, de novo pelo contrato) e o da pessoa (+9), os dois
/// únicos campos que a comparação lê; achada, devolve o registro; senão a marca 8 sai e 0.
unsafe extern "thiscall" fn find_next_sale_percentage(this: usize, p: usize) -> usize {
    let m = (cqa5_d(this + 0x86e).wrapping_add(cqa5_d(p).wrapping_mul(8))) as *mut u8;
    if *m & 8 == 0 {
        return 0;
    }
    if contract_club_stored(p) == 0 {
        return 0;
    }
    let mut chave = [0u8; 0x10];
    chave[9..13].copy_from_slice(&(cqa5_d(p) as u32).to_le_bytes());
    // ponytail: a original lê o id do clube sem conferir o contrato (acabou de achá-lo); sem ele, aqui 0
    let c = contract_club_stored(p);
    let id = if c != 0 { cqa5_d(c) as u32 } else { 0 };
    chave[1..5].copy_from_slice(&id.to_le_bytes());
    let bsearch: unsafe extern "C" fn(*const u8, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let r = bsearch(chave.as_ptr(), cqa5_d(this + 0x880), cqa5_d(this + 0x888), 0xd, 0x8e54d0);
    if r != 0 {
        return r;
    }
    *m = (*m).wrapping_sub(8);
    0
}

/// Comparador da 0x8d0dd0: lado a lado na marca da pessoa, em [0xae1080] e na devolução (só lê o resto).
unsafe extern "thiscall" fn compare_find_next_sale_percentage(this: usize, p: usize) -> usize {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    let m = cqa5_d(this + 0x86e).wrapping_add(cqa5_d(p).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(m, 1), (0xae1080, 4)], || f(this, p), || find_next_sale_percentage(this, p));
    SCOREBOARD.record(igual && r == n, || format!("find_next_sale_percentage(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo, mr));
    r
}
