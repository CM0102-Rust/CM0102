// ---------- 264a leva: a renda das partidas: a renda da partida nas finanças dos dois clubes, a renda do estádio e a multa da liga (fin_prm.cpp) ----------

unsafe fn cqb1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb1_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqb1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqb1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqb1_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O registro de finanças (0x167 bytes) do clube da partida em `campo` (+0x1c o mandante, +0x20 o visitante), como a
/// original o acha sem a 0x5a0590: o clube parceiro (0x540a50: 3 pushes; add esp, 0xc -> cdecl(clube, &a, 1)) se ele
/// existe e o clube é o B (a = 0), e [tabela] + id × 0x167. Some(0) sem clube; None se for seleção (0x53b900: push;
/// add esp, 4 -> cdecl): a original sai sem fazer nada.
unsafe fn cqb1_financas(tabela: usize, partida: usize, campo: usize) -> Option<usize> {
    let clube = cqb1_d(partida + campo);
    if clube == 0 {
        return Some(0);
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) != 0 {
        return None;
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut eh_a = 0u32;
    let p = parceiro(clube, &mut eh_a, 1);
    let c = if p != 0 && eh_a == 0 { p } else { clube };
    Some(cqb1_d(tabela).wrapping_add((cqb1_i(c) as usize).wrapping_mul(0x167)))
}

/// A divisão com o arredondamento para zero da original (os imul por constante mágica e o cdq/sar).
fn cqb1_div(a: i32, b: i32) -> i32 {
    a.wrapping_div(b)
}

/// { 0x59a5d0 } (thiscall, ret 8, fin_prm.cpp): `FinanceTable::process_match_income(partida, resultado)`, pelo fechamento
/// da partida do dia (0x6b5f00). Sem resultado ou sem partida: a caixa (a original). Seleção de um dos lados: nada. Os
/// valores: com público (+0x34 do resultado), a TV pelo público (1 / (sorteio(10) + 2) inteiro, que é 0, mais 1,75, ×
/// público) e a renda do estádio (+0x38); os custos do jogo (sem os dois clubes de +0x64 = 1: público / 4; senão por
/// faixas do público ou da capacidade +0x3c do estádio +0x18 da partida, com piso de 500 pela marca +0x40); os gastos
/// do mandante (de +0x64 = 1: sorteio + público × 1,25 + reputação / 2 + 300, × 1,25 acima de 4750, ou sorteio(400) +
/// público × 1,25 + 600 sem reputação; senão dois sorteios + público); o dinheiro da TV da competição pela fase (+0x3b)
/// ou pelo tipo (+0x3a: as finais pelas reputações, 6 e 4 pelo +0x69 da competição); tudo dividido pelo tipo da nação
/// (+0x7e: 2 e 3) e outra vez no País de Gales e nas Irlandas. Na liga do mandante (+0x57 = +0x14, fora de campo
/// neutro +0x3c): a TV 3/4 ao mandante e 1/4 ao visitante (+0x158, +0xb8, saldo), o líquido ao mandante (+0x140,
/// +0xa0, saldo) e, com estádio próprio (+0x6d, +0x69), a manutenção ((sorteio(15) + 1) × 0,01 + 0,85) × o +0x3c do
/// estádio (+0x12c, +0x8c, menos no saldo). Fora dela: metade de cada um para os dois.
unsafe extern "thiscall" fn renda_da_partida(tabela: usize, partida: usize, resultado: usize) {
    if resultado == 0 || partida == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1458)(tabela, partida, resultado); // a caixa
    }
    let Some(mandante) = cqb1_financas(tabela, partida, 0x1c) else { return };
    let Some(visitante) = cqb1_financas(tabela, partida, 0x20) else { return };
    let (mut tv, mut publico, mut renda) = (0i32, 0i32, 0i32);
    if cqb1_i(resultado + 0x34) != 0 {
        publico = cqb1_i(resultado + 0x34);
        renda = cqb1_i(resultado + 0x38);
        let q = cqb1_div(1, cqb1_sortear(10).wrapping_add(2));
        crate::comparacao::ramo("cqb_1 publico pagante");
        tv = ftol((q as f64 + f64_em(0x968f98)) * publico as f64); // cqb1: a TV pelo público (1,75)
    }
    let casa = cqb1_d(partida + 0x1c);
    let fora = cqb1_d(partida + 0x20);
    let mut custo; // o ebp da original
    if cqb1_b(casa + 0x64) != 1 && cqb1_b(fora + 0x64) != 1 {
        custo = ftol(publico as f64 * f64_em(0x9689e0));
    } else {
        let est = cqb1_d(partida + 0x18);
        let x = if est == 0 {
            let k = if publico > 20000 {
                0x96a6a0
            } else if publico > 10000 {
                0x96a698
            } else if publico > 5000 {
                0x968e48
            } else {
                0x968968
            };
            publico as f64 * f64_em(k)
        } else {
            let cap = cqb1_i(est + 0x3c);
            let k = if cap > 17500 {
                0x968fe0
            } else if cap > 12500 {
                0x968968
            } else if cap > 5000 {
                if publico < 5000 { 0x968e00 } else { 0x96a6a8 }
            } else {
                0x969510
            };
            cap as f64 * f64_em(k)
        };
        custo = ftol(x);
        if cqb1_b(partida + 0x40) != 0 && custo < 500 {
            custo = 500;
        }
    }
    let casa = cqb1_d(partida + 0x1c);
    let mut gastos; // o [esp+0x10] da original (começa com o público)
    if cqb1_b(casa + 0x64) == 1 {
        let rep = cqb1_w(casa + 0x80);
        if rep != 0 {
            let r = cqb1_sortear(0xc8);
            let x = (r as f64 + publico as f64 * f64_em(0x968928)) + rep as f64 * f64_em(0x968968) + f64_em(0x96a6b0);
            gastos = ftol(x);
            if rep > 0x128e {
                gastos = ftol(gastos as f64 * f64_em(0x968928));
            }
        } else {
            let r = cqb1_sortear(0x190);
            gastos = ftol((r as f64 + publico as f64 * f64_em(0x968928)) + f64_em(0x96a690));
        }
    } else {
        let rep = cqb1_w(casa + 0x80);
        let r1 = if rep != 0 { cqb1_sortear(ftol(rep as i32 as f64 * f64_em(0x9689e0))) } else { cqb1_sortear(0xfa) };
        let r1f = r1 as f32; // fstp dword: o primeiro sorteio guardado em float
        let r2 = cqb1_sortear(0xfa);
        gastos = ftol((r2 as f64 + r1f as f64) + publico as f64);
    }
    let mut dinheiro = 0i32; // o [esp+0x1c] da original
    let tipo_ok = match (cqb1_b(partida + 0x3b) as i8 as i32).wrapping_add(1) as u32 {
        1 => {
            dinheiro = 25000;
            true
        }
        2 => {
            dinheiro = 75000;
            true
        }
        3 => {
            dinheiro = 125000;
            true
        }
        _ => false, // 0 (fase −1) e fora da tabela: pelo tipo
    };
    if !tipo_ok {
        let t = cqb1_b(partida + 0x3a);
        if t == 8 || t == 7 {
            let soma = cqb1_w(casa + 0x80) as i32 + cqb1_w(fora + 0x80) as i32;
            dinheiro = if soma > 10000 {
                125000
            } else if soma <= 7500 {
                25000
            } else {
                75000
            };
        } else if t == 6 || t == 4 {
            let w = cqb1_w(cqb1_d(partida + 0x14) + 0x69);
            let m = if w > 5 { w as i32 } else { 5 };
            dinheiro = cqb1_div(m.wrapping_mul(if t == 6 { 125000 } else { 25000 }), 20);
        }
    }
    let nac = cqb1_d(casa + 0x53);
    let tipo = if nac != 0 { cqb1_b(nac + 0x7e) as i8 } else { 3 };
    if tipo == 2 {
        dinheiro = cqb1_div(dinheiro, 5);
        tv = cqb1_div(tv, 2);
        custo = cqb1_div(custo, 4);
        gastos = cqb1_div(gastos, 2);
    } else if tipo != 1 {
        dinheiro = cqb1_div(dinheiro, 10);
        tv = cqb1_div(tv, 10);
        custo = cqb1_div(custo, 15);
        gastos = cqb1_div(gastos, 10);
    }
    if casa != 0 && nac != 0 {
        let n = cqb1_i(nac);
        if n == cqb1_i(0x9cf510) || n == cqb1_i(0x9cf2dc) || n == cqb1_i(0x9cf3e4) {
            dinheiro = cqb1_div(dinheiro, 5);
            tv = cqb1_div(tv, 2);
            custo = cqb1_div(custo, 4);
            gastos = cqb1_div(gastos, 2);
        }
    }
    let liquido = tv.wrapping_sub(gastos).wrapping_sub(custo).wrapping_add(renda);
    let somar = |fin: usize, a: usize, b: usize, v: i32| {
        cpz13_somar32(fin + a, v);
        cpz13_somar32(fin + b, v);
        cpz13_somar_saldo(fin, v as i64);
    };
    if cqb1_b(partida + 0x3c) == 0 && cqb1_d(cqb1_d(partida + 0x1c) + 0x57) == cqb1_d(partida + 0x14) {
        let x = dinheiro as f64;
        somar(mandante, 0x158, 0xb8, ftol(x * f64_em(0x969040)));
        somar(visitante, 0x158, 0xb8, ftol(x * f64_em(0x9689e0)));
        crate::comparacao::ramo("cqb_1 liquido ao mandante");
        somar(mandante, 0x140, 0xa0, liquido); // cqb1: o líquido ao mandante
        let casa = cqb1_d(partida + 0x1c);
        let est = cqb1_d(casa + 0x69);
        if cqb1_b(casa + 0x6d) != 0 && est != 0 {
            let base = cqb1_i(est + 0x3c);
            let r = cqb1_sortear(0xf).wrapping_add(1);
            let c = ftol((r as f64 * f64_em(0x967898) + f64_em(0x969520)) * base as f64);
            cpz13_somar32(mandante + 0x12c, c);
            cpz13_somar32(mandante + 0x8c, c);
            cpz13_somar_saldo(mandante, -(c as i64));
        }
        return;
    }
    let metade = ftol(dinheiro as f64 * f64_em(0x968968));
    somar(mandante, 0x158, 0xb8, metade);
    somar(visitante, 0x158, 0xb8, metade);
    let l2 = cqb1_div(liquido, 2);
    somar(mandante, 0x140, 0xa0, l2);
    somar(visitante, 0x140, 0xa0, l2);
}

/// Comparador da 0x59a5d0: lado a lado nos dois registros de finanças e no estado do sorteio (os sorteios de 10, dos
/// gastos e da manutenção). Sem resultado, sem partida, sem um dos clubes ou com seleção: só a original.
unsafe extern "thiscall" fn comparar_renda_da_partida(tabela: usize, partida: usize, resultado: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1458);
    if resultado == 0 || partida == 0 || cqb1_d(partida + 0x1c) == 0 || cqb1_d(partida + 0x20) == 0 {
        return o(tabela, partida, resultado);
    }
    let (Some(fm), Some(fv)) = (cqb1_financas(tabela, partida, 0x1c), cqb1_financas(tabela, partida, 0x20)) else {
        return o(tabela, partida, resultado); // seleção: a original não faz nada
    };
    if fm == fv {
        return o(tabela, partida, resultado); // ponytail: o mesmo registro nos dois lados, só a original
    }
    let mut reg = vec![(fm, 0x167), (fv, 0x167)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(tabela, partida, resultado), || crate::aleatorio::sem_contar(|| renda_da_partida(tabela, partida, resultado)));
    PLACAR.registrar(igual, || format!("renda_da_partida(partida {:x}) original={:x?} rust={:x?}", partida, mo, mr));
}

/// { 0x59af40 } (thiscall, ret 8, fin_prm.cpp): `FinanceTable::calc_gate_receipts(partida, publico)`, pelo registro da
/// partida (0x729ff0, o +0x38 do resultado, que a 0x59a5d0 soma ao líquido do mandante). Sem partida: a caixa (a
/// original); sem público: 0; seleção mandante: 0. Em campo neutro (+0x3c) da competição +0x14: (+0x69 dela + 8) ×
/// público. Senão o preço pela reputação do mandante (+0x80 / 500): sem competição ou em amistoso (0x5c6710: push comp;
/// ecx = [0xae2c74]; sem add esp -> thiscall, ret 4), × 0,5 × público; senão + 4 (um dos clubes de +0x64 = 1) ou + 3, ×
/// público; na liga do mandante (+0x57), menos o desconto (float +0x160 das finanças) × o valor. Depois pelo tipo da
/// nação (+0x7e: 2 → /4; 3 → /10), × 1,25 na liga de 0x9cf570 ou pelos clubes de 0x9d0250 (1,5), 0x9d0110 e 0x9d024c
/// (1,3), 0x9d0248 (0,75) e 0x9d0244 (0,9), e × 1,2 na nação de 0x9cf314.
unsafe extern "thiscall" fn renda_do_estadio(tabela: usize, partida: usize, publico: i32) -> i32 {
    if partida == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i32) -> i32>(1459)(tabela, partida, publico); // a caixa
    }
    if publico == 0 {
        return 0;
    }
    let Some(fin) = cqb1_financas(tabela, partida, 0x1c) else { return 0 };
    let comp = cqb1_d(partida + 0x14);
    let mut v: i32;
    if cqb1_b(partida + 0x3c) != 0 && comp != 0 {
        v = (cqb1_w(comp + 0x69) as i32).wrapping_add(8).wrapping_mul(publico);
    } else {
        let casa = cqb1_d(partida + 0x1c);
        crate::comparacao::ramo("cqb_1 preco pela reputacao");
        let preco = cqb1_div(cqb1_w(casa + 0x80) as i32, 500); // cqb1: o preço pela reputação
        let amistoso = comp != 0 && {
            let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
            f(cqb1_d(0xae2c74), comp) != 0
        };
        if comp == 0 || amistoso {
            v = ftol(preco as f64 * f64_em(0x968968) * publico as f64);
        } else if cqb1_b(casa + 0x64) == 1 || cqb1_b(cqb1_d(partida + 0x20) + 0x64) == 1 {
            v = preco.wrapping_add(4).wrapping_mul(publico);
        } else {
            v = preco.wrapping_add(3).wrapping_mul(publico);
        }
        if fin != 0 && cqb1_d(casa + 0x57) == comp {
            let desconto = *((fin + 0x160) as *const f32) as f64;
            v = ftol(v as f64 - desconto * v as f64);
        }
    }
    let casa = cqb1_d(partida + 0x1c);
    let nac = cqb1_d(casa + 0x53);
    let tipo = if nac != 0 { cqb1_b(nac + 0x7e) as i8 } else { 3 };
    if tipo == 2 {
        v = cqb1_div(v, 4);
    } else if tipo != 1 {
        v = cqb1_div(v, 10);
    }
    let liga = cqb1_d(casa + 0x57);
    let k = if liga != 0 && cqb1_i(liga) == cqb1_i(0x9cf570) {
        Some(0x968928)
    } else {
        let id = cqb1_i(casa);
        if id == cqb1_i(0x9d0250) {
            Some(0x968940)
        } else if id == cqb1_i(0x9d0110) {
            Some(0x968de8)
        } else if id == cqb1_i(0x9d0248) {
            Some(0x969040)
        } else if id == cqb1_i(0x9d0244) {
            Some(0x9689d8)
        } else if id == cqb1_i(0x9d024c) {
            Some(0x968de8)
        } else {
            None
        }
    };
    if let Some(k) = k {
        v = ftol(v as f64 * f64_em(k));
    }
    if nac != 0 && cqb1_i(nac) == cqb1_i(0x9cf314) {
        v = ftol(v as f64 * f64_em(0x968978));
    }
    v
}

/// Comparador da 0x59af40: a devolução (só lê; a 0x540a50 pode zerar a marca +0x244 do clube, igual nas duas). Sem
/// partida: só a original (a caixa).
unsafe extern "thiscall" fn comparar_renda_do_estadio(tabela: usize, partida: usize, publico: i32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i32) -> i32>(1459);
    if partida == 0 {
        return o(tabela, partida, publico);
    }
    let r = o(tabela, partida, publico);
    let n = renda_do_estadio(tabela, partida, publico);
    PLACAR.registrar(r == n, || format!("renda_do_estadio(partida {:x}, publico {}) original={} rust={}", partida, publico, r, n));
    r
}

/// { 0x599850 } (thiscall, ret 4, fin_prm.cpp): `Finance::pay_league_fine(valor)`, pelas multas da liga (0x49f570,
/// 0x6a9e60): soma o valor às multas do ano (+0x128) e do mês (+0x88) e o tira do saldo.
unsafe extern "thiscall" fn pagar_multa_da_liga(this: usize, valor: i32) {
    cpz13_somar32(this + 0x128, valor);
    cpz13_somar32(this + 0x88, valor);
    cpz13_somar_saldo(this, -(valor as i64));
}

/// Comparador da 0x599850: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn comparar_pagar_multa_da_liga(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(1460);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pagar_multa_da_liga(this, valor));
    PLACAR.registrar(igual, || format!("pagar_multa_da_liga(valor {}) original={:x?} rust={:x?}", valor, mo, mr));
}

// ---------- 264a leva: os eventos raros das finanças: o impulso das finanças, a troca do presidente e os outros custos (fin_prm.cpp) ----------

unsafe fn cqb2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O byte com sinal em `a`.
unsafe fn cqb2_c(a: usize) -> i8 {
    *(a as *const i8)
}

/// O saldo (i64 em +0) separado em (baixo, alto), como a original o lê em dois dwords.
unsafe fn cqb2_saldo(fin: usize) -> (u32, i32) {
    (core::ptr::read_unaligned(fin as *const u32), core::ptr::read_unaligned((fin + 4) as *const i32))
}

/// O valor entra em +0x154, +0xb4 e no saldo (estendido com sinal).
unsafe fn cqb2_creditar(fin: usize, v: i32) {
    cpv6_somar(fin + 0x154, v);
    cpv6_somar(fin + 0xb4, v);
    cpv6_somar_saldo(fin, v);
}

/// A nação do clube é uma das de 0x9cf4a8, 0x9cf408 ou 0x9cf2c0 (o valor do evento cai a um terço).
unsafe fn cqb2_nacao_de_um_terco(clube: usize) -> bool {
    let nac = cqb2_d(clube + 0x53);
    if nac == 0 {
        return false;
    }
    let n = cpv6_i(nac);
    n == cpv6_i(0x9cf4a8) || n == cpv6_i(0x9cf408) || n == cpv6_i(0x9cf2c0)
}

/// { 0x59e050 } (thiscall, ret 8, fin_prm.cpp): `FinanceTable::try_finance_boost_event(clube, finanças)`, pelo passe dos
/// eventos da semana (0x59ed70). Fora do País de Gales, das Irlandas e das nações de 0x9cf408, 0x9cf2c0, 0x9cf4a8 e
/// 0x9cf244, e sem a marca +0x64 em 2 ou 3. O divisor pela faixa (0x5986b0): ricos 0,001; 0 pela reputação (até 4250:
/// 0,0125 com o presidente +0xbf de +0x57 e a ficha +0x69 de +0x20 acima de 15, senão 0,0075; até 6250: 0,05 ou 0,025
/// do mesmo jeito; acima: 0,025); −1: 0,1 acima de 5250, 0,075 acima de 3250, 0,025; −2: 0,125 acima de 3750, 0,075. Com
/// a ficha do presidente: sorteio(+0xf dela) acima de sorteio(30 / divisor), sorteio(+0x20 × 1 milhão) acima da dívida
/// e reputação abaixo de 750 × +0x20: (3 × (+0x20)² + 750) × reputação + 1 milhão, mais o saldo baixo (menos se o saldo
/// é negativo), com o teto de reputação² + 1,5 milhão abaixo de 2000. Sem ela: sorteio(15000 / divisor) abaixo da
/// reputação e sorteio(8) zero: (reputação + 1000) × 1000. Um terço nas nações de 0x9cf4a8, 0x9cf408 e 0x9cf2c0. Em
/// +0x154, +0xb4 e no saldo; a notícia (0x59f300) pelo tipo (pela verba 0x598800, a marca +0x82 e sorteios de 4: 3, 5,
/// 6, 7 com +0x82 = 1, ou 1); com a ficha, o +0x20 dela cai 1 acima de 5; encerra a crise (+0x165 = 0); devolve 1.
unsafe extern "thiscall" fn impulso_das_financas(this: usize, clube: usize, fin: usize) -> u32 {
    if clube != 0 {
        let nac = cqb2_d(clube + 0x53);
        if nac != 0 {
            let n = cpv6_i(nac);
            if [0x9cf510usize, 0x9cf2dc, 0x9cf3e4, 0x9cf408, 0x9cf2c0, 0x9cf4a8, 0x9cf244].iter().any(|&g| n == cpv6_i(g)) {
                return 0;
            }
            let m = cpv6_b(clube + 0x64);
            if m == 2 || m == 3 {
                return 0;
            }
        }
    }
    let rep = cpv6_w(clube + 0x80);
    let pres = cqb2_d(clube + 0xbf);
    let forte = |a: usize, b: usize| a != 0 && b != 0; // (ponteiros conferidos antes dos bytes)
    let div = match cpv6_faixa(fin) {
        -2 => f64_em(if rep > 0xea6 { 0x968fd0 } else { 0x96a730 }),
        -1 => f64_em(if rep > 0x1482 { 0x967880 } else if rep > 0xcb2 { 0x96a730 } else { 0x967878 }),
        0 => {
            if rep <= 0x109a {
                let ficha = if pres != 0 { cqb2_d(pres + 0x69) } else { 0 };
                if forte(pres, ficha) && cqb2_c(pres + 0x57) > 0xf && cqb2_c(ficha + 0x20) > 0xf {
                    f64_em(0x96a740)
                } else {
                    crate::comparacao::ramo("cqb_2 divisor da faixa 0 baixa");
                    f64_em(0x96a738) // cqb2: o divisor da faixa 0 de reputação baixa
                }
            } else if rep < 0x186a {
                let ficha = if pres != 0 { cqb2_d(pres + 0x69) } else { 0 };
                if forte(pres, ficha) && cqb2_c(ficha + 0x20) > 0xf && cqb2_c(pres + 0x57) > 0xf {
                    f64_em(0x967888)
                } else {
                    f64_em(0x967878)
                }
            } else {
                f64_em(0x967878)
            }
        }
        _ => f64_em(0x96a350),
    };
    let ficha = if pres != 0 { cqb2_d(pres + 0x69) } else { 0 };
    let valor;
    if pres != 0 && ficha != 0 {
        crate::comparacao::ramo("cqb_2 com a ficha");
        let r1 = cpv6_sortear(ftol(f64_em(0x968dd8) / div)); // cqb2: o sorteio com a ficha
        let r2 = cpv6_sortear(cqb2_c(ficha + 0xf) as i32);
        if r2 <= r1 {
            return 0;
        }
        let (lo, hi) = cqb2_saldo(fin);
        let n = cqb2_c(cqb2_d(cqb2_d(clube + 0xbf) + 0x69) + 0x20) as i32;
        let r3 = cpv6_sortear(n.wrapping_mul(1_000_000)) as i64;
        let divida = ((hi as i64) << 32 | lo as i64).wrapping_neg();
        if r3 <= divida {
            return 0;
        }
        let rep = rep as i32;
        let n = cqb2_c(cqb2_d(cqb2_d(clube + 0xbf) + 0x69) + 0x20) as i32;
        if rep >= n.wrapping_mul(750) {
            return 0;
        }
        // ponytail: a original faz pow(+0x20, 2,0) (0x945ff0); o quadrado de um byte é exato nos dois
        let mut v = ftol(((n as f64 * n as f64) * f64_em(0x968930) + f64_em(0x96a5f0)) * rep as f64 + f64_em(0x9688d8));
        v = if hi < 0 { v.wrapping_sub(lo as i32) } else { v.wrapping_add(lo as i32) };
        if cpv6_w(clube + 0x80) < 0x7d0 {
            let teto = rep.wrapping_mul(rep).wrapping_add(0x16e360);
            if v > teto {
                v = teto;
            }
        }
        if cqb2_nacao_de_um_terco(clube) {
            v /= 3;
        }
        valor = v;
    } else {
        crate::comparacao::ramo("cqb_2 sem a ficha");
        let r = cpv6_sortear(ftol(f64_em(0x96a728) / div)); // cqb2: o sorteio sem a ficha
        if rep as i32 <= r {
            return 0;
        }
        if cpv6_sortear(8) != 0 {
            return 0;
        }
        let mut v = (cpv6_w(clube + 0x80) as i32).wrapping_add(1000).wrapping_mul(1000);
        if cqb2_nacao_de_um_terco(clube) {
            v /= 3;
        }
        valor = v;
    }
    cqb2_creditar(fin, valor);
    let verba = cpv6_verba(fin) as i32;
    let marca = cpv6_b(clube + 0x82);
    let com_ficha = pres != 0 && ficha != 0;
    let mut tipo = 0u32;
    if verba <= 0 && marca == 0 {
        tipo = 3;
    } else if marca != 0 && cpv6_sortear(4) == 0 {
        tipo = 5;
    }
    if tipo == 0 {
        if cpv6_sortear(4) == 0 {
            if com_ficha {
                let r2 = cpv6_w(clube + 0x80) as i32;
                let t = ftol(r2.wrapping_mul(r2) as f64 * f64_em(0x968e80));
                if valor < t && valor > 50000 {
                    tipo = 6;
                }
            } else {
                tipo = 6;
            }
        }
    }
    if tipo == 0 {
        let nac = cqb2_d(clube + 0x53);
        if cpv6_b(clube + 0x82) == 0 && cpv6_w(clube + 0x80) > 0x128e && cpv6_sortear(4) == 0
            && (!com_ficha || (nac != 0 && cpv6_i(nac) != cpv6_i(0x9cf360)))
        {
            cpv6_noticia(this, clube, valor, verba as u32, 7);
            *((clube + 0x82) as *mut u8) = 1;
            tipo = 7;
        } else {
            tipo = 1;
        }
    }
    if tipo != 7 {
        cpv6_noticia(this, clube, valor, verba as u32, tipo);
    }
    if com_ficha {
        let f = cqb2_d(cqb2_d(clube + 0xbf) + 0x69);
        let x = cqb2_c(f + 0x20);
        if x > 5 {
            *((f + 0x20) as *mut i8) = x - 1;
        }
    }
    *((fin + 0x165) as *mut u8) = 0;
    1
}

