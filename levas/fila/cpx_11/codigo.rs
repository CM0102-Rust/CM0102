// ---------- %%LEVA%%a leva: o fim da temporada da liga: os prêmios em dinheiro, os registros pela situação do clube e a notícia da situação (league.cpp) ----------

unsafe fn cpx11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpx11_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de finanças do clube (FinanceTable::get_club_finance 0x5a0590 de [0xae2a38]).
unsafe fn cpx11_financas(clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    f(cpx11_d(0xae2a38), clube)
}

/// O ensaio do comparador da 0x682e10: a cópia roda sem a notícia do prêmio (0x48e3e0, que aloca e não pode rodar duas
/// vezes).
static mut LEAGUE_PRIZE_MONEY_DRY_RUN: bool = false;

/// O divisor dos prêmios: 1 + 2 + … + n (16 bits, como o ax da original).
unsafe fn sum_of_positions(n: i16) -> i32 {
    let (mut s, mut k) = (0i32, 0i32);
    if n > 0 {
        loop {
            s = s.wrapping_add(k).wrapping_add(1);
            k += 1;
            if k as i16 >= n {
                break;
            }
        }
    }
    s as i16 as i32
}

/// { 0x682e10 } (thiscall, ret 0, league.cpp): `League::pay_prize_money()`: com +0xe1 = 2, nada. Com +0xe1 = 3, a
/// competição ([0xadadfc] pelo id de +4) paga pela virtual +0x6c (sem a tabela das competições: a caixa da linha
/// 0x1df). Senão o fundo (+0x28) dividido por 1 + 2 + … + n (n = +0x3e linhas da tabela) é a parte; o clube da linha i
/// (+0xb1, 0x3b cada) recebe (n − i) partes no registro de finanças (0x5a0590; 0x5999a0 soma no saldo, em +0xa8 e
/// +0x148) e, acima de 1000, a notícia do prêmio (0x48e3e0 com a posição i + 1).
unsafe extern "thiscall" fn pay_prize_money(liga: usize) {
    let e1 = cpx11_b(liga + 0xe1);
    if e1 == 2 {
        return;
    }
    if e1 == 3 {
        let t = cpx11_d(0xadadfc);
        if t == 0 {
            return show_error_at(0x9f15f4, 0x1df);
        }
        let comp = cpx11_d(t.wrapping_add(cpx11_d(cpx11_d(liga + 4)).wrapping_mul(4)));
        let pagar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(cpx11_d(cpx11_d(comp) + 0x6c));
        return pagar(comp, liga);
    }
    let s = sum_of_positions(cpx11_w(liga + 0x3e));
    let fundo = cpx11_d(liga + 0x28) as i32;
    if s == 0 || (fundo == i32::MIN && s == -1) {
        // ponytail: a original divide por zero (ou estoura) e cai; só a original, para cair do mesmo jeito
        return original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(liga);
    }
    let parte = fundo / s;
    if cpx11_w(liga + 0x3e) <= 0 {
        return;
    }
    let somar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5999a0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, i32) = core::mem::transmute(0x48e3e0usize);
    let mut i: i16 = 0;
    loop {
        let valor = (cpx11_w(liga + 0x3e) as i32).wrapping_sub(i as i32).wrapping_mul(parte);
        let clube = cpx11_d(cpx11_d(liga + 0xb1).wrapping_add(0x3b * i as usize));
        let fin = cpx11_financas(clube);
        crate::comparacao::ramo("cpx_11 premio pago");
        somar(fin, valor); // cpx11: o prêmio
        if valor > 1000 && !LEAGUE_PRIZE_MONEY_DRY_RUN {
            // ponytail: a original empurra a posição no dl e o resto do edx (29 × i, sobra da conta da linha); a
            // 0x48e3e0 lê só o byte
            let pos = ((29 * i as i32 as u32) & !0xff) | (i as u8).wrapping_add(1) as u32;
            noticia(liga, clube, valor as u32, (valor >> 31) as u32, pos, 0, 0, -2);
        }
        i = i.wrapping_add(1);
        if i >= cpx11_w(liga + 0x3e) {
            break;
        }
    }
}

