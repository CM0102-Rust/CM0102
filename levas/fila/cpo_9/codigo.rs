// ---------- %%LEVA%%a leva: a lista de observação e o empréstimo: acrescentar o par na lista, prever a resposta do clube vendedor e o período do empréstimo (shortlist_manager.cpp, transfer_manager.cpp) ----------

unsafe fn cpo9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo9_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpo9_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpo9_w(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// Acrescenta o par (id da pessoa, id do clube) no fim do vetor `[this + vet]` (conta em `this + conta`) e o ordena
/// (0x945333 com `cmp`).
unsafe fn insert_sorted_pair(this: usize, vet: usize, conta: usize, pessoa: usize, clube: usize, cmp: usize) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let n = cpo9_i(this + conta);
    let e = cpo9_d(this + vet).wrapping_add((n as usize).wrapping_mul(8));
    cpo9_w(e, cpo9_d(pessoa) as u32);
    cpo9_w(e + 4, cpo9_d(clube) as u32);
    cpo9_w(this + conta, (n + 1) as u32);
    ordenar(cpo9_d(this + vet), (n + 1) as u32, 8, cmp);
}

/// { 0x8423f0 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::add_shortlist_entry(pessoa, clube)`: cria
/// os dois vetores de pares na primeira vez (0x944e46 de 0x320 bytes, capacidade +8/+0xc = 100; sem memória: a caixa
/// "Error" e o jogo para); com o par (pessoa, clube; sem clube −1) já no vetor pela pessoa (0x946464 com 0x757ea0), nada.
/// Senão cresce de 100 em 100 o vetor cheio (0x945667), acrescenta o par nos dois e os ordena (pela pessoa com
/// 0x757ea0, pelo clube com 0x844740) e, fora de clube humano (0x600a50), soma 1 no nibble baixo do +0xb do registro de
/// 79 bytes da pessoa ([0xae2d0c]: quantas listas a têm).
unsafe extern "thiscall" fn add_shortlist_entry(this: usize, pessoa: usize, clube: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    if cpo9_d(this) == 0 {
        let v = alocar(0x320);
        cpo9_w(this, v as u32);
        if v == 0 {
            fatal_error_at(0xa825a4, 0x1594);
        }
        cpo9_w(this + 8, 100);
    }
    if cpo9_d(this + 4) == 0 {
        let v = alocar(0x320);
        cpo9_w(this + 4, v as u32);
        if v == 0 {
            fatal_error_at(0xa825a4, 0x159d);
        }
        cpo9_w(this + 0xc, 100);
    }
    if cpo9_d(this) != 0 {
        let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
        let chave = [cpo9_d(pessoa) as u32, if clube != 0 { cpo9_d(clube) as u32 } else { 0xffff_ffff }];
        if bsearch(chave.as_ptr(), cpo9_d(this), cpo9_i(this + 0x10), 8, 0x757ea0) != 0 {
            return;
        }
    }
    crate::comparacao::ramo("cpo_9 nova entrada");
    if cpo9_i(this + 8) == cpo9_i(this + 0x10) {
        let v = realocar(cpo9_d(this), (cpo9_i(this + 8) as u32).wrapping_mul(8).wrapping_add(0x320));
        cpo9_w(this, v as u32);
        if v == 0 {
            fatal_error_at(0xa825a4, 0x15b5);
        }
        cpo9_w(this + 8, cpo9_i(this + 8).wrapping_add(100) as u32);
    }
    if cpo9_i(this + 0xc) == cpo9_i(this + 0x14) {
        let v = realocar(cpo9_d(this + 4), (cpo9_i(this + 0xc) as u32).wrapping_mul(8).wrapping_add(0x320));
        cpo9_w(this + 4, v as u32);
        if v == 0 {
            fatal_error_at(0xa825a4, 0x15be);
        }
        cpo9_w(this + 0xc, cpo9_i(this + 0xc).wrapping_add(100) as u32);
    }
    insert_sorted_pair(this, 0, 0x10, pessoa, clube, 0x757ea0); // o vetor pela pessoa
    insert_sorted_pair(this, 4, 0x14, pessoa, clube, 0x844740); // o vetor pelo clube
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 0, 1, 0, 0) == 0 {
        let r = (cpo9_d(0xae2d0c).wrapping_add(cpo9_d(pessoa).wrapping_mul(0x4f)) + 0xb) as *mut u8;
        let a = *r;
        *r = (a & 0xf0) | (a.wrapping_add(1) & 0xf);
    }
}