/// O comparador dos eventos raros, em ensaio (modelo CPV6_ENSAIO: a cópia sem a notícia nem a verba, que alocam; sem a
/// troca do presidente na 0x59e940): a cópia e a original a partir do mesmo estado (o registro de finanças, o sorteio e
/// os `extras`). Sem evento: tudo igual; com evento: a devolução, o saldo, +0xb4, +0x154, +0x165 e os `conferir` (a
/// tipo da notícia e a marca +0x82 dependem da verba, e os sorteios depois dela também).
unsafe fn cqb2_comparar(nome: &str, fin: usize, extras: &[(usize, usize)], conferir: &[usize], original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let mut regioes = vec![(fin, 0x167)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    regioes.extend_from_slice(extras);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPV6_ENSAIO = true;
    let n = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(copia));
    CPV6_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let r = original();
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = if r == 0 && n == 0 {
        mo == mr
    } else {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let campos = r == n && a[..8] == b[..8] && a[0xb4..0xb8] == b[0xb4..0xb8] && a[0x154..0x158] == b[0x154..0x158] && a[0x165] == b[0x165];
        campos && conferir.iter().all(|&k| mo.0[3 + k].1 == mr.0[3 + k].1)
    };
    PLACAR.registrar(igual, || format!("{}(fin {:#x}) original={} {:x?} rust={} {:x?}", nome, fin, r, mo, n, mr));
    r
}

unsafe extern "thiscall" fn comparar_impulso_das_financas(this: usize, clube: usize, fin: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1461);
    let pres = if clube != 0 { cqb2_d(clube + 0xbf) } else { 0 };
    let ficha = if pres != 0 { cqb2_d(pres + 0x69) } else { 0 };
    if clube == 0 || fin == 0 {
        return o(this, clube, fin); // ponytail: sem clube ou sem finanças, só a original
    }
    let mut extras = vec![(clube + 0x82, 1)];
    let mut conferir = Vec::new();
    if ficha != 0 {
        extras.push((ficha + 0x20, 1)); // o +0x20 da ficha cai em todo evento com ela
        conferir.push(1);
    }
    cqb2_comparar("impulso_das_financas", fin, &extras, &conferir, || o(this, clube, fin), || impulso_das_financas(this, clube, fin))
}

/// { 0x59e940 } (thiscall, ret 8, fin_prm.cpp): `FinanceTable::try_board_takeover(clube, finanças)`, pelo passe dos
/// eventos da semana (0x59ed70, clubes endividados). O divisor pela faixa (0x5986b0: −2: 0,4 acima de reputação 4250,
/// senão 0,25; −1: 0,2 ou 0,05; outras: 0,001) e o fator (reputação × 0,002 / o +0x69 da liga +0x57, no mínimo 0,75;
/// sem liga 0,5). Com a ficha (+0x69) do presidente (+0xbf): sorteio(+0x16 dela) zero; sem ela: sorteio(20) zero. Depois
/// sorteio(15000 / divisor) abaixo de reputação × fator: (reputação + 1000) × 1000 menos o saldo baixo; acima de 40
/// milhões, com a reputação abaixo de sorteio(2500) + 2000, só o que zera o saldo. Em +0xb4, +0x154 e no saldo; com a
/// ficha, o presidente novo: a nação (a do clube se sorteio(1000) ≠ 0, senão uma sorteada), os nomes (0x539c60,
/// 0x53a5b0, 0x53a1b0) e a ficha (+0x1d, +0xf, +0x16, +0x20 = sorteio(20) + 1; o +0x20 de novo se ficou abaixo de 5); a
/// notícia (0x59f300, tipo 2, com a verba 0x598800); encerra a crise (+0x165 = 0); devolve 1.
unsafe extern "thiscall" fn troca_do_presidente(this: usize, clube: usize, fin: usize) -> u32 {
    let rep = cpv6_w(clube + 0x80);
    let div = match cpv6_faixa(fin) {
        -2 => if rep > 0x109a { 0.4f64 } else { 0.25 },
        -1 => if rep > 0x109a { 0.2 } else { 0.05 },
        _ => 0.001,
    };
    let liga = cqb2_d(clube + 0x57);
    let mut fator = if liga != 0 {
        let k = rep as i32 as f64 * f64_em(0x969048) / cpv6_w(liga + 0x69) as i32 as f64;
        if !(k >= f64_em(0x969040)) { f64_em(0x969040) } else { k } // fcomp: menor ou sem ordem (NaN) vira 0,75
    } else {
        0.5
    };
    let pres = cqb2_d(clube + 0xbf);
    let ficha = if pres != 0 { cqb2_d(pres + 0x69) } else { 0 };
    let com_ficha = pres != 0 && ficha != 0;
    let primeiro = if com_ficha { cpv6_sortear(cqb2_c(ficha + 0x16) as i32) } else { cpv6_sortear(0x14) };
    if primeiro != 0 {
        return 0;
    }
    fator *= cpv6_w(clube + 0x80) as i32 as f64;
    let r = cpv6_sortear(ftol(f64_em(0x96a728) / div));
    if r as f64 >= fator {
        return 0;
    }
    crate::comparacao::ramo("cqb_2 troca do presidente");
    let (lo, _) = cqb2_saldo(fin);
    let mut v = (cpv6_w(clube + 0x80) as i32).wrapping_add(1000).wrapping_mul(1000).wrapping_sub(lo as i32);
    if v > 0x2625a00 {
        let s = cpv6_sortear(0x9c4);
        if (cpv6_w(clube + 0x80) as i32) < s.wrapping_add(0x7d0) {
            v = (cqb2_saldo(fin).0 as i32).wrapping_neg();
        }
    }
    cpv6_somar(fin + 0xb4, v);
    cpv6_somar(fin + 0x154, v);
    cpv6_somar_saldo(fin, v);
    let verba = cpv6_verba(fin);
    if com_ficha && !CPV6_ENSAIO {
        let pres = cqb2_d(clube + 0xbf);
        let nac = cqb2_d(clube + 0x53);
        let nova = if nac != 0 && cpv6_sortear(0x3e8) != 0 {
            nac
        } else {
            let k = cpv6_sortear(cpv6_i(0xae2350));
            cqb2_d(0xae23a8).wrapping_add((k as usize).wrapping_mul(0x122))
        };
        core::ptr::write_unaligned((pres + 0x1a) as *mut usize, nova);
        // 0x539c60, 0x53a5b0, 0x53a1b0: push nação; call (add esp, 0xc depois das três) -> cdecl(nação)
        let nome1: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x539c60usize);
        let nome3: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53a5b0usize);
        let nome2: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53a1b0usize);
        let x = nome1(cqb2_d(cqb2_d(clube + 0xbf) + 0x1a));
        core::ptr::write_unaligned((cqb2_d(clube + 0xbf) + 4) as *mut u32, x);
        let x = nome3(cqb2_d(cqb2_d(clube + 0xbf) + 0x1a));
        core::ptr::write_unaligned((cqb2_d(clube + 0xbf) + 0xc) as *mut u32, x);
        let x = nome2(cqb2_d(cqb2_d(clube + 0xbf) + 0x1a));
        core::ptr::write_unaligned((cqb2_d(clube + 0xbf) + 8) as *mut u32, x);
        if cqb2_d(cqb2_d(clube + 0xbf) + 0x69) != 0 {
            for o in [0x1dusize, 0xf, 0x16, 0x20] {
                let x = (cpv6_sortear(0x14) as u8).wrapping_add(1);
                *((cqb2_d(cqb2_d(clube + 0xbf) + 0x69) + o) as *mut u8) = x;
            }
            if cqb2_c(cqb2_d(cqb2_d(clube + 0xbf) + 0x69) + 0x20) < 5 {
                let x = (cpv6_sortear(0x14) as u8).wrapping_add(1);
                *((cqb2_d(cqb2_d(clube + 0xbf) + 0x69) + 0x20) as *mut u8) = x;
            }
        }
    }
    cpv6_noticia(this, clube, v, verba, 2);
    *((fin + 0x165) as *mut u8) = 0;
    1
}

unsafe extern "thiscall" fn comparar_troca_do_presidente(this: usize, clube: usize, fin: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1462);
    if clube == 0 || fin == 0 {
        return o(this, clube, fin); // ponytail: sem clube ou sem finanças, só a original
    }
    cqb2_comparar("troca_do_presidente", fin, &[], &[], || o(this, clube, fin), || troca_do_presidente(this, clube, fin))
}

/// { 0x5999e0 } (thiscall, ret 4, fin_prm.cpp): `Finance::pay_other_costs(valor)`, pela 0x7e5b10: soma o valor aos
/// outros custos do ano (+0x120) e do mês (+0x80) e o tira do saldo.
unsafe extern "thiscall" fn pagar_outros_custos(this: usize, valor: i32) {
    cpv6_somar(this + 0x120, valor);
    cpv6_somar(this + 0x80, valor);
    cpz13_somar_saldo(this, -(valor as i64));
}

/// Comparador da 0x5999e0: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn comparar_pagar_outros_custos(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(1463);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pagar_outros_custos(this, valor));
    PLACAR.registrar(igual, || format!("pagar_outros_custos(valor {}) original={:x?} rust={:x?}", valor, mo, mr));
}

// ---------- 264a leva: as datas internacionais dos amistosos: a tabela das datas, as datas no ciclo da Copa e o torneio do clube humano (friendly.cpp) ----------

unsafe fn cqb3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqb3_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqb3_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// As 110 datas internacionais da 0x5ca4f0, na ordem em que a original as grava: (+0 o dia da semana, +1 o dia, +2 o
/// mês a partir de 0, +0xb o ano a somar ao da Copa, o global do continente em +0xc, o global da competição em +0x10
/// ou 0 para −1). Extraídas da desmontagem (0x5ca588..0x5cb8e6); as 5 últimas vão pela 0x5cb9b0.
const CQB3_DATAS: [(u8, u8, u8, u8, usize, usize); 110] = [
    (5, 10, 9, 0, 0x9cfa08, 0),
    (2, 18, 10, 0, 0x9cfa08, 0),
    (6, 15, 11, 0, 0x9cfa08, 0),
    (6, 12, 0, 1, 0x9cfa08, 0),
    (6, 24, 1, 1, 0x9cfa08, 0),
    (2, 24, 2, 1, 0x9cfa08, 0),
    (2, 28, 3, 1, 0x9cfa08, 0),
    (2, 9, 5, 1, 0x9cfa08, 0),
    (6, 24, 6, 1, 0x9cfa08, 0),
    (2, 18, 7, 1, 0x9cfa08, 0),
    (5, 9, 9, 1, 0x9cfa08, 0),
    (5, 1, 5, 2, 0x9cfa08, 0x9cf77c),
    (5, 15, 5, 2, 0x9cfa08, 0x9cf77c),
    (6, 1, 9, 2, 0x9cfa08, 0x9cf77c),
    (6, 10, 10, 2, 0x9cfa08, 0x9cf77c),
    (6, 12, 0, 3, 0x9cfa08, 0x9cf77c),
    (6, 24, 1, 3, 0x9cfa08, 0x9cf77c),
    (5, 5, 3, 3, 0x9cfa08, 0x9cf77c),
    (6, 27, 3, 3, 0x9cfa08, 0x9cf77c),
    (5, 7, 5, 3, 0x9cfa08, 0x9cf77c),
    (6, 24, 6, 3, 0x9cfa08, 0x9cf77c),
    (6, 17, 7, 3, 0x9cfa08, 0x9cf77c),
    (6, 28, 8, 3, 0x9cfa08, 0x9cf77c),
    (5, 20, 9, 3, 0x9cfa08, 0x9cf77c),
    (2, 16, 7, 0, 0x9cfa1c, 0),
    (2, 6, 8, 0, 0x9cfa1c, 0),
    (2, 7, 9, 0, 0x9cfa1c, 0),
    (2, 15, 10, 0, 0x9cfa1c, 0),
    (2, 29, 2, 1, 0x9cfa1c, 0),
    (2, 26, 3, 1, 0x9cfa1c, 0),
    (2, 16, 7, 1, 0x9cfa1c, 0),
    (2, 6, 8, 1, 0x9cfa1c, 0),
    (2, 7, 9, 1, 0x9cfa1c, 0),
    (2, 26, 3, 2, 0x9cfa1c, 0x9cf778),
    (6, 26, 6, 2, 0x9cfa1c, 0x9cf778),
    (2, 6, 8, 2, 0x9cfa1c, 0x9cf778),
    (5, 7, 9, 2, 0x9cfa1c, 0x9cf778),
    (2, 15, 10, 2, 0x9cfa1c, 0x9cf778),
    (2, 26, 3, 3, 0x9cfa1c, 0x9cf778),
    (6, 4, 5, 3, 0x9cfa1c, 0x9cf778),
    (6, 26, 6, 3, 0x9cfa1c, 0x9cf778),
    (2, 16, 7, 3, 0x9cfa1c, 0x9cf778),
    (2, 6, 8, 3, 0x9cfa1c, 0x9cf778),
    (2, 7, 9, 3, 0x9cfa1c, 0x9cf778),
    (2, 15, 10, 3, 0x9cfa1c, 0x9cf778),
    (5, 10, 9, 0, 0x9cfa18, 0),
    (2, 18, 10, 0, 0x9cfa18, 0),
    (6, 15, 11, 0, 0x9cfa18, 0),
    (6, 10, 0, 1, 0x9cfa18, 0),
    (2, 24, 2, 1, 0x9cfa18, 0),
    (6, 25, 6, 1, 0x9cfa18, 0),
    (1, 7, 8, 1, 0x9cfa18, 0),
    (5, 9, 9, 1, 0x9cfa18, 0),
    (2, 17, 10, 1, 0x9cfa18, 0),
    (6, 10, 11, 2, 0x9cfa18, 0x9cf76c),
    (6, 1, 1, 3, 0x9cfa18, 0x9cf76c),
    (5, 10, 3, 3, 0x9cfa18, 0x9cf76c),
    (5, 15, 3, 3, 0x9cfa18, 0x9cf76c),
    (5, 10, 6, 3, 0x9cfa18, 0x9cf76c),
    (5, 17, 8, 3, 0x9cfa18, 0x9cf76c),
    (5, 29, 9, 3, 0x9cfa18, 0x9cf76c),
    (5, 29, 10, 3, 0x9cfa18, 0x9cf76c),
    (5, 24, 2, 0, 0x9cfa10, 0),
    (2, 25, 3, 0, 0x9cfa10, 0),
    (5, 2, 8, 0, 0x9cfa10, 0x9cf784),
    (2, 15, 10, 0, 0x9cfa10, 0),
    (5, 7, 9, 0, 0x9cfa10, 0x9cf784),
    (2, 15, 10, 1, 0x9cfa10, 0),
    (5, 24, 2, 1, 0x9cfa10, 0x9cf784),
    (2, 25, 3, 1, 0x9cfa10, 0x9cf784),
    (2, 6, 5, 1, 0x9cfa10, 0x9cf784),
    (5, 2, 8, 1, 0x9cfa10, 0x9cf784),
    (2, 6, 8, 1, 0x9cfa10, 0x9cf784),
    (5, 24, 2, 2, 0x9cfa10, 0),
    (2, 25, 3, 2, 0x9cfa10, 0),
    (5, 7, 9, 2, 0x9cfa10, 0x9cf784),
    (2, 15, 10, 2, 0x9cfa10, 0),
    (2, 15, 10, 3, 0x9cfa10, 0),
    (5, 7, 9, 2, 0x9cfa10, 0x9cf780),
    (5, 24, 2, 3, 0x9cfa10, 0x9cf780),
    (2, 25, 3, 3, 0x9cfa10, 0x9cf780),
    (2, 6, 5, 3, 0x9cfa10, 0x9cf780),
    (5, 2, 8, 3, 0x9cfa10, 0x9cf780),
    (2, 6, 8, 3, 0x9cfa10, 0x9cf780),
    (5, 7, 9, 3, 0x9cfa10, 0x9cf780),
    (5, 25, 7, 0, 0x9cfa14, 0),
    (2, 18, 10, 0, 0x9cfa14, 0),
    (6, 15, 11, 0, 0x9cfa14, 0),
    (6, 10, 0, 1, 0x9cfa14, 0),
    (6, 23, 1, 1, 0x9cfa14, 0),
    (2, 24, 2, 1, 0x9cfa14, 0),
    (2, 28, 3, 1, 0x9cfa14, 0),
    (1, 7, 8, 1, 0x9cfa14, 0),
    (5, 9, 9, 1, 0x9cfa14, 0),
    (5, 25, 6, 2, 0x9cfa14, 0x9cf770),
    (5, 25, 7, 2, 0x9cfa14, 0x9cf770),
    (6, 17, 10, 2, 0x9cfa14, 0x9cf770),
    (6, 26, 0, 3, 0x9cfa14, 0x9cf770),
    (6, 23, 1, 3, 0x9cfa14, 0x9cf770),
    (6, 2, 2, 3, 0x9cfa14, 0x9cf770),
    (5, 15, 2, 3, 0x9cfa14, 0x9cf770),
    (6, 20, 3, 3, 0x9cfa14, 0x9cf770),
    (6, 29, 5, 3, 0x9cfa14, 0x9cf770),
    (2, 9, 8, 3, 0x9cfa14, 0x9cf770),
    (2, 1, 9, 3, 0x9cfa14, 0x9cf770),
    (6, 16, 10, 3, 0x9cfa14, 0x9cf770),
    (2, 9, 5, 1, 0x9cfa0c, 0),
    (1, 7, 8, 1, 0x9cfa0c, 0),
    (2, 4, 5, 3, 0x9cfa0c, 0x9cf774),
    (5, 13, 8, 3, 0x9cfa0c, 0x9cf774),
];

/// { 0x5ca4f0 } (thiscall, ret 0, friendly.cpp): `Friendly::build_international_date_table()`, pelo construtor da
/// competição de amistosos (0x5c1f80, na criação do jogo). Conta 0xc8 em +0x34f0 e a tabela de 0xfa0 bytes (malloc
/// 0x944e46: push n; add esp, 4 -> cdecl) em +0x34ec (sem memória: a caixa da linha 0x15d5, conta 0: a original);
/// grava as 110 datas de 20 bytes (as 105 primeiras direto, as 5 últimas pela 0x5cb9b0: mov ecx, entrada; 6 pushes;
/// sem add esp -> thiscall, ret 0x18); com a conta abaixo de 110 (nunca: é a 0xc8 de cima) a caixa da linha 0x15d8;
/// conta 110, a tabela reduzida a 0x898 bytes (realloc 0x945667: 2 pushes; add esp, 8 -> cdecl), as datas no ciclo
/// da Copa (0x5cc510: mov ecx, esi; sem push -> thiscall) e o último dia feito (+0x34f4) = 400.
unsafe extern "thiscall" fn tabela_das_datas_internacionais(this: usize) {
    *((this + 0x34f0) as *mut i32) = 0xc8;
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = malloc(0xfa0);
    *((this + 0x34ec) as *mut usize) = t;
    if t == 0 {
        // ponytail: sem memória, a caixa; a original refaz a conta e cai nela (raro demais para copiar a caixa)
        *((this + 0x34f0) as *mut i32) = 0xc8;
        return orig::<unsafe extern "thiscall" fn(usize)>(1464)(this);
    }
    let preencher: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(0x5cb9b0usize);
    for (k, &(semana, dia, mes, ano, continente, competicao)) in CQB3_DATAS.iter().enumerate() {
        let e = cqb3_d(this + 0x34ec) + k * 0x14;
        let comp = if competicao == 0 { -1 } else { cqb3_i(competicao) };
        if k >= 105 {
            // ponytail: a original empurra registradores inteiros (o ecx, o edx, o ebx) com o byte certo embaixo; a 0x5cb9b0
            // só lê os bytes: aqui, os bytes limpos
            preencher(e, semana as u32, dia as u32, mes as u32, ano as u32, cqb3_i(continente) as u32, comp as u32);
            continue;
        }
        *(e as *mut u8) = semana;
        *((e + 1) as *mut u8) = dia;
        *((e + 2) as *mut u8) = mes;
        *((e + 0xb) as *mut u8) = ano; // cqb3: o ano a somar ao da Copa
        core::ptr::write_unaligned((e + 0xc) as *mut i32, cqb3_i(continente));
        core::ptr::write_unaligned((e + 0x10) as *mut i32, comp);
    }
    if cqb3_i(this + 0x34f0) < 0x6e {
        return; // ponytail: inalcançável (a conta é a 0xc8 gravada acima); a original dá a caixa da linha 0x15d8
    }
    crate::comparacao::ramo("cqb_3 tabela montada");
    *((this + 0x34f0) as *mut i32) = 0x6e;
    let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    *((this + 0x34ec) as *mut usize) = realloc(cqb3_d(this + 0x34ec), 0x898);
    let resolver: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5cc510usize);
    resolver(this);
    *((this + 0x34f4) as *mut u16) = 0x190;
}

/// Comparador da 0x5ca4f0: a cópia primeiro (aloca a tabela dela, que sai copiada e liberada: free 0x9452ca, cdecl),
/// depois a original a partir dos mesmos +0x34ec..+0x34f5; confere a conta, o último dia feito e as 110 datas. Roda
/// uma vez, no construtor dos amistosos.
unsafe extern "thiscall" fn comparar_tabela_das_datas_internacionais(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1464);
    let campos = crate::comparacao::Memoria::ler(&[(this + 0x34ec, 10)]);
    tabela_das_datas_internacionais(this);
    let (t, n) = (cqb3_d(this + 0x34ec), cqb3_i(this + 0x34f0));
    if t == 0 || n != 0x6e {
        campos.escrever();
        return o(this); // ponytail: sem memória na cópia, só a original
    }
    let mr = crate::comparacao::Memoria::ler(&[(t, 0x898), (this + 0x34f0, 6)]);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    free(t);
    campos.escrever();
    o(this);
    let t = cqb3_d(this + 0x34ec);
    if t == 0 {
        return PLACAR.registrar(false, || "tabela_das_datas_internacionais: a original ficou sem tabela".to_string());
    }
    let mo = crate::comparacao::Memoria::ler(&[(t, 0x898), (this + 0x34f0, 6)]);
    PLACAR.registrar(mo.0[0].1 == mr.0[0].1 && mo.0[1].1 == mr.0[1].1, || format!("tabela_das_datas_internacionais original={:x?} rust={:x?}", mo, mr));
}

/// { 0x5cc510 } (thiscall, ret 0, friendly.cpp): `Friendly::resolve_international_dates()`, pela tabela (0x5ca4f0). O
/// ano da Copa: o primeiro a partir do de hoje ([0xae2c92]) com (ano − 2) múltiplo de 4. Cada data (+0x34ec, conta
/// +0x34f0) ganha a data em +3 pela 0x54a130 (lea ecx, entrada + 3; 4 pushes; sem add esp -> thiscall(data, dia, mês,
/// ano, dia da semana), ret 0x10) com o dia (+1) e o dia da semana (+0) em byte com sinal, o mês (+2) e o ano da Copa
/// mais o +0xb (com sinal), menos 4 se passar de 3 anos à frente de hoje (palavras). Depois o qsort pela data (0x945333:
/// 4 pushes; add esp, 0x10 -> cdecl(tabela, conta, 0x14, 0x5cc610)).
unsafe extern "thiscall" fn resolver_datas_internacionais(this: usize) {
    let hoje = cqb3_w(0xae2c92);
    let mut ciclo = hoje as u32;
    while ((ciclo & 0xffff) as i32 - 2) % 4 != 0 {
        crate::comparacao::ramo("cqb_3 ano da copa adiante");
        ciclo = ciclo.wrapping_add(1); // cqb3: o ano da Copa
    }
    let acertar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x54a130usize);
    let mut k = 0i32;
    while k < cqb3_i(this + 0x34f0) {
        let e = cqb3_d(this + 0x34ec).wrapping_add((k as usize).wrapping_mul(0x14));
        let mut ano = (cqb3_b(e + 0xb) as i8 as i16 as u16).wrapping_add(ciclo as u16);
        if ano as i32 - hoje as i32 > 3 {
            crate::comparacao::ramo("cqb_3 ano para tras");
            ano = ano.wrapping_add(0xfffc); // cqb3: menos 4 se passar de 3 anos à frente
        }
        let com_sinal = |o: usize| cqb3_b(e + o) as i8 as i16 as u16 as u32;
        // ponytail: o dia e o dia da semana vão sobre o edx de metade alta zerada (como a original); o mês e o ano vão
        // sobre o eax, de metade alta lixo, e a 0x54a130 só lê o byte do mês e a palavra do ano: aqui, limpos
        acertar(e + 3, com_sinal(1), cqb3_b(e + 2) as u32, ano as u32, com_sinal(0));
        k += 1;
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cqb3_d(this + 0x34ec), cqb3_i(this + 0x34f0), 0x14, 0x5cc610);
}