/// Comparador: com +0xe1 = 3 (a virtual da competição paga por conta própria) ou o divisor zero: só a original. Nos
/// outros casos, o ensaio: a cópia sem a notícia do prêmio (0x48e3e0 aloca) e a original a partir do mesmo estado, nos
/// registros de finanças (0x167 bytes) de todos os clubes da tabela.
unsafe extern "thiscall" fn comparar_pay_prize_money(liga: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let n = cpx11_w(liga + 0x3e);
    let e1 = cpx11_b(liga + 0xe1);
    if e1 == 3 || sum_of_positions(n) == 0 {
        return o(liga); // ponytail: a virtual ou a divisão por zero, só a original
    }
    let mut regioes: Vec<(usize, usize)> = Vec::new();
    if e1 != 2 {
        for i in 0..n.max(0) as usize {
            let fin = cpx11_financas(cpx11_d(cpx11_d(liga + 0xb1) + 0x3b * i));
            if fin != 0 && !regioes.iter().any(|r| r.0 == fin) {
                regioes.push((fin, 0x167));
            }
        }
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    LEAGUE_PRIZE_MONEY_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| pay_prize_money(liga)));
    LEAGUE_PRIZE_MONEY_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(liga);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("pay_prize_money({:x}) +0xe1 {} n {} original={:x?} rust={:x?}", liga, e1, n, mo, mr));
}

/// { 0x686940 } (thiscall, ret 0x18, league.cpp): `League::update_staff_records_for_status(clube, situação, fase,
/// saída, _, _)` (a virtual +0x44 da liga): situação 5, o título (Staff_Records 0x868c50 de [0xdd7f00] com o clube e
/// a competição +4); 0, o acesso (0x869480 com peso 100); 3, o rebaixamento (0x86a1c0). Devolve 0 e não grava a saída.
unsafe extern "thiscall" fn league_update_staff_records_for_status(liga: usize, clube: usize, situacao: u32, _fase: u32, _saida: usize, _a: u32, _b: u32) -> u32 {
    let registros = cpx11_d(0xdd7f00);
    let comp = cpx11_d(liga + 4);
    match situacao as u8 {
        5 => {
            crate::comparacao::ramo("cpx_11 registro do titulo");
            let titulo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x868c50usize);
            titulo(registros, clube, comp);
        }
        0 => {
            crate::comparacao::ramo("cpx_11 registro do acesso");
            let acesso: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869480usize);
            acesso(registros, clube, comp, 0x64);
        }
        3 => {
            crate::comparacao::ramo("cpx_11 registro do rebaixamento");
            let rebaixamento: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86a1c0usize);
            rebaixamento(registros, clube, comp);
        }
        _ => {}
    }
    0
}

// ponytail: os três registros acrescentam ao histórico das pessoas e mexem na reputação (alocam): só a original; a
// cópia leva os ramos.
unsafe extern "thiscall" fn comparar_league_update_staff_records_for_status(liga: usize, clube: usize, situacao: u32, fase: u32, saida: usize, a: u32, b: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> u32>(%%I1%%)(liga, clube, situacao, fase, saida, a, b)
}

