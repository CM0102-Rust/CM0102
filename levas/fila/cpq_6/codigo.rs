// ---------- %%LEVA%%a leva: o dinheiro da transferência: a compensação pela pessoa, pagar a transferência e guardar as cláusulas, e se o clube vendedor tem de aceitar (transfer_manager.cpp) ----------

unsafe fn cpq6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpq6_clube(id: usize) -> usize {
    cpq6_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpq6_pessoa(id: usize) -> usize {
    cpq6_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O clube do contrato da `pessoa` (0x4cf5a0 com o gerente de contratos [0xae1050], guardado em [0xae1080]); contrato
/// de tipo 5 (+0x35 & 0x3f) não conta.
unsafe fn seller_club_of_person(pessoa: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contract_of(cpq6_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    if c != 0 && cpq6_b(c + 0x35) & 0x3f != 5 {
        cpq6_clube(cpq6_d(c + 4))
    } else {
        0
    }
}

unsafe fn put_u32_le(r: &mut [u8; 13], k: usize, v: usize) {
    r[k..k + 4].copy_from_slice(&(v as u32).to_le_bytes());
}

/// As estáticas das operações de data (valor e guarda), para os comparadores.
const DATE_OP_STATICS: [(usize, usize); 8] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2410, 8), (0xae2430, 1), (0xae2470, 8), (0xae2418, 1)];

/// { 0x8d0780 } (stdcall, ret 0xc, transfer_manager.cpp, quadro SEH só da data local):
/// `transfer_manager_calc_compensation_fee(pessoa, comprador, sem_regras_novas)`: sem pessoa ou comprador, a caixa
/// "Error" 0x372e e 0. Comissão técnica (sem ficha de jogador +0x61, com +0x69) de contrato tipo 2: o que falta do
/// contrato (dias até o fim +0x2d × o salário +0xc / 7) entre 1000 e 1000000. Regras novas ([0xae233c] = 1, sem o
/// argumento) com contrato tipo 2: a média entre o que falta e o valor do jogador no vendedor (0x4d0c60). Senão (contrato
/// que não é de tipo 5, as duas nações): entre clubes da nação [0x9cf4a8], metade do valor; fora disso, só abaixo de 24
/// anos na data (o fim do contrato, no tipo 2; senão hoje) e entre nações britânicas ou a Irlanda (0x5446d0), metade
/// do valor. Os outros casos, 0.
unsafe extern "stdcall" fn transfer_manager_calc_compensation_fee(pessoa: usize, comprador: usize, sem_regras_novas: u32) -> i32 {
    let padrao: unsafe extern "thiscall" fn(*mut [u32; 2]) -> usize = core::mem::transmute(0x54a110usize);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], usize) -> usize = core::mem::transmute(0x417c10usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
    let idade: unsafe extern "C" fn(usize, *const [u32; 2]) -> u16 = core::mem::transmute(0x53ab90usize);
    let britanica: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5446d0usize);
    let gerente = || cpq6_d(0xae1050);
    // o que falta do contrato: os dias até o fim × o salário semanal, por 7 (divisão truncada, como a original)
    let resto = |c: usize| dias_ate(c + 0x2d, cpq6_d(0xae2c90) as u32, cpq6_d(0xae2c94) as u32).wrapping_mul(cpq6_d(c + 0xc) as i32) / 7;
    let mut data = [0u32; 2];
    let mut t = [0u32; 2];
    padrao(&mut data);
    if pessoa == 0 || comprador == 0 {
        show_error_at(0xa8d514, 0x372e);
        return 0;
    }
    if cpq6_d(pessoa + 0x61) == 0 && cpq6_d(pessoa + 0x69) != 0 {
        let c = contract_of(gerente(), pessoa);
        if c == 0 || cpq6_b(c + 0x35) & 0x3f != 2 {
            return 0;
        }
        return resto(c).clamp(1000, 1_000_000);
    }
    if cpq6_b(0xae233c) == 1 && sem_regras_novas == 0 {
        let c = contract_of(gerente(), pessoa);
        if c == 0 || cpq6_b(c + 0x35) & 0x3f != 2 {
            return 0;
        }
        let r = resto(c);
        let v = valor(gerente(), pessoa, cpq6_clube(cpq6_d(c + 4)), 1, 0xffff_ffff, 0);
        return v.wrapping_add(r) / 2;
    }
    let c = contract_of(gerente(), pessoa);
    if c == 0 {
        return 0;
    }
    let tipo = cpq6_b(c + 0x35) & 0x3f;
    if tipo == 5 {
        return 0;
    }
    let vendedor = cpq6_clube(cpq6_d(c + 4));
    if cpq6_d(comprador + 0x53) == 0 || cpq6_d(vendedor + 0x53) == 0 {
        return 0;
    }
    if tipo == 2 {
        copia(&mut data, &mut t, c + 0x2d);
    } else {
        copia(&mut data, &mut t, 0xae2c90);
    }
    let n = cpq6_d(0x9cf4a8);
    if cpq6_d(cpq6_d(vendedor + 0x53)) == n && cpq6_d(cpq6_d(comprador + 0x53)) == n {
        return valor(gerente(), pessoa, vendedor, 1, 0xffff_ffff, 0) / 2;
    }
    if idade(pessoa, &data) as u8 as i8 >= 0x18 {
        return 0;
    }
    if britanica(cpq6_d(comprador + 0x53)) == 0 || britanica(cpq6_d(vendedor + 0x53)) == 0 {
        return 0;
    }
    crate::comparison::branch("cpq_6 compensacao britanica");
    valor(gerente(), pessoa, vendedor, 1, 0xffff_ffff, 0) / 2 // menor de 24 anos entre clubes britânicos
}

