// ---------- %%LEVA%%a leva: o fechamento das propostas de contrato da oferta, a fila da transferência livre e as negociações de troca do jogador (transfer_manager.cpp) ----------

/// O nó de uma lista do jogo (0x68d0d0/0x68d1b0): contagem u16 em +0, cabeça +2, cauda +6, cursor +0xa;
/// cada nó tem o dado em +0 e o próximo em +6.
unsafe fn llist_data_values(l: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = core::ptr::read_unaligned((l + 2) as *const usize);
    while no != 0 {
        v.push(*(no as *const usize));
        no = core::ptr::read_unaligned((no + 6) as *const usize);
    }
    v
}

/// 0x4cf5a0 (o contrato do jogador `pessoa`, thiscall em [0xae1050]) gravado em [0xae1080]; devolve o
/// registro do clube do contrato ([0xae23b4] + 581 × [contrato+4]) ou 0 sem contrato.
unsafe fn transfer_manager_contract_club(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(*(0xae1050 as *const usize), pessoa);
    *(0xae1080 as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    (*(0xae23b4 as *const usize)).wrapping_add((*((c + 4) as *const usize)).wrapping_mul(581))
}

/// { 0x8c6db0 } (stdcall, ret 4, transfer_manager.cpp): fecha (estado +0x50 = 4) as propostas de contrato
/// ligadas à oferta: a do jogador-alvo (0x4cc9e0 com o id +0xc e o clube +8) e a de cada jogador da troca
/// (lista +4, com o clube do contrato do alvo, 0x4cf5a0).
unsafe extern "stdcall" fn transfer_manager_close_offer_contract_offers(oferta: usize) {
    let id = *((oferta + 0xc) as *const usize);
    let pessoa = (*(0xae23bc as *const usize)).wrapping_add(id.wrapping_mul(110));
    let clube = transfer_manager_contract_club(pessoa);
    let proposta: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x4cc9e0usize);
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let r = proposta(*(0xae1050 as *const usize), id as u32, *((oferta + 8) as *const u32));
    if r != 0 {
        *((r + 0x50) as *mut u8) = 4;
    }
    let l = *((oferta + 4) as *const usize);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut u32, core::ptr::read_unaligned((l + 2) as *const u32));
    let mut e = prox(l);
    while e != 0 {
        // sem contrato do alvo a original lê [0] (cai); a cópia também
        let t = proposta(*(0xae1050 as *const usize), *(e as *const u32), core::ptr::read_volatile(clube as *const u32));
        if t != 0 {
            *((t + 0x50) as *mut u8) = 4;
        }
        e = prox(l);
    }
}

/// { 0x8c16b0 } (thiscall, ret 0xc, transfer_manager.cpp): põe o jogador na fila da transferência livre
/// (lista +0x92c, registro de 9 bytes: id, motivo, compensação). Emprestado (0x4cf6d0): a original
/// (caixa de erro).
unsafe fn queue_free_transfer_with(gerente: usize, pessoa: usize, motivo: u8, compensacao: u32, anexar: &mut dyn FnMut(usize, &[u8; 9])) -> bool {
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    if emprestado(*(0xae1050 as *const usize), pessoa) != 0 {
        return false;
    }
    let mut r = [0u8; 9];
    r[..4].copy_from_slice(&(*(pessoa as *const u32)).to_le_bytes());
    r[4] = motivo;
    r[5..].copy_from_slice(&compensacao.to_le_bytes());
    anexar(gerente + 0x92c, &r);
    true
}

unsafe extern "thiscall" fn queue_free_transfer(gerente: usize, pessoa: usize, motivo: u8, compensacao: u32) {
    let anexar_9: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    if !queue_free_transfer_with(gerente, pessoa, motivo, compensacao, &mut |l, r| anexar_9(l, r.as_ptr() as usize, 9)) {
        original_fn::<unsafe extern "thiscall" fn(usize, usize, u8, u32)>(%%I1%%)(gerente, pessoa, motivo, compensacao);
    }
}