/// Comparador da 0x5cc510: lado a lado nas datas da tabela (só grava nelas; o qsort e a 0x54a130 não sorteiam).
unsafe extern "thiscall" fn comparar_resolver_datas_internacionais(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1465);
    let (t, n) = (cqb3_d(this + 0x34ec), cqb3_i(this + 0x34f0));
    let reg = if t != 0 && n > 0 { vec![(t, n as usize * 0x14)] } else { Vec::new() };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || resolver_datas_internacionais(this));
    PLACAR.registrar(igual, || format!("resolver_datas_internacionais(conta {}) original={:x?} rust={:x?}", n, mo, mr));
}

/// { 0x5c6830 } (thiscall, ret 0, friendly.cpp): `Friendly::find_human_club_tournament()`, pelas telas dos amistosos. O
/// clube (+0x39) do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]); entre os 127 torneios (0x69 bytes a partir
/// de +0xb1), o primeiro com esse anfitrião (+0) e em uso (+0x29 ≠ 0): o ponteiro dele; senão 0.
unsafe extern "thiscall" fn torneio_do_clube_humano(this: usize) -> usize {
    let tecnico = cqb3_d(0xb74c26usize.wrapping_add((*(0xb77c7a as *const i16) as i32 as usize).wrapping_mul(0x300)));
    if tecnico == 0 {
        return 0;
    }
    let clube = cqb3_d(tecnico + 0x39);
    if clube == 0 {
        return 0;
    }
    for i in 0..0x7fusize {
        let t = this + 0x69 * i + 0xb1;
        let anfitriao = cqb3_d(t);
        if anfitriao != 0 && anfitriao == clube && cqb3_d(t + 0x29) != 0 {
            return t;
        }
    }
    0
}

comparar_valor!(comparar_torneio_do_clube_humano, torneio_do_clube_humano, 1466, () -> usize);

// ---------- 264a leva: os amistosos da pré-temporada da IA: os amistosos dos clubes, o torneio acrescentado e o convite aberto do clube humano (friendly.cpp) ----------

unsafe fn cqb4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb4_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqb4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqb4_c(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqb4_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O registro de clube (0x245 bytes) do id `id`.
unsafe fn cqb4_clube(id: i32) -> usize {
    cqb4_d(0xae23b4).wrapping_add((id as usize).wrapping_mul(0x245))
}

/// A data de 8 bytes em `a` está antes de hoje ([0xae2c92] o ano, sem sinal; [0xae2c90] o dia, com sinal).
unsafe fn cqb4_antes_de_hoje(a: usize) -> bool {
    let (ano, hoje) = (cqb4_w(0xae2c92) as u16, cqb4_w(0xae2c90));
    let y = cqb4_w(a + 2) as u16;
    y < ano || (y == ano && cqb4_w(a) < hoje)
}

/// A data de 8 bytes em `a` está depois de hoje.
unsafe fn cqb4_depois_de_hoje(a: usize) -> bool {
    let (ano, hoje) = (cqb4_w(0xae2c92) as u16, cqb4_w(0xae2c90));
    let y = cqb4_w(a + 2) as u16;
    y > ano || (y == ano && cqb4_w(a) > hoje)
}

/// { 0x5c4770 } (thiscall, ret 0xc, friendly.cpp): `Friendly::plan_club_friendlies(entradas, ordenados, conta)`, pelo
/// plano da pré-temporada da IA (0x5c27a0). Libera o plano antigo (+0x34d8; free 0x9452ca, cdecl), aloca o novo (malloc
/// 0x944e46 de 2000 bytes, cdecl; sem memória a caixa da linha 0x6f1), capacidade +0x34e4 = 100, conta +0x34e0 e
/// +0x34dc zerados. Para cada uma das 35 ligas ativas (0xb63d60, 0x48 bytes) de nação com +0x11c, cuja temporada (+0x2a)
/// não começou e cuja data +0x39 menos 0x34 (0x54cab0: lea ecx, [liga + 0x39]; 2 pushes; sem add esp -> thiscall) já
/// chegou: os candidatos da nação (bsearch 0x946464: 5 pushes; add esp, 0x14 -> cdecl, pela 0x5c5290; recua até o
/// primeiro da nação; o último da lista nunca entra) até o primeiro de outra nação. Cada clube que não é humano
/// (0x600a50: 5 pushes; add esp, 0x14 -> cdecl(clube, 1, 1, 0, 0)), não é o de +0x1db = [0x9cf6f8] e tem liga (+0x57)
/// registrada em [0xadadfc]: faixa = reputação / 500 + (sorteio(3) + 1, 3 vira 2 com sorteio(4) zero); até sorteio(3) +
/// 4 amistosos (pela conta +4 do candidato): o adversário (0x5c4cb0: 5 pushes; ecx = this -> thiscall) e, achado, o
/// amistoso em data livre (0x5c4380: 3 pushes; ecx = this -> thiscall) com o de faixa (+5) maior como anfitrião e
/// mandante; sem adversário ou com o amistoso marcado, a faixa × 0,8 (no mínimo 1). No fim, o qsort do plano pela data
/// (0x945333: 4 pushes; add esp, 0x10 -> cdecl, pela 0x778e90).
unsafe extern "thiscall" fn amistosos_dos_clubes(this: usize, entradas: usize, ordenados: usize, conta: u32) {
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    if cqb4_d(this + 0x34d8) != 0 {
        free(cqb4_d(this + 0x34d8));
    }
    *((this + 0x34e4) as *mut i32) = 100;
    *((this + 0x34e0) as *mut i32) = 0;
    *((this + 0x34dc) as *mut i32) = 0;
    let p = malloc(0x7d0);
    *((this + 0x34d8) as *mut usize) = p;
    if p == 0 {
        aviso_em(0x9cd0a0, 0x6f1);
        return;
    }
    let menos: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54cab0usize);
    let busca: unsafe extern "C" fn(*const usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let adversario: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x5c4cb0usize);
    let planejar: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x5c4380usize);
    for k in 0..0x23usize {
        let liga = 0xb63d60 + k * 0x48;
        let nacao = cqb4_d(liga);
        if nacao == 0 || *((nacao + 0x11c) as *const u8) == 0 {
            continue;
        }
        if cqb4_antes_de_hoje(liga + 0x2a) {
            continue;
        }
        let mut d = [0u8; 8];
        let r = menos(liga + 0x39, &mut d, 0x34);
        if cqb4_depois_de_hoje(r) {
            continue;
        }
        // a chave da busca: um candidato de 0x16 bytes com a nação (+6) e a faixa 0xff (+5)
        let mut chave = [0u8; 0x16];
        chave[5] = 0xff;
        chave[6..10].copy_from_slice(&cqb4_i(nacao).to_le_bytes());
        let pchave = chave.as_ptr() as usize;
        let mut q = busca(&pchave, ordenados, conta, 4, 0x5c5290);
        if q == 0 {
            continue;
        }
        let id = cqb4_i(cqb4_d(liga));
        while q > ordenados {
            if cqb4_i(cqb4_d(q - 4) + 6) != id {
                break;
            }
            q -= 4;
        }
        let ultimo = ordenados.wrapping_add((conta as usize).wrapping_mul(4)).wrapping_sub(4);
        while q < ultimo {
            let cand = cqb4_d(q);
            let clube = cqb4_clube(cqb4_i(cand));
            if cqb4_d(clube + 0x53) != cqb4_d(liga) {
                break;
            }
            q += 4;
            if humano(clube, 1, 1, 0, 0) != 0 {
                continue;
            }
            if cqb4_i(clube + 0x1db) == cqb4_i(0x9cf6f8) {
                continue;
            }
            let lg = cqb4_d(clube + 0x57);
            if lg == 0 || cqb4_d(cqb4_d(0xadadfc) + (cqb4_i(lg) as usize).wrapping_mul(4)) == 0 {
                continue;
            }
            crate::comparacao::ramo("cqb_4 clube planejado");
            let mut b = (cqb4_sortear(3) as u8).wrapping_add(1); // cqb4: o acréscimo da faixa
            if b == 3 && cqb4_sortear(4) == 0 {
                b = 2;
            }
            let mut faixa = ((cqb4_w(clube + 0x80) as i32 / 500) as u8).wrapping_add(b);
            let quantos = (cqb4_sortear(3) as u8).wrapping_add(4) as i8; // cqb4: quantos amistosos
            let mut j = 0i8;
            while cqb4_c(cand + 4) < quantos && j < quantos {
                // ponytail: a original empurra o dword do local da faixa (os bytes altos são sobra); a 0x5c4cb0 só lê o byte
                let opp = adversario(this, cand, entradas, conta, faixa as u32, 0);
                let marcado = if opp != 0 {
                    crate::comparacao::ramo("cqb_4 adversario achado");
                    if cqb4_c(cand + 5) > cqb4_c(opp + 5) { // cqb4: o de faixa maior em casa
                        planejar(this, cand, opp, cand)
                    } else {
                        planejar(this, cand, cand, opp)
                    }
                } else {
                    1
                };
                if marcado != 0 {
                    let f = ftol(faixa as i8 as i32 as f64 * f64_em(0x9689b0)) as u8;
                    faixa = if (f as i8) < 1 { 1 } else { f };
                }
                j = j.wrapping_add(1);
            }
        }
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cqb4_d(this + 0x34d8), cqb4_i(this + 0x34e0), 0x14, 0x778e90);
}

/// Comparador da 0x5c4770: só a original (ponytail: ela libera e aloca o plano, e a 0x5c4380 aloca as listas de datas e
/// de adversários dos candidatos; não roda duas vezes), conferindo o plano que ela deixa: o plano alocado, a capacidade
/// (+0x34e4) não menor que a conta (+0x34e0), o +0x34dc zerado e o plano em ordem de data (a 0x778e90 entre vizinhos).
unsafe extern "thiscall" fn comparar_amistosos_dos_clubes(this: usize, entradas: usize, ordenados: usize, conta: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1467)(this, entradas, ordenados, conta);
    let (p, n, cap, z) = (cqb4_d(this + 0x34d8), cqb4_i(this + 0x34e0), cqb4_i(this + 0x34e4), cqb4_i(this + 0x34dc));
    if p == 0 {
        return; // sem memória: a caixa
    }
    let ordem: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x778e90usize);
    let ordenado = (1..n.max(0) as usize).all(|i| ordem(p + (i - 1) * 0x14, p + i * 0x14) <= 0);
    PLACAR.registrar(n >= 0 && cap >= n && z == 0 && ordenado, || format!("amistosos_dos_clubes: plano {:#x} conta {} capacidade {} +0x34dc {} em ordem {}", p, n, cap, z, ordenado));
}

/// O torneio livre: o primeiro dos 127 (0x69 bytes a partir de +0xb1) sem anfitrião (+0), ou 0.
unsafe fn cqb4_torneio_livre(this: usize) -> usize {
    (0..0x7fusize).map(|i| this + 0x69 * i + 0xb1).find(|&t| cqb4_d(t) == 0).unwrap_or(0)
}

/// { 0x5c6e00 } (thiscall, ret 0x28, friendly.cpp): `Friendly::add_tournament(anfitrião, tipo, início, a, b, c, d, e,
/// convidados, nome)`, pelo torneio da IA (0x5c3be0) e pelas telas do clube (0x4701f0). O primeiro torneio livre (sem
/// livre: 0) ganha o anfitrião (+0), o tipo (+0xc), os bytes a e b (+0xd, +0xe), a data de início (+4, pela 0x417c10:
/// lea ecx, [t + 4]; 2 pushes; sem add esp -> thiscall), as palavras c, d, e (+0xf, +0x11, +0x13), +0x29, +0x2d e
/// +0x65 zerados, o nome (+0x31, com o zero) e os 4 convidados (+0x15, dwords) com as marcas +0x25..+0x28 em 1.
unsafe extern "thiscall" fn acrescentar_torneio(this: usize, anfitriao: usize, tipo: u32, inicio: usize, a: u32, b: u32, c: u32, d: u32, e: u32, convidados: usize, nome: usize) -> usize {
    let t = cqb4_torneio_livre(this);
    if t == 0 {
        return 0;
    }
    *((t + 0xc) as *mut u8) = tipo as u8;
    *((t + 0xd) as *mut u8) = a as u8;
    core::ptr::write_unaligned(t as *mut usize, anfitriao);
    *((t + 0xe) as *mut u8) = b as u8;
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut tmp = [0u8; 8];
    copiar(t + 4, &mut tmp, inicio);
    core::ptr::write_unaligned((t + 0x11) as *mut u16, d as u16);
    core::ptr::write_unaligned((t + 0x13) as *mut u16, e as u16);
    core::ptr::write_unaligned((t + 0x65) as *mut u32, 0);
    core::ptr::write_unaligned((t + 0x29) as *mut u32, 0);
    core::ptr::write_unaligned((t + 0x2d) as *mut u32, 0);
    core::ptr::write_unaligned((t + 0xf) as *mut u16, c as u16);
    let mut k = 0usize;
    loop {
        let x = *((nome + k) as *const u8);
        *((t + 0x31 + k) as *mut u8) = x;
        if x == 0 {
            break;
        }
        k += 1;
    }
    crate::comparacao::ramo("cqb_4 torneio acrescentado");
    for k in 0..4usize {
        core::ptr::write_unaligned((t + 0x15 + 4 * k) as *mut u32, cqb4_d(convidados + 4 * k) as u32);
        *((t + 0x25 + k) as *mut u8) = 1;
    }
    t
}

/// Comparador da 0x5c6e00: lado a lado no torneio livre (0x69 bytes, mais o que um nome comprido passar) e a devolução.
unsafe extern "thiscall" fn comparar_acrescentar_torneio(this: usize, anfitriao: usize, tipo: u32, inicio: usize, a: u32, b: u32, c: u32, d: u32, e: u32, convidados: usize, nome: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32, u32, u32, usize, usize) -> usize>(1468);
    let t = cqb4_torneio_livre(this);
    let mut tam = 0usize;
    while nome != 0 && *((nome + tam) as *const u8) != 0 {
        tam += 1;
    }
    let reg = if t != 0 { vec![(t, 0x69usize.max(0x31 + tam + 1))] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, anfitriao, tipo, inicio, a, b, c, d, e, convidados, nome), || acrescentar_torneio(this, anfitriao, tipo, inicio, a, b, c, d, e, convidados, nome));
    PLACAR.registrar(igual && r == n, || format!("acrescentar_torneio(anfitriao {:x}) original={:#x} {:x?} rust={:#x} {:x?}", anfitriao, r, mo, n, mr));
    r
}

