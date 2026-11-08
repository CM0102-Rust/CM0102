// ---------- %%LEVA%%a leva: a situação da pessoa nas listas de observação dos clubes, a marca de lista de empréstimos e o teto de preço pelo estatuto (shortlist_manager.cpp) ----------

unsafe fn cpy2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy2_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpy2_clube(id: usize) -> usize {
    cpy2_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A entrada do clube no Shortlist_Manager (+0xdc, 9 bytes por clube: +0 as 40 vagas de 0x25 bytes, +4 o estado, +8).
unsafe fn cpy2_entrada(this: usize, clube: usize) -> usize {
    cpy2_d(this + 0xdc).wrapping_add(cpy2_d(clube).wrapping_mul(9))
}

/// As datas estáticas da 0x54c5b0, da 0x54c8e0 e da 0x54cdb0 (valor e guarda).
const STAFF_STATUS_DATE_STATICS: [(usize, usize); 6] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2470, 8), (0xae2418, 1)];

/// Tira o par (pessoa, clube) de um dos vetores ordenados de pares (8 bytes) do Shortlist_Manager (`base` = +0 ou +4,
/// conta em `base` + 0x10) pela busca binária (0x946464: cdecl(chave, vetor, conta, 8, comparação), add esp, 0x14):
/// os de depois descem uma casa, o último fica zerado e a conta cai. Devolve se achou.
unsafe fn remove_sorted_pair(base: usize, chave: &[u32; 2], comparar: usize) -> bool {
    let bsearch: unsafe extern "C" fn(*const u32, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let p = bsearch(chave.as_ptr(), cpy2_d(base), cpy2_d(base + 0x10) as u32, 8, comparar);
    if p == 0 {
        return false;
    }
    let mut i = (p.wrapping_sub(cpy2_d(base)) >> 3) as i32;
    while i < cpy2_d(base + 0x10) as i32 - 1 {
        let v = cpy2_d(base);
        i += 1;
        let k = v.wrapping_add(i as usize * 8);
        cpy2_pd(k - 8, cpy2_d(k) as u32);
        cpy2_pd(k - 4, cpy2_d(k + 4) as u32);
    }
    let fim = cpy2_d(base).wrapping_add(cpy2_d(base + 0x10).wrapping_mul(8));
    cpy2_pd(fim - 4, 0);
    cpy2_pd(fim - 8, 0);
    cpy2_pd(base + 0x10, (cpy2_d(base + 0x10) as u32).wrapping_sub(1));
    true
}

/// A data de hoje mais (0x54c5b0) ou menos (0x54c8e0) `dias` (thiscall(0xae2c90, saída, dias), ret 8) copiada para a
/// data da vaga em +0x1b (0x417c10: thiscall(destino, saída, origem), ret 8).
unsafe fn set_slot_date(v: usize, rotina: usize, dias: u32) {
    let deslocar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(rotina);
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    let r = deslocar(0xae2c90, &mut a, dias);
    copiar(v + 0x1b, &mut b, r);
}

/// O par (pessoa, clube) do evento 2 ou 5: a vaga guarda a pessoa em +0x13 (bloqueada até a data de +0x1b), sem lance
/// (+0x17 = −1) e sem valores (+0xb, +7); o estatuto da vaga (bits 1..4 de +0x24) vai ao nibble alto de +0x4f do
/// contrato da pessoa no clube; a entrada fica com 100 em +8.
unsafe fn block_slot_for_person(v: usize, entrada: usize, pessoa: usize, contrato: usize, rotina: usize, dias: u32) {
    set_slot_date(v, rotina, dias);
    cpy2_pd(v + 0x13, cpy2_d(pessoa) as u32);
    cpy2_pd(v + 0x17, 0xffff_ffff);
    cpy2_pd(v + 0xb, 0);
    cpy2_pd(v + 7, 0);
    if contrato != 0 {
        let c = (contrato + 0x4f) as *mut u8;
        *c = (*c & 0xf) | (((cpy2_b(v + 0x24) >> 1) & 0xf) << 4);
    }
    *((entrada + 8) as *mut u8) = 0x64;
}

/// { 0x843e10 } (thiscall, ret 0x10, shortlist_manager.cpp, quadro SEH da LLIST local): `Shortlist_Manager::
/// update_staff_status(pessoa, clube, evento, tipo)`, pelas ofertas, contratos, técnicos e regens (0x4d07e0, 0x690730,
/// 0x7a59e0, 0x8bf680, 0x8c44e0...) e por ela mesma. Com o evento 4 e o clube nulo ou o da pessoa (+0x39): cada clube
/// interessado (0x842180 com 0, 3, 0) recebe o evento 1 com o tipo 2 (recursão). Clube nulo: 1. Com o evento 5, ou 1..3
/// de outro clube: o par (pessoa, clube) sai dos dois vetores ordenados (+0/+0x10 pela 0x757ea0, +4/+0x14 pela
/// 0x844740; vetor nulo: 0); achado nos dois e o clube não humano (0x600a50 com 0, 1), o nibble baixo de +0xb do
/// registro de 79 bytes da pessoa ([0xae2d0c]) cai um. Depois, nas 40 vagas do clube (tipo 1: fora as de jogador 0xb;
/// tipo 0: só elas) com a pessoa no alvo (+0xf; no evento 4, também em +0x13): 1 = a vaga se esvazia e é preenchida de
/// novo (0x83f3a0); 2 e 5 = bloqueio de 120 e 14 dias (0x54c5b0); 3 = esvazia, data de hoje − 120 (0x54c8e0); 4 =
/// solta +0x13 e o alvo, data de hoje − 120. Devolve 1.
unsafe extern "thiscall" fn update_staff_status(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) -> u8 {
    // 0x68d020/0x68d040/0x68d1b0: thiscall(lista); 0x842180: thiscall(this, pessoa, lista, 0, 3, 0) (5 push); a
    // recursão: thiscall(this, pessoa, clube, 1, 2) (4 push); 0x600a50: cdecl(clube, 0, 1, 0, 0) (add esp, 0x14);
    // 0x4cf5f0: thiscall([0xae1050], pessoa, clube) (2 push); 0x83f3a0: thiscall(this, vaga, clube, pessoa) (3 push)
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x842180usize);
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(0x843e10usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x83f3a0usize);
    let mut lista = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let l = lista.as_mut_ptr() as usize;
    lista_nova(l);
    let ev = evento as u8;
    let id = cpy2_d(pessoa);
    if (clube == 0 || clube == cpy2_d(pessoa + 0x39)) && ev == 4 {
        interessados(this, pessoa, l, 0, 3, 0);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, cpy2_d(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            situacao(this, pessoa, cpy2_clube(cpy2_d(e + 4)), 1, 2);
            e = proximo(l);
        }
    }
    if clube == 0 {
        destruir(l);
        return 1;
    }
    if ev == 5 || (ev != 4 && clube != cpy2_d(pessoa + 0x39)) {
        if cpy2_d(this) == 0 || cpy2_d(this + 4) == 0 {
            destruir(l);
            return 0;
        }
        let chave = [id as u32, cpy2_d(clube) as u32];
        let mut faltou = !remove_sorted_pair(this, &chave, 0x757ea0);
        faltou |= !remove_sorted_pair(this + 4, &chave, 0x844740);
        if humano(clube, 0, 1, 0, 0) == 0 && !faltou {
            crate::comparacao::ramo("cpy_2 contagem da lista");
            let r = (cpy2_d(0xae2d0c).wrapping_add(id.wrapping_mul(0x4f)) + 0xb) as *mut u8;
            *r = (*r & 0xf0) | ((*r).wrapping_sub(1) & 0xf); // cpy2: uma lista a menos na conta da pessoa
        }
    }
    let entrada = cpy2_entrada(this, clube);
    let contrato = contract_of(cpy2_d(0xae1050), pessoa, clube);
    for i in 0..40usize {
        let v = cpy2_d(entrada).wrapping_add(i * 0x25);
        let t = tipo as u8;
        if t == 1 {
            if cpy2_b(v) == 0xb {
                continue;
            }
        } else if t == 0 && cpy2_b(v) != 0xb {
            continue;
        }
        if cpy2_d(v + 0xf) != id && (ev != 4 || cpy2_d(v + 0x13) != id) {
            continue;
        }
        match ev {
            1 => {
                cpy2_pd(v + 0xf, 0xffff_ffff);
                cpy2_pd(v + 0x17, 0xffff_ffff);
                if cpy2_d(v + 0x13) == id {
                    cpy2_pd(v + 0x13, 0xffff_ffff);
                }
                *((v + 0x24) as *mut u8) &= 0xfe;
                crate::comparacao::ramo("cpy_2 vaga refeita");
                preencher(this, v, clube, pessoa); // cpy2: a vaga é preenchida de novo
            }
            2 => block_slot_for_person(v, entrada, pessoa, contrato, 0x54c5b0, 0x78),
            5 => {
                crate::comparacao::ramo("cpy_2 bloqueio de 14 dias");
                block_slot_for_person(v, entrada, pessoa, contrato, 0x54c5b0, 0xe); // cpy2: o bloqueio de 14 dias
            }
            3 => {
                cpy2_pd(v + 0xf, 0xffff_ffff);
                cpy2_pd(v + 0x17, 0xffff_ffff);
                set_slot_date(v, 0x54c8e0, 0x78);
                if cpy2_d(v + 0x13) == id {
                    cpy2_pd(v + 0x13, 0xffff_ffff);
                }
            }
            4 => {
                if cpy2_d(v + 0x13) == id {
                    cpy2_pd(v + 0x13, 0xffff_ffff);
                }
                if cpy2_d(v + 0xf) == id {
                    cpy2_pd(v + 0xf, 0xffff_ffff);
                    cpy2_pd(v + 0x17, 0xffff_ffff);
                    set_slot_date(v, 0x54c8e0, 0x78);
                }
            }
            _ => {}
        }
    }
    destruir(l);
    1
}