/// { 0x84af00 } (stdcall, ret 0x20, shortlist_manager.cpp): `shortlist_manager_predict_selling_club_response(vendedor,
/// jogador, oferta, comprador, pedido, empréstimo)`: 0 aceita, 1 negocia, 2 recusa. Sem vendedor ou com o contrato com
/// ele (0x4cf5f0) marcado em +0x49: 0; o técnico do vendedor é o jogador: 2; sem contrato de tipo 2 (+0x35): 2 no
/// empréstimo, 0 senão. Empréstimo: clube humano (0x600a50): 0; listado para empréstimo (0x544590) com a reputação do
/// comprador × [0x968928] acima do +0xb da ficha: 0; estatuto 6/7 (+0x4f) ou +0xf da ficha 20, com a reputação acima de
/// +0xb × [0x969040], +0x3d do contrato abaixo de −50 e o +0x1db do vendedor diferente de [0x9cf6e8]: 0; senão 2.
/// Compra: o preço (humano: 0x4d42d0 × [0x9689b0], sem cláusula 0x867b50 e com o técnico de férias 0x5fd210 a opção 0
/// 0x5fea50: 1 recusa, 2 o valor +0x52; senão o `pedido` não negativo, o valor sem ficha ou 0x4d1600); oferta no preço:
/// 0; humano com a oferta acima do preço − 1.500.000 e listado (0x5444a0): 1; preço × [0x96f9c0] acima da oferta: 2; 1.
unsafe extern "stdcall" fn shortlist_manager_predict_selling_club_response(vendedor: usize, jogador: usize, oferta: i64, comprador: usize, pedido: i64, emprestimo: u32) -> u8 {
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let k = com_clube(cpo9_d(0xae1050), jogador, vendedor);
    if vendedor == 0 || (k != 0 && cpo9_b(k + 0x49) != 0) {
        return 0;
    }
    if cpo9_d(vendedor + 0xcf) == jogador {
        return 2;
    }
    if k == 0 || cpo9_b(k + 0x35) & 0x3f != 2 {
        return if emprestimo as u8 != 0 { 2 } else { 0 };
    }
    if emprestimo as u8 != 0 {
        if humano(vendedor, 0, 1, 0, 0) != 0 {
            return 0;
        }
        let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544590usize);
        let rep = cpo9_h(comprador + 0x80) as f64;
        if listado(jogador, 0) != 0 && rep * read_f64_at(0x968928) > cpo9_h(cpo9_d(jogador + 0x61) + 0xb) as f64 {
            return 0;
        }
        let st = cpo9_b(k + 0x4f) & 0xf0;
        if st == 0x60 || st == 0x70 || cpo9_b(cpo9_d(jogador + 0x61) + 0xf) == 0x14 {
            let v = cpo9_h(cpo9_d(jogador + 0x61) + 0xb) as f64 * read_f64_at(0x969040);
            if rep > v && (cpo9_b(k + 0x3d) as i8) < -50 && cpo9_d(vendedor + 0x1db) != cpo9_d(0x9cf6e8) {
                return 0;
            }
        }
        return 2;
    }
    let preco: i32 = if humano(vendedor, 0, 1, 0, 0) != 0 {
        let pedido_humano: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> i32 = core::mem::transmute(0x4d42d0usize);
        let mut v = match_pl_ftol(pedido_humano(cpo9_d(0xae1050), vendedor, comprador, jogador) as f64 * read_f64_at(0x9689b0));
        let clausula: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x867b50usize);
        if clausula(k) == 0 {
            let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
            let ferias: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
            if ferias(cpo9_d(0xb63c98), tecnico(vendedor)) != 0 {
                let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(0x5fea50usize);
                match opcao(cpo9_d(0xb63c98), 0, tecnico(vendedor)) as i8 {
                    1 => return 2,
                    2 => v = cpo9_i(jogador + 0x52),
                    _ => {}
                }
            }
        }
        v
    } else if pedido >= 0 {
        pedido as i32
    } else if cpo9_d(jogador + 0x61) == 0 {
        cpo9_i(jogador + 0x52)
    } else {
        let calcular: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
        calcular(cpo9_d(0xae1050), vendedor, comprador, jogador, 0, 1, 1)
    };
    if oferta >= preco as i64 {
        return 0;
    }
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    if humano(vendedor, 0, 1, 0, 0) != 0 && (preco.wrapping_sub(0x16e360) as i64) < oferta && listado(jogador, 0) != 0 {
        return 1;
    }
    if preco as f64 * read_f64_at(0x96f9c0) > oferta as f64 {
        return 2;
    }
    crate::comparacao::ramo("cpo_9 negocia");
    1
}