/// { 0x5c68b0 } (thiscall, ret 0, friendly.cpp): `Friendly::find_human_club_open_invitation()`, pelas telas dos
/// amistosos. Com a lista de amistosos (+0x34c8) e o clube (+0x39) do técnico humano da vez ([0xb74c26 + 0x300 ×
/// [0xb77c7a]]): o cursor da lista (+0xa) volta ao começo (+2) e o primeiro amistoso desse clube (+0xc) ainda sem
/// data (+8 ≠ 0) pelo próximo da lista (0x68d1b0: ecx = a lista; sem push -> thiscall); senão 0.
unsafe extern "thiscall" fn convite_aberto_do_clube_humano(this: usize) -> usize {
    let lista = cqb4_d(this + 0x34c8);
    if lista == 0 {
        return 0;
    }
    let tecnico = cqb4_d(0xb74c26usize.wrapping_add((*(0xb77c7a as *const i16) as i32 as usize).wrapping_mul(0x300)));
    if tecnico == 0 {
        return 0;
    }
    let clube = cqb4_d(tecnico + 0x39);
    if clube == 0 {
        return 0;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, cqb4_d(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    loop {
        let m = proximo(cqb4_d(this + 0x34c8));
        if m == 0 {
            return 0;
        }
        if cqb4_d(m + 0xc) == clube && cqb4_d(m + 8) != 0 {
            return m;
        }
    }
}

/// Comparador da 0x5c68b0: lado a lado no cursor da lista (+0xa) e a devolução.
unsafe extern "thiscall" fn comparar_convite_aberto_do_clube_humano(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(1469);
    let lista = cqb4_d(this + 0x34c8);
    let reg = if lista != 0 { vec![(lista + 0xa, 4)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || convite_aberto_do_clube_humano(this));
    PLACAR.registrar(igual && r == n, || format!("convite_aberto_do_clube_humano original={:#x} {:x?} rust={:#x} {:x?}", r, mo, n, mr));
    r
}

// ---------- 264a leva: a lesão nova (o registro, a duração pelo fisioterapeuta, a recaída e a aposentadoria), a notícia da lesão e a ausência sem licença (injury.cpp, awol.cpp) ----------

unsafe fn cqb5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqb5_c(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cqb5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqb5_por_w(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cqb5_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O registro físico de 31 bytes da pessoa `id` em [this].
unsafe fn cqb5_registro(this: usize, id: usize) -> usize {
    cqb5_d(this).wrapping_add(id.wrapping_mul(0x1f))
}

/// A lesão `i` (0x30 bytes) do vetor [this+4].
unsafe fn cqb5_lesao(this: usize, i: usize) -> usize {
    cqb5_d(this + 4).wrapping_add(i.wrapping_mul(0x30))
}

/// O contrato da pessoa (0x4cf5a0: thiscall([0xae1050], pessoa), ret 4), guardado em [0xae1080].
unsafe fn cqb5_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cqb5_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato `c` (+4 = id no registro de 0x245 bytes em [0xae23b4]; 0 sem contrato).
unsafe fn cqb5_clube_do_contrato(c: usize) -> usize {
    if c == 0 { 0 } else { cqb5_d(0xae23b4).wrapping_add(cqb5_d(c + 4).wrapping_mul(0x245)) }
}

/// A tabela dos tipos de lesão (13 bytes por tipo): +0 (0x9e7adb) dias de base (i16), +2 (0x9e7add) dias variáveis
/// (i16), +4 (0x9e7adf) percentual da volta ao treino (i8), +5 (0x9e7ae0) pode voltar (i32).
fn cqb5_tipo(tipo: i8) -> usize {
    (tipo as i32 * 13) as usize
}

/// Os dias da volta ao treino (+0x2a) pela duração `dias`: ftol(percentual × 0,01 × dias) (fild, fmul qword
/// [0x967898], fimul, _ftol 0x945640; x87 em dupla precisão), no máximo 60 dias antes do fim.
unsafe fn cqb5_volta_ao_treino(tipo: i8, dias: i16) -> i16 {
    let pct = cqb5_c(0x9e7adf + cqb5_tipo(tipo));
    let x = ftol(pct as f64 * f64_em(0x967898) * dias as f64) as i16;
    if (x as i32) + 0x3c < dias as i32 { dias.wrapping_sub(0x3c) } else { x }
}

/// { 0x62d250 } (thiscall, ret 0x20, injury.cpp, quadro de 0x220 bytes sem SEH): `Injury::add_injury(tipo, retroativa,
/// pessoa, partida, surto, arg da notícia, recorrência, só_recorrência)`, pelo dia das lesões (0x62b7b0), pelos efeitos
/// físicos da partida (0x62c080), pelos registros iniciais (0x62cf70), pela recaída (0x62e500), pelos surtos de doença
/// (0x62e9e0) e por duas originais. Pessoa nula, sem ficha (+0x61) ou tipo fora de 0..0x51: a original (caixa e −1); id
/// além de [0xae2364]: −1. O físico do dia (0x62c680) e o treino ([0xde1f44], 0x8b1750 com 0). Só recorrência: com
/// recorrência e o tipo que pode voltar, a marca de recaída do registro (+0x12 tipo, +0x13: 1 se sorteio(rec / 10) e
/// sorteio(rec) derem zero com rec negativo; rec se positivo); −1. Senão, com lesão (+0xe ≠ −1): só os tipos 0x4d, 0x4e,
/// 0x4f e 0x51 a trocam (os dias restantes +0x2c ficam, a velha vai ao histórico 0x58e340 e a vaga fica livre). +0x1d
/// = tipo; a primeira vaga livre (byte 0xff) do vetor [this+4] (conta [this+8]) ou mais 100 vagas (realloc; sem
/// memória, a caixa da linha 0x909 e −1). O tipo na vaga; a partida (pelo clube da casa +0x1c ou de fora +0x20 que tem
/// a pessoa nos 50 do elenco +0xd7) grava o registro da partida em +1 (0x7e4750), senão +0x1a = 0xff. O melhor
/// fisioterapeuta do clube (0x544410 dos três +0x1cf com +0x69). Dias = base + variáveis × (35 − melhor) / 20 (menos
/// sorteio deles se sorteio(melhor + 5) > sorteio(20); 0 no tipo 0x51) + os restantes da velha (+ sorteios pela idade
/// e pela propensão +0x2d da ficha, com base acima de 10). O +0xa do registro de 79 bytes sobe 1 a cada sorteio(5 ×
/// natural + 12) zero, por dia, até 100. Lesão de agora (retroativa 0): de 100 dias, fora do clube humano e fora da
/// lista, a marca 100 do clube na lista de observação; dias ≥ sorteio(50) + 30, os clubes que miram o jogador o
/// suspendem (0x842b00). +0x28 = +0x2c = dias, +0x2a = +0x2e = a volta ao treino; +0x20 = a data (hoje, ou hoje −
/// sorteio(dias) − 1 na retroativa). O físico de novo, a notícia (0x62dca0), +0xe = a vaga, a recorrência, a seleção
/// (0x775cd0), +0x1c = 0, a recaída (0x62e500), a contagem das recaídas (de duas: aposentadoria 0x7a7f20 por sorteio),
/// a aposentadoria pela idade e pela volta ao treino longa e o surto (0x62e820) nos tipos 0, 1 e 2. Devolve a vaga.
unsafe extern "thiscall" fn lesao_nova(this: usize, tipo_arg: u32, retroativa: u32, p: usize, jogo: usize, surto: u32, noticia: u32, rec_arg: u32, so_recorrencia: u32) -> i32 {
    let tipo = tipo_arg as u8 as i8;
    let original = orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize, u32, u32, u32, u32) -> i32>(1470);
    if p == 0 {
        return original(this, tipo_arg, retroativa, p, jogo, surto, noticia, rec_arg, so_recorrencia); // a caixa: só a original
    }
    if cqb5_d(p) as i32 >= cqb5_d(0xae2364) as i32 {
        return -1;
    }
    if cqb5_d(p + 0x61) == 0 || !(0..0x52).contains(&tipo) {
        return original(this, tipo_arg, retroativa, p, jogo, surto, noticia, rec_arg, so_recorrencia); // as caixas: só a original
    }
    let fisico: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x62c680usize);
    fisico(this, p);
    if cqb5_d(0xde1f44) != 0 {
        let treino: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8b1750usize);
        treino(cqb5_d(0xde1f44), p, 0);
    }
    let t = cqb5_tipo(tipo);
    let rec = rec_arg as u8 as i8;
    if so_recorrencia != 0 {
        if rec == 0 || cqb5_d(0x9e7ae0 + t) == 0 {
            return -1;
        }
        if rec < 0 {
            if cqb5_sorteio(rec as i32 / 10) == 0 {
                let r = cqb5_registro(this, cqb5_d(p));
                *((r + 0x12) as *mut u8) = tipo as u8;
                *((r + 0x13) as *mut u8) = 1;
            }
            return -1;
        }
        let r = cqb5_registro(this, cqb5_d(p));
        *((r + 0x12) as *mut u8) = tipo as u8;
        *((r + 0x13) as *mut u8) = rec as u8;
        return -1;
    }
    let antiga = cqb5_d(cqb5_registro(this, cqb5_d(p)) + 0xe) as i32;
    let mut restantes: u16 = 0; // a palavra baixa do local (a alta é 0)
    if antiga != -1 {
        if !(tipo == 0x4d || tipo == 0x4e || tipo == 0x4f || tipo == 0x51) {
            return -1;
        }
        crate::comparacao::ramo("cqb_5 troca a lesao");
        let r = cqb5_registro(this, cqb5_d(p));
        let l = cqb5_lesao(this, antiga as usize);
        restantes = cqb5_w(l + 0x2c) as u16;
        let historico: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize = core::mem::transmute(0x58e340usize);
        historico(cqb5_d(this + 0xc), r + 0x14, r + 0x18, l);
        *(l as *mut u8) = 0xff;
        core::ptr::write_unaligned((cqb5_registro(this, cqb5_d(p)) + 0xe) as *mut i32, -1);
    }
    *((cqb5_registro(this, cqb5_d(p)) + 0x1d) as *mut u8) = tipo as u8;
    // a vaga
    let mut vaga: i32 = -1;
    let vetor = cqb5_d(this + 4);
    if vetor != 0 {
        let n = cqb5_d(this + 8) as i32;
        let mut i = 0;
        while i < n {
            if cqb5_b(vetor + i as usize * 0x30) == 0xff {
                vaga = i;
                break;
            }
            i += 1;
        }
    }
    if vaga == -1 {
        let novo = (cqb5_d(this + 8) as i32).wrapping_add(100);
        let tam = (novo as usize).wrapping_mul(0x30);
        let v = if vetor != 0 {
            let realocar: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945667usize);
            realocar(vetor, tam)
        } else {
            let alocar: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944e46usize);
            alocar(tam)
        };
        *((this + 4) as *mut usize) = v;
        if v == 0 {
            aviso_em(0x9e7f28, 0x909); // injury.cpp
            return -1;
        }
        let velho = cqb5_d(this + 8) as i32;
        let mut i = velho;
        while i < novo {
            *((cqb5_d(this + 4) + i as usize * 0x30) as *mut u8) = 0xff;
            i += 1;
        }
        *((this + 8) as *mut i32) = novo;
        vaga = velho;
    }
    let off = vaga as usize * 0x30;
    *((cqb5_d(this + 4) + off) as *mut u8) = tipo as u8;
    let mut da_partida = false;
    if jogo != 0 {
        for k in 0..50usize {
            let mut clube = cqb5_d(jogo + 0x1c);
            let achou = if cqb5_d(clube + 0xd7 + 4 * k) == p {
                true
            } else if cqb5_d(cqb5_d(jogo + 0x20) + 0xd7 + 4 * k) == p {
                clube = cqb5_d(jogo + 0x20);
                true
            } else {
                false
            };
            if achou {
                if clube != 0 {
                    let registrar: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7e4750usize);
                    registrar(jogo, clube, cqb5_d(this + 4) + off + 1);
                    da_partida = true;
                }
                break;
            }
        }
    }
    if !da_partida {
        *((cqb5_d(this + 4) + off + 0x1a) as *mut u8) = 0xff;
    }
    // o melhor fisioterapeuta
    let mut melhor: i8 = 0;
    if cqb5_d(p + 0x39) != 0 {
        let conhecer: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
        for k in 0..3usize {
            let f = cqb5_d(cqb5_d(p + 0x39) + 0x1cf + 4 * k);
            if f == 0 {
                continue;
            }
            let np = cqb5_d(f + 0x69);
            if np == 0 {
                continue;
            }
            if conhecer(np, cqb5_b(np + 0x1e) as i8) as i8 > melhor {
                let np = cqb5_d(cqb5_d(cqb5_d(p + 0x39) + 0x1cf + 4 * k) + 0x69);
                melhor = conhecer(np, cqb5_b(np + 0x1e) as i8) as i8;
            }
        }
    }
    // os dias
    crate::comparacao::ramo("cqb_5 duracao");
    let mut extra = (cqb5_w(0x9e7add + t) as i32).wrapping_mul(0x23 - melhor as i32) / 20; // cqb5: o fisioterapeuta encurta
    let s1 = cqb5_sorteio(melhor as i32 + 5);
    let s2 = cqb5_sorteio(0x14);
    if s1 > s2 {
        extra = extra.wrapping_sub(cqb5_sorteio(extra as i16 as i32));
    }
    if tipo == 0x51 {
        extra = 0;
    }
    extra = extra.wrapping_add(restantes as i32);
    let idade = cqb5_b(p + 0x18) as i8;
    if tipo != 0x51 && cqb5_w(0x9e7adb + t) > 10 {
        if idade > 0x1e {
            let ax = (cqb5_c(cqb5_d(p + 0x61) + 0x2d) as i16).wrapping_add(idade as i16).wrapping_sub(0x23);
            if ax > 0 {
                extra = extra.wrapping_add(cqb5_sorteio(ax as i32));
            }
        }
        let al = cqb5_b(cqb5_d(p + 0x61) + 0x2d) as i8;
        if al > 10 {
            extra = extra.wrapping_add(cqb5_sorteio((al as i16 - 10) as i32));
        }
    }
    let dias = cqb5_w(0x9e7adb + t).wrapping_add(extra as i16);
    let n = dias as i32;
    let mut i = 0;
    while i < n {
        let g = cqb5_d(0xae2d0c).wrapping_add(cqb5_d(p).wrapping_mul(0x4f)) + 0xa;
        if cqb5_b(g) >= 100 {
            break;
        }
        if cqb5_sorteio(cqb5_c(cqb5_d(p + 0x61) + 0x34) * 5 + 0xc) == 0 {
            let g = cqb5_d(0xae2d0c).wrapping_add(cqb5_d(p).wrapping_mul(0x4f)) + 0xa;
            *(g as *mut u8) = cqb5_b(g).wrapping_add(1);
        }
        i += 1;
    }
    if retroativa == 0 {
        let clube = cqb5_d(p + 0x39);
        if clube != 0 {
            let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
            if humano(clube, 0, 1, 0, 0) == 0 && dias >= 100 {
                let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
                if listada(p, 0) == 0 && cqb5_d(0xde1f54) != 0 {
                    let tm = cqb5_d(0xde1f54);
                    let m = cqb5_d(cqb5_d(tm + 0x856) + 0xdc).wrapping_add(cqb5_d(cqb5_d(p + 0x39)).wrapping_mul(9)) + 8;
                    *(m as *mut u8) = 100;
                }
            }
        }
        let s = cqb5_sorteio(0x32);
        if n >= s + 0x1e && cqb5_d(0xde1f54) != 0 && cqb5_d(cqb5_d(0xde1f54) + 0x856) != 0 {
            crate::comparacao::ramo("cqb_5 suspende as miras");
            let suspender: unsafe extern "thiscall" fn(usize, usize, u8) = core::mem::transmute(0x842b00usize);
            suspender(cqb5_d(cqb5_d(0xde1f54) + 0x856), p, 0); // cqb5: os clubes que o miram suspendem
        }
    }
    cqb5_por_w(cqb5_d(this + 4) + off + 0x28, dias);
    cqb5_por_w(cqb5_d(this + 4) + off + 0x2c, dias);
    crate::comparacao::ramo("cqb_5 volta ao treino");
    let volta = cqb5_volta_ao_treino(tipo, dias); // cqb5: a volta ao treino pelo percentual
    cqb5_por_w(cqb5_d(this + 4) + off + 0x2a, volta);
    cqb5_por_w(cqb5_d(this + 4) + off + 0x2e, cqb5_w(cqb5_d(this + 4) + off + 0x2a));
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut saida = [0u8; 8];
    if retroativa != 0 {
        let s = cqb5_sorteio(n);
        let menos: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c8e0usize);
        let mut d = [0u8; 8];
        let r = menos(0xae2c90, d.as_mut_ptr(), (s + 1) as u32);
        copiar(cqb5_d(this + 4) + off + 0x20, saida.as_mut_ptr(), r);
    } else {
        copiar(cqb5_d(this + 4) + off + 0x20, saida.as_mut_ptr(), 0xae2c90);
    }
    fisico(this, p);
    let noticia_fn: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x62dca0usize);
    noticia_fn(this, p, cqb5_d(this + 4) + off, noticia);
    core::ptr::write_unaligned((cqb5_registro(this, cqb5_d(p)) + 0xe) as *mut i32, vaga);
    if rec != 0 && cqb5_d(0x9e7ae0 + t) != 0 {
        if rec < 0 {
            if cqb5_sorteio(rec as i32 / 10) == 0 {
                *((cqb5_registro(this, cqb5_d(p)) + 0x12) as *mut u8) = tipo as u8;
                if cqb5_sorteio(rec as i32) == 0 {
                    *((cqb5_registro(this, cqb5_d(p)) + 0x13) as *mut u8) = 1;
                }
            }
        } else {
            *((cqb5_registro(this, cqb5_d(p)) + 0x12) as *mut u8) = tipo as u8;
            *((cqb5_registro(this, cqb5_d(p)) + 0x13) as *mut u8) = rec as u8;
        }
    }
    if cqb5_d(p + 0x24) != 0 && cqb5_d(0xb74318) != 0 {
        let tirar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x775cd0usize);
        tirar(cqb5_d(0xb74318), p, cqb5_w(cqb5_d(this + 4) + off + 0x28) as u16 as u32);
    }
    *((cqb5_registro(this, cqb5_d(p)) + 0x1c) as *mut u8) = 0;
    let recaida: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x62e500usize);
    recaida(this, cqb5_registro(this, cqb5_d(p)), cqb5_d(this + 4) + off);
    let aposentar: unsafe extern "thiscall" fn(usize, usize, i16) = core::mem::transmute(0x7a7f20usize);
    let r = cqb5_registro(this, cqb5_d(p));
    if cqb5_b(r + 0x12) as i8 == tipo && rec >= 0 {
        *((r + 0x13) as *mut u8) = cqb5_b(r + 0x13).wrapping_add(1);
        let c = cqb5_b(cqb5_registro(this, cqb5_d(p)) + 0x13) as i8;
        if c > 1 {
            let mut e = 200 - cqb5_w(0x9e7adb + t) as i32;
            if e <= 5 {
                e = 5;
            }
            if rec > 0 {
                e /= rec as i32;
            }
            let s1 = cqb5_sorteio(c as i32);
            let s2 = cqb5_sorteio(e);
            if s1 > s2 && cqb5_d(0xb7434c) != 0 {
                crate::comparacao::ramo("cqb_5 aposenta pelas recaidas");
                aposentar(cqb5_d(0xb7434c), p, cqb5_w(cqb5_d(this + 4) + off + 0x2a));
            }
        }
    }
    // a aposentadoria pela idade e pela volta ao treino longa
    let volta = || cqb5_w(cqb5_d(this + 4) + off + 0x2a) as i32;
    let propensao = || cqb5_c(cqb5_d(p + 0x61) + 0x2d);
    let anos = || cqb5_c(p + 0x18);
    let encerra = (idade > 0x24 && volta() > 0x78 - propensao() - anos() && cqb5_sorteio(volta()) > 0x5a - anos())
        || (idade > 0x21 && volta() > 0x96 - propensao() - anos() && cqb5_sorteio(volta()) > 0x5a - anos())
        || (idade > 0x1f && volta() > 0x12c - anos() && cqb5_sorteio(volta()) > 0xc8 - anos())
        || (volta() > 0x1f4 && cqb5_sorteio(cqb5_c(cqb5_d(p + 0x61) + 0x34) + 5) == 0);
    if encerra && cqb5_d(0xb7434c) != 0 {
        crate::comparacao::ramo("cqb_5 aposenta pela lesao");
        aposentar(cqb5_d(0xb7434c), p, cqb5_w(cqb5_d(this + 4) + off + 0x2a));
    }
    if surto != 0 && (tipo == 1 || tipo == 0 || tipo == 2) {
        let clube = cqb5_d(p + 0x39);
        if clube != 0 {
            let surto_fn: unsafe extern "thiscall" fn(usize, usize, u32, i32, usize) = core::mem::transmute(0x62e820usize);
            surto_fn(this, 0xae2c90, tipo_arg, cqb5_d(clube) as i32, 0);
        }
    }
    vaga
}

/// Comparador da 0x62d250: só a original (aloca, entrega a notícia, agenda aposentadoria e chama originais que sorteiam
/// no meio, como o treino 0x8b1750 e a recaída 0x62e500, que pode lesionar de novo: não pode rodar duas vezes). Confere
/// o que a lesão garante depois: argumentos inválidos ou só recorrência dão −1; −1 de resto só com a pessoa ainda
/// lesionada (+0xe ≠ −1) ou sem vetor; vaga válida com a pessoa ainda nela (+0xe = vaga, sem recaída que a tenha
/// trocado): o tipo na vaga e em +0x1d, +0x1c = 0, +0x2c = +0x28, +0x2e = +0x2a e +0x2a = a volta ao treino pela
/// duração +0x28 (o x87 da cópia).
unsafe extern "thiscall" fn comparar_lesao_nova(this: usize, tipo_arg: u32, retroativa: u32, p: usize, jogo: usize, surto: u32, noticia: u32, rec_arg: u32, so_recorrencia: u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize, u32, u32, u32, u32) -> i32>(1470);
    let r = o(this, tipo_arg, retroativa, p, jogo, surto, noticia, rec_arg, so_recorrencia);
    let tipo = tipo_arg as u8 as i8;
    let valida = p != 0 && (cqb5_d(p) as i32) < cqb5_d(0xae2364) as i32 && cqb5_d(p + 0x61) != 0 && (0..0x52).contains(&tipo);
    let ok = if !valida || so_recorrencia != 0 {
        r == -1
    } else {
        let reg = cqb5_registro(this, cqb5_d(p));
        let atual = cqb5_d(reg + 0xe) as i32;
        if r == -1 {
            atual != -1 || cqb5_d(this + 4) == 0
        } else if r < 0 || atual != r {
            r >= 0 // trocada depois (a recaída lesionou de novo): nada a conferir
        } else {
            let l = cqb5_lesao(this, r as usize);
            cqb5_b(l) as i8 == tipo
                && cqb5_b(reg + 0x1d) as i8 == tipo
                && cqb5_b(reg + 0x1c) == 0
                && cqb5_w(l + 0x2c) == cqb5_w(l + 0x28)
                && cqb5_w(l + 0x2e) == cqb5_w(l + 0x2a)
                && cqb5_w(l + 0x2a) == cqb5_volta_ao_treino(tipo, cqb5_w(l + 0x28))
        }
    };
    PLACAR.registrar(ok, || format!("lesao_nova(tipo {:#x}, retroativa {}, pessoa {:#x}, recorrencia {}, so {}) = {}", tipo_arg, retroativa, p, rec_arg as u8 as i8, so_recorrencia, r));
    r
}

/// { 0x62dca0 } (o mapa a dá como stdcall: a original troca o ecx por [0xde1f54] logo na entrada; o único ponto de
/// chamada, na 0x62d250, põe a Injury em ecx: thiscall com o this sem uso; ret 0xc; quadro SEH sem destrutor):
/// `injury_post_injury_news(pessoa, lesão, arg)`. Os clubes interessados (0x8d1da0 em [0xde1f54] com 0x5a, até 30) e a
/// seleção (+0x24) se couber; a notícia 0x7d0 (local): 0..3 a pessoa (id e os +0x33 de +4/+8/+0xc, 0 se faltam), 4 =
/// o tipo, 5/6 = a data da lesão (+0x20 com sinal, +0x22 sem), 7 = os dias (+0x28), 8/9 = o nascimento (+0x10, +0x12),
/// 0xa..0xc = o registro da partida (+0x12, +0x16, +9) ou −1 sem partida (+0x1a = 0xff), 0xd = arg, 0xe = id do clube
/// ou −1, 0xf = 0x420e9; a partida na notícia (0x7e4de0 com o byte 0x10). Mais de 100 dias: com potencial (+0xd da
/// ficha) acima de 8000, a todos (0x786780); senão aos interessados (0x7861e0). Até 100: clube humano (0x7875e0), ao
/// clube; o clube do contrato (0x4cf5a0) se for outro; a seleção.
unsafe extern "thiscall" fn noticia_da_lesao(_this: usize, p: usize, l: usize, arg: u32) {
    crate::comparacao::ramo("cqb_5 noticia da lesao");
    let tm = cqb5_d(0xde1f54);
    let mut clubes = [0usize; 30];
    let mut n: i8 = 0;
    if tm != 0 {
        let interessados: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1da0usize);
        n = interessados(tm, p, 0x5a, clubes.as_mut_ptr(), 0x1e) as i8;
    }
    let sel = cqb5_d(p + 0x24);
    if sel != 0 && (0..0x1e).contains(&n) {
        clubes[n as usize] = sel;
        n += 1;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nw = local.as_mut_ptr() as usize;
    construir(nw, 0x7d0, 0);
    gravar(nw, 0, cqb5_d(p) as u32);
    for k in 1..4usize {
        let q = cqb5_d(p + 4 * k);
        gravar(nw, k as i8, if q != 0 { cqb5_d(q + 0x33) as u32 } else { 0 });
    }
    gravar(nw, 4, cqb5_c(l) as u32);
    gravar(nw, 5, cqb5_w(l + 0x20) as i32 as u32);
    gravar(nw, 6, cqb5_w(l + 0x22) as u16 as u32);
    gravar(nw, 7, cqb5_w(l + 0x28) as i32 as u32);
    gravar(nw, 8, cqb5_w(p + 0x10) as i32 as u32);
    gravar(nw, 9, cqb5_w(p + 0x12) as u16 as u32);
    if cqb5_b(l + 0x1a) != 0xff {
        gravar(nw, 0xa, cqb5_d(l + 0x12) as u32);
        gravar(nw, 0xb, cqb5_d(l + 0x16) as u32);
        gravar(nw, 0xc, cqb5_d(l + 9) as u32);
    } else {
        gravar(nw, 0xa, u32::MAX);
        gravar(nw, 0xb, u32::MAX);
        gravar(nw, 0xc, u32::MAX);
    }
    gravar(nw, 0xd, arg);
    let clube = cqb5_d(p + 0x39);
    gravar(nw, 0xe, if clube != 0 { cqb5_d(clube) as u32 } else { u32::MAX });
    gravar(nw, 0xf, 0x420e9);
    let mut marca: u8 = 0x10;
    let partida_na_noticia: unsafe extern "C" fn(usize, usize, *mut u8) = core::mem::transmute(0x7e4de0usize);
    partida_na_noticia(l + 1, nw, &mut marca);
    let news = cqb5_d(0xb7431c);
    if cqb5_w(l + 0x28) > 100 {
        let f = cqb5_d(p + 0x61);
        if f != 0 && cqb5_w(f + 0xd) > 0x1f40 {
            let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
            todos(cqb5_d(0xb7431c), nw);
        } else {
            let mut i: i8 = 0;
            while i < n {
                entregar(cqb5_d(0xb7431c), nw, clubes[i as usize]);
                i += 1;
            }
        }
        return;
    }
    if cqb5_d(p + 0x39) == 0 {
        return;
    }
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    if humano(news, cqb5_d(p + 0x39)) == 0 {
        return;
    }
    entregar(cqb5_d(0xb7431c), nw, cqb5_d(p + 0x39));
    let c = cqb5_contrato(p);
    if c != 0 && cqb5_clube_do_contrato(c) != 0 {
        let x = cqb5_clube_do_contrato(cqb5_contrato(p));
        if x != cqb5_d(p + 0x39) {
            let y = cqb5_clube_do_contrato(cqb5_contrato(p));
            entregar(cqb5_d(0xb7431c), nw, y);
        }
    }
    let sel = cqb5_d(p + 0x24);
    if sel != 0 {
        entregar(cqb5_d(0xb7431c), nw, sel);
    }
}

/// Comparador da 0x62dca0: só a original (entrega a notícia, que não pode sair duas vezes; a cópia leva o ramo no modo
/// Rust). ponytail: sem plano a conferir além do que a original entrega.
unsafe extern "thiscall" fn comparar_noticia_da_lesao(this: usize, p: usize, l: usize, arg: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1471)(this, p, l, arg)
}

/// { 0x418f60 } (thiscall, ret 8, awol.cpp): `AWOL::is_player_awol(pessoa, só_ler)`, pela seleção, pelo treino, pelo
/// preço pedido, pelo dia da comissão e pelas transferências. Pessoa nula ou id além de [0xae2364]: 0 (só o al). Com
/// `só_ler`, o bit 0 da marca da pessoa ([this+0xc] + id). Senão, com o bit e sem contrato (0x4cf5a0, guardado em
/// [0xae1080]) ou sem clube do contrato, a ausência acaba (0x4196a0); devolve o bit relido.
unsafe extern "thiscall" fn ausente_sem_licenca(this: usize, p: usize, so_ler: u32) -> u32 {
    if p == 0 || cqb5_d(p) as i32 >= cqb5_d(0xae2364) as i32 {
        return 0;
    }
    let marca = || cqb5_b(cqb5_d(this + 0xc) + cqb5_d(p)) as u32 & 1;
    if so_ler == 0 && marca() != 0 {
        let c = cqb5_contrato(p);
        if c == 0 || cqb5_clube_do_contrato(c) == 0 {
            crate::comparacao::ramo("cqb_5 ausencia acaba sem clube");
            let acabar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4196a0usize);
            acabar(this, p);
        }
    }
    marca()
}

/// Comparador da 0x418f60 (quente: sem alocar): lado a lado em [0xae1080] e no al devolvido; com o bit, sem `só_ler` e
/// sem contrato ou clube (a 0x4196a0 tira e libera o nó da lista: não pode rodar duas vezes), só a original.
unsafe extern "thiscall" fn comparar_ausente_sem_licenca(this: usize, p: usize, so_ler: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1472);
    if p != 0 && (cqb5_d(p) as i32) < cqb5_d(0xae2364) as i32 && so_ler == 0 && cqb5_b(cqb5_d(this + 0xc) + cqb5_d(p)) & 1 != 0 {
        let c = cqb5_contrato(p); // a original o refaz igual (e regrava [0xae1080])
        if c == 0 || cqb5_clube_do_contrato(c) == 0 {
            return o(this, p, so_ler);
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(this, p, so_ler), || ausente_sem_licenca(this, p, so_ler));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("ausente_sem_licenca(pessoa {:#x}, so_ler {}) original={} {:x?} rust={} {:x?}", p, so_ler, r as u8, mo, n as u8, mr));
    r
}

// ---------- 264a leva: a parte dos clubes anteriores na transferência, o percentual da próxima venda pago e a notícia dele (transfer_manager.cpp) ----------

