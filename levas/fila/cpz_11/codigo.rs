// ---------- %%LEVA%%a leva: a folha salarial e a janela dos ingressos: a folha do clube, as folhas refeitas na criação e os dias até a temporada (finance.cpp) ----------

unsafe fn cpz11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz11_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpz11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpz11_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpz11_por(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

/// { 0x598370 } (thiscall, finance.cpp): `Finance::update_wage_bill()`, pelo passe diário das finanças (0x59b9c0, no
/// sábado, clube com nação), pela assinatura de contrato (0x4cf720), pelo passe diário dos contratos (0x4c7ac0), pelo
/// empréstimo (0x4d01e0, 0x4d07e0) e pelas folhas da criação (0x59cf50). Zera o maior salário (+0x10) e a folha (+0xc)
/// e soma os salários (+0xc do contrato) das 50 vagas do elenco (+0xd7; contrato com o clube 0x4cf5f0; salário −1: o
/// salário padrão 0x864550 truncado; emprestado (0x4cf6d0) fora do tipo 6: menos o salário do outro contrato) e do
/// pessoal (+0x19f × 5, +0x1b3 × 7, +0x1cf × 3, contrato 0x4cf5a0). Elenco com menos de 15 vagas ocupadas: as que
/// faltam a um salário pela reputação (+0x80: acima de 4750, rep − 1950; acima de 2250, (rep − 2250) × 0,5 + 550; senão
/// rep × 0,25; fora do tipo 1 (+0x64), × 0,5; dividido pelo tipo da nação (+0x7e, entre 1 e 3)). Mais reputação² × k
/// (tipo 1: Inglaterra e Itália 0,001 / 0,0008 / 0,0006 acima de 5250 / 3250; outras 0,0004; fora do tipo 1: 0,0002
/// acima de 3250, senão 0,0001), truncado. O teto salarial (+0x14) refeito por Finance::get_max_wage (0x5968d0, 0, −1).
unsafe extern "thiscall" fn update_wage_bill(this: usize) {
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let padrao: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32, u32) -> f64 = core::mem::transmute(0x864550usize);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let teto: unsafe extern "thiscall" fn(usize, u32, i32) -> i32 = core::mem::transmute(0x5968d0usize);
    cpz11_por(this + 0x10, 0);
    cpz11_por(this + 0xc, 0);
    let clube = cpz11_d(0xae23b4).wrapping_add(cpz11_d(this + 8).wrapping_mul(0x245));
    let mut ocupadas: i8 = 0;
    for k in 0..50usize {
        let p = cpz11_d(clube + 0xd7 + 4 * k);
        if p == 0 {
            continue;
        }
        ocupadas = ocupadas.wrapping_add(1);
        let c = com_clube(cpz11_d(0xae1050), p, clube);
        if c == 0 {
            continue;
        }
        let mut w = cpz11_i(c + 0xc);
        if w == -1 {
            w = match_pl_ftol(padrao(c, p, clube, 0, -1, 0xb, 0));
        }
        if ficha(cpz11_d(0xae1050), p) != 0 && cpz11_b(c + 0x35) & 0x3f != 6 {
            w = w.wrapping_sub(cpz11_i(ficha(cpz11_d(0xae1050), p) + 0xc)); // a parte do outro contrato
        }
        if w > cpz11_i(this + 0x10) {
            cpz11_por(this + 0x10, w);
        }
        cpz11_por(this + 0xc, cpz11_i(this + 0xc).wrapping_add(w));
    }
    // o pessoal: a soma vai à folha depois das três listas (a mesma soma, em 32 bits)
    let mut comissao = 0i32;
    for (base, n) in [(0x19fusize, 5usize), (0x1b3, 7), (0x1cf, 3)] {
        for k in 0..n {
            let p = cpz11_d(clube + base + 4 * k);
            if p == 0 {
                continue;
            }
            let c = contract_of(cpz11_d(0xae1050), p);
            if c == 0 {
                continue;
            }
            let w = cpz11_i(c + 0xc);
            if w > cpz11_i(this + 0x10) {
                cpz11_por(this + 0x10, w);
            }
            comissao = comissao.wrapping_add(w);
        }
    }
    if comissao != 0 {
        crate::comparison::branch("cpz_11 staff payroll");
        cpz11_por(this + 0xc, cpz11_i(this + 0xc).wrapping_add(comissao)); // cpz11: o pessoal na folha
    }
    let tipo1 = cpz11_b(clube + 0x64) == 1;
    let rep = cpz11_w(clube + 0x80) as i32;
    if ocupadas < 15 {
        let mut v = if rep > 0x128e {
            rep - 0x79e
        } else if rep > 0x8ca {
            match_pl_ftol((rep - 0x8ca) as f64 * read_f64(0x968968) + read_f64(0x96a620))
        } else {
            match_pl_ftol(rep as f64 * read_f64(0x9689e0))
        };
        if !tipo1 {
            v = match_pl_ftol(v as f64 * read_f64(0x968968));
        }
        let nacao = cpz11_d(clube + 0x53);
        if nacao != 0 {
            let t = (cpz11_b(nacao + 0x7e) as i8).clamp(1, 3) as i32;
            v = v.wrapping_div(t);
        }
        let falta = 15 - ocupadas as i32;
        cpz11_por(this + 0xc, cpz11_i(this + 0xc).wrapping_add(falta.wrapping_mul(v)));
    }
    let k = if tipo1 {
        let nacao = cpz11_d(clube + 0x53);
        let grande = nacao != 0 && (cpz11_i(nacao) == cpz11_i(0x9cf2e4) || cpz11_i(nacao) == cpz11_i(0x9cf360));
        if !grande {
            0x96a608
        } else if rep > 0x1482 {
            0x96a350
        } else if rep > 0xcb2 {
            0x96a618
        } else {
            0x96a610
        }
    } else if rep > 0xcb2 {
        0x96a600
    } else {
        0x96a5c0
    };
    crate::comparison::branch("cpz_11 payroll by injury_word_0c");
    let folha = match_pl_ftol(rep.wrapping_mul(rep) as f64 * read_f64(k) + cpz11_i(this + 0xc) as f64); // cpz11: a reputação na folha
    cpz11_por(this + 0xc, folha);
    let t = teto(this, 0, -1);
    cpz11_por(this + 0x14, t);
}

