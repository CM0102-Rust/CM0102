// ---------- %%LEVA%%a leva: a cópia do registro de suspensão (discipline.cpp), o salário padrão do contrato (staff_contracts.cpp) e a condição de jogo (injury.cpp) ----------

/// { 0x556c70 } (thiscall, ret 4, discipline.cpp): `BanRecord::operator=`: copia o registro de suspensão (0x14 bytes) de
/// `origem`: +0 e +1 (bytes), +2 (palavra), a data em +4 (0x417c10, retorno numa local), +0xc..+0xf (byte a byte),
/// +0x10 (byte), +0x11 (palavra) e os bits 0..2 de +0x13 (os outros bits ficam). Devolve `this`.
unsafe extern "thiscall" fn discipline_operator(this: usize, origem: usize) -> usize {
    let b = |o: usize| *((this + o) as *mut u8) = *((origem + o) as *const u8);
    let w = |o: usize| core::ptr::write_unaligned((this + o) as *mut u16, core::ptr::read_unaligned((origem + o) as *const u16));
    b(0);
    b(1);
    w(2);
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    copiar(this + 4, &mut local, origem + 4);
    for o in 0xc..0x10 {
        b(o);
    }
    b(0x10);
    w(0x11);
    let m = (this + 0x13) as *mut u8;
    *m = (*m & !7) | (*((origem + 0x13) as *const u8) & 7); // as três marcas
    this
}

/// { 0x8626e0 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::set_default_wage(pessoa)`: o salário (+0xc) é
/// o padrão da pessoa no clube dela ([+0x39]) pela 0x862720 (stdcall, 6 argumentos, float; o ecx vai junto, como na
/// original), truncado (0x945640), com mínimo 55. Devolve o truncado (antes do mínimo).
unsafe extern "thiscall" fn set_default_wage(this: usize, pessoa: usize) -> u32 {
    let padrao: unsafe extern "thiscall" fn(usize, usize, usize, i32, i32, i32, i32) -> f64 = core::mem::transmute(0x862720usize);
    let v = match_pl_ftol(padrao(this, pessoa, core::ptr::read_unaligned((pessoa + 0x39) as *const usize), 1, -1, 0xb, 0));
    *((this + 0xc) as *mut i32) = if v < 0x37 { 0x37 } else { v }; // o mínimo de 55
    v as u32
}

/// { 0x62c510 } (thiscall, ret 8, injury.cpp): `Injury::get_match_condition(jogador, em_porcento)`: sem jogador, 0; id
/// fora das pessoas ([0xae2364]): 10000 (100 em porcento); senão atualiza o físico do dia (0x62c680, uma vez por dia)
/// e devolve a condição (registro de 31 bytes em [this], +0xa), em porcento × 0,01f truncado. Só o ax é o retorno.
unsafe extern "thiscall" fn get_match_condition(this: usize, jogador: usize, em_porcento: u32) -> u16 {
    if jogador == 0 {
        return 0;
    }
    let id = *(jogador as *const i32);
    if id >= *(0xae2364 as *const i32) {
        return if em_porcento != 0 { 100 } else { 10000 };
    }
    let atualizar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x62c680usize);
    atualizar(this, jogador);
    let c = core::ptr::read_unaligned((*(this as *const usize)).wrapping_add((*(jogador as *const i32) as usize).wrapping_mul(31)).wrapping_add(0xa) as *const i16);
    if em_porcento != 0 {
        return match_pl_ftol(c as f64 * *(0x96bac8 as *const f32) as f64) as u16; // em porcento
    }
    c as u16
}

unsafe extern "thiscall" fn comparar_discipline_operator(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x14)], || o(this, origem), || discipline_operator(this, origem));
    PLACAR.record(igual && r == n, || format!("discipline_operator {:x?} x {:x?}", mo, mr));
    r
}

/// Comparador: lado a lado pelo salário (+0xc). A 0x862720 grava a mesma ficha em [0xae1080] nas duas vezes.
unsafe extern "thiscall" fn comparar_set_default_wage(this: usize, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this + 0xc, 4)], || o(this, pessoa), || set_default_wage(this, pessoa));
    PLACAR.record(igual && r == n, || format!("set_default_wage({:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

// a original roda primeiro e atualiza o físico do dia (0x62c680, com sorteio); a cópia, depois, acha o dia já feito
comparar_valor!(comparar_get_match_condition, get_match_condition, %%I2%%, (jogador: usize, em_porcento: u32) -> u16);