unsafe fn cqb6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqb6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube pelo id (registro de 0x245 bytes em [0xae23b4]).
unsafe fn cqb6_clube(id: usize) -> usize {
    cqb6_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa pelo id (registro de 0x6e bytes em [0xae23bc]).
unsafe fn cqb6_pessoa(id: usize) -> usize {
    cqb6_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato da pessoa (0x4cf5a0: thiscall(gerência [0xae1050], pessoa), ret 4), guardado em [0xae1080] como a
/// original faz depois de cada chamada.
unsafe fn cqb6_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cqb6_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato `c` (+4 = id; 0 sem contrato), sem olhar o tipo.
unsafe fn cqb6_clube_do_contrato(c: usize) -> usize {
    if c == 0 { 0 } else { cqb6_clube(cqb6_d(c + 4)) }
}

/// As finanças do clube (0x5a0590: thiscall([0xae2a38], clube), ret 4).
unsafe fn cqb6_financas(clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    f(cqb6_d(0xae2a38), clube)
}

/// O valor (i32 estendido para 64 bits, como o `cdq` da original) entra nas vendas (0x599800) ou sai pelas compras
/// (0x599890) das finanças `fin` (thiscall(fin, baixo, alto), ret 8).
unsafe fn cqb6_dinheiro(rotina: usize, fin: usize, v: i32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(rotina);
    f(fin, v as u32, (v >> 31) as u32);
}

/// A notícia local: construir (0x77bbe0: thiscall(notícia, tipo, 0), ret 8), gravar um dado (0x785790:
/// thiscall(notícia, k, v), ret 8) e entregar ao técnico do clube (0x7861e0: thiscall([0xb7431c], notícia, clube),
/// ret 8).
unsafe fn cqb6_construir(n: usize, tipo: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    f(n, tipo, 0);
}

unsafe fn cqb6_gravar(n: usize, k: i8, v: u32) {
    let f: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    f(n, k, v);
}

unsafe fn cqb6_entregar(n: usize, clube: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    f(cqb6_d(0xb7431c), n, clube);
}

/// O ensaio dos comparadores: a cópia roda sem as notícias e sem os totais do clube (0x448ad0), que alocam ou entregam
/// e não podem rodar duas vezes; CQB6_ENTREGAS conta as entregas que a original faria (elas podem mexer em [0xae1080]
/// pela resposta automática do humano de férias).
static mut CQB6_ENSAIO: bool = false;
static mut CQB6_ENTREGAS: u32 = 0;

/// { 0x8e4f20 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor; o this não é usado):
/// `TRANSFER_MANAGER::pay_former_club_fee_cuts(oferta)`, pela transferência efetivada com valor (+0x1b) de jogador sem
/// +0x69 (0x8c9390). Jogador (+0xc da oferta) com mais de 22 anos (+0x18): a notícia 0xfc3 (local; 0 = id, 1..3 = os
/// +0x33 de +4/+8/+0xc, 4 = o comprador +8, 7 = o clube do contrato ou −1); a soma (16 bits) dos pesos (byte +0x12)
/// das cinco vagas de clube anterior do registro de 79 bytes ([0xae2d0c], passo 5, clube em +0xe, −1 = vaga vazia;
/// soma 0 vira 1). Cada vaga com clube: ((valor / 20) / soma) × peso (peso abaixo de 1 vale 1) entra nas vendas das
/// finanças do COMPRADOR (0x5a0590 com +8, 0x599800; é o que a original faz); acima de 11 × reputação (+0x80) − 10000
/// do clube da vaga, a notícia com 5 = o valor e 6 = o id do clube vai ao técnico dele.
unsafe extern "thiscall" fn parte_dos_clubes_anteriores(_this: usize, o: usize) {
    let p = cqb6_pessoa(cqb6_d(o + 0xc));
    if (cqb6_b(p + 0x18) as i8) < 0x17 { // cqb6: só acima de 22 anos
        return;
    }
    crate::comparacao::ramo("cqb_6 parte dos clubes anteriores");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    if !CQB6_ENSAIO {
        cqb6_construir(n, 0xfc3);
    }
    let r = cqb6_d(0xae2d0c).wrapping_add(cqb6_d(p).wrapping_mul(0x4f));
    let mut soma: i16 = 0;
    for k in 0..5 {
        if cqb6_d(r + 0xe + 5 * k) as i32 != -1 {
            soma = soma.wrapping_add(cqb6_b(r + 0x12 + 5 * k) as i8 as i16);
        }
    }
    let vinte = (cqb6_d(o + 0x1b) as i32) / 20;
    let divisor = if soma == 0 { 1 } else { soma as i32 };
    if !CQB6_ENSAIO {
        cqb6_gravar(n, 0, cqb6_d(p) as u32);
        cqb6_gravar(n, 1, cqb6_d(cqb6_d(p + 4) + 0x33) as u32);
        cqb6_gravar(n, 2, cqb6_d(cqb6_d(p + 8) + 0x33) as u32);
        cqb6_gravar(n, 3, cqb6_d(cqb6_d(p + 0xc) + 0x33) as u32);
        cqb6_gravar(n, 4, cqb6_d(o + 8) as u32);
    }
    let c = cqb6_contrato(p);
    let v7 = if c == 0 || cqb6_b(c + 0x35) & 0x3f == 5 || cqb6_clube_do_contrato(c) == 0 {
        u32::MAX
    } else {
        let c2 = cqb6_contrato(p);
        // ponytail: a original lê [0] se o segundo contrato faltasse; inalcançável (o mesmo contrato acabou de passar)
        if c2 != 0 && cqb6_b(c2 + 0x35) & 0x3f != 5 { cqb6_d(cqb6_clube_do_contrato(c2)) as u32 } else { u32::MAX }
    };
    if !CQB6_ENSAIO {
        cqb6_gravar(n, 7, v7);
    }
    for k in 0..5 {
        let r = cqb6_d(0xae2d0c).wrapping_add(cqb6_d(p).wrapping_mul(0x4f)) + 5 * k;
        let i = cqb6_d(r + 0xe);
        if i as i32 == -1 {
            continue;
        }
        let peso = cqb6_b(r + 0x12) as i8;
        let peso = if peso < 1 { 1 } else { peso as i32 };
        let clube = cqb6_clube(i);
        let valor = vinte.wrapping_div(divisor).wrapping_mul(peso); // cqb6: a parte da vaga
        let fin = cqb6_financas(cqb6_clube(cqb6_d(o + 8)));
        crate::comparacao::ramo("cqb_6 parte paga");
        cqb6_dinheiro(0x599800, fin, valor); // cqb6: entra nas vendas do comprador
        if valor > (cqb6_w(clube + 0x80) as i32).wrapping_mul(11).wrapping_sub(10000) {
            CQB6_ENTREGAS += 1;
            if !CQB6_ENSAIO {
                cqb6_gravar(n, 5, valor as u32);
                cqb6_gravar(n, 6, cqb6_d(clube) as u32);
                cqb6_entregar(n, clube);
            }
        }
    }
}

/// Comparador da 0x8e4f20: a cópia em ensaio (sem a notícia) e a original a partir do mesmo estado, lado a lado nas
/// finanças do comprador (0x167 bytes) e em [0xae1080]; com entrega da notícia, só as finanças.
unsafe extern "thiscall" fn comparar_parte_dos_clubes_anteriores(this: usize, o: usize) {
    let original = orig::<unsafe extern "thiscall" fn(usize, usize)>(1473);
    let fin = cqb6_financas(cqb6_clube(cqb6_d(o + 8)));
    CQB6_ENTREGAS = 0;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(fin, 0x167), (0xae1080, 4)], || original(this, o), || {
        CQB6_ENSAIO = true;
        crate::aleatorio::sem_contar(|| parte_dos_clubes_anteriores(this, o));
        CQB6_ENSAIO = false;
    });
    let entregas = CQB6_ENTREGAS;
    let igual = igual || (entregas != 0 && mo.0[0] == mr.0[0]);
    PLACAR.registrar(igual, || format!("parte_dos_clubes_anteriores(oferta {:#x}) entregas={} original={:x?} rust={:x?}", o, entregas, mo, mr));
}

/// { 0x8cb0f0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::pay_next_sale_percentage(pessoa, oferta)`,
/// pela transferência efetivada com a marca 8 da pessoa (0x8c91e0). Com contrato (e clube do contrato), procura pela
/// busca binária (0x946464, entradas de 0xd bytes em [this+0x880], conta [this+0x888], comparação 0x8e54d0) a
/// cláusula com +1 = id do clube do contrato e +9 = id da pessoa. Achada: a base é o valor da oferta (+0x1b) mais o
/// valor (+0x52) de cada jogador da troca (+4, LLIST de ids, cursor recomeçado) quando a pessoa é a da oferta (+0xc);
/// senão o valor da pessoa para o clube do contrato (0x4d0c60 com 0, −1, 0). A parte = percentual (byte +0, com sinal)
/// × base / 100; de 1000 para cima: a notícia (0x8db3a0, com 1 se era a da oferta), os totais de transferência do
/// clube (0x448ad0 em [0xad9dac]: a parte, o clube do contrato, o clube da cláusula +5), a parte entra nas vendas do
/// clube da cláusula e sai pelas compras do clube do contrato. A cláusula fica (quem a tira é a 0x8cb4f0).
unsafe extern "thiscall" fn percentual_da_proxima_venda_pago(this: usize, p: usize, o: usize) -> u32 {
    let c = cqb6_contrato(p);
    if c == 0 || cqb6_clube_do_contrato(c) == 0 {
        return 0;
    }
    let mut chave = [0u8; 0xd];
    core::ptr::write_unaligned(chave.as_mut_ptr().add(9) as *mut usize, cqb6_d(p));
    let c2 = cqb6_contrato(p);
    // ponytail: a original lê [0] se o segundo contrato faltasse; inalcançável (o mesmo contrato acabou de passar)
    let id_clube = if c2 != 0 { cqb6_d(cqb6_clube_do_contrato(c2)) } else { usize::MAX };
    core::ptr::write_unaligned(chave.as_mut_ptr().add(1) as *mut usize, id_clube);
    let busca: unsafe extern "C" fn(*const u8, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let e = busca(chave.as_ptr(), cqb6_d(this + 0x880), cqb6_d(this + 0x888), 0xd, 0x8e54d0);
    if e == 0 {
        return 0;
    }
    let (base, da_oferta) = if cqb6_d(p) == cqb6_d(o + 0xc) {
        let mut v = cqb6_d(o + 0x1b) as i32;
        let l = cqb6_d(o + 4);
        if l != 0 {
            let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
            core::ptr::write_unaligned((l + 0xa) as *mut usize, cqb6_d(l + 2));
            let mut i = proximo(cqb6_d(o + 4));
            while i != 0 {
                v = v.wrapping_add(cqb6_d(cqb6_pessoa(cqb6_d(i)) + 0x52) as i32);
                i = proximo(cqb6_d(o + 4));
            }
        }
        (v, 1u32)
    } else {
        let clube = cqb6_clube_do_contrato(cqb6_contrato(p));
        let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
        (valor(cqb6_d(0xae1050), p, clube, 0, -1, 0), 0)
    };
    let parte = (cqb6_b(e) as i8 as i32).wrapping_mul(base) / 100;
    if parte < 1000 {
        return 0;
    }
    crate::comparacao::ramo("cqb_6 percentual pago");
    if !CQB6_ENSAIO {
        let noticia: unsafe extern "thiscall" fn(usize, usize, i32, usize, u32) = core::mem::transmute(0x8db3a0usize);
        noticia(this, e, parte, o, da_oferta);
    }
    let da_clausula = cqb6_clube(cqb6_d(e + 5));
    let fin_recebe = cqb6_financas(da_clausula);
    let fin_paga = cqb6_financas(cqb6_clube_do_contrato(cqb6_contrato(p)));
    let do_contrato = cqb6_clube_do_contrato(cqb6_contrato(p));
    if !CQB6_ENSAIO {
        let totais: unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32 = core::mem::transmute(0x448ad0usize);
        totais(cqb6_d(0xad9dac), parte, do_contrato, cqb6_clube(cqb6_d(e + 5)));
    }
    cqb6_dinheiro(0x599800, fin_recebe, parte);
    cqb6_dinheiro(0x599890, fin_paga, parte);
    1
}

/// O que a 0x8cb0f0 vai mexer, para as regiões do comparador: as finanças do clube da cláusula e do clube do
/// contrato, se a cláusula existe (sem gravar nada além de [0xae1080], que a original regrava).
unsafe fn cqb6_financas_da_clausula(this: usize, p: usize) -> Option<(usize, usize)> {
    let c = cqb6_contrato(p);
    if c == 0 || cqb6_clube_do_contrato(c) == 0 {
        return None;
    }
    let mut chave = [0u8; 0xd];
    core::ptr::write_unaligned(chave.as_mut_ptr().add(9) as *mut usize, cqb6_d(p));
    core::ptr::write_unaligned(chave.as_mut_ptr().add(1) as *mut usize, cqb6_d(cqb6_clube_do_contrato(c)));
    let busca: unsafe extern "C" fn(*const u8, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let e = busca(chave.as_ptr(), cqb6_d(this + 0x880), cqb6_d(this + 0x888), 0xd, 0x8e54d0);
    if e == 0 {
        return None;
    }
    Some((cqb6_financas(cqb6_clube(cqb6_d(e + 5))), cqb6_financas(cqb6_clube_do_contrato(c))))
}

/// Comparador da 0x8cb0f0: a cópia em ensaio (sem a notícia e sem os totais do clube) e a original a partir do mesmo
/// estado, lado a lado nas finanças dos dois clubes, em [0xae1080] e no sorteio (a 0x4d0c60 pode sortear); pago (a
/// notícia e os totais da original podem regravar [0xae1080]), só as finanças e a devolução.
unsafe extern "thiscall" fn comparar_percentual_da_proxima_venda_pago(this: usize, p: usize, o: usize) -> u32 {
    let original = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1474);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    if let Some((a, b)) = cqb6_financas_da_clausula(this, p) {
        regioes.push((a, 0x167));
        regioes.push((b, 0x167));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || original(this, p, o), || {
        CQB6_ENSAIO = true;
        let n = crate::aleatorio::sem_contar(|| percentual_da_proxima_venda_pago(this, p, o));
        CQB6_ENSAIO = false;
        n
    });
    let igual = r == n && (igual || (r != 0 && mo.0[3..] == mr.0[3..]));
    PLACAR.registrar(igual, || format!("percentual_da_proxima_venda_pago(pessoa {:#x}, oferta {:#x}) original={} {:x?} rust={} {:x?}", p, o, r, mo, n, mr));
    r
}

/// { 0x8db3a0 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH sem destrutor; o this não é usado):
/// `TRANSFER_MANAGER::post_sale_percentage_news(cláusula, parte, oferta, da_oferta)`, pelo percentual pago (0x8cb0f0).
/// A notícia 0xfa7 (local): 0..3 a pessoa da cláusula (+9: id e os +0x33 de +4/+8/+0xc), 4 = o percentual (byte +0),
/// 5 = o clube da cláusula (+5), 6 = o clube vendedor (+1), 7 = a parte, 8 = `da_oferta`, 9 = o comprador (+8 da
/// oferta) com `da_oferta`; senão o id do clube do contrato (0x4cf5a0; sem contrato ou sem clube: acaba sem entregar).
/// Vai aos técnicos do clube da cláusula e do vendedor (0x7861e0).
unsafe extern "thiscall" fn noticia_do_percentual_da_venda(_this: usize, e: usize, parte: i32, o: usize, da_oferta: u32) {
    crate::comparacao::ramo("cqb_6 noticia do percentual");
    let p = cqb6_pessoa(cqb6_d(e + 9));
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    cqb6_construir(n, 0xfa7);
    cqb6_gravar(n, 0, cqb6_d(p) as u32);
    cqb6_gravar(n, 1, cqb6_d(cqb6_d(p + 4) + 0x33) as u32);
    cqb6_gravar(n, 2, cqb6_d(cqb6_d(p + 8) + 0x33) as u32);
    cqb6_gravar(n, 3, cqb6_d(cqb6_d(p + 0xc) + 0x33) as u32);
    cqb6_gravar(n, 4, cqb6_b(e) as i8 as i32 as u32);
    cqb6_gravar(n, 5, cqb6_d(e + 5) as u32);
    cqb6_gravar(n, 6, cqb6_d(e + 1) as u32);
    cqb6_gravar(n, 7, parte as u32);
    cqb6_gravar(n, 8, da_oferta);
    let v9 = if da_oferta != 0 {
        cqb6_d(o + 8)
    } else {
        let c = cqb6_contrato(p);
        if c == 0 || cqb6_clube_do_contrato(c) == 0 {
            return;
        }
        let c2 = cqb6_contrato(p);
        // ponytail: a original lê [0] se o segundo contrato faltasse; inalcançável (o mesmo contrato acabou de passar)
        if c2 != 0 { cqb6_d(cqb6_clube_do_contrato(c2)) } else { usize::MAX }
    };
    cqb6_gravar(n, 9, v9 as u32);
    cqb6_entregar(n, cqb6_clube(cqb6_d(e + 5)));
    cqb6_entregar(n, cqb6_clube(cqb6_d(e + 1)));
}

/// Comparador da 0x8db3a0: só a original (a notícia é entregue e não pode sair duas vezes; a cópia leva o ramo no
/// modo Rust). ponytail: sem plano a conferir além do que a original entrega.
unsafe extern "thiscall" fn comparar_noticia_do_percentual_da_venda(this: usize, e: usize, parte: i32, o: usize, da_oferta: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, i32, usize, u32)>(1475)(this, e, parte, o, da_oferta)
}

// ---------- 264a leva: a permissão de trabalho concedida, o estatuto pelo motivo da recusa e a vaga da negociação (transfer_manager.cpp) ----------

unsafe fn cqb7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb7_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqb7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqb7_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O clube do contrato `c` (+4 = id no registro de 0x245 bytes em [0xae23b4]; 0 sem contrato), sem olhar o tipo.
unsafe fn cqb7_clube_do_contrato(c: usize) -> usize {
    if c == 0 { 0 } else { cqb7_d(0xae23b4).wrapping_add(cqb7_d(c + 4).wrapping_mul(0x245)) }
}

/// A data `a` vem depois da data em `d` (0x456740: thiscall(a, d.dword0, d.dword1), ret 8; a data vai por valor).
unsafe fn cqb7_depois(a: usize, d: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    f(a, cqb7_d(d) as u32, cqb7_d(d + 4) as u32) != 0
}

/// O nome (texto C em [pessoa+4]) contém `agulha` (0x945a60, strstr: cdecl(texto, agulha)) e o clube da oferta (+4 =
/// id; nome em +4 do clube) é o de 0xa8eb48 ("Falkirk"; a comparação de bytes que a original faz em linha).
unsafe fn cqb7_excecao(p: usize, agulha: usize, oferta: usize) -> bool {
    let nome = cqb7_d(p + 4);
    if nome == 0 {
        return false;
    }
    let strstr: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945a60usize);
    if strstr(nome, agulha) == 0 || oferta == 0 {
        return false;
    }
    let mut a = cqb7_d(0xae23b4).wrapping_add(cqb7_d(oferta + 4).wrapping_mul(0x245)) + 4;
    let mut b = 0xa8eb48usize;
    loop {
        let (x, y) = (cqb7_b(a), cqb7_b(b));
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        a += 1;
        b += 1;
    }
}

/// As marcas da pessoa na gerência (8 bytes em [this+0x86e] + id × 8): bit 0x20 em +0 (tem registro de permissão),
/// +3 o histórico de 16 jogos da seleção (u16, bit 0 o mais novo) e +5 a conta dos jogos da seleção já vistos (u16).
unsafe fn cqb7_marcas(this: usize, p: usize) -> usize {
    cqb7_d(this + 0x86e).wrapping_add(cqb7_d(p).wrapping_mul(8))
}

/// { 0x8c7950 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::is_work_permit_granted(pessoa, nação,
/// oferta, registro, sorteio)`, pela permissão na lista de observação (0x84b180, sem oferta, registro nem sorteio),
/// pela oferta de contrato aceita (0x8c38b0, com a oferta) e pelas permissões da oferta (0x8c7690). Contrato da pessoa
/// (0x4cf5a0, sem gravar [0xae1080]): de tipo 2 é o "empréstimo", outro é o "contrato". Com o registro de permissão
/// (o dado, ou achado pela busca binária 0x946464 em [this+0x93a], conta [this+0x93e], 14 bytes, quando a marca 0x20
/// existe): sem oferta, sai se jogos (+4) > 3 × recusas (+6) ou recusas < 4; com oferta, a mesma regra se o empréstimo
/// acaba depois da oferta (+0x2d) ou o clube do contrato não é o da oferta (+4), e sai se o empréstimo não acaba depois.
/// Não jogador (+0x69 sem ficha +0x61): reputação (+0xc) acima de 160. Em 2001, os dois casos do Collin e do Devon no
/// Falkirk. Potencial (+0xd da ficha) acima de 8500 (com sorteio: 8000 + sorteio(1000)); salário da oferta (+0xc) acima
/// de 34999; menos de 24 anos com habilidade (+7) acima de 190: sai. Menos de 5 jogos pela seleção (+0x22): não sai;
/// mais de 20 com reputação da nação (+0x8e) acima de 3000 e menos de 32 anos: sai. O histórico anda pelos jogos novos
/// da seleção ([[0xb74318]+4] + id da nação × 0x6e + 2; cada um empurra um bit zero) e a conta fica. A janela: 16
/// jogos, ou os do período de dois anos (0x58d710, até 16) a partir de 2003; com mais de 11 jogos marcados (sorteio:
/// 11 − sorteio(3) + sorteio(3)) ou mais de 70% (70 − sorteio(10) + sorteio(10)): sai.
unsafe extern "thiscall" fn permissao_concedida(this: usize, p: usize, _nacao: usize, oferta: usize, registro: usize, sorteio: u32) -> u32 {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cqb7_d(0xae1050), p);
    let (emprestimo, contrato) = if c != 0 && cqb7_b(c + 0x35) & 0x3f != 2 { (0, c) } else { (c, 0) };
    let mut reg = registro;
    let tem_registro = if reg != 0 {
        true
    } else if (p == 0 || (cqb7_d(p) as i32) < cqb7_d(0xae2364) as i32) && cqb7_b(cqb7_marcas(this, p)) & 0x20 != 0 {
        let chave = cqb7_d(p) as u32;
        let busca: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
        reg = busca(&chave, cqb7_d(this + 0x93a), cqb7_d(this + 0x93e), 0xe, 0x8e55c0);
        reg != 0
    } else {
        false
    };
    if tem_registro {
        let jogos = core::ptr::read_unaligned((reg + 4) as *const u16) as u32;
        let recusas = core::ptr::read_unaligned((reg + 6) as *const u16) as u32;
        let pela_conta = jogos > recusas * 3 || recusas < 4;
        if oferta == 0 {
            if pela_conta {
                return 1;
            }
        } else {
            let fora = (emprestimo != 0 && cqb7_depois(oferta + 0x2d, emprestimo + 0x2d))
                || (contrato != 0 && cqb7_d(oferta + 4) != cqb7_d(contrato + 4));
            if fora && pela_conta {
                return 1;
            }
            if emprestimo != 0 && !cqb7_depois(oferta + 0x2d, emprestimo + 0x2d) {
                return 1;
            }
        }
    }
    let np = cqb7_d(p + 0x69);
    if np != 0 && cqb7_d(p + 0x61) == 0 {
        return (cqb7_w(np + 0xc) > 0xa0) as u32;
    }
    if cqb7_w(0xae2c92) == 0x7d1 && (cqb7_excecao(p, 0x9b5ac0, oferta) || cqb7_w(0xae2c92) == 0x7d1 && cqb7_excecao(p, 0x9b5ab0, oferta)) {
        crate::comparacao::ramo("cqb_7 excecao de 2001");
        return 1;
    }
    let ficha = cqb7_d(p + 0x61);
    if sorteio != 0 {
        if ficha != 0 && cqb7_w(ficha + 0xd) as i32 > cqb7_sorteio(1000) + 8000 {
            return 1;
        }
    } else if ficha != 0 {
        crate::comparacao::ramo("cqb_7 confere o potencial");
        if cqb7_w(ficha + 0xd) > 0x2134 { // cqb7: o potencial acima de 8500
            return 1;
        }
    }
    if oferta != 0 && cqb7_d(oferta + 0xc) as i32 > 0x88b7 {
        return 1;
    }
    let idade = cqb7_b(p + 0x18) as i8;
    if idade < 0x18 && ficha != 0 {
        crate::comparacao::ramo("cqb_7 confere o jovem");
        if cqb7_w(ficha + 7) > 0xbe { // cqb7: o jovem acima de 190
            return 1;
        }
    }
    let jogos = cqb7_b(p + 0x22);
    if jogos < 5 {
        return 0;
    }
    if jogos > 0x14 {
        let nacao = cqb7_d(p + 0x1a);
        if nacao != 0 && cqb7_w(nacao + 0x8e) > 0xbb8 && idade < 0x20 {
            return 1;
        }
    }
    // o histórico pelos jogos novos da seleção
    let selecao = cqb7_w(cqb7_d(cqb7_d(0xb74318) + 4).wrapping_add(cqb7_d(cqb7_d(p + 0x1a)).wrapping_mul(0x6e)) + 2);
    let novos = selecao.wrapping_sub(cqb7_w(cqb7_marcas(this, p) + 5));
    if novos > 0 {
        for _ in 0..novos {
            let h = cqb7_marcas(this, p) + 3;
            core::ptr::write_unaligned(h as *mut u16, core::ptr::read_unaligned(h as *const u16) << 1);
            *(h as *mut u8) &= 0xfe;
        }
    }
    core::ptr::write_unaligned((cqb7_marcas(this, p) + 5) as *mut i16, selecao);
    let ano = cqb7_w(0xae2c92) as u16;
    let janela: i16 = if (ano as i32) - 2 < 0x7d1 {
        0x10
    } else {
        let mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x54cfb0usize);
        let ate = mes(0xae2c90);
        let de = mes(0xae2c90);
        let periodo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u16 = core::mem::transmute(0x58d710usize);
        let n = periodo(cqb7_d(cqb7_d(0xb74318) + 0x1c), cqb7_d(p + 0x1a), de, (ano as u32).wrapping_sub(2), ate, ano as u32) as i16;
        if n > 0x10 { 0x10 } else { n }
    };
    let mut marcados: i16 = 0;
    let mut k: i16 = 0;
    while k < janela && k < 0x10 {
        let mascara = 1u16 << k; // ftol(pow(2.0, k)) (0x945ff0 e 0x945640): exato de 0 a 15
        if core::ptr::read_unaligned((cqb7_marcas(this, p) + 3) as *const u16) & mascara != 0 {
            marcados += 1;
        }
        k += 1;
    }
    if sorteio != 0 {
        let a = cqb7_sorteio(3);
        let limite = 0xb - a + cqb7_sorteio(3);
        if marcados as i32 > limite {
            return 1;
        }
        if janela == 0 {
            return 0;
        }
        let b = cqb7_sorteio(10);
        let limite = 0x46 - b + cqb7_sorteio(10);
        return ((marcados as i32 * 100) / janela as i32 > limite) as u32;
    }
    crate::comparacao::ramo("cqb_7 jogos pela selecao");
    if marcados > 0xb { // cqb7: mais de 11 jogos marcados
        return 1;
    }
    if janela == 0 {
        return 0;
    }
    ((marcados as i32 * 100) / janela as i32 > 0x46) as u32
}