/// { 0x8c5fc0 } (thiscall, ret 8, transfer_manager.cpp): conta as negociações ([+0xc], [+0x10] delas) do
/// clube do contrato do jogador (+8) cuja oferta ([gerente] + 4 × [+0x20]) segue aberta (estados 2, 3, 4,
/// 0x10 e 0x14 não; 0xf só com +0x2e < 3) e que têm o jogador na lista da troca (+4); com `saida`, põe
/// nela o índice da oferta (0x68d0d0, 4 bytes). Jogador fora do banco ou sem contrato: 0.
unsafe extern "thiscall" fn get_exchange_negotiations_for_player(gerente: usize, pessoa: usize, saida: usize) -> i32 {
    if pessoa != 0 && *(pessoa as *const i32) >= *(0xae2364 as *const i32) {
        return 0;
    }
    // como `transfer_manager_contract_club`, à parte para a mutação desta rotina não cair na 0x8c6db0
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(*(0xae1050 as *const usize), pessoa);
    *(0xae1080 as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    let p = (*(0xae23b4 as *const usize)).wrapping_add((*((c + 4) as *const usize)).wrapping_mul(581));
    if p == 0 {
        return 0;
    }
    let clube = *(p as *const u32);
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let anexar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    let negociacoes = *((gerente + 0xc) as *const usize);
    let mut total = 0;
    let mut i = 0;
    while i < *((gerente + 0x10) as *const i32) {
        let neg = *((negociacoes + i as usize * 4) as *const usize);
        i += 1;
        if neg == 0 || *((neg + 8) as *const u32) != clube {
            continue;
        }
        let k = *((neg + 0x20) as *const usize);
        let oferta = *((*(gerente as *const usize)).wrapping_add(k.wrapping_mul(4)) as *const usize);
        if oferta == 0 {
            continue;
        }
        match *((oferta + 0x2c) as *const u8) {
            2 | 3 | 4 | 0x10 | 0x14 => continue,
            0xf if *((oferta + 0x2e) as *const u8) >= 3 => continue,
            _ => {}
        }
        let l = *((neg + 4) as *const usize);
        if l == 0 {
            continue;
        }
        core::ptr::write_unaligned((l + 0xa) as *mut u32, core::ptr::read_unaligned((l + 2) as *const u32));
        let mut e = prox(l);
        while e != 0 && *(e as *const u32) != *(pessoa as *const u32) {
            e = prox(l);
        }
        if e == 0 {
            continue;
        }
        if saida != 0 {
            anexar(saida, neg + 0x20, 4);
        }
        total += 1;
    }
    total
}

unsafe extern "stdcall" fn compare_transfer_manager_close_offer_contract_offers(oferta: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(%%I0%%);
    // as propostas que podem fechar: as do id do alvo e dos ids da troca (o clube não filtra aqui)
    let mut ids = vec![*((oferta + 0xc) as *const u32)];
    let l = *((oferta + 4) as *const usize);
    if l != 0 {
        ids.extend(llist_data_values(l).iter().map(|&e| *(e as *const u32)));
    }
    let propostas = *((*(0xae1050 as *const usize) + 0x34) as *const usize);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if l != 0 {
        regioes.push((l + 0xa, 4));
    }
    if propostas != 0 {
        regioes.push((propostas + 0xa, 4));
        for r in llist_data_values(propostas) {
            if ids.contains(&*(r as *const u32)) {
                regioes.push((r + 0x50, 1));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regioes, || o(oferta), || transfer_manager_close_offer_contract_offers(oferta));
    SCOREBOARD.record(igual, || format!("transfer_manager_close_offer_contract_offers(id {}) {:x?} x {:x?}", ids[0], mo, mr));
}

unsafe extern "thiscall" fn compare_queue_free_transfer(gerente: usize, pessoa: usize, motivo: u8, compensacao: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u8, u32)>(%%I1%%);
    // ponytail: a fila aloca um nó (não roda duas vezes): a cópia só monta o registro (0x4cf6d0 é
    // consulta pura), a original põe na fila e confere-se o nó novo da cauda
    let mut registro = None;
    if !queue_free_transfer_with(gerente, pessoa, motivo, compensacao, &mut |_, r| registro = Some(*r)) {
        return o(gerente, pessoa, motivo, compensacao);
    }
    let l = gerente + 0x92c;
    let antes = core::ptr::read_unaligned(l as *const u16);
    o(gerente, pessoa, motivo, compensacao);
    let depois = core::ptr::read_unaligned(l as *const u16);
    let cauda = core::ptr::read_unaligned((l + 6) as *const usize);
    let novo = if cauda == 0 { None } else { Some(*(*(cauda as *const usize) as *const [u8; 9])) };
    SCOREBOARD.record(depois == antes.wrapping_add(1) && novo == registro,
        || format!("queue_free_transfer {} -> {} original={:x?} rust={:x?}", antes, depois, novo, registro));
}

unsafe extern "thiscall" fn compare_get_exchange_negotiations_for_player(gerente: usize, pessoa: usize, saida: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(%%I2%%);
    if saida != 0 {
        // ponytail: com lista de saída (só a tela 0x47ec79) a original anexa (aloca); só a original
        return o(gerente, pessoa, saida);
    }
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if pessoa != 0 && *(pessoa as *const i32) < *(0xae2364 as *const i32) {
        let guardado = *(0xae1080 as *const usize);
        let p = transfer_manager_contract_club(pessoa);
        *(0xae1080 as *mut usize) = guardado;
        if p != 0 {
            // os cursores das listas de troca das negociações do clube (as que a rotina pode percorrer)
            let clube = *(p as *const u32);
            let negociacoes = *((gerente + 0xc) as *const usize);
            for i in 0..(*((gerente + 0x10) as *const i32)).max(0) as usize {
                let neg = *((negociacoes + i * 4) as *const usize);
                if neg != 0 && *((neg + 8) as *const u32) == clube && *((neg + 4) as *const usize) != 0 {
                    regioes.push((*((neg + 4) as *const usize) + 0xa, 4));
                }
            }
        }
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&regioes, || o(gerente, pessoa, saida),
        || get_exchange_negotiations_for_player(gerente, pessoa, saida));
    SCOREBOARD.record(igual && r == n, || format!("get_exchange_negotiations_for_player original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}