/// O caso que não pode rodar duas vezes: a recursão do evento 4 pelos clubes interessados, ou o evento 1 com a pessoa no
/// alvo de alguma vaga do clube (a 0x83f3a0 preenche a vaga: sorteia, acrescenta pares com realocação e dá notícias).
unsafe fn is_original_only_case(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) -> bool {
    let ev = evento as u8;
    if ev == 4 && (clube == 0 || clube == cpy2_d(pessoa + 0x39)) {
        return true;
    }
    if ev != 1 || clube == 0 {
        return false;
    }
    // de outro clube, o evento 1 passa pelos vetores; um deles nulo devolve 0 antes das vagas
    if clube != cpy2_d(pessoa + 0x39) && (cpy2_d(this) == 0 || cpy2_d(this + 4) == 0) {
        return false;
    }
    let entrada = cpy2_entrada(this, clube);
    (0..40usize).any(|i| {
        let v = cpy2_d(entrada).wrapping_add(i * 0x25);
        let ok = match tipo as u8 {
            1 => cpy2_b(v) != 0xb,
            0 => cpy2_b(v) == 0xb,
            _ => true,
        };
        ok && cpy2_d(v + 0xf) == cpy2_d(pessoa)
    })
}

/// Comparador: lado a lado pelos dois vetores de pares e suas contas, pelo byte da conta de listas da pessoa, pelas 40
/// vagas e o +8 da entrada do clube, pelo byte +0x4f do contrato e pelas datas estáticas, e a devolução. Na recursão do
/// evento 4 e no evento 1 que preenche a vaga, só a original (a cópia leva os ramos).
unsafe extern "thiscall" fn comparar_update_staff_status(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8>(%%I0%%);
    if is_original_only_case(this, pessoa, clube, evento, tipo) {
        return o(this, pessoa, clube, evento, tipo);
    }
    let mut reg = STAFF_STATUS_DATE_STATICS.to_vec();
    for base in [this, this + 4] {
        reg.push((base + 0x10, 4));
        if cpy2_d(base) != 0 {
            reg.push((cpy2_d(base), cpy2_d(base + 0x10).wrapping_mul(8)));
        }
    }
    reg.push((cpy2_d(0xae2d0c).wrapping_add(cpy2_d(pessoa).wrapping_mul(0x4f)) + 0xb, 1));
    if clube != 0 {
        let entrada = cpy2_entrada(this, clube);
        reg.push((cpy2_d(entrada), 40 * 0x25));
        reg.push((entrada + 8, 1));
        let contract_of: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
        let c = contract_of(cpy2_d(0xae1050), pessoa, clube);
        if c != 0 {
            reg.push((c + 0x4f, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, clube, evento, tipo),
        || update_staff_status(this, pessoa, clube, evento, tipo));
    PLACAR.registrar(igual && r == n, || format!("update_staff_status(pessoa {:x}, clube {:x}, evento {}, tipo {}) original={} rust={} {:x?} x {:x?}",
        pessoa, clube, evento as u8, tipo as u8, r, n, mo, mr));
    r
}

/// { 0x8442b0 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_manager_add_to_loan_list(pessoa)`, pelas telas de
/// transferência (0x8f6770, 0x8f8120). Com contrato (0x4cf5a0, sem guardar em [0xae1080]), o bit 1 de +0x4e (na lista
/// de empréstimos) liga. Devolve o eax que a original deixa: o contrato.
unsafe extern "stdcall" fn shortlist_manager_add_to_loan_list(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cpy2_d(0xae1050), pessoa);
    if c != 0 {
        *((c + 0x4e) as *mut u8) |= 2;
    }
    c
}

/// Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
unsafe extern "stdcall" fn comparar_shortlist_manager_add_to_loan_list(pessoa: usize) -> usize {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> usize>(%%I1%%);
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cpy2_d(0xae1050), pessoa);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || shortlist_manager_add_to_loan_list(pessoa));
    PLACAR.registrar(igual && r == n, || format!("shortlist_manager_add_to_loan_list(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// { 0x84b1f0 } (thiscall, ret 0xc, shortlist_manager.cpp): `shortlist_manager_max_fee_for_player_status(clube, pessoa,
/// estatuto)`, pela tela de contrato (0x4dd0f0, ecx = o Shortlist_Manager em +0x856 da gerência de transferências).
/// Sem pessoa ou sem ficha de jogador (+0x61): −1. Senão o teto da vaga (0x84b230) com as posições fortes do jogador
/// (0x5b7d20) e o estatuto.
unsafe extern "thiscall" fn shortlist_manager_max_fee_for_player_status(this: usize, clube: usize, pessoa: usize, estatuto: u32) -> i32 {
    // 0x5b7d20: cdecl(ficha) (add esp, 4); 0x84b230: chamada com mov ecx, esi (o this) e 3 push (clube, posições,
    // estatuto), ret 0xc: thiscall daqui (a cópia stdcall do motor não lê o ecx)
    if pessoa == 0 || cpy2_d(pessoa + 0x61) == 0 {
        return -1;
    }
    let posicoes: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5b7d20usize);
    let teto: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x84b230usize);
    let m = posicoes(cpy2_d(pessoa + 0x61));
    teto(this, clube, m, estatuto)
}

/// Comparador: a devolução (rotina pura).
unsafe extern "thiscall" fn comparar_shortlist_manager_max_fee_for_player_status(this: usize, clube: usize, pessoa: usize, estatuto: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32>(%%I2%%);
    let r = o(this, clube, pessoa, estatuto);
    let n = shortlist_manager_max_fee_for_player_status(this, clube, pessoa, estatuto);
    PLACAR.registrar(r == n, || format!("shortlist_manager_max_fee_for_player_status(clube {:x}, pessoa {:x}, estatuto {}) original={} rust={}", clube, pessoa, estatuto as i8, r, n));
    r
}
