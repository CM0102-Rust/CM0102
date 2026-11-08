// ---------- %%LEVA%%a leva: o contrato de pessoal (staff_contracts.cpp): os dias até o fim, limpar todas as queixas e conferir a retirada do pedido de transferência ----------

unsafe fn cph1_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cph1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// A pessoa do contrato: o id em +0 no vetor de pessoas [0xae23bc] (110 bytes cada).
unsafe fn staff_of_contract(contrato: usize) -> usize {
    (cph1_d(0xae23bc) as usize).wrapping_add((cph1_d(contrato) as usize).wrapping_mul(110))
}

/// { 0x85fa00 } (thiscall, staff_contracts.cpp): `STAFF_CONTRACT::days_to_end_date`: os dias da data do jogo
/// (0xae2c90) até a data do fim do contrato (+0x2d), pela 0x54cdb0 (`dias_ate`, a data do jogo por valor). O jogo lê
/// só a palavra; devolve o eax inteiro da 0x54cdb0.
unsafe extern "thiscall" fn days_to_end_date(this: usize) -> u32 {
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x54cdb0usize);
    dias(this + 0x2d, cph1_d(0xae2c90), cph1_d(0xae2c94)) // a data do fim do contrato
}

/// { 0x867520 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::clear_all_grievances(conferir)`: zera as
/// queixas (+0x45), tira os vínculos de opinião da pessoa (0x549c90) e, com `conferir`, confere a retirada do pedido de
/// transferência (0x867240 com 0).
unsafe extern "thiscall" fn clear_all_grievances(this: usize, conferir: u32) {
    core::ptr::write_unaligned((this + 0x45) as *mut u32, 0); // as queixas
    let vinculos: unsafe extern "C" fn(usize) = core::mem::transmute(0x549c90usize);
    vinculos(staff_of_contract(this));
    if conferir != 0 {
        let retirada: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x867240usize);
        retirada(this, 0);
    }
}

/// A decisão da 0x867240 (sem efeito além da ficha [0xae1080], gravada com o contrato da pessoa): o clube do contrato
/// (fora do tipo 5) existe, a pessoa é jogador (+0x61), o contrato é desse clube (+4 = id), pediu transferência (bit 3
/// de +0x4e), não está em +0x49, o critério da comissão (0x866f60) dá 0, o nibble alto de +0x4f não é 7 e não há
/// contrato acertado com outro clube (0x8c5450 em [0xde1f54]). Devolve a pessoa quando retira.
unsafe fn should_withdraw_transfer_request(this: usize) -> Option<usize> {
    let pessoa = staff_of_contract(this);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contract_of(cph1_d(0xae1050) as usize, pessoa);
    *(0xae1080 as *mut usize) = c; // a ficha do contrato
    let clube = if c != 0 && cph1_b(c + 0x35) & 0x3f != 5 {
        let i = cph1_d(c + 4) as usize;
        (cph1_d(0xae23b4) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4))
    } else {
        0
    };
    if clube == 0 || cph1_d(pessoa + 0x61) == 0 || cph1_d(this + 4) != cph1_d(clube) {
        return None;
    }
    if cph1_b(this + 0x4e) & 8 == 0 || cph1_b(this + 0x49) != 0 {
        return None; // sem pedido de transferência
    }
    let criterio: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x866f60usize);
    if criterio(this) != 0 || cph1_b(this + 0x4f) & 0xf0 == 0x70 {
        return None;
    }
    let acertado: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5450usize);
    if acertado(cph1_d(0xde1f54) as usize, pessoa, 0) != 0 {
        return None;
    }
    Some(pessoa)
}

/// { 0x867240 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::check_transfer_request_withdrawal(ctx)`:
/// quando a decisão (`should_withdraw_transfer_request`) manda, retira o pedido (0x4d9460 em [0xae1050] com o contrato e `ctx`), apaga o
/// bit 3 de +0x4e, a moral (+0x44) vira +0x5b + +0x59 + 5 da pessoa, zera +0x3e e sobe +0x3d a 15; e cada queixa
/// acesa (bits 0x10, 1, 4, 8, 0x100, 0x80, 0x10000 de +0x45, nessa ordem) é apagada, confere de novo (0x867240 com o
/// +0x45 de antes) e tira o vínculo de opinião (0x549d60 com o bit). Sem SEH, sem caixa, sem sorteio.
unsafe extern "thiscall" fn check_transfer_request_withdrawal(this: usize, ctx: u32) {
    let Some(pessoa) = should_withdraw_transfer_request(this) else { return };
    let retirar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x4d9460usize);
    retirar(cph1_d(0xae1050) as usize, this, ctx);
    *((this + 0x4e) as *mut u8) &= 0xf7; // o pedido retirado
    *((this + 0x44) as *mut u8) = cph1_b(pessoa + 0x5b).wrapping_add(cph1_b(pessoa + 0x59)).wrapping_add(5);
    if cph1_b(this + 0x3e) != 0 {
        *((this + 0x3e) as *mut u8) = 0;
    }
    if (cph1_b(this + 0x3d) as i8) < 15 {
        *((this + 0x3d) as *mut u8) = 15; // a felicidade mínima
    }
    let conferir: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x867240usize);
    let contract_bond: unsafe extern "C" fn(usize, i32) = core::mem::transmute(0x549d60usize);
    for bit in [0x10u32, 1, 4, 8, 0x100, 0x80, 0x10000] {
        let q = cph1_d(this + 0x45);
        if q & bit != 0 {
            core::ptr::write_unaligned((this + 0x45) as *mut u32, q & !bit); // a queixa apagada
            conferir(this, q);
            contract_bond(staff_of_contract(this), bit as i32);
        }
    }
}

// a 0x54cdb0 só lê: compara o valor
comparar_valor!(comparar_days_to_end_date, days_to_end_date, %%I0%%, () -> u32);

/// Comparador: sem `conferir`, lado a lado pelas queixas e pelo registro de opiniões da pessoa (79 bytes, o que a
/// 0x549c90 escreve); com `conferir`, a retirada (0x4d9460) mexe na transferência: só a original (a 0x867240 passa
/// pelo comparador dela).
unsafe extern "thiscall" fn comparar_clear_all_grievances(this: usize, conferir: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I1%%);
    if conferir != 0 {
        return o(this, conferir);
    }
    let mut reg = vec![(this + 0x45, 4)];
    if let Some(c) = database_staff_state_record(staff_of_contract(this)) {
        reg.push((c, 79));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, conferir), || clear_all_grievances(this, conferir));
    SCOREBOARD.record(igual, || format!("clear_all_grievances({:x}) {:x?} x {:x?}", this, mo, mr));
}

/// Comparador: quando a decisão da cópia é não retirar, lado a lado pela ficha [0xae1080] e pelo contrato (0x50
/// bytes); quando retira, só a original (a 0x4d9460 mexe na transferência e a 0x867240 se chama de novo).
unsafe extern "thiscall" fn comparar_check_transfer_request_withdrawal(this: usize, ctx: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I2%%);
    let ficha = cph1_d(0xae1080);
    let retira = should_withdraw_transfer_request(this).is_some();
    *(0xae1080 as *mut u32) = ficha;
    if retira {
        return o(this, ctx);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(0xae1080, 4), (this, 0x50)], || o(this, ctx), || check_transfer_request_withdrawal(this, ctx));
    SCOREBOARD.record(igual, || format!("check_transfer_request_withdrawal({:x}, {}) {:x?} x {:x?}", this, ctx, mo, mr));
}