/// { 0x8ca8a0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::pay_fee_and_record_clauses(oferta)`: soma o
/// valor (+0x1b) aos totais das transferências (0x448ad0); o comprador (+8) paga e o vendedor (clube do contrato da
/// pessoa +0xc, 0x4cf5a0) recebe a primeira parcela (o valor / parcelas +0x40, ou tudo), em 64 bits (0x599890,
/// 0x599800). Com parcelas, guarda as que faltam (13 bytes: a parcela, os ids do comprador e do vendedor, parcelas − 1)
/// na lista +0x872 (0x68d0d0) e soma ao +0x1c das finanças a parcela × −5/21 (comprador) e × 5/21 (vendedor). Guarda
/// as cláusulas: a porcentagem da próxima venda (+0x3f, 0x8caaf0, marca 8), o prêmio por jogo no time (+0x35 com o byte
/// +0x39, 0x8cac40, marca 2) e o prêmio por jogo pela seleção (+0x3a com +0x3e, 0x8cad90, marca 4); as marcas no
/// registro de 8 bytes da pessoa em [this+0x86e].
unsafe extern "thiscall" fn pay_fee_and_record_clauses(this: usize, oferta: usize) {
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let totais: unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32 = core::mem::transmute(0x448ad0usize);
    let pagar: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(0x599890usize);
    let receber: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(0x599800usize);
    let por_na_lista: unsafe extern "thiscall" fn(usize, *const u8, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let proxima_venda: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x8caaf0usize);
    let premio_time: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x8cac40usize);
    let premio_selecao: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x8cad90usize);
    let tabela = || cpq6_d(0xae2a38);
    let marcar = |bit: u8| {
        let m = (cpq6_d(this + 0x86e) + cpq6_d(oferta + 0xc) * 8) as *mut u8;
        *m |= bit;
    };
    let comprador = cpq6_clube(cpq6_d(oferta + 8));
    let vendedor = seller_club_of_person(cpq6_pessoa(cpq6_d(oferta + 0xc)));
    let fin_c = financas(tabela(), comprador);
    // ponytail: sem vendedor, a original leria as finanças de um lugar sem valor (e cairia adiante, em [vendedor]); a
    // cópia não paga ninguém
    let fin_v = if vendedor != 0 { financas(tabela(), vendedor) } else { 0 };
    let valor = cpq6_d(oferta + 0x1b) as i32;
    let n = cpq6_b(oferta + 0x40) as i8;
    let primeira = if n > 0 { (valor / n as i32) as i64 } else { valor as i64 };
    totais(cpq6_d(0xad9dac), valor, comprador, vendedor);
    crate::comparison::branch("cpq_6 pagar transferencia");
    pagar(fin_c, primeira);
    if fin_v != 0 {
        receber(fin_v, primeira); // o vendedor recebe
    }
    let mut r = [0u8; 13];
    let n = cpq6_b(oferta + 0x40) as i8;
    if n > 0 {
        let parcela = cpq6_d(oferta + 0x1b) as i32 / n as i32;
        put_u32_le(&mut r, 0, parcela as usize);
        put_u32_le(&mut r, 4, cpq6_d(comprador));
        put_u32_le(&mut r, 8, cpq6_d(vendedor));
        r[12] = (n as u8).wrapping_sub(1);
        let fc = financas(tabela(), comprador);
        let fv = financas(tabela(), vendedor);
        if fc != 0 {
            let s = (fc + 0x1c) as *mut i32;
            *s = (*s).wrapping_add(match_pl_ftol(parcela as f64 * read_f64_at(0x970420)));
        }
        if fv != 0 {
            let s = (fv + 0x1c) as *mut i32;
            *s = (*s).wrapping_add(match_pl_ftol(parcela as f64 * read_f64_at(0x970418)));
        }
        por_na_lista(this + 0x872, r.as_ptr(), 0xd);
    }
    let p = cpq6_b(oferta + 0x3f);
    if (p as i8) > 0 {
        r[0] = p;
        put_u32_le(&mut r, 1, cpq6_d(comprador));
        put_u32_le(&mut r, 5, cpq6_d(vendedor));
        put_u32_le(&mut r, 9, cpq6_d(oferta + 0xc));
        proxima_venda(this, r.as_ptr());
        marcar(8);
    }
    let v = cpq6_d(oferta + 0x35) as i32;
    if v > 0 {
        put_u32_le(&mut r, 0, cpq6_d(oferta + 0xc));
        put_u32_le(&mut r, 4, v as usize);
        put_u32_le(&mut r, 9, cpq6_d(vendedor));
        r[8] = cpq6_b(oferta + 0x39);
        premio_time(this, r.as_ptr());
        marcar(2);
    }
    let v = cpq6_d(oferta + 0x3a) as i32;
    if v > 0 {
        put_u32_le(&mut r, 0, cpq6_d(oferta + 0xc));
        put_u32_le(&mut r, 4, v as usize);
        put_u32_le(&mut r, 9, cpq6_d(vendedor));
        r[8] = cpq6_b(oferta + 0x3e);
        premio_selecao(this, r.as_ptr());
        marcar(4);
    }
}