/// Comparador da 0x8c7950: lado a lado nas marcas da pessoa (8 bytes), no sorteio e na devolução (a cópia em
/// sem_contar).
unsafe extern "thiscall" fn comparar_permissao_concedida(this: usize, p: usize, nacao: usize, oferta: usize, registro: usize, sorteio: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(1476);
    let mut regioes = vec![crate::aleatorio::REGIOES_DO_ESTADO[0], crate::aleatorio::REGIOES_DO_ESTADO[1]];
    if p != 0 && (cqb7_d(p) as i32) < cqb7_d(0xae2364) as i32 {
        regioes.push((cqb7_marcas(this, p), 8));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, p, nacao, oferta, registro, sorteio),
        || crate::aleatorio::sem_contar(|| permissao_concedida(this, p, nacao, oferta, registro, sorteio)));
    PLACAR.registrar(igual && r == n, || format!("permissao_concedida(pessoa {:#x}, oferta {:#x}, registro {:#x}, sorteio {}) original={} {:x?} rust={} {:x?}", p, oferta, registro, sorteio, r, mo, n, mr));
    r
}

/// { 0x8c5800 } (thiscall, ret 8, transfer_manager.cpp; o this não é usado): `transfer_manager_get_squad_status_for_reason
/// (pessoa, motivo)`, pela tela da oferta. O contrato da pessoa (0x4cf5a0, guardado em [0xae1080]) dá o clube; o
/// contrato com ele (0x4cf5f0); sem ele, 0. O motivo (16 bits com sinal) 0x200 dá 1, 0x400 dá 2, 0x2000 dá 5; os
/// outros, o estatuto do contrato (+0x4f >> 4). Devolve no al.
unsafe extern "thiscall" fn estatuto_pelo_motivo(_this: usize, p: usize, motivo: u32) -> u32 {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cqb7_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let cc = com_clube(cqb7_d(0xae1050), p, cqb7_clube_do_contrato(c));
    if cc == 0 {
        return 0;
    }
    match motivo as i16 as i32 {
        0x200 => 1,
        0x400 => 2,
        0x2000 => 5,
        _ => (cqb7_b(cc + 0x4f) >> 4) as u32,
    }
}

/// Comparador da 0x8c5800: lado a lado em [0xae1080] e no al devolvido.
unsafe extern "thiscall" fn comparar_estatuto_pelo_motivo(this: usize, p: usize, motivo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1477);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(this, p, motivo), || estatuto_pelo_motivo(this, p, motivo));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("estatuto_pelo_motivo(pessoa {:#x}, motivo {:#x}) original={} {:x?} rust={} {:x?}", p, motivo, r as u8, mo, n as u8, mr));
    r
}

/// { 0x8c6200 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::get_negotiation_slot(i)`, pela resposta
/// automática às notícias (0x782f70) e pelas telas: o endereço da vaga `i` do vetor de negociações ([this+0xc], conta
/// [this+0x10], com sinal); fora da conta, o começo do vetor.
unsafe extern "thiscall" fn vaga_da_negociacao(this: usize, i: i32) -> usize {
    if i < cqb7_d(this + 0x10) as i32 {
        cqb7_d(this + 0xc).wrapping_add((i as usize).wrapping_mul(4))
    } else {
        cqb7_d(this + 0xc)
    }
}

/// Comparador da 0x8c6200: pelo valor (só lê).
unsafe extern "thiscall" fn comparar_vaga_da_negociacao(this: usize, i: i32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, i32) -> usize>(1478)(this, i);
    let n = vaga_da_negociacao(this, i);
    PLACAR.registrar(r == n, || format!("vaga_da_negociacao({}) original={:#x} rust={:#x}", i, r, n));
    r
}

// ---------- 264a leva: a reação do elenco à história de um jogador, o aviso aos rivais de posição e o rótulo das respostas à mídia (media.cpp) ----------

unsafe fn cqb8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb8_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqb8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqb8_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O vínculo da `pessoa` com o `clube` (0x4cf5f0: thiscall em [0xae1050], 2 na pilha; grava [0xae1080]).
unsafe fn cqb8_vinculo(pessoa: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    f(cqb8_d(0xae1050), pessoa, clube)
}

/// O contrato inteiro (+0 a +0x4f) da `pessoa` no `clube`, como região do comparador.
unsafe fn cqb8_contrato(pessoa: usize, clube: usize) -> Option<(usize, usize)> {
    let c = cqb8_vinculo(pessoa, clube);
    (c != 0).then_some((c, 0x50))
}

/// Uma rotina sem porta que por dentro chama unidades da fita (0x544520 e 0x544600 chamam a 0x4cf5f0; a 0x62c400
/// alcança a 0x600a50 e portas): na comparação pela fita, a original grava essas chamadas internas no nível de cima
/// (com o comparador da rotina, a original e a cópia dele gravam cada uma a sua). A cópia roda a rotina numa fita de
/// rascunho e pede as mesmas chamadas por `chamar`, na ordem. Fora da comparação (sem GRAVAR_POS) só chama.
unsafe fn cqb8_pela_fita<R>(chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, f: impl FnOnce() -> R) -> R {
    if !GRAVAR_POS.load(std::sync::atomic::Ordering::Relaxed) {
        return f();
    }
    let externa = fita().replace((Vec::new(), Vec::new()));
    let r = f();
    let gravadas = core::mem::replace(&mut *fita(), externa).map_or(Vec::new(), |g| g.1);
    for c in gravadas {
        chamar(c.0, c.1, c.2);
    }
    r
}

/// O vetor que a 0x755340 monta na pilha (58 dwords, ela percorre 57): os 50 do elenco (0x541f90 com 0, cdecl), o
/// +0xbf do clube, o técnico (+0xcf) só sem humano no clube (0x600a50 com 0, 1, 0, 0, cdecl, por `chamar`: unidade da
/// fita) e fora dos tipos 11 a 15 (0x53b620, cdecl, o byte +0x3d), o +0xd3 e os cinco dwords de +0x19f.
unsafe fn cqb8_vetor(clube: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> [usize; 58] {
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let tipo: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x53b620usize);
    let mut v = [0usize; 58];
    reunir(clube, v.as_mut_ptr(), 0);
    v[50] = cqb8_d(clube + 0xbf);
    let tecnico = cqb8_d(clube + 0xcf);
    v[51] = if chamar(0x600a50, 0, [clube as u32, 0, 1, 0, 0]) != 0 || (tecnico != 0 && tipo(*((tecnico + 0x3d) as *const u8) as u32) != 0) {
        0
    } else {
        tecnico
    };
    v[52] = cqb8_d(clube + 0xd3);
    for k in 0..5 {
        v[53 + k] = cqb8_d(clube + 0x19f + 4 * k);
    }
    v
}

/// { 0x755340 } (thiscall, ret 0x14, media.cpp): `Media::apply_squad_reaction_to_player_story(tópico, jogador, clube,
/// ids, n)`, depois de uma história sobre o jogador (o boato de transferência 0x754a30 com o tópico 0x1f e os clubes
/// interessados; 0x72c1f0 e 0x733a60 sem ids). A chance começa em 10 (5 se o jogador é favorito do clube, 0x547790; 15
/// se rejeitado, 0x5477d0) e cresce por clube interessado: 100 no clássico de Glasgow, 10 se a liga tem o confronto
/// (0x4b02e0), 15 se o clube é rival dele (+0xb3/+0xb7/+0xbb), 5 se é da mesma divisão (fora da divisão inferior).
/// Sorteio(chance) acima de sorteio(10): o clube rejeita o jogador (0x547940 com 1). Listado (0x544520): −25; sem
/// vínculo: −15; senão −10 no fim de contrato (0x867d30) e mais a felicidade (+0x3d) / 10; no mínimo 5. Para cada uma
/// das 57 pessoas do vetor (cqb8_vetor) com preferências (+0x65): a nota (10, o +0x1d da ficha de técnico + 10, ou pelo
/// +0x5d, idade, +0x5b e lista a pedido do jogador do elenco), ajustada pelo tópico 0x20/0x1e, pelo +0x59/+0x5a/+0x5b,
/// por gostar ou não do jogador (0x547810/0x547840) e pelo jogador ser favorito ou rejeitado do clube, no mínimo 5;
/// sorteio(chance) acima de sorteio(nota): a pessoa passa a não gostar do jogador (0x547af0 com 1) e, com vínculo e
/// sorteio(nota) abaixo de sorteio(10), a marca 1 do contrato (0x867560) ou, nos tópicos 0x1e/0x20 ou com sorteio(nota)
/// zero, as queixas 0x800 (0x867700). As três com efeito vão por `chamar` (as portas do pós-jogo), e também as unidades
/// da fita que a original chama: o vínculo 0x4cf5f0 (0x75551e, 0x7557dc), o humano 0x600a50 (0x755598) e as 0x4cf5f0
/// de dentro da 0x544520 (0x7554e8) e da 0x544600 (0x75567f, 0x7556ad), estas por `cqb8_pela_fita`.
unsafe fn cqb8_reacao_com(topico: u32, jogador: usize, clube: usize, ids: usize, n: u32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547790usize);
    let rejeitado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5477d0usize);
    let confronto: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4b02e0usize);
    let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547940usize);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x867d30usize);
    let a_pedido: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544600usize);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    if clube == 0 || jogador == 0 {
        return;
    }
    crate::comparacao::ramo("cqb_8 reacao do elenco");
    let mut chance: i32 = 0xa; // cqb8: a chance inicial
    if favorito(jogador, clube) != 0 {
        chance = 5;
    } else if rejeitado(jogador, clube) != 0 {
        chance = 0xf;
    }
    if ids != 0 {
        let n = n as i8 as i32;
        let mut k = 0i32;
        while k < n {
            let id = cqb8_d(ids + 4 * k as usize);
            let outro = cqb8_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245));
            k += 1;
            let (celtic, rangers) = (cqb8_d(0x9d0118), cqb8_d(0x9d011c));
            let meu = cqb8_d(clube);
            if (meu == celtic || meu == rangers) && (cqb8_d(outro) == celtic || cqb8_d(outro) == rangers) {
                chance += 100;
                continue;
            }
            if confronto(clube, outro) != 0 {
                chance += 10;
                continue;
            }
            if clube == cqb8_d(outro + 0xb3) || clube == cqb8_d(outro + 0xb7) || clube == cqb8_d(outro + 0xbb) {
                chance += 0xf;
                continue;
            }
            let div = cqb8_d(clube + 0x57);
            if div != 0 && cqb8_d(div) != cqb8_d(0x9cf764) && div == cqb8_d(outro + 0x57) {
                chance += 5;
            }
        }
    }
    let r1 = cqb8_sortear(chance);
    if r1 > cqb8_sortear(10) {
        rejeitar(clube, jogador, 1);
    }
    // cqb8: a 0x4cf5f0 (de dentro da 0x544520 e direta) é unidade da fita: vai por `chamar`
    if cqb8_pela_fita(chamar, || listada(jogador, 0)) != 0 {
        chance -= 0x19;
    } else {
        let c = chamar(0x4cf5f0, cqb8_d(0xae1050), [jogador as u32, clube as u32, 0, 0, 0]) as usize;
        if c == 0 {
            chance -= 0xf;
        } else {
            if faixa(c) != 0 {
                chance -= 10;
            }
            chance += (cqb8_b(c + 0x3d) as i32 / 10) as i8 as i32;
        }
    }
    if chance < 5 {
        chance = 5;
    }
    let v = cqb8_vetor(clube, chamar);
    let topico = topico as u16;
    for &p in v.iter().take(57) {
        if p == 0 || p == jogador || cqb8_d(p + 0x65) == 0 {
            continue;
        }
        let mut nota: i8 = 10;
        let tecnico = cqb8_d(p + 0x69);
        if tecnico != 0 {
            nota = cqb8_b(tecnico + 0x1d).wrapping_add(10);
        } else if cqb8_d(p + 0x61) != 0 {
            if cqb8_b(jogador + 0x18) > 0x14 && cqb8_b(p + 0x18) < 0x17 && cqb8_w(cqb8_d(p + 0x61) + 0xb) < 0x109a {
                continue;
            }
            crate::comparacao::ramo("cqb_8 nota do jogador");
            nota = cqb8_b(p + 0x5d).wrapping_add(10); // cqb8: a nota do jogador do elenco
            if cqb8_b(p + 0x18) < 0x14 {
                nota = nota.wrapping_sub(5);
            }
            let m = cqb8_b(p + 0x5b);
            if m == 0x14 {
                nota = nota.wrapping_add(1);
            } else if m == 1 {
                nota = nota.wrapping_sub(if cqb8_pela_fita(chamar, || a_pedido(p, 0)) != 0 { 0xf } else { 10 });
                if cqb8_w(cqb8_d(p + 0x61) + 0xb) > 0x1676 {
                    nota = nota.wrapping_sub(10);
                }
            } else if m < 6 {
                nota = nota.wrapping_sub(if cqb8_pela_fita(chamar, || a_pedido(p, 0)) != 0 { 10 } else { 5 });
                if cqb8_w(cqb8_d(p + 0x61) + 0xb) > 0x1676 {
                    nota = nota.wrapping_sub(5);
                }
            }
        }
        if topico == 0x20 && cqb8_b(p + 0x59) > 0xf && cqb8_b(p + 0x5d) < 6 {
            nota = nota.wrapping_sub(5);
        }
        let a59 = cqb8_b(p + 0x59);
        if a59 > 0xf {
            if cqb8_b(p + 0x5b) != 0x14 {
                nota = nota.wrapping_sub(2);
            }
        } else if a59 < 5 {
            nota = nota.wrapping_add(2);
        }
        if topico == 0x1e {
            let (a5b, a5a) = (cqb8_b(p + 0x5b), cqb8_b(p + 0x5a));
            if a5b > 0xf {
                if a5a < 0xf {
                    nota = nota.wrapping_sub(1);
                }
            } else if a5b >= 6 {
                nota = nota.wrapping_sub(if a5a >= 0xf { 1 } else { 3 });
            } else {
                nota = nota.wrapping_sub(if a5a < 6 { 10 } else { 5 });
            }
        }
        if gosta(jogador, p) != 0 {
            nota = nota.wrapping_add(0x19);
        } else if nao_gosta(jogador, p) != 0 {
            nota = nota.wrapping_add(cqb8_b(jogador + 0x5b).wrapping_sub(0x19));
            if nota > 0xf {
                nota = 0xf;
            }
        }
        if favorito(jogador, clube) != 0 {
            nota = nota.wrapping_add(5);
        } else if rejeitado(jogador, clube) != 0 {
            nota = nota.wrapping_sub(5);
        }
        if nota < 5 {
            nota = 5;
        }
        let r1 = cqb8_sortear(chance);
        if r1 > cqb8_sortear(nota as i32) { // cqb8: o desafeto sorteado
            crate::comparacao::ramo("cqb_8 desafeto sorteado");
            chamar(0x547af0, 0, [p as u32, jogador as u32, 1, 0, 0]);
            let c = chamar(0x4cf5f0, cqb8_d(0xae1050), [p as u32, clube as u32, 0, 0, 0]) as usize;
            if c != 0 {
                let r = cqb8_sortear(nota as i32);
                if r < cqb8_sortear(10) {
                    if topico != 0x1e && topico != 0x20 && cqb8_sortear(nota as i32) != 0 {
                        chamar(0x867560, c, [1, 0, 0, 0, 0]);
                    } else {
                        chamar(0x867700, c, [0x800, 0, 0, 0, 0]);
                    }
                }
            }
        }
    }
}

unsafe extern "thiscall" fn reacao_do_elenco_a_historia(_this: usize, topico: u32, jogador: usize, clube: usize, ids: usize, n: u32) {
    cqb8_reacao_com(topico, jogador, clube, ids, n, &mut |r, t, a| despachar_pos(r, t, a))
}

/// Comparador: pela fita das portas do pós-jogo (comparar_pos): as desafeições (0x547af0), a marca (0x867560) e as
/// queixas (0x867700) rodam uma vez, na original, e a cópia confere a chamada e recebe o estado de depois. Regiões: o
/// sorteador, [0xae1080], as seis pessoas do clube (+0x9b, a 0x547940), o contrato do jogador e, de cada pessoa do
/// vetor, as preferências (+0x1c..+0x33), o +0x58..+0x5d e o contrato no clube (+0 a +0x4f).
unsafe extern "thiscall" fn comparar_reacao_do_elenco_a_historia(this: usize, topico: u32, jogador: usize, clube: usize, ids: usize, n: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize, usize, usize, u32)>(1479);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((0xae1080, 4));
    if clube != 0 && jogador != 0 {
        reg.push((clube + 0x9b, 24));
        reg.extend(cqb8_contrato(jogador, clube));
        for &p in cqb8_vetor(clube, &mut |r, t, a| despachar_pos(r, t, a)).iter().take(57) {
            if p == 0 {
                continue;
            }
            let pref = cqb8_d(p + 0x65);
            if pref != 0 {
                reg.push((pref + 0x1c, 24));
            }
            reg.push((p + 0x58, 6));
            reg.extend(cqb8_contrato(p, clube));
        }
    }
    comparar_pos("reacao_do_elenco_a_historia", reg, || o(this, topico, jogador, clube, ids, n),
        |chamar| cqb8_reacao_com(topico, jogador, clube, ids, n, chamar));
}

/// { 0x758010 } (thiscall, ret 0x10, media.cpp): `Media::warn_squad_of_position_rival(jogador, clube, _, _)`, pelo
/// boato de transferência (0x754a30) e pela resposta à história do jogador (0x741020). Com ficha, clube com mais de 2
/// do elenco na máscara da posição dele (0x548150 com 0x5481a0), divisão com competição ([0xadadfc]) e reputação
/// (+0xb da ficha) acima de 4750: cada um dos 50 do elenco (+0xd7) com ficha, vínculo (0x4cf5f0) de felicidade (+0x3d)
/// positiva, o jogador sem lesão de estado 2 (0x62c400 em [0xb63d28]), sem as marcas 0x101 (+0x45), fora da lista
/// (0x544520), acima de 20 anos, do tipo 0x10/0x20/0x30 (+0x4f & 0xf0), com +0x5a abaixo de n + 5 e +0x5b abaixo de
/// n + 10, sorteio(+0x5a) abaixo de sorteio(n), a mesma máscara e reputação acima de 3750: a primeira vez, a história
/// 0x25 dele (0x72c1f0 em [0xb6b7c8]); sem a marca 0x800 (0x8676f0), a marca 0x800 (0x867560). As duas com efeito vão
/// por `chamar`, e também o vínculo 0x4cf5f0 (0x7580c2, unidade da fita) e, por `cqb8_pela_fita`, o que a 0x62c400
/// (0x7580ee) e a 0x544520 (0x75810a) chamam de unidades por dentro.
unsafe fn cqb8_rival_com(jogador: usize, clube: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let mascara: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5481a0usize);
    let contar: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(0x548150usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
    let bandeiras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8676f0usize);
    if jogador == 0 || cqb8_d(jogador + 0x61) == 0 || clube == 0 {
        return;
    }
    let n = contar(clube, mascara(jogador)) as i8;
    if n <= 2 {
        return;
    }
    let div = cqb8_d(clube + 0x57);
    if div == 0 || cqb8_d(cqb8_d(0xadadfc) + 4 * cqb8_d(div)) == 0 {
        return;
    }
    if cqb8_w(cqb8_d(jogador + 0x61) + 0xb) <= 0x128e {
        return;
    }
    crate::comparacao::ramo("cqb_8 rival de posicao");
    let n = n as i32;
    let mut postou = false;
    for i in 0..50 {
        let p = cqb8_d(clube + 0xd7 + 4 * i);
        if p == 0 || cqb8_d(p + 0x61) == 0 || p == jogador {
            continue;
        }
        // cqb8: a 0x4cf5f0 é unidade da fita (direta e de dentro da 0x544520; a 0x62c400 alcança a 0x600a50)
        let c = chamar(0x4cf5f0, cqb8_d(0xae1050), [p as u32, clube as u32, 0, 0, 0]) as usize;
        if c == 0 || cqb8_d(p + 0x61) == 0 || cqb8_b(c + 0x3d) <= 0 {
            continue;
        }
        if cqb8_pela_fita(chamar, || lesao(cqb8_d(0xb63d28), jogador)) == 2 || cqb8_d(c + 0x45) & 0x101 != 0
            || cqb8_pela_fita(chamar, || listada(p, clube)) != 0
        {
            continue;
        }
        if cqb8_b(p + 0x18) <= 0x14 {
            continue;
        }
        let t = *((c + 0x4f) as *const u8) & 0xf0;
        if t != 0x10 && t != 0x20 && t != 0x30 {
            continue;
        }
        let a5a = cqb8_b(p + 0x5a) as i32;
        if a5a >= n + 5 || cqb8_b(p + 0x5b) as i32 >= n + 10 {
            continue;
        }
        let r1 = cqb8_sortear(n);
        if cqb8_sortear(a5a) >= r1 {
            continue;
        }
        let m = mascara(jogador);
        if m != mascara(p) || cqb8_w(cqb8_d(p + 0x61) + 0xb) <= 0xea6 {
            continue;
        }
        if !postou {
            chamar(0x72c1f0, cqb8_d(0xb6b7c8), [p as u32, clube as u32, 0x25, 0, jogador as u32]);
            postou = true;
        }
        if bandeiras(c) & 0x800 != 0 {
            continue;
        }
        chamar(0x867560, c, [0x800, 0, 0, 0, 0]);
    }
}

unsafe extern "thiscall" fn aviso_ao_rival_de_posicao(_this: usize, jogador: usize, clube: usize, _a: u32, _b: u32) {
    cqb8_rival_com(jogador, clube, &mut |r, t, a| despachar_pos(r, t, a))
}

/// Comparador: pela fita das portas do pós-jogo (a história 0x72c1f0 e a marca 0x867560 rodam só na original). Regiões:
/// o sorteador, [0xae1080] e, de cada um dos 50 do elenco, o +0x58..+0x5d e o contrato no clube (+0 a +0x4f).
unsafe extern "thiscall" fn comparar_aviso_ao_rival_de_posicao(this: usize, jogador: usize, clube: usize, a: u32, b: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(1480);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((0xae1080, 4));
    if clube != 0 {
        for i in 0..50 {
            let p = cqb8_d(clube + 0xd7 + 4 * i);
            if p != 0 {
                reg.push((p + 0x58, 6));
                reg.extend(cqb8_contrato(p, clube));
            }
        }
    }
    comparar_pos("aviso_ao_rival_de_posicao", reg, || o(this, jogador, clube, a, b), |chamar| cqb8_rival_com(jogador, clube, chamar));
}

/// As respostas 1 e 2 de cada caso da tabela de saltos da 0x735160 (a 3ª, de todos, é "No Comment", 0xa27808); o caso
/// vem da tabela de bytes do exe em 0x7356f4 (tipo − 3, até 0x7f; acima, o 21).
const CQB8_RESPOSTAS: [(usize, usize); 22] = [
    (0xa27700, 0xa2771c), (0xa27670, 0xa2767c), (0xa27688, 0xa276a4), (0xa276e4, 0xa276f0), (0xa2773c, 0xa27770),
    (0xa2764c, 0xa2765c), (0xa27430, 0xa2743c), (0xa2762c, 0xa2763c), (0xa2744c, 0xa27460), (0xa275d0, 0xa27534),
    (0xa27554, 0xa27574), (0xa27510, 0xa27534), (0xa275b4, 0xa275c0), (0xa274e0, 0xa274f8), (0xa27588, 0xa275a0),
    (0xa274b4, 0xa274c4), (0xa2744c, 0xa27494), (0xa2744c, 0xa27610), (0xa275e8, 0xa275f8), (0xa2744c, 0xa2747c),
    (0xa27798, 0xa277c8), (0xa2740c, 0xa2741c),
];

/// { 0x735160 } (stdcall, ret 0x10, media.cpp): `media_format_response_option_label(tipo, opção, saída, tamanho)`: o
/// rótulo da opção de resposta a uma história ("Deny rumour", "Defend Player", "No Comment"...) na saída (0x66f4e0),
/// pela tela das histórias (0x742b3b, com 0xde1f64 e 0x7d0). Saída nula ou tamanho abaixo de 15: nada.
unsafe extern "stdcall" fn rotulo_da_resposta_da_midia(tipo: u32, opcao: u32, saida: usize, tamanho: u32) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    if saida == 0 || (tamanho as i16) < 0xf {
        return;
    }
    let k = (tipo as i16 as i32).wrapping_sub(3) as u32;
    let caso = if k > 0x7f { 21 } else { *((0x7356f4 + k as usize) as *const u8) as usize };
    let (um, dois) = CQB8_RESPOSTAS[caso];
    let texto = match opcao as i8 {
        1 => um,
        2 => dois,
        _ => 0xa27808,
    };
    formatar(saida, texto);
}

/// Comparador: lado a lado nos primeiros bytes da saída (até 0x50, o maior rótulo tem 61).
unsafe extern "stdcall" fn comparar_rotulo_da_resposta_da_midia(tipo: u32, opcao: u32, saida: usize, tamanho: u32) {
    let o = orig::<unsafe extern "stdcall" fn(u32, u32, usize, u32)>(1481);
    let reg = if saida != 0 && (tamanho as i16) >= 0xf { vec![(saida, (tamanho as u16 as usize).min(0x50))] } else { Vec::new() };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(tipo, opcao, saida, tamanho), || rotulo_da_resposta_da_midia(tipo, opcao, saida, tamanho));
    PLACAR.registrar(igual, || format!("rotulo_da_resposta_da_midia({}, {}) {:x?} x {:x?}", tipo as i16, opcao as i8, mo, mr));
}