/// Comparador: lado a lado na folha (+0xc), no maior salário (+0x10), no teto (+0x14) e no último contrato consultado
/// ([0xae1080], que o salário padrão 0x864550 grava); as outras chamadas (contratos, teto 0x5968d0) só leem.
unsafe extern "thiscall" fn compare_update_wage_bill(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this + 0xc, 12), (0xae1080, 4)], || o(this), || update_wage_bill(this));
    SCOREBOARD.record(igual, || format!("update_wage_bill(fin {:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// { 0x59cf50 } (thiscall, finance.cpp): `FinanceTable::refresh_all_wage_bills()`, só pela criação do jogo (0x828110):
/// para cada clube de [0xae23b4] até a primeira seleção (0x53b900), a folha (0x598370) do registro de finanças dele
/// ([this] + i × 0x167).
unsafe extern "thiscall" fn refresh_all_wage_bills(this: usize) {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x598370usize);
    let mut fin = cpz11_d(this);
    let mut i = 0i32;
    while i < cpz11_i(0xae235c) {
        let clube = cpz11_d(0xae23b4).wrapping_add((i as usize).wrapping_mul(0x245));
        if selecao(clube) != 0 {
            break;
        }
        folha(fin);
        fin = fin.wrapping_add(0x167);
        i += 1;
    }
}

/// Comparador: lado a lado na tabela de finanças inteira ([this], [0xae235c] × 0x167 bytes) e em [0xae1080].
unsafe extern "thiscall" fn compare_refresh_all_wage_bills(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let n = (cpz11_i(0xae235c).max(0) as usize).wrapping_mul(0x167);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(cpz11_d(this), n), (0xae1080, 4)], || o(this), || refresh_all_wage_bills(this));
    SCOREBOARD.record(igual, || format!("refresh_all_wage_bills({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// { 0x59b4c0 } (ret 8, finance.cpp): `finance_get_days_to_season_start(clube, &passou)`, pelo passe diário das
/// finanças (0x59b9c0; ecx com a tabela, que ela não lê): dias até o começo da temporada da nação do clube (linha de
/// 72 bytes em 0xb63d60 pela 0x669360; sem linha, a caixa "Error" da linha 0x1185 e 0). `passou` = 1 se o ano de hoje
/// chega ao ano da temporada (2002 com +0x17, senão o ano de [0xdd6ec8] + 1). Mês do começo (+0x34) abaixo de 2: no ano
/// do começo (+0x2c), só depois do mês 12 − mês do começo; noutro ano, só antes do mês do começo; senão, só a 0..2
/// meses do começo. Na janela, os dias até a data do começo (+0x2a, 0x54cdb0) se positivos; fora, 0.
unsafe extern "thiscall" fn finance_get_days_to_season_start(_this: usize, clube: usize, passou: *mut u32) -> i32 {
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669360usize);
    let mes_de: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x54cfb0usize);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    *passou = 0;
    let k = linha(clube) as i8;
    if k < 0 {
        show_error_at(0x9c8784, 0x1185);
        return 0;
    }
    let e = 0xb63d60usize.wrapping_add((k as isize * 72) as usize);
    let ano: u16 = if cpz11_d(e + 0x17) != 0 { 0x7d2 } else { (cpz11_w(0xdd6eca) as u16).wrapping_add(1) };
    if cpz11_w(0xae2c92) as u16 >= ano {
        *passou = 1;
    }
    let inicio = cpz11_b(e + 0x34) as i8;
    let ano_do_inicio = cpz11_w(e + 0x2c) as u16;
    let mes = mes_de(0xae2c90) as i8;
    let janela = if inicio < 2 {
        if ano_do_inicio == cpz11_w(0xae2c92) as u16 {
            mes as i32 > 12 - inicio as i32
        } else {
            mes < inicio
        }
    } else {
        let d = inicio.wrapping_sub(mes);
        (0..=2).contains(&d)
    };
    if !janela {
        return 0;
    }
    let r = dias_ate(e + 0x2a, cpz11_d(0xae2c90) as u32, cpz11_d(0xae2c94) as u32);
    if r > 0 {
        crate::comparison::branch("cpz_11 days until the season");
        return r; // cpz11: os dias até a temporada
    }
    0
}

/// Comparador: sem linha da nação, só a original (a caixa); senão lado a lado na saída `passou` e na data estática da
/// 0x54cdb0 (0xae2470), com a devolução.
unsafe extern "thiscall" fn compare_finance_get_days_to_season_start(this: usize, clube: usize, passou: *mut u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u32) -> i32>(%%I2%%);
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669360usize);
    if (linha(clube) as i8) < 0 {
        return o(this, clube, passou);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(passou as usize, 4), (0xae2470, 8)], || o(this, clube, passou), || finance_get_days_to_season_start(this, clube, passou));
    SCOREBOARD.record(igual && r == n, || format!("finance_get_days_to_season_start(clube {:#x}) original={} {:x?} rust={} {:x?}", clube, r, mo, n, mr));
    r
}
