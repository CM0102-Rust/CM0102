// ---------- %%LEVA%%a leva: os eventos raros das finanças: o impulso das finanças, a troca do presidente e os outros custos (fin_prm.cpp) ----------

unsafe fn cqb2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O byte com sinal em `a`.
unsafe fn cqb2_c(a: usize) -> i8 {
    *(a as *const i8)
}

/// O saldo (i64 em +0) separado em (baixo, alto), como a original o lê em dois dwords.
unsafe fn balance_halves(fin: usize) -> (u32, i32) {
    (core::ptr::read_unaligned(fin as *const u32), core::ptr::read_unaligned((fin + 4) as *const i32))
}

/// O valor entra em +0x154, +0xb4 e no saldo (estendido com sinal).
unsafe fn credit_income(fin: usize, v: i32) {
    cpv6_somar(fin + 0x154, v);
    cpv6_somar(fin + 0xb4, v);
    credit_balance(fin, v);
}

/// A nação do clube é uma das de 0x9cf4a8, 0x9cf408 ou 0x9cf2c0 (o valor do evento cai a um terço).
unsafe fn is_scandinavian_club(clube: usize) -> bool {
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
unsafe extern "thiscall" fn try_finance_boost_event(this: usize, clube: usize, fin: usize) -> u32 {
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
    let div = match wealth_band(fin) {
        -2 => read_f64_at(if rep > 0xea6 { 0x968fd0 } else { 0x96a730 }),
        -1 => read_f64_at(if rep > 0x1482 { 0x967880 } else if rep > 0xcb2 { 0x96a730 } else { 0x967878 }),
        0 => {
            if rep <= 0x109a {
                let ficha = if pres != 0 { cqb2_d(pres + 0x69) } else { 0 };
                if forte(pres, ficha) && cqb2_c(pres + 0x57) > 0xf && cqb2_c(ficha + 0x20) > 0xf {
                    read_f64_at(0x96a740)
                } else {
                    crate::comparison::branch("cqb_2 divisor da band 0 baixa");
                    read_f64_at(0x96a738) // cqb2: o divisor da faixa 0 de reputação baixa
                }
            } else if rep < 0x186a {
                let ficha = if pres != 0 { cqb2_d(pres + 0x69) } else { 0 };
                if forte(pres, ficha) && cqb2_c(ficha + 0x20) > 0xf && cqb2_c(pres + 0x57) > 0xf {
                    read_f64_at(0x967888)
                } else {
                    read_f64_at(0x967878)
                }
            } else {
                read_f64_at(0x967878)
            }
        }
        _ => read_f64_at(0x96a350),
    };
    let ficha = if pres != 0 { cqb2_d(pres + 0x69) } else { 0 };
    let valor;
    if pres != 0 && ficha != 0 {
        crate::comparison::branch("cqb_2 com a ficha");
        let r1 = cpv6_sortear(match_pl_ftol(read_f64_at(0x968dd8) / div)); // cqb2: o sorteio com a ficha
        let r2 = cpv6_sortear(cqb2_c(ficha + 0xf) as i32);
        if r2 <= r1 {
            return 0;
        }
        let (lo, hi) = balance_halves(fin);
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
        let mut v = match_pl_ftol(((n as f64 * n as f64) * read_f64_at(0x968930) + read_f64_at(0x96a5f0)) * rep as f64 + read_f64_at(0x9688d8));
        v = if hi < 0 { v.wrapping_sub(lo as i32) } else { v.wrapping_add(lo as i32) };
        if cpv6_w(clube + 0x80) < 0x7d0 {
            let teto = rep.wrapping_mul(rep).wrapping_add(0x16e360);
            if v > teto {
                v = teto;
            }
        }
        if is_scandinavian_club(clube) {
            v /= 3;
        }
        valor = v;
    } else {
        crate::comparison::branch("cqb_2 sem a ficha");
        let r = cpv6_sortear(match_pl_ftol(read_f64_at(0x96a728) / div)); // cqb2: o sorteio sem a ficha
        if rep as i32 <= r {
            return 0;
        }
        if cpv6_sortear(8) != 0 {
            return 0;
        }
        let mut v = (cpv6_w(clube + 0x80) as i32).wrapping_add(1000).wrapping_mul(1000);
        if is_scandinavian_club(clube) {
            v /= 3;
        }
        valor = v;
    }
    credit_income(fin, valor);
    let verba = transfer_budget(fin) as i32;
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
                let t = match_pl_ftol(r2.wrapping_mul(r2) as f64 * read_f64_at(0x968e80));
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
            post_finance_boost_news(this, clube, valor, verba as u32, 7);
            *((clube + 0x82) as *mut u8) = 1;
            tipo = 7;
        } else {
            tipo = 1;
        }
    }
    if tipo != 7 {
        post_finance_boost_news(this, clube, valor, verba as u32, tipo);
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

/// O comparador dos eventos raros, em ensaio (modelo RARE_FINANCE_EVENT_DRY_RUN: a cópia sem a notícia nem a verba, que alocam; sem a
/// troca do presidente na 0x59e940): a cópia e a original a partir do mesmo estado (o registro de finanças, o sorteio e
/// os `extras`). Sem evento: tudo igual; com evento: a devolução, o saldo, +0xb4, +0x154, +0x165 e os `conferir` (a
/// tipo da notícia e a marca +0x82 dependem da verba, e os sorteios depois dela também).
unsafe fn compare_rare_finance_event(nome: &str, fin: usize, extras: &[(usize, usize)], conferir: &[usize], original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let mut regioes = vec![(fin, 0x167)];
    regioes.extend(crate::rng::STATE_REGIONS);
    regioes.extend_from_slice(extras);
    let antes = crate::comparison::MemorySnapshot::read(&regioes);
    RARE_FINANCE_EVENT_DRY_RUN = true;
    let n = crate::rng::pause_trace(|| crate::rng::without_counting(copia));
    RARE_FINANCE_EVENT_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regioes);
    antes.write_back();
    let r = original();
    let mo = crate::comparison::MemorySnapshot::read(&regioes);
    let igual = if r == 0 && n == 0 {
        mo == mr
    } else {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let campos = r == n && a[..8] == b[..8] && a[0xb4..0xb8] == b[0xb4..0xb8] && a[0x154..0x158] == b[0x154..0x158] && a[0x165] == b[0x165];
        campos && conferir.iter().all(|&k| mo.0[3 + k].1 == mr.0[3 + k].1)
    };
    SCOREBOARD.record(igual, || format!("{}(fin {:#x}) original={} {:x?} rust={} {:x?}", nome, fin, r, mo, n, mr));
    r
}