/// { 0x8cd240 } (stdcall, ret 0xc, transfer_manager.cpp, quadro SEH só dos valores locais):
/// `transfer_manager_selling_club_must_accept(oferta, estrito, motivo)`: sem vendedor (clube do contrato da pessoa
/// +0xc que não é de tipo 5, 0x4cf5a0), 1. Com cláusula de rescisão ativa (0x867b50), sem troca (+4) nem parcelas
/// (+0x40) e o valor (+0x1b) não abaixo da multa (+0x21 do contrato; os dois por 0x43f460 com 4): motivo 0x1b e 1. Com
/// o vendedor em crise (+0x165 das finanças = 1), o preço pedido (0x4d1600) não abaixo de 20 × a reputação (+0x80;
/// pelo menos 20000), o valor ponderado da oferta (0x8e7b50) acima de 50000 e o dobro dele acima do preço: motivo 0x1c
/// e 1 (com `estrito`, só se o preço ficar abaixo de 1,3 × o valor ponderado). Senão 0.
unsafe extern "stdcall" fn transfer_manager_selling_club_must_accept(oferta: usize, estrito: u32, motivo: *mut u16) -> u32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let ativa: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x867b50usize);
    let exibir: unsafe extern "thiscall" fn(*const u32, u32) -> i64 = core::mem::transmute(0x43f460usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let preco: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
    let ponderado: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7b50usize);
    let comprador = cpq6_clube(cpq6_d(oferta + 8));
    let pessoa = cpq6_pessoa(cpq6_d(oferta + 0xc));
    let vendedor = seller_club_of_person(pessoa);
    if vendedor == 0 {
        return 1;
    }
    let c = contract_of(cpq6_d(0xae1050), pessoa);
    crate::comparison::branch("cpq_6 clube deve aceitar");
    if ativa(c) != 0 && cpq6_d(oferta + 4) == 0 && cpq6_b(oferta + 0x40) == 0 {
        let (multa, valor) = (cpq6_d(c + 0x21) as u32, cpq6_d(oferta + 0x1b) as u32);
        let m = exibir(&multa, 4);
        let v = exibir(&valor, 4);
        if v >= m {
            if !motivo.is_null() {
                *motivo = 0x1b;
            }
            return 1;
        }
    }
    if cpq6_b(financas(cpq6_d(0xae2a38), vendedor) + 0x165) != 1 {
        return 0;
    }
    let pedido = preco(cpq6_d(0xae1050), vendedor, comprador, pessoa, 0, 1, 1);
    let w = ponderado(oferta, 0, 0, 1, 0, 1);
    let piso = (*((vendedor + 0x80) as *const i16) as i32 * 20).max(20000);
    if pedido < piso || w.wrapping_mul(2) <= pedido || w <= 50000 {
        return 0;
    }
    if estrito != 0 {
        let w2 = ponderado(oferta, 0, 0, 1, 0, 1);
        if !((pedido as f64) < w2 as f64 * read_f64_at(0x968de8)) {
            return 0;
        }
    }
    if !motivo.is_null() {
        *motivo = 0x1c;
    }
    1
}

