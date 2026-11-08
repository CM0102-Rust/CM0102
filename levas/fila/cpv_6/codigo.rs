// ---------- %%LEVA%%a leva: os eventos financeiros da semana: o passe dos eventos do clube, a diretoria que paga a dívida e a injeção de dinheiro (finance.cpp) ----------

unsafe fn cpv6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv6_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpv6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpv6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpv6_somar(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, cpv6_i(a).wrapping_add(v));
}

unsafe fn balance(fin: usize) -> i64 {
    core::ptr::read_unaligned(fin as *const i64)
}

/// O saldo (i64 em +0) mais `v` estendido com sinal (o cdq/adc da original).
unsafe fn credit_balance(fin: usize, v: i32) {
    core::ptr::write_unaligned(fin as *mut i64, balance(fin).wrapping_add(v as i64));
}

unsafe fn cpv6_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

unsafe fn wealth_band(fin: usize) -> i8 {
    let f: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    f(fin) as i8
}

/// Um evento da semana (0x59e050, 0x59dd40, 0x59e5a0, 0x59e940): thiscall(tabela, clube, finanças), 1 se aconteceu.
unsafe fn run_weekly_event(rotina: usize, this: usize, clube: usize, fin: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(rotina);
    f(this, clube, fin)
}

/// O ensaio do comparador: a cópia roda sem a notícia (0x59f300) e sem a verba (0x598800), que alocam e não podem
/// rodar duas vezes.
static mut CPV6_ENSAIO: bool = false;

/// A notícia do evento (0x59f300: thiscall(tabela, clube, valor, verba, tipo), ret 0x10); no ensaio, nada.
unsafe fn post_finance_boost_news(this: usize, clube: usize, valor: i32, verba: u32, tipo: u32) {
    if CPV6_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) = core::mem::transmute(0x59f300usize);
    f(this, clube, valor, verba, tipo);
}

/// A verba de transferências (Finance::get_transfer_budget 0x598800 com 0, 0, 0, 1, 0) que vai na notícia; no
/// ensaio, 0 (a notícia não sai).
unsafe fn transfer_budget(fin: usize) -> u32 {
    if CPV6_ENSAIO {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x598800usize);
    f(fin, 0, 0, 0, 1, 0)
}

/// O fim da semana do clube no estado de crise (0x5a02f0: thiscall(tabela, id)).
unsafe fn post_receivers_appeased(this: usize, clube: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x5a02f0usize);
    f(this, cpv6_d(clube) as u32);
}

/// O estado (+0x165) volta a 0; se era 1 (crise), a notícia do fim da crise (0x5a02f0) antes.
unsafe fn leave_crisis(this: usize, clube: usize, fin: usize) {
    let s = cpv6_b(fin + 0x165);
    if s == 1 {
        post_receivers_appeased(this, clube);
    }
    if s == 1 || s == 2 {
        *((fin + 0x165) as *mut u8) = 0;
    }
}

/// O centro de treino no máximo (20) cai um nível com sorteio(10) zero, se a reputação for menor que 7250.
unsafe fn decay_training_ground(clube: usize) {
    let nivel = (clube + 0x7f) as *mut i8;
    if *nivel == 0x14 && cpv6_w(clube + 0x80) < 0x1c52 && cpv6_sortear(10) == 0 {
        *nivel = (*nivel).wrapping_sub(1);
    }
}

