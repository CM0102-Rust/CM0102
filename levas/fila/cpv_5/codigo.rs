// ---------- %%LEVA%%a leva: as receitas da semana e o centro de treino: o merchandising, os ingressos da temporada e a obra do centro de treino (finance.cpp) ----------

unsafe fn cpv5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv5_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpv5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpv5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpv5_somar(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, cpv5_i(a).wrapping_add(v));
}

/// O saldo (i64 em +0 do registro de finanças) mais `v` (estendido com sinal, como o cdq/adc da original).
unsafe fn cpv5_somar_saldo(fin: usize, v: i32) {
    let s = core::ptr::read_unaligned(fin as *const i64);
    core::ptr::write_unaligned(fin as *mut i64, s.wrapping_add(v as i64));
}

unsafe fn cpv5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O registro de finanças (0x167 bytes) do clube na tabela `this` (FinanceTable), pelo id (+0) do clube.
unsafe fn cpv5_financas(this: usize, clube: usize) -> usize {
    cpv5_d(this).wrapping_add((cpv5_d(clube) as u32).wrapping_mul(0x167) as usize)
}

/// { 0x59b7e0 } (thiscall, ret 4, finance.cpp): `FinanceTable::add_weekly_merchandising_income(clube)`, pelo passe
/// diário das finanças (0x59b9c0, no dia 6 da semana, clubes de liga). A base: com o clube de +0xbf e o estádio dele
/// (+0x69), o nível (+0xf) × 25 + a reputação (+0x80), × 0,65 truncado se o byte +0x6d for zero, escolhe o fator por
/// 5500 e 4000; sem eles, a reputação escolhe por 5250 e 3750 (um degrau acima na nação do Japão 0x9cf36c). Público
/// (+0x73) × reputação × fator, truncado; × (sorteio(11) + 95) × 0,01, truncado, soma em +0xbc, +0x15c e no saldo.
unsafe extern "thiscall" fn receita_do_merchandising(this: usize, clube: usize) {
    let outro = cpv5_d(clube + 0xbf);
    let est = if outro != 0 { cpv5_d(outro + 0x69) } else { 0 };
    let rep = cpv5_w(clube + 0x80) as i32;
    let fator = if est != 0 {
        let s = (cpv5_b(est + 0xf) as i8 as i32).wrapping_mul(25);
        let mut v = rep.wrapping_add(s);
        if cpv5_b(clube + 0x6d) == 0 {
            v = ftol(v as f64 * f64_em(0x96a5d0));
        }
        if v > 0x157c {
            0x96a6d0
        } else if v > 0xfa0 {
            0x96a6c8
        } else {
            0x96a6b8
        }
    } else {
        let nac = cpv5_d(clube + 0x53);
        let japao = nac != 0 && cpv5_i(nac) == cpv5_i(0x9cf36c);
        if rep > 0x1482 {
            if japao { 0x96a6c0 } else { 0x96a6d0 }
        } else if rep > 0xea6 {
            if japao { 0x96a6d0 } else { 0x96a6c8 }
        } else if japao {
            0x96a6c8
        } else {
            0x96a6b8
        }
    };
    let base = ftol(cpv5_i(clube + 0x73) as f64 * rep as f64 * f64_em(fator));
    crate::comparacao::ramo("cpv_5 merchandising");
    let r = cpv5_sortear(0xb); // cpv5: o sorteio do merchandising
    let v = ftol((r + 0x5f).wrapping_mul(base) as f64 * f64_em(0x967898));
    let fin = cpv5_financas(this, clube);
    cpv5_somar(fin + 0x15c, v);
    cpv5_somar(fin + 0xbc, v);
    cpv5_somar_saldo(fin, v);
}

/// Comparador: lado a lado no registro de finanças do clube e no estado do sorteio.
unsafe extern "thiscall" fn comparar_receita_do_merchandising(this: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let mut regioes = vec![(cpv5_financas(this, clube), 0x167)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, clube), || crate::aleatorio::sem_contar(|| receita_do_merchandising(this, clube)));
    PLACAR.registrar(igual, || format!("receita_do_merchandising(clube {:#x}) original={:x?} rust={:x?}", clube, mo, mr));
}