// ---------- 264a leva: as suspensões internacionais cumpridas no jogo da seleção, o recurso da suspensão e o texto da vaga da suspensão (discipline.cpp) ----------

unsafe fn cqb9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Os itens (8 bytes: +0 id da pessoa, +4 id da nação) do índice das nações da disciplina ([this+0xc], conta +0x12
/// com sinal) da nação do `time` (+0x53), na ordem do índice: bsearch (0x946464 com 0x55e5f0, cdecl) pela chave (−1,
/// nação), que iguala qualquer pessoa da nação; volta até o primeiro igual e segue até o último. 0 itens sem achar.
unsafe fn cqb9_itens(this: usize, time: usize) -> (usize, usize) {
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let chave = [u32::MAX, cqb9_d(cqb9_d(time + 0x53)) as u32];
    let base = cqb9_d(this + 0xc);
    let mut e = bsearch(chave.as_ptr(), base, cqb9_w(this + 0x12) as i32, 8, 0x55e5f0);
    if e == 0 {
        return (0, 0);
    }
    // os itens são iguais à chave pela nação (+4): a chave tem −1 no id
    let igual = |x: usize| cqb9_d(x + 4) as u32 == chave[1];
    while e > base && igual(e - 8) {
        e -= 8;
    }
    let mut fim = e;
    while fim <= base.wrapping_add((cqb9_w(this + 0x12) as i32 as usize).wrapping_mul(8)).wrapping_sub(8) && igual(fim) {
        fim += 8;
    }
    (e, fim)
}

/// { 0x557120 } (thiscall, ret 4, discipline.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `Discipline::serve_international_bans(time)`, pelo jogo da seleção (0x7e8090, no órgão das suspensões). As pessoas da
/// nação do time no índice das nações (cqb9_itens) vão para uma lista (0x68d0d0 com o item e 4: a cópia do id, porque
/// a contagem pode soltar a suspensão e reordenar o índice) e, para cada uma, um jogo cumprido na regra 0
/// (0x556e90 com 0 e 1). No fim a lista é destruída.
unsafe extern "thiscall" fn cumprir_suspensoes_internacionais(this: usize, time: usize) {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let por: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let contar: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(0x556e90usize);
    let mut l = [0u8; 14];
    let lista = l.as_mut_ptr() as usize;
    nova(lista);
    let (ini, fim) = cqb9_itens(this, time);
    if ini != 0 {
        let mut e = ini;
        while e < fim {
            por(lista, e, 4);
            e += 8;
        }
        // o cursor (+0xa) no começo (+2)
        core::ptr::write_unaligned((lista + 0xa) as *mut usize, cqb9_d(lista + 2));
        let mut p = proximo(lista);
        while p != 0 {
            crate::comparacao::ramo("cqb_9 jogo cumprido");
            contar(this, cqb9_d(p) as u32, 0, 1); // cqb9: o jogo cumprido na regra 0
            p = proximo(lista);
        }
    }
    soltar(lista);
}

/// Comparador: a contagem (0x556e90) grava o histórico no arquivo quando a suspensão termina e não pode rodar duas
/// vezes: o plano antes (as pessoas da nação e, de cada uma, o que a 0x556e90 fará, cpm4_jogo), a original roda;
/// confere, pessoa a pessoa, o contador novo ou a suspensão solta (livre: +0 = 0xff, +2 = −1).
unsafe extern "thiscall" fn comparar_cumprir_suspensoes_internacionais(this: usize, time: usize) {
    let (ini, fim) = cqb9_itens(this, time);
    let mut plano = Vec::new();
    let mut e = ini;
    while ini != 0 && e < fim {
        let id = cqb9_d(e) as u32;
        let j = cpm4_jogo(this, id, 0, 1);
        let antes = if j.suspensao != 0 { *((j.suspensao + 1) as *const u8) } else { 0 };
        plano.push((id, j, antes));
        e += 8;
    }
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1482)(this, time);
    let mut erro = None;
    for (id, j, antes) in &plano {
        if j.suspensao == 0 {
            continue;
        }
        let s = j.suspensao;
        let (jogos, contador, dias) = (*(s as *const u8), *((s + 1) as *const u8), cqb9_w(s + 2));
        let ok = if j.cumprida { jogos == 0xff && dias == -1 } else { contador == j.contador.unwrap_or(*antes) };
        if !ok && erro.is_none() {
            erro = Some(format!("pessoa {} contador {} (esperava {:?}, antes {}), cumprida={} suspensão {:#x}/{}", id, contador, j.contador, antes, j.cumprida, jogos, dias));
        }
    }
    PLACAR.registrar(erro.is_none(), || format!("cumprir_suspensoes_internacionais(time {:#x}, {} pessoas): {}", time, plano.len(), erro.clone().unwrap_or_default()));
}

/// O plano do recurso: a entrada da pessoa no órgão da `regra` (0x7e82e0 sem criar), a suspensão (20 bytes em
/// [this+4]), os valores novos de +0 (jogos) ou +2 (dias) e se ela é solta.
struct Cqb9Recurso {
    orgao: usize,
    entrada: usize,
    suspensao: usize,
    jogos: u8,
    dias: i16,
    solta: bool,
}

unsafe fn cqb9_recurso(this: usize, id: u32, regra: u32, reduz: u32, menos_jogos: u32, menos_dias: u32, novo: u32, dias_fixo: u32) -> Cqb9Recurso {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x7e82e0usize);
    let orgao = cqb9_d(cqb9_d(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4)));
    let entrada = achar(orgao, id, 0);
    let mut r = Cqb9Recurso { orgao, entrada, suspensao: 0, jogos: 0, dias: 0, solta: false };
    if entrada == 0 || cqb9_w(entrada + 6) == -1 {
        return r;
    }
    let s = cqb9_d(this + 4).wrapping_add((cqb9_w(entrada + 6) as i32 as usize).wrapping_mul(20));
    r.suspensao = s;
    r.jogos = *(s as *const u8);
    r.dias = cqb9_w(s + 2);
    if r.jogos != 0xff {
        if novo as u8 != 0xff {
            r.jogos = r.jogos.wrapping_sub(novo as u8);
            if (r.jogos as i8) < 0 {
                r.jogos = 0;
                r.solta = true;
            }
        } else if reduz as u8 != 0 {
            r.jogos = r.jogos.wrapping_sub(menos_jogos as u8);
            r.solta = r.jogos as i8 <= *((s + 1) as *const i8);
        } else {
            r.solta = true;
        }
    } else if dias_fixo as u16 != 0xffff {
        r.dias = r.dias.wrapping_sub(menos_dias as i16);
        if r.dias <= 0 {
            r.dias = 0;
            r.solta = true;
        }
    } else if reduz as u8 != 0 {
        r.dias = r.dias.wrapping_sub(menos_dias as i16);
        r.solta = r.dias <= 0;
    } else {
        r.solta = true;
    }
    r
}

/// { 0x557000 } (thiscall, ret 0x20, discipline.cpp): `Discipline::apply_appeal_result(id, regra, reduz, menos_jogos,
/// menos_dias, novo, dias_fixo, _)`, pelo resultado do recurso (0x6abf99, notícias do técnico humano). Com a entrada da
/// pessoa no órgão (0x7e82e0 sem criar) e suspensão (+6): o órgão tira as suspensões pendentes dela (0x7e91d0); por
/// jogos (+0 ≠ 0xff): `novo` ≠ 0xff tira `novo` jogos (abaixo de 0: zera e solta), senão `reduz` tira `menos_jogos` e
/// solta se ficar até o contador (+1), senão solta; por dias: `dias_fixo` ≠ −1 tira `menos_dias` (até 0: zera e solta),
/// senão `reduz` tira e solta até 0, senão solta. Soltar: 0x556d00 com o índice.
unsafe extern "thiscall" fn recurso_da_suspensao(this: usize, id: u32, regra: u32, reduz: u32, menos_jogos: u32, menos_dias: u32, novo: u32, dias_fixo: u32, _x: u32) {
    let tirar_pendentes: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x7e91d0usize);
    let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x556d00usize);
    let r = cqb9_recurso(this, id, regra, reduz, menos_jogos, menos_dias, novo, dias_fixo);
    if r.suspensao == 0 {
        return;
    }
    crate::comparacao::ramo("cqb_9 recurso");
    tirar_pendentes(r.orgao, id);
    let s = r.suspensao;
    *(s as *mut u8) = r.jogos;
    core::ptr::write_unaligned((s + 2) as *mut i16, r.dias);
    if r.solta {
        // ponytail: a original empurra o registrador com o índice na palavra baixa (o alto é sobra); a 0x556d00 só lê
        // a palavra
        soltar(this, id, cqb9_w(r.entrada + 6) as u16 as u32);
    }
}

/// Comparador: o órgão tira as suspensões pendentes (0x7e91d0, lista que solta nós) e a 0x556d00 pode gravar o
/// histórico: só a original, com o plano antes; confere a suspensão solta (+0 = 0xff, +2 = −1) ou os jogos e os dias
/// novos.
unsafe extern "thiscall" fn comparar_recurso_da_suspensao(this: usize, id: u32, regra: u32, reduz: u32, menos_jogos: u32, menos_dias: u32, novo: u32, dias_fixo: u32, x: u32) {
    let r = cqb9_recurso(this, id, regra, reduz, menos_jogos, menos_dias, novo, dias_fixo);
    orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32)>(1483)(this, id, regra, reduz, menos_jogos, menos_dias, novo, dias_fixo, x);
    if r.suspensao == 0 {
        return;
    }
    let s = r.suspensao;
    let (jogos, dias) = (*(s as *const u8), cqb9_w(s + 2));
    let ok = if r.solta { jogos == 0xff && dias == -1 } else { jogos == r.jogos && dias == r.dias };
    PLACAR.registrar(ok, || format!("recurso_da_suspensao({}, regra {}) jogos {:#x} dias {} (esperava {:#x}/{}, solta={})", id, regra as i8, jogos, dias, r.jogos, r.dias, r.solta));
}

/// A vaga `vaga` (byte com sinal) da suspensão da pessoa no registro de 29 bytes ([this+0x14]: 5 bytes por vaga): o
/// órgão da regra (0xff = sem vaga).
unsafe fn cqb9_vaga(this: usize, pessoa: usize, vaga: u32) -> u8 {
    let id = cqb9_d(pessoa);
    *((cqb9_d(this + 0x14) + id.wrapping_mul(29) + (vaga as i8 as isize as usize).wrapping_mul(5)) as *const u8)
}

/// { 0x55d3c0 } (thiscall, ret 0x10, discipline.cpp): `Discipline::get_slot_text(pessoa, vaga, saída, tamanho)`, pela
/// tela da pessoa (0x87618a, com 0xc8). Pessoa de id a partir de [0xae2364]: 0. A regra da vaga (cqb9_vaga) 0xff: 0;
/// senão o texto do órgão dela ([[this] + 4 × regra], 0x7e88d0 com a pessoa, a saída e o tamanho). Pessoa nula: a
/// original (lê o id de 0).
unsafe extern "thiscall" fn texto_da_vaga_da_suspensao(this: usize, pessoa: usize, vaga: u32, saida: usize, tamanho: u32) -> u32 {
    if pessoa == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u32>(1484)(this, pessoa, vaga, saida, tamanho);
    }
    if cqb9_d(pessoa) as i32 >= cqb9_d(0xae2364) as i32 {
        return 0;
    }
    let regra = cqb9_vaga(this, pessoa, vaga);
    if regra == 0xff {
        return 0;
    }
    let texto: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x7e88d0usize);
    texto(cqb9_d(cqb9_d(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4))), pessoa, saida, tamanho)
}

/// Comparador: sem regra na vaga (ou pessoa fora da tabela), a devolução (0, sem gravar nada); com regra, o texto do órgão
/// (0x7e88d0) pode contar a suspensão por dias (0x556e90, que grava o histórico no arquivo): só a original.
unsafe extern "thiscall" fn comparar_texto_da_vaga_da_suspensao(this: usize, pessoa: usize, vaga: u32, saida: usize, tamanho: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u32>(1484);
    if pessoa == 0 || (cqb9_d(pessoa) as i32) < cqb9_d(0xae2364) as i32 && cqb9_vaga(this, pessoa, vaga) != 0xff {
        // ponytail: o texto do órgão não roda duas vezes; a cópia só repassa a ele
        return o(this, pessoa, vaga, saida, tamanho);
    }
    let r = o(this, pessoa, vaga, saida, tamanho);
    let n = texto_da_vaga_da_suspensao(this, pessoa, vaga, saida, tamanho);
    PLACAR.registrar(r == n, || format!("texto_da_vaga_da_suspensao(pessoa {:#x}, vaga {}) original={} rust={}", pessoa, vaga as i8, r, n));
    r
}

// ---------- 264a leva: a lista encadeada do jogo: tirar o item pelo conteúdo, esvaziar e o item do cursor (LLIST, vizinhança de manager_manager.cpp) ----------

unsafe fn cqb10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb10_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// O nó da lista (14 bytes: +0 o item, +4 o tamanho em palavra com sinal, +6 o próximo, +0xa o anterior) cujo item tem
/// os mesmos bytes que `dado`, a partir da cabeça (+2). O tamanho vai como na original (movsx para o ecx do repe
/// cmpsb): 0 é igual (o repe não roda e o ZF vem do xor de antes).
unsafe fn cqb10_achar(this: usize, dado: usize) -> usize {
    let mut no = cqb10_d(this + 2); // cqb10: a busca começa na cabeça
    while no != 0 {
        let n = core::ptr::read_unaligned((no + 4) as *const i16) as i32 as u32;
        let item = cqb10_d(no);
        let mut k = 0u32;
        while k < n && *((item + k as usize) as *const u8) == *((dado + k as usize) as *const u8) {
            k += 1;
        }
        if k == n {
            return no;
        }
        no = cqb10_d(no + 6);
    }
    0
}

/// { 0x68d1f0 } (thiscall, ret 4): `LLIST::remove(dado)`: o primeiro nó com os bytes do `dado` (cqb10_achar) sai da
/// lista: a cabeça (+2) ou o próximo do anterior, a cauda (+6) ou o anterior do próximo, a conta (+0, palavra) − 1, o
/// cursor (+0xa) anda se estava nele; o item (0x68d010) e o nó (0x944c94) são liberados. Devolve 1; sem achar, 0. Por
/// ~77 chamadores (listas de ofertas, faltas, ultimatos, vagas de técnico, amistosos...).
unsafe extern "thiscall" fn tirar_da_lista(this: usize, dado: usize) -> u32 {
    let liberar_item: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d010usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let no = cqb10_achar(this, dado);
    if no == 0 {
        return 0;
    }
    crate::comparacao::ramo("cqb_10 tirado");
    let (prox, ant) = (cqb10_d(no + 6), cqb10_d(no + 0xa));
    if no == cqb10_d(this + 2) {
        cqb10_por(this + 2, prox);
    } else {
        cqb10_por(ant + 6, prox);
    }
    if no == cqb10_d(this + 6) {
        crate::comparacao::ramo("cqb_10 cauda");
        cqb10_por(this + 6, ant); // cqb10: a cauda volta ao anterior
    } else {
        cqb10_por(prox + 0xa, ant);
    }
    let conta = core::ptr::read_unaligned(this as *const i16);
    core::ptr::write_unaligned(this as *mut i16, conta.wrapping_sub(1));
    if no == cqb10_d(this + 0xa) {
        crate::comparacao::ramo("cqb_10 cursor");
        cqb10_por(this + 0xa, cqb10_d(no + 6)); // cqb10: o cursor anda
    }
    liberar_item(no);
    liberar(no);
    1
}

/// Comparador: a 0x68d1f0 libera o nó e o item (não roda duas vezes): o plano antes (o nó achado pela cópia, a cabeça,
/// a cauda, a conta, o cursor e os vizinhos); a original tira; confere a devolução, o cabeçalho (14 bytes) e os elos
/// dos vizinhos.
unsafe extern "thiscall" fn comparar_tirar_da_lista(this: usize, dado: usize) -> u32 {
    let no = cqb10_achar(this, dado);
    let antes: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    let (prox, ant) = if no != 0 { (cqb10_d(no + 6), cqb10_d(no + 0xa)) } else { (0, 0) };
    let (cabeca, cauda, cursor) = (cqb10_d(this + 2), cqb10_d(this + 6), cqb10_d(this + 0xa));
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1485)(this, dado);
    let depois: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    let ok = if no == 0 {
        r == 0 && depois == antes
    } else {
        let conta = core::ptr::read_unaligned(antes.as_ptr() as *const i16).wrapping_sub(1);
        r == 1
            && core::ptr::read_unaligned(this as *const i16) == conta
            && cqb10_d(this + 2) == if no == cabeca { prox } else { cabeca }
            && cqb10_d(this + 6) == if no == cauda { ant } else { cauda }
            && cqb10_d(this + 0xa) == if no == cursor { prox } else { cursor }
            && (no == cabeca || cqb10_d(ant + 6) == prox)
            && (no == cauda || cqb10_d(prox + 0xa) == ant)
    };
    PLACAR.registrar(ok, || format!("tirar_da_lista({:#x}, {:#x}) original={} nó {:#x} cabeçalho {:x?} -> {:x?}", this, dado, r, no, antes, depois));
    r
}

/// { 0x68d080 } (thiscall, ret 0): `LLIST::clear()`: libera, da cabeça (+2) em diante, o item (0x68d010) e o nó
/// (0x944c94) de cada um (a cabeça vai andando) e zera a conta (+0, palavra), a cabeça, a cauda (+6) e o cursor (+0xa).
unsafe extern "thiscall" fn esvaziar_lista(this: usize) {
    let liberar_item: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d010usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    while cqb10_d(this + 2) != 0 {
        let no = cqb10_d(this + 2);
        let prox = cqb10_d(no + 6);
        liberar_item(no);
        liberar(no);
        cqb10_por(this + 2, prox);
    }
    core::ptr::write_unaligned(this as *mut u16, 0);
    cqb10_por(this + 2, 0);
    cqb10_por(this + 6, 0);
    cqb10_por(this + 0xa, 0);
}

/// Comparador: libera os nós (não roda duas vezes): só a original; confere o cabeçalho zerado (14 bytes).
unsafe extern "thiscall" fn comparar_esvaziar_lista(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1486)(this);
    let depois: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    PLACAR.registrar(depois == [0u8; 14], || format!("esvaziar_lista({:#x}) cabeçalho {:x?}", this, depois));
}

/// { 0x68d2e0 } (thiscall, ret 0): `LLIST::get_current()`: o item do nó do cursor (+0xa), sem andar; sem cursor, 0.
unsafe extern "thiscall" fn item_do_cursor(this: usize) -> usize {
    let no = cqb10_d(this + 0xa);
    if no == 0 {
        return 0;
    }
    cqb10_d(no)
}

unsafe extern "thiscall" fn comparar_item_do_cursor(this: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> usize>(1487)(this);
    let n = item_do_cursor(this);
    PLACAR.registrar(r == n, || format!("item_do_cursor({:#x}) original={:#x} rust={:#x}", this, r, n));
    r
}

// ---------- 264a leva: a copa: o sorteio pela semente, o sorteio que separa as nações e o começo da temporada da copa (Cup.cpp) ----------