unsafe extern "thiscall" fn compare_try_finance_boost_event(this: usize, clube: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%);
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
    compare_rare_finance_event("try_finance_boost_event", fin, &extras, &conferir, || o(this, clube, fin), || try_finance_boost_event(this, clube, fin))
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
unsafe extern "thiscall" fn try_board_takeover(this: usize, clube: usize, fin: usize) -> u32 {
    let rep = cpv6_w(clube + 0x80);
    let div = match wealth_band(fin) {
        -2 => if rep > 0x109a { 0.4f64 } else { 0.25 },
        -1 => if rep > 0x109a { 0.2 } else { 0.05 },
        _ => 0.001,
    };
    let liga = cqb2_d(clube + 0x57);
    let mut fator = if liga != 0 {
        let k = rep as i32 as f64 * read_f64_at(0x969048) / cpv6_w(liga + 0x69) as i32 as f64;
        if !(k >= read_f64_at(0x969040)) { read_f64_at(0x969040) } else { k } // fcomp: menor ou sem ordem (NaN) vira 0,75
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
    let r = cpv6_sortear(match_pl_ftol(read_f64_at(0x96a728) / div));
    if r as f64 >= fator {
        return 0;
    }
    crate::comparison::branch("cqb_2 troca do presidente");
    let (lo, _) = balance_halves(fin);
    let mut v = (cpv6_w(clube + 0x80) as i32).wrapping_add(1000).wrapping_mul(1000).wrapping_sub(lo as i32);
    if v > 0x2625a00 {
        let s = cpv6_sortear(0x9c4);
        if (cpv6_w(clube + 0x80) as i32) < s.wrapping_add(0x7d0) {
            v = (balance_halves(fin).0 as i32).wrapping_neg();
        }
    }
    cpv6_somar(fin + 0xb4, v);
    cpv6_somar(fin + 0x154, v);
    credit_balance(fin, v);
    let verba = transfer_budget(fin);
    if com_ficha && !RARE_FINANCE_EVENT_DRY_RUN {
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
    post_finance_boost_news(this, clube, v, verba, 2);
    *((fin + 0x165) as *mut u8) = 0;
    1
}

unsafe extern "thiscall" fn compare_try_board_takeover(this: usize, clube: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%);
    if clube == 0 || fin == 0 {
        return o(this, clube, fin); // ponytail: sem clube ou sem finanças, só a original
    }
    compare_rare_finance_event("try_board_takeover", fin, &[], &[], || o(this, clube, fin), || try_board_takeover(this, clube, fin))
}

/// { 0x5999e0 } (thiscall, ret 4, fin_prm.cpp): `Finance::pay_other_costs(valor)`, pela 0x7e5b10: soma o valor aos
/// outros custos do ano (+0x120) e do mês (+0x80) e o tira do saldo.
unsafe extern "thiscall" fn pay_other_costs(this: usize, valor: i32) {
    cpv6_somar(this + 0x120, valor);
    cpv6_somar(this + 0x80, valor);
    add_to_balance(this, -(valor as i64));
}

/// Comparador da 0x5999e0: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn compare_pay_other_costs(this: usize, valor: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(%%I2%%);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, valor), || pay_other_costs(this, valor));
    SCOREBOARD.record(igual, || format!("pay_other_costs(valor {}) original={:x?} rust={:x?}", valor, mo, mr));
}