/// O ano da data padrão (0x54a110: dia 40 de 1972).
unsafe fn default_date_year() -> u16 {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mut data = [0u8; 8];
    let p = padrao(data.as_mut_ptr() as usize);
    cpo9_h(p + 2) as u16
}

/// A data (dia do ano +0, ano +2) passa do fim do contrato `c` (+0x2d dia, +0x2f ano)?
unsafe fn is_after_contract_end(dia: i16, ano: u16, c: usize) -> bool {
    let fim = cpo9_h(c + 0x2f) as u16;
    ano > fim || (ano == fim && dia > cpo9_h(c + 0x2d))
}

/// { 0x8d0590 } (thiscall, ret 0x1c, transfer_manager.cpp): `TRANSFER_MANAGER::get_loan_period(clube, a2, jogador,
/// início_mês, início_ano, fim_mês, fim_ano)`: as regras da nação do clube (+0x53; das 28 em +0x8ac, a primeira sem
/// achar) dão o período (método +0x28). Sem contrato de tipo 2 (0x4cf5a0), com o contrato já vencido hoje ou o fim mais
/// de 5 anos à frente: o fim vira o ano da data padrão (1972). Senão, enquanto a data (dia do mês de hoje, fim_mês,
/// fim_ano; 0x549f70) passar do fim do contrato, recua um mês (o início acompanha se ficar depois do fim); fim antes do
/// ano de hoje: o ano padrão.
unsafe extern "thiscall" fn transfer_manager_get_loan_period(this: usize, clube: usize, a2: u32, jogador: usize, ini_mes: *mut i8, ini_ano: *mut u16, mes: *mut i8, ano: *mut u16) {
    let nacao = cpo9_d(clube + 0x53);
    let mut regra = cpo9_d(this + 0x8ac);
    for k in 0..0x1cusize {
        let r = cpo9_d(this + 0x8ac + k * 4);
        if cpo9_d(r + 0xa) == nacao {
            if r != 0 {
                regra = r;
            }
            break;
        }
    }
    let periodo: unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut u16, *mut i8, *mut u16, usize) = core::mem::transmute(cpo9_d(cpo9_d(regra) + 0x28));
    periodo(regra, clube, a2, ini_mes, ini_ano, mes, ano, jogador);
    let dia_do_mes: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54cf70usize);
    let dia = dia_do_mes(0xae2c90);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contract_of(cpo9_d(0xae1050), jogador);
    if c == 0 || cpo9_b(c + 0x35) & 0x3f != 2 || is_after_contract_end(cpo9_h(0xae2c90), cpo9_h(0xae2c92) as u16, c) {
        *ano = default_date_year();
        return;
    }
    if *ano as i32 > cpo9_h(0xae2c92) as u16 as i32 + 5 {
        *ano = default_date_year();
        return;
    }
    // ponytail: a original empurra o mês e o ano em registradores com lixo acima; a 0x549f70 lê o byte e a palavra
    let data_de: unsafe extern "thiscall" fn(usize, i32, i32, u32, i32) -> usize = core::mem::transmute(0x549f70usize);
    loop {
        let mut data = [0u8; 8];
        let d = data_de(data.as_mut_ptr() as usize, dia, *mes as i32, *ano as u32, -1);
        if !is_after_contract_end(cpo9_h(d), cpo9_h(d + 2) as u16, c) {
            return;
        }
        crate::comparacao::ramo("cpo_9 recua o mes");
        let m = (*mes).wrapping_sub(1);
        *mes = m;
        if m < 0 {
            *ano = (*ano).wrapping_sub(1);
            *mes = 11;
        }
        if *ini_ano > *ano || (*ini_ano == *ano && *ini_mes > *mes) {
            *ini_mes = *mes;
            *ini_ano = *ano;
        }
        if *ano < cpo9_h(0xae2c92) as u16 {
            *ano = default_date_year();
            return;
        }
    }
}

