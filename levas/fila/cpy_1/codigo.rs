// ---------- %%LEVA%%a leva: as vagas da lista de observação com lance, e as marcas de lista de transferências e de empréstimos do contrato (shortlist_manager.cpp) ----------

unsafe fn cpy1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy1_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// O contrato principal da pessoa (CONTRACT_MANAGER::get_contract 0x4cf5a0: thiscall([0xae1050], pessoa), ret 4).
/// As três marcas não guardam o contrato em [0xae1080].
unsafe fn contract_of_staff(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    f(cpy1_d(0xae1050), pessoa)
}

/// As datas estáticas da 0x54c5b0, da 0x54c8e0 e da 0x54cdb0 (valor e guarda).
const SHORTLIST_MANAGER_DATE_STATICS: [(usize, usize); 6] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2470, 8), (0xae2418, 1)];

/// A entrada do clube no Shortlist_Manager (+0xdc, 9 bytes por clube: +0 as 40 vagas de 0x25 bytes, +4 o estado).
unsafe fn cpy1_entrada(this: usize, clube: usize) -> usize {
    cpy1_d(this + 0xdc).wrapping_add(cpy1_d(clube).wrapping_mul(9))
}

/// { 0x844540 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::count_slots_with_bids(clube, posicao)`,
/// pela avaliação das listas (0x84a020, com 0xb), pela avaliação do elenco (0x83cb40, com −1) e pelo laço do dia
/// (0x5cccd0, com −1). Das 40 vagas da lista do clube, conta as que têm alvo (+0xf) e lance (+0x17), as duas acima de
/// −1, e posição (+0) igual a `posicao` (0xff: qualquer). Com a lista no estado 2 (+4 da entrada) e o clube do humano
/// (0x600a50 com 1, 1): se hoje − 20 dias (0x54c8e0) passa da data de chegada do técnico (+0x3e da ficha em +0xcf do
/// clube) e o alvo não tem ofertas (0x8c5cf0), a vaga perde o alvo e o lance (−1) e não conta. A conta é um byte.
unsafe extern "thiscall" fn count_slots_with_bids(this: usize, clube: usize, posicao: u32) -> u8 {
    // 0x600a50: cdecl(clube, 1, 1, 0, 0) (add esp, 0x14 depois); 0x54c8e0: thiscall(0xae2c90, saída, 0x14); 0x8c5cf0:
    // thiscall([0xde1f54], pessoa, 0, 0)
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let menos_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let entrada = cpy1_entrada(this, clube);
    let mut n: u8 = 0;
    for i in 0..40usize {
        let v = cpy1_d(entrada).wrapping_add(i * 0x25);
        if (cpy1_d(v + 0xf) as i32) <= -1 {
            continue;
        }
        if (cpy1_d(v + 0x17) as i32) <= -1 {
            crate::comparison::branch("cpy_1 alvo sem lance");
            continue; // cpy1: alvo sem lance não conta
        }
        if posicao as u8 != 0xff && cpy1_b(v) != posicao as u8 {
            crate::comparison::branch("cpy_1 outra posicao");
            continue; // cpy1: lance de outra posição não conta
        }
        if cpy1_b(entrada + 4) == 2 && humano(clube, 1, 1, 0, 0) != 0 {
            let mut buf = [0u8; 8];
            let r = menos_dias(0xae2c90, &mut buf, 0x14);
            let dia = core::ptr::read_unaligned(r as *const i16);
            let ano = cpy1_w(r + 2);
            let chegada = cpy1_d(clube + 0xcf).wrapping_add(0x3e);
            let ano_c = cpy1_w(chegada + 2);
            let velho = ano > ano_c || (ano == ano_c && dia > core::ptr::read_unaligned(chegada as *const i16));
            if velho {
                let alvo = cpy1_d(0xae23bc).wrapping_add(cpy1_d(v + 0xf).wrapping_mul(0x6e));
                if ofertas(cpy1_d(0xde1f54), alvo, 0, 0) == 0 {
                    crate::comparison::branch("cpy_1 lance vencido");
                    *((v + 0xf) as *mut i32) = -1; // cpy1: a vaga perde o alvo
                    *((v + 0x17) as *mut i32) = -1;
                    continue;
                }
            }
        }
        crate::comparison::branch("cpy_1 vaga com lance");
        n = n.wrapping_add(1); // cpy1: conta a vaga com lance
    }
    n
}

/// Comparador: lado a lado pelas 40 vagas do clube e pelas datas estáticas, e a devolução.
unsafe extern "thiscall" fn comparar_count_slots_with_bids(this: usize, clube: usize, posicao: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(%%I0%%);
    let mut reg = SHORTLIST_MANAGER_DATE_STATICS.to_vec();
    reg.push((cpy1_d(cpy1_entrada(this, clube)), 40 * 0x25));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, clube, posicao), || count_slots_with_bids(this, clube, posicao));
    PLACAR.record(igual && r == n, || format!("count_slots_with_bids(clube {:x}, posicao {}) original={} rust={} {:x?} x {:x?}", clube, posicao as i8, r, n, mo, mr));
    r
}

/// { 0x844280 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_manager_remove_from_transfer_list(pessoa)`, pelas
/// telas de transferência (0x8f6770, 0x8f8120). Sem contrato (0x4cf5a0): 0. Com ele, o bit 0 de +0x4e (na lista de
/// transferências) sai e devolve 1 no byte baixo (os de cima são os do ponteiro do contrato, como a original).
unsafe extern "stdcall" fn shortlist_manager_remove_from_transfer_list(pessoa: usize) -> u32 {
    let c = contract_of_staff(pessoa);
    if c == 0 {
        return 0;
    }
    *((c + 0x4e) as *mut u8) &= 0xfe;
    (c as u32 & !0xff) | 1
}

/// Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
unsafe extern "stdcall" fn comparar_shortlist_manager_remove_from_transfer_list(pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(%%I1%%);
    let c = contract_of_staff(pessoa);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(pessoa), || shortlist_manager_remove_from_transfer_list(pessoa));
    PLACAR.record(igual && r == n, || format!("shortlist_manager_remove_from_transfer_list(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// { 0x8442d0 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_manager_remove_from_loan_list(pessoa)`, pela
/// renovação do contrato (0x4dc3a0) e pelas telas de transferência. Sem contrato: 0; com ele, o bit 1 de +0x4e (na
/// lista de empréstimos) sai e devolve 1 no byte baixo (os de cima, os do ponteiro do contrato).
unsafe extern "stdcall" fn shortlist_manager_remove_from_loan_list(pessoa: usize) -> u32 {
    let c = contract_of_staff(pessoa);
    if c == 0 {
        return 0;
    }
    *((c + 0x4e) as *mut u8) &= 0xfd;
    (c as u32 & !0xff) | 1
}

/// Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
unsafe extern "stdcall" fn comparar_shortlist_manager_remove_from_loan_list(pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(%%I2%%);
    let c = contract_of_staff(pessoa);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(pessoa), || shortlist_manager_remove_from_loan_list(pessoa));
    PLACAR.record(igual && r == n, || format!("shortlist_manager_remove_from_loan_list(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}