/// { 0x59ed70 } (thiscall, ret 8, finance.cpp): `FinanceTable::process_club_finance_events(clube, semana)`, pelo passe
/// diário das finanças (0x59b9c0: na virada da semana com semana = 1 para todo clube de liga com reputação de 5000 ou
/// nação ativa; nos outros dias só para o clube em crise, com semana = sorteio zero). As finanças do clube (do parceiro
/// A, 0x540a50) e a faixa (0x5986b0): 2 e 1 (ricos) e 0 encerram a crise (+0x165) e baixam o contador de aperto (+0x166)
/// até 5/0/10; 1 e 0 tentam o impulso (0x59e050), a dívida (0x59dd40) e a injeção (0x59e5a0); −1 marca o estado 2, sobe
/// o aperto pelo byte +0x6d, desgasta o centro de treino e tenta a dívida, a injeção e a troca da diretoria (0x59e940);
/// −2 entra em crise (notícia 0x59fc60, tira a marca +0x82, e no clube humano 0x600a50 baixa o valor dos contratos do
/// elenco pela 0x4d0c60, marcando o bit 4 de +0x4e) ou, já em crise, tenta a troca, a injeção e o impulso, desgasta o
/// centro e sobe o aperto 3. Depois, liga ativa ([0xadadfc]) e aperto acima de 50: tira o aquecimento (+0x4d, 0x59ffb0)
/// do clube de reputação abaixo de 4250, ou fecha 10% dos lugares (+0x3c para +0x44, 0x59f900, aperto 20); o centro de
/// treino cai pelo sorteio.
unsafe extern "thiscall" fn process_club_finance_events(this: usize, clube: usize, semana: u32) {
    if clube == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, clube, semana);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut eh_a: u32 = 0;
    let p = parceiro(clube, &mut eh_a, 1);
    let c = if p != 0 && eh_a == 0 { p } else { clube };
    let fin = cpv6_d(this).wrapping_add((cpv6_d(c) as u32).wrapping_mul(0x167) as usize);
    let aperto = (fin + 0x166) as *mut i8;
    match wealth_band(fin) {
        -1 => {
            let s = cpv6_b(fin + 0x165);
            if s != 2 {
                if s == 1 {
                    post_receivers_appeased(this, clube);
                }
                *((fin + 0x165) as *mut u8) = 2;
            }
            if semana == 0 {
                return;
            }
            if cpv6_b(clube + 0x6d) != 0 {
                *aperto = (*aperto).wrapping_add(1);
            }
            decay_training_ground(clube);
            if run_weekly_event(0x59dd40, this, clube, fin) == 0 && run_weekly_event(0x59e5a0, this, clube, fin) == 0 {
                run_weekly_event(0x59e940, this, clube, fin);
            }
        }
        -2 => {
            if cpv6_b(fin + 0x165) != 1 {
                let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x59fc60usize);
                let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
                let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
                let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
                let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
                let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
                noticia(this, clube);
                if cpv6_b(clube + 0x82) != 0 {
                    *((clube + 0x82) as *mut u8) = 0;
                }
                *((fin + 0x165) as *mut u8) = 1;
                if humano(clube, 1, 1, 0, 0) == 0 {
                    return;
                }
                let mut elenco = [0usize; 64];
                reunir(clube, elenco.as_mut_ptr(), 0);
                for &j in elenco.iter().take(50) {
                    if j == 0 || cpv6_d(j + 0x61) == 0 {
                        continue;
                    }
                    let g = cpv6_d(0xae1050);
                    let k = contract_of(g, j);
                    *(0xae1080 as *mut usize) = k;
                    let dono = if k != 0 { cpv6_d(0xae23b4).wrapping_add(cpv6_d(k + 4).wrapping_mul(0x245)) } else { 0 };
                    let ct = contrato(cpv6_d(0xae1050), j, dono);
                    if dono == clube && ct != 0 {
                        let v = valor(cpv6_d(0xae1050), j, dono, 0, -1, 1);
                        if v < cpv6_i(j + 0x52) {
                            *((ct + 0x4e) as *mut u8) |= 4;
                            core::ptr::write_unaligned((j + 0x52) as *mut i32, v);
                        }
                    }
                }
                return;
            }
            if semana == 0 {
                return;
            }
            if run_weekly_event(0x59e940, this, clube, fin) == 0 && run_weekly_event(0x59e5a0, this, clube, fin) == 0 {
                run_weekly_event(0x59e050, this, clube, fin);
            }
            decay_training_ground(clube);
            if cpv6_b(clube + 0x6d) != 0 {
                *aperto = (*aperto).wrapping_add(3);
            }
        }
        1 => {
            leave_crisis(this, clube, fin);
            if semana == 0 {
                return;
            }
            if *aperto > 0 {
                *aperto -= 1;
            }
            if run_weekly_event(0x59e050, this, clube, fin) == 0 && run_weekly_event(0x59dd40, this, clube, fin) == 0 {
                run_weekly_event(0x59e5a0, this, clube, fin);
            }
        }
        2 => {
            leave_crisis(this, clube, fin);
            if semana == 0 {
                return;
            }
            if *aperto > 5 {
                *aperto -= 1;
            }
        }
        _ => {
            leave_crisis(this, clube, fin);
            if semana == 0 {
                return;
            }
            if *aperto > 10 {
                *aperto -= 1;
            }
            crate::comparacao::ramo("cpv_6 impulso na faixa 0");
            let ok = run_weekly_event(0x59e050, this, clube, fin) != 0; // cpv6: o impulso na faixa 0
            if !ok && run_weekly_event(0x59dd40, this, clube, fin) == 0 {
                run_weekly_event(0x59e5a0, this, clube, fin);
            }
        }
    }
    // a crise prolongada
    let liga = cpv6_d(clube + 0x57);
    if liga == 0 || cpv6_i(cpv6_d(0xadadfc) + cpv6_d(liga).wrapping_mul(4)) == 0 || *aperto <= 0x32 {
        return;
    }
    let est = cpv6_d(clube + 0x69);
    if est == 0 {
        return;
    }
    let nivel = (clube + 0x7f) as *mut i8;
    if cpv6_b(est + 0x4d) != 0 && cpv6_w(clube + 0x80) < 0x109a {
        let aquecimento: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x59ffb0usize);
        aquecimento(this, clube, 0, 7);
        *((cpv6_d(clube + 0x69) + 0x4d) as *mut u8) = 0;
        let n = *nivel;
        if n == 0x14 {
            let r = cpv6_sortear(4) as u8;
            *nivel = (*nivel).wrapping_add(0xffu8.wrapping_sub(r) as i8);
            return;
        }
        if n <= 1 || cpv6_sortear(n as i32) == 0 {
            return;
        }
        *nivel = (*nivel).wrapping_sub(1);
        return;
    }
    let lugares = cpv6_i(est + 0x3c);
    if lugares <= 0xabe || cpv6_sortear(cpv6_w(clube + 0x80) as i32) >= lugares {
        return;
    }
    let t = match_pl_ftol(lugares as f64 * read_f64_at(0x967880)) as i16 as i32;
    let dez = (t / 10).wrapping_mul(10);
    let fechar = dez as i16 as i32;
    let resto = cpv6_i(est + 0x40);
    let tirou = if resto > fechar {
        core::ptr::write_unaligned((est + 0x40) as *mut i32, resto - fechar);
        1
    } else {
        0
    };
    let fechados: unsafe extern "thiscall" fn(usize, usize, i32, u32) = core::mem::transmute(0x59f900usize);
    fechados(this, clube, dez, tirou);
    let est = cpv6_d(clube + 0x69);
    cpv6_somar(est + 0x3c, fechar.wrapping_neg());
    let est = cpv6_d(clube + 0x69);
    cpv6_somar(est + 0x44, fechar);
    *aperto = 0x14;
    let n = *nivel;
    if n > 0xf {
        let r = cpv6_sortear(5) as u8;
        *nivel = (*nivel).wrapping_add(0xffu8.wrapping_sub(r) as i8);
        return;
    }
    if n <= 7 && (n <= 1 || cpv6_sortear(n as i32) == 0) {
        return;
    }
    *nivel = (*nivel).wrapping_sub(1);
}

