// ---------- %%LEVA%%a leva: a contraproposta com cláusula e os motivos da recusa do lance (transfer_manager.cpp) ----------

unsafe fn cps4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps4_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cps4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cps4_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// O valor da oferta (+0x1b) × `f`, truncado (_ftol 0x945640; o x87 em precisão dupla).
unsafe fn offer_value_times(o: usize, f: f64) -> usize {
    match_pl_ftol(cps4_d(o + 0x1b) as i32 as f64 * f) as usize
}

/// O clube do contrato da pessoa (0x4cf5a0, guardado em [0xae1080]), ou 0.
unsafe fn current_contract_club(pessoa: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contract_of(cps4_d(0xae1050), pessoa);
    cps4_por(0xae1080, c);
    if c == 0 {
        0
    } else {
        cps4_d(0xae23b4).wrapping_add(cps4_d(c + 4).wrapping_mul(0x245))
    }
}

/// { 0x8c8fd0 } (thiscall com o `this` sem uso, ret 0x14, transfer_manager.cpp, quadro SEH do `new`):
/// `TRANSFER_MANAGER::build_clause_offer_option(clube, jogador, saida, apelo, valor)`: pela resposta do clube do computador
/// ao lance (0x8c8090). Com o clube do lance mais de 3 acima na reputação (+0x80) do clube atual (0x4cf5a0), ficha de
/// jogador (+0x61) com potencial (+7) ao menos 25 acima da habilidade (+5) e até 30 anos (+0x18): uma oferta nova
/// (0x944cf1 de 0x41 bytes, 0x8e5680) em `*saida`, iniciada com o valor (0x8e78f0), e apelo 50. Clube do lance abaixo de
/// 15 com a reputação da ficha (+0xb / 500) mais de 2 acima: 20% da próxima venda (+0x3f), o valor × 0,8 e apelo 80.
/// Senão, jogador sem jogos pela seleção (+0x22) de nação (+0x1a) acima de 5000 (+0x8e): o prêmio por jogo na seleção
/// (+0x3a, valor × 0,3, por 10 jogos +0x3e), o valor × 0,8 e apelo 70. Senão, reputação da ficha mais de 3 abaixo da do
/// clube: o prêmio por jogo no time (+0x35, valor × 0,3, por 20 jogos +0x39), o valor × 0,8 e apelo 60. Nenhum: a
/// oferta é destruída (0x8e5710, 0x944c94; `*saida` fica com o ponteiro, como na original) e 0.
unsafe extern "thiscall" fn build_clause_offer_option(_this: usize, clube: usize, jogador: usize, saida: *mut usize, apelo: *mut u8, valor: u32) -> u32 {
    let atual = current_contract_club(jogador);
    if (cps4_w(clube + 0x80) as i16 as i32) - (cps4_w(atual + 0x80) as i16 as i32) <= 3 {
        return 0;
    }
    let ficha = cps4_d(jogador + 0x61);
    if ficha == 0 || (cps4_w(ficha + 7) as i16 as i32) - (cps4_w(ficha + 5) as i16 as i32) < 0x19 || cps4_b(jogador + 0x18) as i8 > 0x1e {
        return 0;
    }
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8e5680usize);
    let iniciar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x8e78f0usize);
    let m = novo(0x41);
    let o = if m != 0 { construir(m) } else { 0 };
    *saida = o;
    iniciar(o, jogador, clube, valor);
    *apelo = 0x32;
    let rep = cps4_w(clube + 0x80) as i16;
    // a reputação da ficha (+0xb) / 500, com o sinal (a multiplicação mágica da original)
    let fama = || (cps4_w(cps4_d(jogador + 0x61) + 0xb) as i16 as i32) / 500;
    if rep < 0xf && fama() - (rep as i32) > 2 {
        crate::comparison::branch("cps_4 write_percent_pair da venda");
        let o = *saida;
        cps4_por(o + 0x1b, offer_value_times(o, 0.8)); // cps4: o valor do percentual
        *((*saida + 0x3f) as *mut u8) = 0x14;
        *apelo = 0x50;
        return 1;
    }
    let nacao = cps4_d(jogador + 0x1a);
    if nacao != 0 && cps4_w(nacao + 0x8e) as i16 > 0x1388 && cps4_b(jogador + 0x22) == 0 {
        crate::comparison::branch("cps_4 premio na selecao");
        let o = *saida;
        cps4_por(o + 0x3a, offer_value_times(o, 0.3));
        *((*saida + 0x3e) as *mut u8) = 0xa;
        let o = *saida;
        cps4_por(o + 0x1b, offer_value_times(o, 0.8)); // cps4: o valor do prêmio na seleção
        *apelo = 0x46;
        return 1;
    }
    if fama() - (rep as i32) < -3 {
        crate::comparison::branch("cps_4 premio no time");
        let o = *saida;
        cps4_por(o + 0x35, offer_value_times(o, 0.3));
        *((*saida + 0x39) as *mut u8) = 0x14;
        let o = *saida;
        cps4_por(o + 0x1b, offer_value_times(o, 0.8)); // cps4: o valor do prêmio no time
        *apelo = 0x3c;
        return 1;
    }
    let o = *saida;
    if o != 0 {
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8e5710usize);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        destruir(o);
        liberar(o);
    }
    0
}