unsafe fn cqb11_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Tira os times dos confrontos da rodada `r` para o vetor de 7 bytes do sorteio (o time +0, o vínculo +2/+4, o
/// nível +6), como nas 0x51d2d0 e 0x51d9f0: o mandante (+0, com +0xe/+0xa) e o visitante (+2, com +0x10/+0xc); com
/// time, a entrada leva o nível do time (+4 do time; a semente), `com_time(vaga)` roda com a vaga ainda intacta e a
/// vaga vira −2 (livre); sem time (−1), a entrada fica −1 com 0xff no nível (o resto da entrada fica como estava).
/// Devolve quantas entradas (a conta de 32 bits, usada em 16).
unsafe fn cqb11_tirar_times(copa: usize, r: i32, vetor: usize, mut com_time: impl FnMut(usize)) -> i32 {
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    while (k as i16) < cpq11_w(cpq11_rodada(copa, r) + 0x1a) {
        for (lado, ligado, jogo) in [(0usize, 0xeusize, 0xausize), (2, 0x10, 0xc)] {
            let t = cpq11_confronto(copa, r, k as i16 as i32);
            let e = cpq11_em(vetor, n as i16 as i32, 7);
            n = n.wrapping_add(1);
            let time = cpq11_w(t + lado);
            if time != -1 {
                cpq11_por_w(e, time as u16);
                cpq11_por_w(e + 2, cpq11_w(t + ligado) as u16);
                cpq11_por_w(e + 4, cpq11_w(t + jogo) as u16);
                *((e + 6) as *mut u8) = cqb11_b(cpq11_time(copa, time) + 4); // a semente do time
                com_time(t + lado);
                cpq11_por_w(t + lado, 0xfffe);
            } else {
                cpq11_por_w(e, 0xffff);
                *((e + 6) as *mut u8) = 0xff;
            }
        }
        k += 1;
    }
    n
}

/// A entrada `e` do vetor vai ao confronto `cur` da rodada `r`: no mandante (+0, +0xe, +0xa) ou no visitante (+2,
/// +0x10, +0xc); com vínculo (+2 da entrada ≠ −1), o confronto ligado na rodada seguinte ganha a rodada (+0x12), o
/// confronto sorteado (+6) e o lado (+8: 1 mandante, 0 visitante).
unsafe fn cqb11_colocar(copa: usize, r: i32, rodada: u32, e: usize, cur: i32, mandante: bool) {
    let t = cpq11_confronto(copa, r, cur as i16 as i32);
    let (lado, ligado, jogo) = if mandante { (0usize, 0xeusize, 0xausize) } else { (2, 0x10, 0xc) };
    cpq11_por_w(t + lado, cpq11_w(e) as u16);
    cpq11_por_w(t + ligado, cpq11_w(e + 2) as u16);
    cpq11_por_w(t + jogo, cpq11_w(e + 4) as u16);
    let seguinte = cpq11_w(e + 2);
    if seguinte != -1 {
        let v = cpq11_confronto(copa, seguinte as i32, cpq11_w(e + 4) as i32);
        cpq11_por_w(v + 0x12, rodada as u16);
        cpq11_por_w(v + 6, cur as u16);
        cpq11_por_w(v + 8, mandante as u16);
    }
}

/// Os times `a` e `b` (índices em [copa + 0xa7]) têm clube e são da mesma nação (+0x53).
unsafe fn cqb11_mesma_nacao(copa: usize, a: i16, b: i16) -> bool {
    let c1 = cpq11_us(cpq11_time(copa, a));
    if c1 == 0 {
        return false;
    }
    let c2 = cpq11_us(cpq11_time(copa, b));
    c2 != 0 && cpq11_d(c2 + 0x53) == cpq11_d(c1 + 0x53)
}

/// O passo da volta pelos confrontos: o próximo (em círculo, 16 bits); de volta ao sorteado, a primeira volta vira a
/// segunda (`segunda`), na segunda cai a regra (`regra`: a nação, ou o "sem a mesma nação" da 0x51d9f0) e, sem ela,
/// devolve false (a caixa).
unsafe fn cqb11_passo(copa: usize, r: i32, cur: &mut i32, inicio: i16, segunda: &mut bool, regra: &mut bool) -> bool {
    *cur = cur.wrapping_add(1);
    if (*cur as i16) >= cpq11_w(cpq11_rodada(copa, r) + 0x1a) {
        *cur = 0;
    }
    if *cur as i16 == inicio {
        if !*segunda {
            *segunda = true;
        } else if *regra {
            *regra = false;
        } else {
            return false;
        }
    }
    true
}

/// { 0x51d2d0 } (thiscall, ret 4, Cup.cpp): `Cup::sort_round_teams(rodada)`: o sorteio pela semente (regra 0x400 da
/// rodada). 1) Tira os times dos confrontos para o vetor de 7 bytes (malloc 0x944e46 de 7 × vagas +0x18; sem memória,
/// a caixa da linha 0xbf8 e o jogo para), cada um com a semente (+4 do time). 2) Ordena pela semente, a maior primeiro
/// (qsort 0x945333 com 0x51ebc0). 3) Cada entrada com time (até a primeira sem, nível 0xff) vai a um confronto
/// sorteado (0x90cfc0 dos confrontos), avançando em círculo: na primeira volta, só confrontos com as duas vagas sem
/// time (< 0): com as duas livres (−2), o lado sorteado (0x90cfc0(2) ≠ 0: mandante); só o mandante livre: mandante;
/// só o visitante: visitante. Da segunda volta em diante (a marca não volta a zero), qualquer vaga livre, mas com a
/// regra 4 da rodada (+0xb) o mandante livre não serve quando o visitante do confronto é da mesma nação, e o
/// visitante livre não serve quando o mandante é; dando a volta inteira de novo, cai a regra das nações (também para
/// as entradas seguintes) e, sem ela, a caixa da linha 0xcff e sai. Libera o vetor (0x9452ca).
unsafe extern "thiscall" fn copa_sortear_pela_semente(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    // os quatro: push n; add esp, 4 (malloc e sorteio) / push p; add esp, 4 (free) / 4 pushes; add esp, 0x10 (qsort) -> cdecl
    let vetor = alocar((cpq11_w(cpq11_rodada(copa, r) + 0x18) as i32).wrapping_mul(7) as u32);
    if vetor == 0 {
        cpq11_sem_memoria(0xbf8, 0x9b56dc); // "CUP::sort_round_teams() : Cannot allocate memory"
    }
    let mut nacao = cqb11_b(cpq11_rodada(copa, r) + 0xb) & 4 != 0; // a regra 4 (+0xb) da rodada
    let n = cqb11_tirar_times(copa, r, vetor, |_| {}) as i16;
    crate::comparacao::ramo("cqb_11 ordena pela semente");
    ordenar(vetor, n as i32, 7, 0x51ebc0); // cqb11: pela semente, a maior primeiro
    let mut segunda = false;
    let mut j: i16 = 0;
    while j < n {
        let e = cpq11_em(vetor, j as i32, 7);
        if cqb11_b(e + 6) == 0xff {
            break;
        }
        let s = sorteio(cpq11_w(cpq11_rodada(copa, r) + 0x1a) as i32);
        let inicio = s as i16;
        let mut cur = s;
        let mandante = loop {
            let t = cpq11_confronto(copa, r, cur as i16 as i32);
            let (casa, fora) = (cpq11_w(t), cpq11_w(t + 2));
            let mut lado: Option<bool> = None;
            if segunda {
                let w0 = cpq11_w(e);
                let livre = casa == -2 && !(nacao && w0 >= 0 && fora >= 0 && cqb11_mesma_nacao(copa, w0, fora));
                if fora == -2 {
                    if livre && sorteio(2) != 0 {
                        lado = Some(true);
                    } else if livre || !(nacao && w0 >= 0 && casa >= 0 && cqb11_mesma_nacao(copa, w0, casa)) {
                        lado = Some(false);
                    }
                } else if livre {
                    lado = Some(true);
                }
            } else if casa < 0 && fora < 0 {
                if fora == -2 {
                    if casa == -2 && sorteio(2) != 0 {
                        crate::comparacao::ramo("cqb_11 semente lado sorteado");
                        lado = Some(true); // cqb11: o lado sorteado
                    } else {
                        lado = Some(false);
                    }
                } else if casa == -2 {
                    lado = Some(true);
                }
            }
            if let Some(m) = lado {
                break m;
            }
            if !cqb11_passo(copa, r, &mut cur, inicio, &mut segunda, &mut nacao) {
                aviso_em(0x9b55ac, 0xcff);
                liberar(vetor);
                return;
            }
        };
        cqb11_colocar(copa, r, rodada, e, cur, mandante);
        j = j.wrapping_add(1);
    }
    liberar(vetor);
}

/// Comparador: sorteia (0x90cfc0) e só usa vetores temporários: lado a lado pelo estado do sorteador e pelos
/// confrontos de todas as rodadas (os vínculos vão às rodadas seguintes), a cópia em sem_contar.
unsafe extern "thiscall" fn comparar_copa_sortear_pela_semente(copa: usize, rodada: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1488);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(cpq11_confrontos(copa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada),
        || crate::aleatorio::sem_contar(|| copa_sortear_pela_semente(copa, rodada)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("copa_sortear_pela_semente(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// { 0x51d9f0 } (thiscall, ret 4, Cup.cpp): `Cup::check_round_teams_for_nation(rodada)`: o sorteio que separa as nações
/// (regra 4 da rodada sem a 0x400). Conta os times de cada nação num vetor zerado de [0xae2350] bytes (0x945501;
/// sem memória, a caixa da linha 0xd27 e o jogo para) enquanto tira os times dos confrontos (malloc de 7 × vagas, a
/// caixa da linha 0xd34), pelo +0 da nação (+0x53) do clube; o nível de cada entrada com time vira a conta da nação
/// dele (sem nação, fica a semente); libera as contas e ordena, a maior primeiro (as nações com mais times saem
/// antes). Cada entrada (até a primeira com 0xff) vai a um confronto sorteado, em círculo: na primeira volta, só
/// confrontos sem time nas duas vagas (com as duas livres, o lado sorteado; só uma livre, ela); da segunda em diante
/// (a marca fica), qualquer vaga livre, menos (enquanto valer a regra, que volta a valer a cada entrada) um confronto
/// com um time da mesma nação do clube da entrada; dando a volta inteira de novo, cai a regra e, sem ela, a caixa da
/// linha 0xe5c e sai. Libera o vetor.
unsafe extern "thiscall" fn copa_sortear_separando_nacoes(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let zerado: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    // calloc: push 1; push [0xae2350]; add esp, 8 -> cdecl (n, 1); free e qsort: add esp, 0x14 cobre os dois -> cdecl
    let contas = zerado(cpq11_d(0xae2350), 1);
    if contas == 0 {
        cpq11_sem_memoria(0xd27, 0x9b5710); // "CUP::check_round_teams_for_nation() : cannot malloc array"
    }
    let vetor = alocar((cpq11_w(cpq11_rodada(copa, r) + 0x18) as i32).wrapping_mul(7) as u32);
    if vetor == 0 {
        cpq11_sem_memoria(0xd34, 0x9b56dc);
    }
    let n = cqb11_tirar_times(copa, r, vetor, |vaga| {
        let clube = cpq11_us(cpq11_time(copa, cpq11_w(vaga)));
        if clube != 0 {
            let nacao = cpq11_us(clube + 0x53);
            if nacao != 0 {
                let c = cpq11_us(nacao).wrapping_add(contas);
                *(c as *mut u8) = cqb11_b(c).wrapping_add(1);
            }
        }
    }) as i16;
    if n > 0 {
        let mut e = vetor;
        for _ in 0..n as i32 {
            if cpq11_w(e) != -1 {
                // ponytail: sem conferir o clube nulo, como a original (que leria o +0x53 dele)
                let nacao = cpq11_us(cpq11_us(cpq11_time(copa, cpq11_w(e))) + 0x53);
                if nacao != 0 {
                    crate::comparacao::ramo("cqb_11 conta da nacao");
                    *((e + 6) as *mut u8) = cqb11_b(cpq11_us(nacao).wrapping_add(contas)); // cqb11: o nível vira a conta da nação
                }
            }
            e += 7;
        }
    }
    liberar(contas);
    ordenar(vetor, n as i32, 7, 0x51ebc0);
    let mut segunda = false;
    let mut j: i16 = 0;
    while j < n {
        let e = cpq11_em(vetor, j as i32, 7);
        let mut regra = true; // volta a valer a cada entrada
        let clube = if cpq11_w(e) != -1 { cpq11_us(cpq11_time(copa, cpq11_w(e))) } else { 0 };
        if cqb11_b(e + 6) == 0xff {
            break;
        }
        let s = sorteio(cpq11_w(cpq11_rodada(copa, r) + 0x1a) as i32);
        let inicio = s as i16;
        let mut cur = s;
        let mandante = loop {
            let t = cpq11_confronto(copa, r, cur as i16 as i32);
            let mut lado: Option<bool> = None;
            if segunda {
                let mut livre = false;
                let mut m = false;
                let casa = cpq11_w(t);
                let c0 = if casa == -2 {
                    m = true;
                    livre = true;
                    0
                } else if casa == -1 {
                    0
                } else {
                    cpq11_us(cpq11_time(copa, casa))
                };
                let fora = cpq11_w(t + 2);
                let c1 = if fora == -2 {
                    if !(livre && sorteio(2) != 0) {
                        m = false;
                        livre = true;
                    }
                    0
                } else if fora == -1 {
                    0
                } else {
                    cpq11_us(cpq11_time(copa, fora))
                };
                let mesma = clube != 0 && regra
                    && ((c0 != 0 && cpq11_d(clube + 0x53) == cpq11_d(c0 + 0x53)) || (c1 != 0 && cpq11_d(clube + 0x53) == cpq11_d(c1 + 0x53)));
                if !mesma && livre {
                    lado = Some(m);
                }
            } else {
                let (casa, fora) = (cpq11_w(t), cpq11_w(t + 2));
                if casa < 0 && fora < 0 {
                    if fora == -2 {
                        lado = Some(casa == -2 && sorteio(2) != 0);
                    } else if casa == -2 {
                        lado = Some(true);
                    }
                }
            }
            if let Some(m) = lado {
                break m;
            }
            if !cqb11_passo(copa, r, &mut cur, inicio, &mut segunda, &mut regra) {
                aviso_em(0x9b55ac, 0xe5c);
                liberar(vetor);
                return;
            }
        };
        cqb11_colocar(copa, r, rodada, e, cur, mandante); // cqb11: a entrada vai ao confronto sorteado
        j = j.wrapping_add(1);
    }
    liberar(vetor);
}

/// Comparador: como o da 0x51d2d0 (sorteia; as contas e o vetor são temporários).
unsafe extern "thiscall" fn comparar_copa_sortear_separando_nacoes(copa: usize, rodada: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1489);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(cpq11_confrontos(copa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada),
        || crate::aleatorio::sem_contar(|| copa_sortear_separando_nacoes(copa, rodada)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("copa_sortear_separando_nacoes(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// { 0x518780 } (thiscall, ret 0, Cup.cpp): `Cup::start_new_season()` da classe base: só zera o byte +0xb1 da copa
/// (as copas derivadas liberam as tabelas delas).
unsafe extern "thiscall" fn copa_comeco_da_temporada(copa: usize) {
    *((copa + 0xb1) as *mut u8) = 0;
}

/// Comparador: lado a lado no byte +0xb1.
unsafe extern "thiscall" fn comparar_copa_comeco_da_temporada(copa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1490);
    let reg = [(copa + 0xb1, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa), || copa_comeco_da_temporada(copa));
    PLACAR.registrar(igual, || format!("copa_comeco_da_temporada({:#x}) {:x?} x {:x?}", copa, mo, mr));
}

// ---------- 264a leva: as nações-chave das competições: a do clube, a do estádio e a entrada da tabela pelo índice ativo (comp.cpp) ----------

/// A entrada `k` (72 bytes) da tabela das 35 nações-chave em 0xb63d60: +0 a nação, +0x14 o índice ativo.
fn cqb12_entrada(k: i8) -> usize {
    0xb63d60 + k as i32 as usize * 72
}

/// O índice `k` da nação `nacao` na tabela das nações-chave (entrada com nação ativa, +0x11c da nação), ou None.
unsafe fn cqb12_achar(nacao: usize) -> Option<i8> {
    let mut k: i8 = 0;
    while k < 0x23 {
        let e = *(cqb12_entrada(k) as *const usize);
        if e != 0 && *((e + 0x11c) as *const u8) != 0 && e == nacao {
            return Some(k);
        }
        k += 1;
    }
    None
}

/// { 0x48dee0 } (cdecl, comp.cpp): `comp_active_key_nation_of_club(clube)`: o índice ativo (+0x14 da entrada) da
/// nação do clube (+0x53) entre as nações-chave ativas; sem nação ou fora da tabela, 0 para uma seleção (0x53b900) e 3
/// para um clube. Devolve só `al` (os chamadores gravam o byte).
unsafe extern "C" fn nacao_chave_do_clube(clube: usize) -> u8 {
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    if nacao != 0 {
        if let Some(k) = cqb12_achar(nacao) {
            crate::comparacao::ramo("cqb_12 nacao chave do clube");
            return *((cqb12_entrada(k) + 0x14) as *const u8); // cqb12: o índice ativo da nação do clube
        }
    }
    // 0x53b900: push clube; add esp, 4 -> cdecl
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) != 0 {
        return 0;
    }
    crate::comparacao::ramo("cqb_12 clube sem nacao chave");
    3 // cqb12: clube fora das nações-chave
}

/// Comparador: rotina pura; compara o byte devolvido.
unsafe extern "C" fn comparar_nacao_chave_do_clube(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(1491)(clube);
    let n = nacao_chave_do_clube(clube);
    PLACAR.registrar(r == n, || format!("nacao_chave_do_clube({:#x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x48df40 } (cdecl, comp.cpp): `comp_active_key_nation_of_stadium(estadio)`: o índice ativo da nação da cidade
/// do estádio (+0x38 a cidade, +0x1f a nação) entre as nações-chave ativas; sem estádio, cidade, nação ou fora da
/// tabela, 3. Devolve só `al`.
unsafe extern "C" fn nacao_chave_do_estadio(estadio: usize) -> u8 {
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    if estadio == 0 || u(estadio + 0x38) == 0 || u(u(estadio + 0x38) + 0x1f) == 0 {
        return 3;
    }
    match cqb12_achar(u(u(estadio + 0x38) + 0x1f)) {
        Some(k) => {
            crate::comparacao::ramo("cqb_12 nacao chave do estadio");
            *((cqb12_entrada(k) + 0x14) as *const u8) // cqb12: o índice ativo da nação do estádio
        }
        None => 3,
    }
}

/// Comparador: rotina pura; compara o byte devolvido.
unsafe extern "C" fn comparar_nacao_chave_do_estadio(estadio: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(1492)(estadio);
    let n = nacao_chave_do_estadio(estadio);
    PLACAR.registrar(r == n, || format!("nacao_chave_do_estadio({:#x}) original={} rust={}", estadio, r, n));
    r
}

/// A entrada da tabela das nações-chave cujo índice ativo (+0x14) é `ativo`, se houver.
unsafe fn cqb12_pelo_ativo(ativo: i8) -> Option<i8> {
    (0..0x23i8).find(|&k| *((cqb12_entrada(k) + 0x14) as *const i8) == ativo)
}

/// { 0x48dcd0 } (cdecl, comp.cpp): `comp_key_nation_entry_from_active_index(ativo)`: a primeira entrada da tabela das
/// nações-chave com aquele índice ativo; nenhuma, a caixa da linha 0x543 de comp.cpp (o jogo segue) e 3.
unsafe extern "C" fn entrada_da_nacao_chave_ativa(ativo: i8) -> i32 {
    match cqb12_pelo_ativo(ativo) {
        Some(k) => k as i32,
        None => {
            aviso_em(0x99b80c, 0x543); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\comp.cpp"
            3
        }
    }
}

/// Comparador: rotina pura; compara a entrada devolvida. Sem entrada (a caixa): só a original.
unsafe extern "C" fn comparar_entrada_da_nacao_chave_ativa(ativo: i8) -> i32 {
    let o = orig::<unsafe extern "C" fn(i8) -> i32>(1493);
    if cqb12_pelo_ativo(ativo).is_none() {
        return o(ativo); // ponytail: a caixa não roda duas vezes
    }
    let r = o(ativo);
    let n = entrada_da_nacao_chave_ativa(ativo);
    PLACAR.registrar(r == n, || format!("entrada_da_nacao_chave_ativa({}) original={} rust={}", ativo, r, n));
    r
}

// ---------- 264a leva: o calendário: o conflito de dois jogos no mesmo dia e os anos de jogos de um clube (fix_man.cpp) ----------

/// Modo de ensaio da 0x5ae380 (o comparador): a cópia para na decisão, antes de mexer no calendário.
static mut CQB13_ENSAIO: bool = false;

unsafe fn cqb13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x5ae380 } (thiscall, ret 8, fix_man.cpp, quadro SEH sem destrutor no caminho normal):
/// `FIXTURE_MANAGER::resolve_same_day_clash(jogo, conflitos)`: o jogo novo `jogo` cai no mesmo dia de outro, o primeiro
/// da lista `conflitos` (o cursor +0xa volta ao começo +2 e 0x68d1b0 o lê); sem um dos dois, a caixa da linha 0xde6
/// e devolve 1. Copia os dois (0x4f bytes) e acha o dia da semana alvo pela data do da lista (+0x2a, +0x28; 0x549ef0,
/// 0x417c10, 0x54a3f0): os dias 0 e 5 da 0x54a3f0 vão a 6, 1 e 3 a 2, o 2 a 1, 4 e 6 a 5 (0x54a2d0 dá a
/// data). A prioridade de cada um é a da competição (+0x69 da +0x14), +0x15 com seleção
/// como mandante (+0x1c, 0x53b900); o da lista sai do dia quando a do novo é maior (ou igual, com a reputação +0x80
/// do mandante do novo maior) e ele ainda não foi jogado (+0x43 = 0xff): tirado do calendário (0x5abc30), o novo entra
/// (0x5aa680); senão sai o novo. O que sai ganha a data nova; com conflito nela (0x5ad780, a lista destruída e
/// liberada: 0x68d040, 0x944c94), jogo fixo (+0x3a = 1) devolve 0, senão 0x5ab510 acha outra data; entra no calendário
/// (0x5aa680) com a notícia da remarcação (0x5aac70, causada pelo outro). Devolve 1.
unsafe extern "thiscall" fn conflito_de_jogos_no_mesmo_dia(this: usize, jogo: usize, conflitos: usize) -> u32 {
    // as datas: ecx = a data, sem pushes -> thiscall (0x54a110, 0x54a3f0); 0x549ef0: push [+0x28]; push [+0x2a];
    // 0x417c10: push ret; push &tmp; 0x54a2d0: push &b; push alvo -> thiscall
    let padrao: unsafe extern "thiscall" fn(*mut [u8; 8]) -> usize = core::mem::transmute(0x54a110usize);
    let nova: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> *mut [u8; 8] = core::mem::transmute(0x549ef0usize);
    let copia: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], *mut [u8; 8]) -> usize = core::mem::transmute(0x417c10usize);
    let semana: unsafe extern "thiscall" fn(*mut [u8; 8]) -> i32 = core::mem::transmute(0x54a3f0usize);
    let ajustar: unsafe extern "thiscall" fn(*mut [u8; 8], u32, *mut [u8; 8]) -> u32 = core::mem::transmute(0x54a2d0usize);
    // 0x68d1b0: ecx = a lista, sem pushes -> thiscall; 0x53b900: push clube; add esp, 4 -> cdecl
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let (mut a, mut b) = ([0u8; 8], [0u8; 8]);
    padrao(&mut a);
    padrao(&mut b);
    let mut gravidade: u32 = 0; // [esp+0x10]: o alvo sem caso da tabela e a gravidade da 0x5ad780
    core::ptr::write_unaligned((conflitos + 0xa) as *mut usize, cqb13_d(conflitos + 2));
    let outro = proximo(conflitos);
    if jogo == 0 || outro == 0 {
        if !CQB13_ENSAIO {
            aviso_em(0x9cc208, 0xde6); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\fix_man.cpp"
        }
        return 1;
    }
    let (mut novo, mut velho) = ([0u8; 0x50], [0u8; 0x50]);
    core::ptr::copy_nonoverlapping(jogo as *const u8, novo.as_mut_ptr(), 0x4f);
    core::ptr::copy_nonoverlapping(outro as *const u8, velho.as_mut_ptr(), 0x4f);
    let (mut tmp, mut tmp2) = ([0u8; 8], [0u8; 8]);
    // ponytail: os bytes de cima dos dois pushes de 16 bits não contam (0x549ef0 lê a palavra)
    let r = nova(&mut tmp, cqb13_w(outro + 0x2a) as u16 as u32, cqb13_w(outro + 0x28) as u16 as u32);
    copia(&mut a, &mut tmp2, r);
    let alvo = match semana(&mut a) as i16 {
        0 | 5 => 6,
        1 | 3 => 2,
        2 => 1,
        4 | 6 => 5,
        _ => gravidade,
    };
    crate::comparacao::ramo("cqb_13 dia alvo");
    ajustar(&mut a, alvo, &mut b); // cqb13: o dia da semana alvo
    let n = novo.as_mut_ptr() as usize;
    let v = velho.as_mut_ptr() as usize;
    let mut pn = cqb13_w(cqb13_d(n + 0x14) + 0x69);
    let mut pv = cqb13_w(cqb13_d(v + 0x14) + 0x69);
    if selecao(cqb13_d(n + 0x1c)) != 0 {
        pn = pn.wrapping_add(0x15);
    }
    if selecao(cqb13_d(v + 0x1c)) != 0 {
        pv = pv.wrapping_add(0x15);
    }
    let novo_vence = pn > pv // cqb13: a prioridade maior fica
        || (pn == pv && cqb13_w(cqb13_d(n + 0x1c) + 0x80) > cqb13_w(cqb13_d(v + 0x1c) + 0x80));
    if novo_vence {
        crate::comparacao::ramo("cqb_13 novo vence");
    }
    let sai_o_velho = novo_vence && *((v + 0x43) as *const u8) == 0xff; // cqb13: o da lista ainda não jogado
    if CQB13_ENSAIO {
        return 1;
    }
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5abc30usize);
    let juntar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
    let conflito: unsafe extern "thiscall" fn(usize, usize, *mut u32, u32) -> usize = core::mem::transmute(0x5ad780usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let outra_data: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5ab510usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x5aac70usize);
    // as do calendário: ecx = this; push o jogo (0x5abc30, 0x5ab510; ret 4), push 0; push o jogo (0x5aa680; ret 8),
    // push 0; push &gravidade; push o jogo (0x5ad780; ret 0xc), push 0; push causa; push o jogo (0x5aac70; ret 0xc);
    // 0x68d040: ecx = a lista; 0x944c94: push lista; add esp, 4 -> cdecl
    let (sai, causa) = if sai_o_velho {
        crate::comparacao::ramo("cqb_13 sai o jogo da lista");
        tirar(this, outro);
        juntar(this, jogo, 0);
        (v, jogo)
    } else {
        (n, outro)
    };
    core::ptr::write_unaligned((sai + 0x2a) as *mut u16, u16::from_le_bytes([b[0], b[1]]));
    core::ptr::write_unaligned((sai + 0x28) as *mut u16, u16::from_le_bytes([b[2], b[3]]));
    let lista = conflito(this, sai, &mut gravidade, 0);
    if lista != 0 {
        destruir(lista);
        liberar(lista);
        if *((sai + 0x3a) as *const u8) == 1 {
            return 0;
        }
        outra_data(this, sai);
    }
    juntar(this, sai, 0);
    noticia(this, sai, causa, 0);
    1
}

/// Comparador em ensaio: a rotina só grava o cursor da lista (+0xa, lido pela 0x68d1b0) e o resto vai pelas
/// chamadas do calendário (tirar, juntar, conflitos que alocam a lista, a notícia), que não podem rodar duas vezes.
/// A cópia roda em ensaio (para na decisão, sem a caixa) e a original depois, a partir do mesmo estado: lado a lado no
/// cabeçalho da lista (+2 começo, +0xa cursor). Sem o jogo ou o primeiro da lista: só a original (a caixa).
unsafe extern "thiscall" fn comparar_conflito_de_jogos_no_mesmo_dia(this: usize, jogo: usize, conflitos: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1494);
    let cabeca = cqb13_d(conflitos + 2);
    if jogo == 0 || cabeca == 0 || cqb13_d(cabeca) == 0 {
        return o(this, jogo, conflitos);
    }
    let reg = [(conflitos + 2, 0xc)];
    let antes = crate::comparacao::Memoria::ler(&reg);
    CQB13_ENSAIO = true;
    conflito_de_jogos_no_mesmo_dia(this, jogo, conflitos);
    CQB13_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    let r = o(this, jogo, conflitos);
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr && (r == 0 || r == 1), || format!("conflito_de_jogos_no_mesmo_dia(jogo {:#x}) devolveu {}, cursor da lista {:x?} x {:x?}", jogo, r, mo, mr));
    r
}

/// { 0x5aee40 } (thiscall, ret 4, fix_man.cpp; o this não é lido): `FIXTURE_MANAGER::get_club_first_fixture_year(&id)`:
/// o primeiro ano com jogos do clube `[id]`: 2001, ou 2002 com o bit 0 da palavra dele em [0xae2a5c]. Devolve `ax`
/// (o chamador da tela compara 16 bits).
unsafe extern "thiscall" fn primeiro_ano_com_jogos(_this: usize, id: usize) -> u16 {
    let b = *((cqb13_d(0xae2a5c) as isize).wrapping_add((*(id as *const i32) as isize).wrapping_mul(2)) as *const u8);
    0x7d1 + (b & 1) as u16
}

/// { 0x5aee60 } (thiscall, ret 4, fix_man.cpp; o this não é lido): `FIXTURE_MANAGER::get_club_last_fixture_year(&id)`:
/// o último ano com jogos do clube: a palavra dele em [0xae2a5c] sem o bit 0 (metade); sem ano, o ano corrente
/// [0xae2c92]. Devolve `ax` (o chamador da tela guarda 16 bits).
unsafe extern "thiscall" fn ultimo_ano_com_jogos(_this: usize, id: usize) -> u16 {
    let w = core::ptr::read_unaligned((cqb13_d(0xae2a5c) as isize).wrapping_add((*(id as *const i32) as isize).wrapping_mul(2)) as *const u16);
    if w & 0xfffe != 0 { w >> 1 } else { *(0xae2c92 as *const u16) }
}

comparar_valor!(comparar_primeiro_ano_com_jogos, primeiro_ano_com_jogos, 1495, (id: usize) -> u16);
comparar_valor!(comparar_ultimo_ano_com_jogos, ultimo_ano_com_jogos, 1496, (id: usize) -> u16);
