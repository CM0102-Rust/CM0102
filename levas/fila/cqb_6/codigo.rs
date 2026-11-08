// ---------- %%LEVA%%a leva: a parte dos clubes anteriores na transferência, o percentual da próxima venda pago e a notícia dele (transfer_manager.cpp) ----------

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
unsafe fn transfer_manager_club_of_contract(c: usize) -> usize {
    if c == 0 { 0 } else { cqb6_clube(cqb6_d(c + 4)) }
}

/// As finanças do clube (0x5a0590: thiscall([0xae2a38], clube), ret 4).
unsafe fn cqb6_financas(clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    f(cqb6_d(0xae2a38), clube)
}

/// O valor (i32 estendido para 64 bits, como o `cdq` da original) entra nas vendas (0x599800) ou sai pelas compras
/// (0x599890) das finanças `fin` (thiscall(fin, baixo, alto), ret 8).
unsafe fn post_finance_amount(rotina: usize, fin: usize, v: i32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(rotina);
    f(fin, v as u32, (v >> 31) as u32);
}

/// A notícia local: construir (0x77bbe0: thiscall(notícia, tipo, 0), ret 8), gravar um dado (0x785790:
/// thiscall(notícia, k, v), ret 8) e entregar ao técnico do clube (0x7861e0: thiscall([0xb7431c], notícia, clube),
/// ret 8).
unsafe fn transfer_manager_construct_news_item(n: usize, tipo: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    f(n, tipo, 0);
}

unsafe fn cqb6_gravar(n: usize, k: i8, v: u32) {
    let f: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    f(n, k, v);
}

unsafe fn send_news_to_club_manager(n: usize, clube: usize) {
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
unsafe extern "thiscall" fn pay_former_club_fee_cuts(_this: usize, o: usize) {
    let p = cqb6_pessoa(cqb6_d(o + 0xc));
    if (cqb6_b(p + 0x18) as i8) < 0x17 { // cqb6: só acima de 22 anos
        return;
    }
    crate::comparacao::ramo("cqb_6 parte dos clubes anteriores");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    if !CQB6_ENSAIO {
        transfer_manager_construct_news_item(n, 0xfc3);
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
    let v7 = if c == 0 || cqb6_b(c + 0x35) & 0x3f == 5 || transfer_manager_club_of_contract(c) == 0 {
        u32::MAX
    } else {
        let c2 = cqb6_contrato(p);
        // ponytail: a original lê [0] se o segundo contrato faltasse; inalcançável (o mesmo contrato acabou de pass_ball)
        if c2 != 0 && cqb6_b(c2 + 0x35) & 0x3f != 5 { cqb6_d(transfer_manager_club_of_contract(c2)) as u32 } else { u32::MAX }
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
        post_finance_amount(0x599800, fin, valor); // cqb6: entra nas vendas do comprador
        if valor > (cqb6_w(clube + 0x80) as i32).wrapping_mul(11).wrapping_sub(10000) {
            CQB6_ENTREGAS += 1;
            if !CQB6_ENSAIO {
                cqb6_gravar(n, 5, valor as u32);
                cqb6_gravar(n, 6, cqb6_d(clube) as u32);
                send_news_to_club_manager(n, clube);
            }
        }
    }
}

/// Comparador da 0x8e4f20: a cópia em ensaio (sem a notícia) e a original a partir do mesmo estado, lado a lado nas
/// finanças do comprador (0x167 bytes) e em [0xae1080]; com entrega da notícia, só as finanças.
unsafe extern "thiscall" fn comparar_pay_former_club_fee_cuts(this: usize, o: usize) {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let fin = cqb6_financas(cqb6_clube(cqb6_d(o + 8)));
    CQB6_ENTREGAS = 0;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(fin, 0x167), (0xae1080, 4)], || original(this, o), || {
        CQB6_ENSAIO = true;
        crate::aleatorio::sem_contar(|| pay_former_club_fee_cuts(this, o));
        CQB6_ENSAIO = false;
    });
    let entregas = CQB6_ENTREGAS;
    let igual = igual || (entregas != 0 && mo.0[0] == mr.0[0]);
    PLACAR.registrar(igual, || format!("pay_former_club_fee_cuts(oferta {:#x}) entregas={} original={:x?} rust={:x?}", o, entregas, mo, mr));
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
unsafe extern "thiscall" fn pay_next_sale_percentage(this: usize, p: usize, o: usize) -> u32 {
    let c = cqb6_contrato(p);
    if c == 0 || transfer_manager_club_of_contract(c) == 0 {
        return 0;
    }
    let mut chave = [0u8; 0xd];
    core::ptr::write_unaligned(chave.as_mut_ptr().add(9) as *mut usize, cqb6_d(p));
    let c2 = cqb6_contrato(p);
    // ponytail: a original lê [0] se o segundo contrato faltasse; inalcançável (o mesmo contrato acabou de pass_ball)
    let id_clube = if c2 != 0 { cqb6_d(transfer_manager_club_of_contract(c2)) } else { usize::MAX };
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
        let clube = transfer_manager_club_of_contract(cqb6_contrato(p));
        let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
        (valor(cqb6_d(0xae1050), p, clube, 0, -1, 0), 0)
    };
    let parte = (cqb6_b(e) as i8 as i32).wrapping_mul(base) / 100;
    if parte < 1000 {
        return 0;
    }
    crate::comparacao::ramo("cqb_6 write_percent_pair pago");
    if !CQB6_ENSAIO {
        let noticia: unsafe extern "thiscall" fn(usize, usize, i32, usize, u32) = core::mem::transmute(0x8db3a0usize);
        noticia(this, e, parte, o, da_oferta);
    }
    let da_clausula = cqb6_clube(cqb6_d(e + 5));
    let fin_recebe = cqb6_financas(da_clausula);
    let fin_paga = cqb6_financas(transfer_manager_club_of_contract(cqb6_contrato(p)));
    let do_contrato = transfer_manager_club_of_contract(cqb6_contrato(p));
    if !CQB6_ENSAIO {
        let totais: unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32 = core::mem::transmute(0x448ad0usize);
        totais(cqb6_d(0xad9dac), parte, do_contrato, cqb6_clube(cqb6_d(e + 5)));
    }
    post_finance_amount(0x599800, fin_recebe, parte);
    post_finance_amount(0x599890, fin_paga, parte);
    1
}