/// { 0x599a20 } (thiscall, ret 4, finance.cpp): `Finance::sell_season_tickets(aplicar)`, pelo passe diário das finanças
/// (0x59b9c0, no dia 6 da semana, quando finance_get_days_to_season_start 0x59b4c0 diz que a temporada ainda vai
/// começar). O clube pelo id (+8, registro de 0x245 bytes); estádio (+0x69) com menos de 1500 lugares (+0x3c) passa a
/// reputação × 2,5 (mínimo 1500). A procura: o valor do clube no gerente de técnicos ([[0xb67a44]] + 73·id + 8) × 0,0001
/// (mínimo 0,15) × público (+0x73), × reputação × 0,00025 (mínimo 0,5), limitada a 87,5% dos lugares (1250 sem estádio).
/// Acima dos já vendidos (+0x18): o passo (+0x164, até 8) escolhe a fração (0x9c8758) da diferença, × (sorteio(11) +
/// 95) / 100 = os novos; a razão vendidos/lugares vai a +0x160 (f32, até 0,875). A receita (preço pela reputação × novos;
/// /4 ou /10 pelo tipo da nação +0x7e; × 1,25 na série A, Chelsea 1,5, Arsenal e Tottenham 1,3, Everton 0,75, Liverpool
/// 0,9, Alemanha 1,2) entra só com `aplicar`; depois (sorteio(rep) + sorteio(rep) + 1) × 0,001 + 1, × novos, em +0x15c.
unsafe extern "thiscall" fn ingressos_da_temporada(this: usize, aplicar: i32) {
    let id = cpv5_d(this + 8);
    let clube = cpv5_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245));
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, i32)>(%%I1%%)(this, aplicar);
    }
    let rep = cpv5_w(clube + 0x80) as i32;
    let est = cpv5_d(clube + 0x69);
    if est != 0 && cpv5_i(est + 0x3c) < 0x5dc {
        core::ptr::write_unaligned((est + 0x3c) as *mut i32, ftol(rep as f64 * f64_em(0x968e18)));
        let est = cpv5_d(clube + 0x69);
        if cpv5_i(est + 0x3c) < 0x5dc {
            core::ptr::write_unaligned((est + 0x3c) as *mut i32, 0x5dc);
        }
    }
    let gerente = cpv5_d(cpv5_d(0xb67a44));
    let valor = cpv5_w(gerente.wrapping_add((cpv5_d(clube) as u32).wrapping_mul(73) as usize) + 8);
    let mut f = valor as f64 * f64_em(0x96a5c0);
    if !(f > f64_em(0x968f20)) {
        f = f64_em(0x968f20);
    }
    let t = ftol(cpv5_i(clube + 0x73) as f64 * f);
    let mut g = rep as f64 * f64_em(0x96a680);
    if !(g > f64_em(0x968968)) {
        g = f64_em(0x968968);
    }
    let u = ftol(t as f64 * g);
    let est = cpv5_d(clube + 0x69);
    let teto = |e: usize| if e != 0 { cpv5_i(e + 0x3c) as f64 * f64_em(0x96a678) } else { f64_em(0x96a5e8) };
    let n = ftol(if (u as f64) < teto(est) { u as f64 } else { teto(est) });
    if n == 0 {
        return;
    }
    let ja = cpv5_i(this + 0x18);
    if ja >= n {
        return;
    }
    let mut passo = (cpv5_b(this + 0x164) as i8).wrapping_add(1);
    if passo < 0 || passo > 8 {
        passo = 8;
    }
    *((this + 0x164) as *mut i8) = passo;
    let b = ftol(n.wrapping_sub(ja) as f64 * f32_em(0x9c8758 + passo as usize * 4) as f64);
    crate::comparacao::ramo("cpv_5 ingressos vendidos");
    let r = cpv5_sortear(0xb);
    let novos = (r + 0x5f).wrapping_mul(b) / 100;
    let total = ja.wrapping_add(novos);
    core::ptr::write_unaligned((this + 0x18) as *mut i32, total); // cpv5: os ingressos vendidos
    let preco = ftol((rep as f64 * f64_em(0x969048) + f64_em(0x968938)) * f64_em(0x968ea8));
    let mut receita = preco.wrapping_mul(novos);
    let est = cpv5_d(clube + 0x69);
    let lugares = if est != 0 { cpv5_i(est + 0x3c) } else { 0 };
    let den = if lugares != 0 { lugares as f64 * f64_em(0x96a678) } else { f64_em(0x96a5e8) };
    let razao = total as f64 / den;
    core::ptr::write_unaligned((this + 0x160) as *mut f32, razao as f32);
    if razao > f64_em(0x96a678) {
        core::ptr::write_unaligned((this + 0x160) as *mut u32, 0x3f600000);
    }
    let nac = cpv5_d(clube + 0x53);
    let tipo = if nac != 0 { cpv5_b(nac + 0x7e) as i8 } else { 3 };
    match tipo {
        1 => {}
        2 => receita /= 4,
        _ => receita /= 10,
    }
    let liga = cpv5_d(clube + 0x57);
    let cid = cpv5_i(clube);
    let fator = if liga != 0 && cpv5_i(liga) == cpv5_i(0x9cf570) {
        0x968928
    } else if cid == cpv5_i(0x9d0250) {
        0x968940
    } else if cid == cpv5_i(0x9d0110) {
        0x968de8
    } else if cid == cpv5_i(0x9d0248) {
        0x969040
    } else if cid == cpv5_i(0x9d0244) {
        0x9689d8
    } else if cid == cpv5_i(0x9d024c) {
        0x968de8
    } else if nac != 0 && cpv5_i(nac) == cpv5_i(0x9cf314) {
        0x968978
    } else {
        0
    };
    if fator != 0 {
        receita = ftol(receita as f64 * f64_em(fator));
    }
    if aplicar != 0 { // cpv5: a receita dos ingressos
        cpv5_somar(this + 0xac, receita);
        cpv5_somar(this + 0x14c, receita);
        cpv5_somar_saldo(this, receita);
    }
    let r1 = cpv5_sortear(rep);
    let r2 = cpv5_sortear(rep); // cpv5: o segundo sorteio da loja
    let k = r1.wrapping_add(r2).wrapping_add(1);
    let v = ftol((k as f64 * f64_em(0x96a350) + f64_em(0x967890)) * novos as f64);
    cpv5_somar(this + 0x15c, v);
    cpv5_somar(this + 0xbc, v);
    cpv5_somar_saldo(this, v);
}