/// Comparador: pelo valor (lado a lado pelas estáticas das datas; o valor do jogador 0x4d0c60 e a idade só calculam).
unsafe extern "stdcall" fn comparar_transfer_manager_calc_compensation_fee(pessoa: usize, comprador: usize, sem_regras_novas: u32) -> i32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32) -> i32>(%%I0%%);
    let (ro, rr, igual, _, _) = crate::comparison::side_by_side(&DATE_OP_STATICS, || o(pessoa, comprador, sem_regras_novas), || transfer_manager_calc_compensation_fee(pessoa, comprador, sem_regras_novas));
    SCOREBOARD.record(igual && ro == rr, || format!("transfer_manager_calc_compensation_fee(pessoa {:x}) {} x {}, estáticas iguais {}", if pessoa != 0 { cpq6_d(pessoa) } else { 0 }, ro, rr, igual));
    ro
}

/// Comparador: paga, acrescenta nas listas (aloca) e marca: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_pay_fee_and_record_clauses(this: usize, oferta: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, oferta)
}

/// Comparador: lado a lado pelo motivo e [0xae1080], e pelo valor devolvido (o preço e o valor ponderado só calculam).
unsafe extern "stdcall" fn comparar_transfer_manager_selling_club_must_accept(oferta: usize, estrito: u32, motivo: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, *mut u16) -> u32>(%%I2%%);
    let mut reg: Vec<(usize, usize)> = vec![(0xae1080, 4)];
    if !motivo.is_null() {
        reg.push((motivo as usize, 2));
    }
    reg.extend_from_slice(&DATE_OP_STATICS);
    let (ro, rr, igual, _, _) = crate::comparison::side_by_side(&reg, || o(oferta, estrito, motivo), || transfer_manager_selling_club_must_accept(oferta, estrito, motivo));
    SCOREBOARD.record(igual && ro == rr, || format!("transfer_manager_selling_club_must_accept(estrito {}) {} x {}, memória igual {}", estrito, ro, rr, igual));
    ro
}