/// { 0x686670 } (thiscall, ret 0x14, league.cpp, quadro SEH sem destrutor): `League::announce_team_status(clube,
/// situação, fase, _, _)`: amistoso (0x5c6710), 0. A virtual +0x44 da competição ([0xadadfc] pelo id de +4) com o
/// clube, a situação, a fase e a saída (0xff); −1 dela: 0. A rodada (+0x3a; com fases, +0x44 = 0xff, e fase ≠ 0xff:
/// a da fase). A notícia 0xc (local): 0 o clube, 1 a competição, 2 a fase, 3 a situação, 4 a rodada, 5 o que a
/// virtual devolveu. Saída ≠ 0xff: a entrega pelo alcance (0x786090 com a nação do clube e o continente). Senão pela
/// situação: 5, à nação da competição (0x7863f0) sem o +0x1c, à divisão (0x7862d0) com ele; 0 e 3, à divisão e à
/// divisão de acesso (+0x1c) ou de rebaixamento (+0x20) da competição, se houver; 1 e 2, à divisão; 6, 7 e 0xfe, ao
/// técnico do clube (0x7861e0). Devolve 1.
unsafe extern "thiscall" fn announce_team_status(liga: usize, clube: usize, situacao: u32, fase: u32, _a: u32, _b: u32) -> u32 {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let a_divisao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7862d0usize);
    let a_nacao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
    let pelo_alcance: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x786090usize);
    // ponytail: a original grava só o byte da saída num dword da pilha e empurra o dword inteiro para a 0x786090, que
    // lê o byte; a cópia passa o alto zerado
    let mut saida: u32 = 0xff;
    let mut rodada = cpx11_w(liga + 0x3a);
    if amistoso(cpx11_d(0xae2c74), cpx11_d(liga + 4)) != 0 {
        return 0;
    }
    let comp = cpx11_d(cpx11_d(0xadadfc).wrapping_add(cpx11_d(cpx11_d(liga + 4)).wrapping_mul(4)));
    let virtual44: unsafe extern "thiscall" fn(usize, usize, u32, u32, *mut u32, u32, u32) -> i32 = core::mem::transmute(cpx11_d(cpx11_d(comp) + 0x44));
    let r = virtual44(comp, clube, situacao, fase, &mut saida, 0, 0);
    if r == -1 {
        return 0;
    }
    if cpx11_b(liga + 0x44) == 0xff && fase as u8 != 0xff {
        let f = cpx11_d(cpx11_d(liga + 0xc).wrapping_add((fase as u8 as i8 as isize as usize).wrapping_mul(4)));
        rodada = cpx11_w(f + 0x3a);
    }
    crate::comparacao::ramo("cpx_11 noticia da situacao");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xc, 0);
    gravar(n, 0, cpx11_d(clube) as u32);
    gravar(n, 1, cpx11_d(cpx11_d(liga + 4)) as u32);
    gravar(n, 2, fase as u8 as i8 as i32 as u32);
    gravar(n, 3, situacao as u8 as i8 as i32 as u32);
    gravar(n, 4, rodada as i32 as u32);
    gravar(n, 5, r as u32);
    let noticias = cpx11_d(0xb7431c);
    if saida as u8 != 0xff {
        let nacao = cpx11_d(clube + 0x53);
        let continente = if nacao != 0 { cpx11_d(nacao + 0x71) } else { 0 };
        pelo_alcance(noticias, n, clube, cpx11_d(liga + 4), nacao, continente, saida);
        return 1;
    }
    // a divisão de acesso (+0x1c) ou de rebaixamento (+0x20) da competição, registros de 0x6b em [0xae23d0]
    let outra = |desl: usize| {
        let c = cpx11_d(cpx11_d(0xadadfc).wrapping_add(cpx11_d(cpx11_d(liga + 4)).wrapping_mul(4)));
        let e = cpx11_d(c + desl) as i32;
        if e != -1 {
            a_divisao(noticias, n, cpx11_d(0xae23d0).wrapping_add((e as usize).wrapping_mul(0x6b)));
        }
    };
    match situacao as u8 {
        5 => {
            if cpx11_d(liga + 0x1c) as i32 == -1 {
                a_nacao(noticias, n, cpx11_d(cpx11_d(liga + 4) + 0x5d));
            } else {
                a_divisao(noticias, n, cpx11_d(liga + 4));
            }
        }
        0 => {
            a_divisao(noticias, n, cpx11_d(liga + 4));
            outra(0x1c);
        }
        7 | 6 | 0xfe => {
            ao_clube(noticias, n, clube);
        }
        1 | 2 => {
            a_divisao(noticias, n, cpx11_d(liga + 4));
        }
        3 => {
            a_divisao(noticias, n, cpx11_d(liga + 4));
            outra(0x20);
        }
        _ => {}
    }
    1
}

// ponytail: a virtual +0x44 acrescenta registros e a notícia vai às caixas dos técnicos (alocam): só a original; a
// cópia leva o ramo.
unsafe extern "thiscall" fn comparar_announce_team_status(liga: usize, clube: usize, situacao: u32, fase: u32, a: u32, b: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32>(%%I2%%)(liga, clube, situacao, fase, a, b)
}