/// { 0x8c5560 } (stdcall, ret 8, transfer_manager.cpp): `transfer_manager_is_rejection_reason_valid(motivo, oferta)`: 0
/// quando o motivo não cabe na oferta (pessoa +0xc, clube do lance +8): 0x2000 (promessa) com 24 anos ou mais (+0x18);
/// 0x1000 com o clube atual (0x4cf5a0) e o do lance em divisões diferentes (+0x57); 0x800 (rival) com os dois clubes,
/// sem os dois humanos (0x600a50), sem confronto na liga (0x4b02e0) e sem um ser parceiro (+0xb3/+0xb7/+0xbb) do outro.
/// Senão 1.
unsafe extern "stdcall" fn transfer_manager_is_rejection_reason_valid(motivo: u32, oferta: usize) -> u32 {
    let pessoa = cps4_d(0xae23bc).wrapping_add(cps4_d(oferta + 0xc).wrapping_mul(0x6e));
    let atual = current_contract_club(pessoa);
    let lance = cps4_d(0xae23b4).wrapping_add(cps4_d(oferta + 8).wrapping_mul(0x245));
    let r = match motivo as u16 {
        0x2000 => (cps4_b(pessoa + 0x18) as i8) < 0x18,
        0x1000 => atual == 0 || cps4_d(atual + 0x57) == cps4_d(lance + 0x57),
        0x800 => {
            if lance == 0 || atual == 0 {
                return 1;
            }
            let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
            let confronto: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4b02e0usize);
            if humano(atual, 0, 1, 0, 0) != 0 && humano(lance, 0, 1, 0, 0) != 0 {
                return 1;
            }
            confronto(lance, atual) != 0
                || [0xb3, 0xb7, 0xbb].iter().any(|&k| cps4_d(lance + k) == atual)
                || [0xb3, 0xb7, 0xbb].iter().any(|&k| cps4_d(atual + k) == lance)
        }
        _ => true,
    };
    r as u32
}

/// { 0x8c54c0 } (stdcall, ret 4, transfer_manager.cpp): `transfer_manager_get_squad_status_reason(oferta)`: o motivo de
/// recusa pelo status no elenco (+0x4f >> 4) do contrato da pessoa (+0xc) com o clube atual (0x4cf5f0): 1 → 0x200, 2 e
/// 3 → 0x400, 5 → 0x2000; senão 0.
unsafe extern "stdcall" fn transfer_manager_get_squad_status_reason(oferta: usize) -> u16 {
    let pessoa = cps4_d(0xae23bc).wrapping_add(cps4_d(oferta + 0xc).wrapping_mul(0x6e));
    let atual = current_contract_club(pessoa);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let c = com_clube(cps4_d(0xae1050), pessoa, atual);
    if c == 0 {
        return 0;
    }
    match cps4_b(c + 0x4f) >> 4 {
        1 => 0x200,
        2 | 3 => 0x400,
        5 => 0x2000,
        _ => 0,
    }
}

// ponytail: aloca e solta a oferta (0x944cf1, 0x944c94): só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn compare_build_clause_offer_option(this: usize, clube: usize, jogador: usize, saida: *mut usize, apelo: *mut u8, valor: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, u32) -> u32>(%%I0%%)(this, clube, jogador, saida, apelo, valor)
}

/// Comparador: lado a lado pelo contrato da vez ([0xae1080]), e pela devolução.
unsafe extern "stdcall" fn compare_transfer_manager_is_rejection_reason_valid(motivo: u32, oferta: usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(u32, usize) -> u32>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(0xae1080, 4)], || o(motivo, oferta), || transfer_manager_is_rejection_reason_valid(motivo, oferta));
    SCOREBOARD.record(igual && r == n, || format!("transfer_manager_is_rejection_reason_valid({:x}, pessoa {}) original={} rust={} {:x?} x {:x?}", motivo as u16, cps4_d(oferta + 0xc), r, n, mo, mr));
    r
}

/// Comparador: lado a lado pelo contrato da vez ([0xae1080]), e pela devolução.
unsafe extern "stdcall" fn compare_transfer_manager_get_squad_status_reason(oferta: usize) -> u16 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u16>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(0xae1080, 4)], || o(oferta), || transfer_manager_get_squad_status_reason(oferta));
    SCOREBOARD.record(igual && r == n, || format!("transfer_manager_get_squad_status_reason(pessoa {}) original={:x} rust={:x} {:x?} x {:x?}", cps4_d(oferta + 0xc), r, n, mo, mr));
    r
}