// ponytail: chama originais que sorteiam, alocam notícias e mexem nos contratos: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_process_club_finance_events(this: usize, clube: usize, semana: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, clube, semana)
}

/// { 0x59dd40 } (thiscall, ret 8, finance.cpp): `FinanceTable::try_board_debt_payment(clube, finanças)`, pelo passe dos
/// eventos (0x59ed70). Faixa (0x5986b0) −2, −1 (divisor 2, ou 0,4 com reputação de 3500), ou 0 com reputação até 3500 e
/// saldo abaixo de 25 × reputação (0,4); as outras: 0. Sorteio(7000) + 3500 acima da reputação; sorteio(6 / divisor)
/// zero; sorteio(nível +0x20 do estádio do clube de +0xbf, ou 10) não zero. A diretoria paga 450 × reputação (abaixo de
/// 1250 com saldo abaixo de −450 × reputação), 300 × (saldo abaixo de −300 × reputação) ou 150 ×: em +0x154, +0xb4 e no
/// saldo; a notícia (0x59f300, tipo 4); devolve 1.
unsafe extern "thiscall" fn try_board_debt_payment(this: usize, clube: usize, fin: usize) -> u32 {
    let rep = cpv6_w(clube + 0x80) as i32;
    let div = match wealth_band(fin) {
        -2 => 2.0f64,
        -1 => {
            if rep >= 0xdac {
                0.4
            } else {
                2.0
            }
        }
        0 => {
            if rep > 0xdac || balance(fin) >= rep.wrapping_mul(25) as i64 {
                return 0;
            }
            0.4
        }
        _ => return 0,
    };
    crate::comparacao::ramo("cpv_6 sorteio da divida");
    let r = cpv6_sortear(7000); // cpv6: o sorteio da dívida
    if rep >= r.wrapping_add(0xdac) {
        return 0;
    }
    let n = match_pl_ftol(read_f64_at(0x969000) / div);
    let outro = cpv6_d(clube + 0xbf);
    if outro != 0 && cpv6_d(outro + 0x69) != 0 {
        if cpv6_sortear(n) != 0 {
            return 0;
        }
        let est = cpv6_d(cpv6_d(clube + 0xbf) + 0x69);
        if cpv6_sortear(cpv6_b(est + 0x20) as i8 as i32) == 0 {
            return 0;
        }
    } else {
        if cpv6_sortear(n) != 0 {
            return 0;
        }
        if cpv6_sortear(10) == 0 {
            return 0;
        }
    }
    let rep = cpv6_w(clube + 0x80) as i32;
    let saldo = balance(fin);
    let valor = if rep < 0x4e2 && saldo < rep.wrapping_mul(-450) as i64 {
        rep.wrapping_mul(450)
    } else if saldo >= rep.wrapping_mul(-300) as i64 {
        rep.wrapping_mul(150)
    } else {
        rep.wrapping_mul(300)
    };
    cpv6_somar(fin + 0x154, valor);
    cpv6_somar(fin + 0xb4, valor);
    credit_balance(fin, valor);
    post_finance_boost_news(this, clube, valor, 0, 4);
    1
}

