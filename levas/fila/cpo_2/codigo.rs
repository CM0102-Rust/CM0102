// ---------- %%LEVA%%a leva: os contratos: tirar o contrato principal, reenviar as ofertas guardadas e o mês do fim da temporada do clube (contract_manager.cpp) ----------

unsafe fn cpo2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo2_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo2_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A vaga do contrato principal da pessoa: o registro de 79 bytes dela em [0xae2d0c] (+0 o índice do contrato, −1 sem).
unsafe fn main_contract_slot(pessoa: usize) -> usize {
    cpo2_d(0xae2d0c).wrapping_add(cpo2_d(pessoa).wrapping_mul(0x4f))
}

/// { 0x4d0b50 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract(pessoa)`: com o contrato
/// principal (índice no registro de 79 bytes da pessoa), libera a entrada dele (+0 = −1 no vetor de 0x50 bytes do
/// CONTRACT_MANAGER), põe −1 na vaga da pessoa e refaz a folha de salários (0x598370) das finanças (0x5a0590) do clube
/// do contrato (+4). Sem contrato: nada.
unsafe extern "thiscall" fn remove_contract(this: usize, pessoa: usize) {
    let indice = cpo2_i(main_contract_slot(pessoa));
    if indice == -1 {
        return;
    }
    let e = cpo2_d(this).wrapping_add((indice as usize).wrapping_mul(0x50));
    let id_clube = cpo2_d(e + 4);
    cpo2_por(e, usize::MAX);
    let clube = cpo2_d(0xae23b4).wrapping_add(id_clube.wrapping_mul(0x245));
    crate::comparison::branch("cpo_2 tirar contrato");
    cpo2_por(main_contract_slot(pessoa), usize::MAX); // a pessoa fica sem contrato
    let get_club_finance: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let f = get_club_finance(cpo2_d(0xae2a38), clube);
    if f != 0 {
        let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x598370usize);
        folha(f);
    }
}

/// { 0x4cf4a0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::resubmit_held_offers(id)`: percorre a lista
/// das ofertas guardadas (+0x38) e, para cada uma da pessoa `id` (+0): com a transferência (+0x59, 0x8c61e0 em
/// [0xde1f54]) cujo clube (+0x24) é o da oferta (+0x5d), põe o estado 0xf em 8 e reenvia a oferta (0x4cc490 com a
/// transferência, o clube, o byte +0x61, 1, 1, 0); sem transferência (+0x59 = −1) reenvia com −1, −1. Tira a oferta da
/// lista (0x68d1f0) e conta. O byte +0x61 vai empurrado no registrador inteiro (os bits de cima do que estava nele:
/// o ponteiro lido ou −1). Devolve quantas.
unsafe extern "thiscall" fn resubmit_held_offers(this: usize, id: usize) -> u32 {
    let lista = cpo2_d(this + 0x38);
    if lista == 0 {
        return 0;
    }
    cpo2_por(lista + 0xa, cpo2_d(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let remover: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    let reenviar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x4cc490usize);
    let mut n = 0u32;
    let mut item = proximo(cpo2_d(this + 0x38));
    while item != 0 {
        if cpo2_d(item) == id {
            let t = cpo2_d(item + 0x59);
            let byte = *((item + 0x61) as *const u8) as u32;
            if t as i32 != -1 {
                let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8c61e0usize);
                let p = achar(cpo2_d(0xde1f54), t);
                if p != 0 {
                    let q = cpo2_d(p);
                    if q != 0 && cpo2_d(q + 0x24) == cpo2_d(item + 0x5d) {
                        if *((q + 0x2c) as *const u8) == 0xf {
                            *((q + 0x2c) as *mut u8) = 8;
                        }
                        crate::comparison::branch("cpo_2 reenviar oferta");
                        reenviar(this, item, t, cpo2_d(item + 0x5d), (q as u32 & 0xffff_ff00) | byte, 1, 1, 0); // com a transferência
                    }
                }
            } else {
                reenviar(this, item, usize::MAX, usize::MAX, 0xffff_ff00 | byte, 1, 1, 0);
            }
            remover(cpo2_d(this + 0x38), item);
            n += 1;
        }
        item = proximo(cpo2_d(this + 0x38));
    }
    n
}

/// { 0x4d45f0 } (stdcall, ret 4, contract_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `contract_manager_season_end_month(clube)`: a data do fim da temporada do clube (Data() 0x54a110 e 0x669bd0 com 0),
/// mais 10 dias (0x54c5b0), e o mês dela (0x54cfb0). Devolve o eax da 0x54cfb0 (o mês 0..11).
unsafe extern "stdcall" fn contract_manager_season_end_month(clube: usize) -> u32 {
    let mut d1 = [0u32; 2];
    let mut d2 = [0u32; 2];
    let construir: unsafe extern "thiscall" fn(*mut u32) -> usize = core::mem::transmute(0x54a110usize);
    let fim: unsafe extern "C" fn(usize, u32, *mut u32) = core::mem::transmute(0x669bd0usize);
    let somar: unsafe extern "thiscall" fn(*mut u32, *mut u32, i32) -> usize = core::mem::transmute(0x54c5b0usize);
    let mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x54cfb0usize);
    construir(d1.as_mut_ptr());
    fim(clube, 0, d1.as_mut_ptr());
    let p = somar(d1.as_mut_ptr(), d2.as_mut_ptr(), 0xa); // dez dias depois
    mes(p)
}

/// Comparador: lado a lado pela vaga da pessoa e pela entrada do contrato (a folha 0x598370 refeita duas vezes dá o
/// mesmo, pelo contrato já tirado).
unsafe extern "thiscall" fn compare_remove_contract(this: usize, pessoa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let vaga = main_contract_slot(pessoa);
    let indice = cpo2_i(vaga);
    if indice == -1 {
        return o(this, pessoa);
    }
    let e = cpo2_d(this).wrapping_add((indice as usize).wrapping_mul(0x50));
    let reg = [(vaga, 4), (e, 8)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, pessoa), || remove_contract(this, pessoa));
    SCOREBOARD.record(igual, || format!("remove_contract(pessoa {:x}, contrato {}) {:x?} x {:x?}", cpo2_d(pessoa), indice, mo, mr));
}

/// Comparador: a original reenvia (0x4cc490, ofertas e transferências) e tira as ofertas da lista (libera): roda só ela
/// e confere o plano feito antes: devolve quantas ofertas da pessoa havia na lista.
unsafe extern "thiscall" fn compare_resubmit_held_offers(this: usize, id: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%);
    let lista = cpo2_d(this + 0x38);
    if lista == 0 {
        return o(this, id);
    }
    // o plano: a original recomeça o cursor (+0xa) antes de percorrer
    cpo2_por(lista + 0xa, cpo2_d(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut esperado = 0u32;
    let mut item = proximo(lista);
    while item != 0 {
        if cpo2_d(item) == id {
            esperado += 1;
        }
        item = proximo(lista);
    }
    let r = o(this, id);
    SCOREBOARD.record(r == esperado, || format!("resubmit_held_offers(id {:x}) devolveu {}, esperava {}", id, r, esperado));
    r
}

/// Comparador: pelo valor (datas e a 0x669bd0 só calculam).
unsafe extern "stdcall" fn compare_contract_manager_season_end_month(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(%%I2%%)(clube);
    let n = contract_manager_season_end_month(clube);
    SCOREBOARD.record(r == n, || format!("contract_manager_season_end_month({:x}) original={} rust={}", clube, r, n));
    r
}