/// Comparador: lado a lado no registro de finanças (this), nos lugares do estádio (+0x3c) e no estado do sorteio
/// (clube nulo: só a original).
unsafe extern "thiscall" fn comparar_ingressos_da_temporada(this: usize, aplicar: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(%%I1%%);
    let clube = cpv5_d(0xae23b4).wrapping_add(cpv5_d(this + 8).wrapping_mul(0x245));
    if clube == 0 {
        return o(this, aplicar);
    }
    let mut regioes = vec![(this, 0x167)];
    let est = cpv5_d(clube + 0x69);
    if est != 0 {
        regioes.push((est + 0x3c, 4));
    }
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, aplicar), || crate::aleatorio::sem_contar(|| ingressos_da_temporada(this, aplicar)));
    PLACAR.registrar(igual, || format!("ingressos_da_temporada(fin {:#x}, {}) original={:x?} rust={:x?}", this, aplicar, mo, mr));
}

/// O custo da obra (+0x8c e +0x12c, menos no saldo) no registro de finanças do clube.
unsafe fn cpv5_cobrar_obra(this: usize, clube: usize, custo: i32) {
    let fin = cpv5_financas(this, clube);
    cpv5_somar(fin + 0x12c, custo);
    cpv5_somar(fin + 0x8c, custo);
    cpv5_somar_saldo(fin, custo.wrapping_neg());
}

/// { 0x59b630 } (thiscall, ret 4, finance.cpp): `FinanceTable::upgrade_training_facilities(clube)`, pelo fim da
/// temporada do clube (0x59cfa0, com o pedido de treino aprovado pela diretoria). O nível (+0x7f) sobe 2 (1 a partir de
/// 19) e cobra nível × 50000 + 75000. Depois, abaixo de 20: com reputação (+0x80) acima de 8250, vai a 20 e cobra
/// (22 − nível) × 250000 (mínimo 2 milhões); abaixo de 14 com reputação acima de 5250, sobe 4 (mínimo 16) e cobra 3,5
/// milhões − nível × 150000 (mínimo 1 milhão); abaixo de 16 com reputação acima de 4250, ou abaixo de 20 com reputação
/// acima de 7250: acima de 7250 vai a 20, senão sobe 2 (mínimo 12), e cobra o novo nível × 50000 + 75000. Teto 20.
unsafe extern "thiscall" fn melhorar_o_centro_de_treino(this: usize, clube: usize) {
    let nivel = (clube + 0x7f) as *mut i8;
    let custo = (*nivel as i32).wrapping_mul(50000).wrapping_add(75000);
    *nivel = if *nivel < 0x13 { (*nivel).wrapping_add(2) } else { (*nivel).wrapping_add(1) };
    cpv5_cobrar_obra(this, clube, custo);
    let n = *nivel;
    let rep = cpv5_w(clube + 0x80);
    let segundo = if n < 0x14 && rep > 0x203a {
        *nivel = 0x14;
        Some((0x16 - n as i32).wrapping_mul(250000).max(2000000))
    } else if n < 0xe && rep > 0x1482 {
        let c = 3500000i32.wrapping_sub((n as i32).wrapping_mul(150000)).max(1000000);
        *nivel = n.wrapping_add(4);
        if *nivel < 0x10 {
            *nivel = 0x10;
        }
        Some(c)
    } else if (n < 0x10 && rep > 0x109a) || (n < 0x14 && rep > 0x1c52) {
        if rep > 0x1c52 {
            *nivel = 0x14;
        } else {
            *nivel = n.wrapping_add(2);
            if *nivel < 0xc {
                *nivel = 0xc;
            }
        }
        Some((*nivel as i32).wrapping_mul(50000).wrapping_add(75000))
    } else {
        None
    };
    if let Some(c) = segundo {
        cpv5_cobrar_obra(this, clube, c);
    }
    if *nivel > 0x14 {
        *nivel = 0x14;
    }
}

/// Comparador: lado a lado no nível do centro de treino (+0x7f) e no registro de finanças do clube.
unsafe extern "thiscall" fn comparar_melhorar_o_centro_de_treino(this: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    let regioes = [(clube + 0x7f, 1), (cpv5_financas(this, clube), 0x167)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, clube), || melhorar_o_centro_de_treino(this, clube));
    PLACAR.registrar(igual, || format!("melhorar_o_centro_de_treino(clube {:#x}) original={:x?} rust={:x?}", clube, mo, mr));
}