/// O comparador dos eventos: a cópia em ensaio (sem notícia nem verba) e a original a partir do mesmo estado (registro de
/// finanças e sorteio). Sem evento: tudo igual, inclusive o sorteio; com evento: a devolução e os campos que o evento
/// grava (saldo, +0xb4, +0x154, +0x165), porque a notícia e a verba da original podem sortear e gravar.
unsafe fn compare_finance_event(nome: &str, fin: usize, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let mut regioes = vec![(fin, 0x167)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
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
        r == n && a[..8] == b[..8] && a[0xb4..0xb8] == b[0xb4..0xb8] && a[0x154..0x158] == b[0x154..0x158] && a[0x165] == b[0x165]
    };
    PLACAR.registrar(igual, || format!("{}(fin {:#x}) original={} {:x?} rust={} {:x?}", nome, fin, r, mo, n, mr));
    r
}

unsafe extern "thiscall" fn comparar_try_board_debt_payment(this: usize, clube: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%);
    compare_finance_event("try_board_debt_payment", fin, || o(this, clube, fin), || try_board_debt_payment(this, clube, fin))
}

/// { 0x59e5a0 } (thiscall, ret 8, finance.cpp): `FinanceTable::try_cash_injection(clube, finanças)`, pelo passe dos
/// eventos (0x59ed70). Reputação (+0x80) de 4750, sem a marca +0x82, fora do País de Gales e das Irlandas. Divisor pela
/// faixa (−2: 0,25; −1: 0,1; outras: 0,001). Com o estádio do clube de +0xbf: sorteio(30 / divisor) abaixo do nível
/// (+0xf) e o nível +0x20 acima de sorteio(5) + 10: (20 × nível + 150) × reputação + 750000; sem ele: sorteio(4 /
/// divisor) zero: (5 × reputação + 15000) × 50. Dívida acima de 10 × o valor: 10% da dívida. Teto: reputação² + 1,5
/// milhão abaixo de 2000; senão 4 milhões (+1 se o valor pass_ball de reputação² / 4). Em +0x154, +0xb4 e no saldo; a
/// notícia (0x59f300, tipo 3, com a verba 0x598800); encerra a crise (+0x165 = 0); devolve 1.
unsafe extern "thiscall" fn try_cash_injection(this: usize, clube: usize, fin: usize) -> u32 {
    if cpv6_w(clube + 0x80) < 0x128e { // cpv6: o filtro da injeção
        return 0;
    }
    if cpv6_b(clube + 0x82) != 0 {
        return 0;
    }
    if clube != 0 {
        let nac = cpv6_d(clube + 0x53);
        if nac != 0 {
            let n = cpv6_i(nac);
            if n == cpv6_i(0x9cf510) || n == cpv6_i(0x9cf2dc) || n == cpv6_i(0x9cf3e4) {
                return 0;
            }
        }
    }
    crate::comparacao::ramo("cpv_6 injecao avaliada");
    let div = match wealth_band(fin) {
        -2 => read_f64_at(0x9689e0),
        -1 => read_f64_at(0x967880),
        _ => read_f64_at(0x96a350),
    };
    let rep = cpv6_w(clube + 0x80) as i32;
    let outro = cpv6_d(clube + 0xbf);
    let mut valor;
    if outro != 0 && cpv6_d(outro + 0x69) != 0 && cpv6_b(clube + 0x82) == 0 {
        let r = cpv6_sortear(match_pl_ftol(read_f64_at(0x968dd8) / div));
        let est = cpv6_d(cpv6_d(clube + 0xbf) + 0x69);
        if (cpv6_b(est + 0xf) as i8 as i32) <= r {
            return 0;
        }
        let e = cpv6_b(cpv6_d(cpv6_d(clube + 0xbf) + 0x69) + 0x20) as i8 as i32;
        if e <= cpv6_sortear(5).wrapping_add(10) {
            return 0;
        }
        valor = e.wrapping_mul(20).wrapping_add(0x96).wrapping_mul(rep).wrapping_add(0xb71b0);
    } else {
        if cpv6_sortear(match_pl_ftol(read_f64_at(0x968938) / div)) != 0 {
            return 0;
        }
        if cpv6_b(clube + 0x82) != 0 {
            return 0;
        }
        valor = rep.wrapping_mul(5).wrapping_add(0x3a98).wrapping_mul(50);
    }
    let saldo = balance(fin);
    if (valor.wrapping_mul(10) as i64) < saldo.wrapping_neg() {
        valor = match_pl_ftol(-(saldo as f64 * read_f64_at(0x967880)));
    }
    if cpv6_w(clube + 0x80) < 0x7d0 {
        let teto = rep.wrapping_mul(rep).wrapping_add(0x16e360);
        if valor > teto {
            valor = teto;
        }
    } else {
        let maior = (valor as f64 > rep.wrapping_mul(rep) as f64 * read_f64_at(0x9689e0)) as i32;
        if valor > 0x3d0900 + maior {
            valor = 0x3d0900 + maior;
        }
    }
    cpv6_somar(fin + 0x154, valor);
    cpv6_somar(fin + 0xb4, valor);
    credit_balance(fin, valor);
    let verba = transfer_budget(fin);
    post_finance_boost_news(this, clube, valor, verba, 3);
    *((fin + 0x165) as *mut u8) = 0;
    1
}

unsafe extern "thiscall" fn comparar_try_cash_injection(this: usize, clube: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%);
    compare_finance_event("try_cash_injection", fin, || o(this, clube, fin), || try_cash_injection(this, clube, fin))
}