/// O que a 0x8cb0f0 vai mexer, para as regiões do comparador: as finanças do clube da cláusula e do clube do
/// contrato, se a cláusula existe (sem gravar nada além de [0xae1080], que a original regrava).
unsafe fn clause_finance_regions(this: usize, p: usize) -> Option<(usize, usize)> {
    let c = cqb6_contrato(p);
    if c == 0 || transfer_manager_club_of_contract(c) == 0 {
        return None;
    }
    let mut chave = [0u8; 0xd];
    core::ptr::write_unaligned(chave.as_mut_ptr().add(9) as *mut usize, cqb6_d(p));
    core::ptr::write_unaligned(chave.as_mut_ptr().add(1) as *mut usize, cqb6_d(transfer_manager_club_of_contract(c)));
    let busca: unsafe extern "C" fn(*const u8, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let e = busca(chave.as_ptr(), cqb6_d(this + 0x880), cqb6_d(this + 0x888), 0xd, 0x8e54d0);
    if e == 0 {
        return None;
    }
    Some((cqb6_financas(cqb6_clube(cqb6_d(e + 5))), cqb6_financas(transfer_manager_club_of_contract(c))))
}

/// Comparador da 0x8cb0f0: a cópia em ensaio (sem a notícia e sem os totais do clube) e a original a partir do mesmo
/// estado, lado a lado nas finanças dos dois clubes, em [0xae1080] e no sorteio (a 0x4d0c60 pode sortear); pago (a
/// notícia e os totais da original podem regravar [0xae1080]), só as finanças e a devolução.
unsafe extern "thiscall" fn comparar_pay_next_sale_percentage(this: usize, p: usize, o: usize) -> u32 {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    if let Some((a, b)) = clause_finance_regions(this, p) {
        regioes.push((a, 0x167));
        regioes.push((b, 0x167));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || original(this, p, o), || {
        CQB6_ENSAIO = true;
        let n = crate::aleatorio::sem_contar(|| pay_next_sale_percentage(this, p, o));
        CQB6_ENSAIO = false;
        n
    });
    let igual = r == n && (igual || (r != 0 && mo.0[3..] == mr.0[3..]));
    PLACAR.registrar(igual, || format!("pay_next_sale_percentage(pessoa {:#x}, oferta {:#x}) original={} {:x?} rust={} {:x?}", p, o, r, mo, n, mr));
    r
}

/// { 0x8db3a0 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH sem destrutor; o this não é usado):
/// `TRANSFER_MANAGER::post_sale_percentage_news(cláusula, parte, oferta, da_oferta)`, pelo percentual pago (0x8cb0f0).
/// A notícia 0xfa7 (local): 0..3 a pessoa da cláusula (+9: id e os +0x33 de +4/+8/+0xc), 4 = o percentual (byte +0),
/// 5 = o clube da cláusula (+5), 6 = o clube vendedor (+1), 7 = a parte, 8 = `da_oferta`, 9 = o comprador (+8 da
/// oferta) com `da_oferta`; senão o id do clube do contrato (0x4cf5a0; sem contrato ou sem clube: acaba sem entregar).
/// Vai aos técnicos do clube da cláusula e do vendedor (0x7861e0).
unsafe extern "thiscall" fn post_sale_percentage_news(_this: usize, e: usize, parte: i32, o: usize, da_oferta: u32) {
    crate::comparacao::ramo("cqb_6 noticia do write_percent_pair");
    let p = cqb6_pessoa(cqb6_d(e + 9));
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    transfer_manager_construct_news_item(n, 0xfa7);
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
        if c == 0 || transfer_manager_club_of_contract(c) == 0 {
            return;
        }
        let c2 = cqb6_contrato(p);
        // ponytail: a original lê [0] se o segundo contrato faltasse; inalcançável (o mesmo contrato acabou de pass_ball)
        if c2 != 0 { cqb6_d(transfer_manager_club_of_contract(c2)) } else { usize::MAX }
    };
    cqb6_gravar(n, 9, v9 as u32);
    send_news_to_club_manager(n, cqb6_clube(cqb6_d(e + 5)));
    send_news_to_club_manager(n, cqb6_clube(cqb6_d(e + 1)));
}

/// Comparador da 0x8db3a0: só a original (a notícia é entregue e não pode sair duas vezes; a cópia leva o ramo no
/// modo Rust). ponytail: sem plano a conferir além do que a original entrega.
unsafe extern "thiscall" fn comparar_post_sale_percentage_news(this: usize, e: usize, parte: i32, o: usize, da_oferta: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, usize, u32)>(%%I2%%)(this, e, parte, o, da_oferta)
}