/// Comparador: só a original, conferindo o plano (feito antes): o par já estava (nada muda) ou entra nos dois vetores
/// (contas + 1, o par achado depois) e o nibble da pessoa soma 1 fora de clube humano.
// ponytail: a original realoca os vetores (0x945667) e os ordena; não roda duas vezes.
unsafe extern "thiscall" fn comparar_add_shortlist_entry(this: usize, pessoa: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let chave = [cpo9_d(pessoa) as u32, if clube != 0 { cpo9_d(clube) as u32 } else { 0xffff_ffff }];
    let tinha = cpo9_d(this) != 0 && bsearch(chave.as_ptr(), cpo9_d(this), cpo9_i(this + 0x10), 8, 0x757ea0) != 0;
    let (n0, n1) = if cpo9_d(this) != 0 { (cpo9_i(this + 0x10), cpo9_i(this + 0x14)) } else { (0, 0) };
    let r = cpo9_d(0xae2d0c).wrapping_add(cpo9_d(pessoa).wrapping_mul(0x4f)) + 0xb;
    let nib = cpo9_b(r);
    let soma = !tinha && humano(clube, 0, 1, 0, 0) == 0;
    o(this, pessoa, clube);
    let esperado = if soma { (nib & 0xf0) | (nib.wrapping_add(1) & 0xf) } else { nib };
    let (d0, d1) = (if tinha { n0 } else { n0 + 1 }, if tinha { n1 } else { n1 + 1 });
    let achado = bsearch(chave.as_ptr(), cpo9_d(this), cpo9_i(this + 0x10), 8, 0x757ea0) != 0;
    let ok = cpo9_i(this + 0x10) == d0 && cpo9_i(this + 0x14) == d1 && cpo9_b(r) == esperado && (achado || clube == 0);
    PLACAR.registrar(ok, || {
        format!("add_shortlist_entry({:#x}, {:#x}) tinha={} contas {}/{} -> {}/{} nibble {:#x} -> {:#x} (esperado {:#x})",
            pessoa, clube, tinha, n0, n1, cpo9_i(this + 0x10), cpo9_i(this + 0x14), nib, cpo9_b(r), esperado)
    });
}

/// Comparador: pelo valor (o byte baixo) e lado a lado pelo sorteio e [0xae1080] (as chamadas de preço podem sortear).
unsafe extern "stdcall" fn comparar_shortlist_manager_predict_selling_club_response(vendedor: usize, jogador: usize, oferta: i64, comprador: usize, pedido: i64, emprestimo: u32) -> u8 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, i64, usize, i64, u32) -> u8>(%%I1%%);
    let mut reg = vec![(0xae1080usize, 4usize)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(
        &reg,
        || o(vendedor, jogador, oferta, comprador, pedido, emprestimo),
        || crate::aleatorio::sem_contar(|| shortlist_manager_predict_selling_club_response(vendedor, jogador, oferta, comprador, pedido, emprestimo)),
    );
    PLACAR.registrar(igual && r == n, || format!("shortlist_manager_predict_selling_club_response({:#x}, {:#x}, {}, {:#x}, {}, {}) original={} rust={} {:x?} x {:x?}", vendedor, jogador, oferta, comprador, pedido, emprestimo as u8, r, n, mo, mr));
    r
}

/// Comparador: lado a lado pelas quatro saídas, o sorteio e [0xae1080].
unsafe extern "thiscall" fn comparar_transfer_manager_get_loan_period(this: usize, clube: usize, a2: u32, jogador: usize, ini_mes: *mut i8, ini_ano: *mut u16, mes: *mut i8, ano: *mut u16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, *mut i8, *mut u16, *mut i8, *mut u16)>(%%I2%%);
    let mut reg = vec![(ini_mes as usize, 1usize), (ini_ano as usize, 2), (mes as usize, 1), (ano as usize, 2), (0xae1080, 4)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(
        &reg,
        || o(this, clube, a2, jogador, ini_mes, ini_ano, mes, ano),
        || crate::aleatorio::sem_contar(|| transfer_manager_get_loan_period(this, clube, a2, jogador, ini_mes, ini_ano, mes, ano)),
    );
    PLACAR.registrar(igual, || format!("transfer_manager_get_loan_period({:#x}, {:#x}) {:x?} x {:x?}", clube, jogador, mo, mr));
}
