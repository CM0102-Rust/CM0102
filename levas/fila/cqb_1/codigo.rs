// ---------- %%LEVA%%a leva: a renda das partidas: a renda da partida nas finanças dos dois clubes, a renda do estádio e a multa da liga (fin_prm.cpp) ----------

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
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(tabela, partida, resultado); // a caixa
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
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
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
        return orig::<unsafe extern "thiscall" fn(usize, usize, i32) -> i32>(%%I1%%)(tabela, partida, publico); // a caixa
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
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i32) -> i32>(%%I1%%);
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
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(%%I2%%);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pagar_multa_da_liga(this, valor));
    PLACAR.registrar(igual, || format!("pagar_multa_da_liga(valor {}) original={:x?} rust={:x?}", valor, mo, mr));
}
