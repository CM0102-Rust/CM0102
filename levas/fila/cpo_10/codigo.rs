// ---------- %%LEVA%%a leva: as transferências: o prazo da resposta à oferta, pagar as prestações do mês e tirar a oferta (transfer_manager.cpp) ----------

unsafe fn cpo10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo10_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpo10_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpo10_w(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpo10_clube(id: usize) -> usize {
    cpo10_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpo10_pessoa(id: usize) -> usize {
    cpo10_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// A regra de transferência da nação `nacao` entre as 28 de `this + 0x8ac` (a de +0xa igual), ou 0.
unsafe fn nation_transfer_rules_or_zero(this: usize, nacao: usize) -> usize {
    for k in 0..0x1cusize {
        let r = cpo10_d(this + 0x8ac + k * 4);
        if cpo10_d(r + 0xa) == nacao {
            return r;
        }
    }
    0
}

/// { 0x8cc5b0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::get_offer_response_delay(oferta, dias,
/// fatias)`: o prazo é o de fechar a janela (0x8e8ed0) da nação do comprador (+8; sem regra: 365) e, com o +4 da oferta,
/// o menor com o da nação do clube do contrato do jogador (+0xc; contrato de tipo 5 não conta). Abaixo de 3: 0 dia e 1
/// fatia; abaixo de 6 ou de 11: 1 ou 2 dias e 1..3 fatias sorteadas. Senão o valor pelo estatuto do contrato com o clube
/// (+0x3d; sem ele −100): o jogador (0x53b620 do +0x11; sem ficha, a caixa "Error") de habilidade +7 da ficha acima de
/// 130 e da reputação do clube / 60, menos de 26 anos, em menos de 3 listas e estatuto positivo, com o interesse
/// (0x846920) acima de [0x968e58]: o prazo × [0x968e18] até [0x968dd8]; senão um sorteio até estatuto/10 + 12 (o maior
/// com 1..3). Dias = valor / 3, fatias = valor % 3.
unsafe extern "thiscall" fn get_offer_response_delay(this: usize, oferta: usize, dias: *mut u8, fatias: *mut u8) {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let janela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8e8ed0usize);
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let comprador = cpo10_clube(cpo10_d(oferta + 8));
    let p = cpo10_pessoa(cpo10_d(oferta + 0xc));
    let c = contract_of(cpo10_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let dono = if c != 0 && cpo10_b(c + 0x35) & 0x3f != 5 { cpo10_clube(cpo10_d(c + 4)) } else { 0 };
    let k = com_clube(cpo10_d(0xae1050), p, dono);
    let mut prazo: u16 = 0x16d;
    let r = nation_transfer_rules_or_zero(this, cpo10_d(comprador + 0x53));
    if r != 0 {
        prazo = janela(r) as u16;
    }
    if cpo10_d(oferta + 4) != 0 {
        let r = nation_transfer_rules_or_zero(this, cpo10_d(dono + 0x53));
        if r != 0 && prazo >= janela(r) as u16 {
            prazo = janela(r) as u16;
        }
    }
    if prazo < 3 {
        *dias = 0;
        *fatias = 1;
        return;
    }
    if prazo < 0xb {
        *dias = if prazo < 6 { 1 } else { 2 };
        *fatias = (sortear(3) as u8).wrapping_add(1);
        return;
    }
    let estatuto: i8 = if k != 0 { cpo10_b(k + 0x3d) as i8 } else { -100 };
    // ponytail: a original empurra o cargo num registrador com lixo acima; a 0x53b620 só lê o byte
    let e_jogador: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x53b620usize);
    if e_jogador(cpo10_b(oferta + 0x11) as u32) != 0 && cpo10_d(p + 0x61) == 0 {
        show_error_at(0xa8d514, 0x29c3);
    }
    let mut valor: Option<u8> = None;
    if dono != 0 && e_jogador(cpo10_b(oferta + 0x11) as u32) != 0 {
        let hab = cpo10_h(cpo10_d(p + 0x61) + 7);
        if hab > 0x82
            && hab as i32 > cpo10_h(dono + 0x80) as i32 / 60
            && (cpo10_b(p + 0x18) as i8) < 0x1a
            && cpo10_b(cpo10_d(0xae2d0c).wrapping_add(cpo10_d(p).wrapping_mul(0x4f)) + 0xb) & 0xf < 3
            && estatuto > 0
        {
            let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(0x846920usize);
            if interesse(cpo10_d(this + 0x856), p, comprador, 0, 0, 1, 0) > read_f64_at(0x968e58) {
                let x = prazo as i32 as f64 * read_f64_at(0x968e18);
                let teto = read_f64_at(0x968dd8);
                valor = Some(match_pl_ftol(if teto < x || x.is_nan() { teto } else { x }) as u8);
            }
        }
    }
    let v = match valor {
        Some(v) => v,
        None => {
            crate::comparacao::ramo("cpo_10 prazo sorteado");
            let s = sortear(estatuto as i32 / 10 + 0xc) as i8;
            if sortear(3) + 1 > s as i32 {
                (sortear(3) as u8).wrapping_add(1)
            } else {
                s as u8
            }
        }
    } as i8 as i32;
    *dias = (v / 3) as u8;
    *fatias = (v % 3) as u8;
}

/// { 0x8caee0 } (thiscall, ret 0, transfer_manager.cpp): `TRANSFER_MANAGER::pay_transfer_instalments()`: no fim do mês,
/// cada prestação da lista (+0x872; 13 bytes: +0 o valor, +4 o clube que paga, +8 o que recebe, +0xc as que faltam) é
/// paga (0x599890 nas finanças do pagador, 0x599800 nas do recebedor, 0x5a0590) e conta uma a menos; faltando menos de
/// 4 ou de 8, a figura das prestações (+0x1c das finanças) recebe as frações ([0x968918]/[0x968f30] ou
/// [0x967880]/[0x969518]); acabada, sai da lista (0x68d1f0); senão os dois clubes ficam marcados (0x945501). Depois, em
/// cada clube com finanças, a figura zera: sem marca; positiva só pagando; negativa só recebendo.
unsafe extern "thiscall" fn pay_transfer_instalments(this: usize) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let livrar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let pagar: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(0x599890usize);
    let receber: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(0x599800usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let recebe = calloc(cpo10_d(0xae235c) as u32, 1);
    let paga = calloc(cpo10_d(0xae235c) as u32, 1);
    let lista = this + 0x872;
    cpo10_w(lista + 0xa, cpo10_d(this + 0x874) as u32);
    loop {
        let e = proximo(lista);
        if e == 0 {
            break;
        }
        let fr = financas(cpo10_d(0xae2a38), cpo10_clube(cpo10_d(e + 8)));
        let fp = financas(cpo10_d(0xae2a38), cpo10_clube(cpo10_d(e + 4)));
        pagar(fp, cpo10_i(e) as i64);
        receber(fr, cpo10_i(e) as i64);
        let n = (cpo10_b(e + 0xc) as i8).wrapping_sub(1);
        *((e + 0xc) as *mut i8) = n;
        let fracoes = if n < 4 { Some((0x968918, 0x968f30)) } else if n < 8 { Some((0x967880, 0x969518)) } else { None };
        if let Some((a, b)) = fracoes {
            cpo10_w(fp + 0x1c, cpo10_i(fp + 0x1c).wrapping_add(match_pl_ftol(cpo10_i(e) as f64 * read_f64_at(a))) as u32);
            cpo10_w(fr + 0x1c, cpo10_i(fr + 0x1c).wrapping_add(match_pl_ftol(cpo10_i(e) as f64 * read_f64_at(b))) as u32);
        }
        if cpo10_b(e + 0xc) == 0 {
            crate::comparacao::ramo("cpo_10 prestacao acabada");
            tirar(lista, e);
        } else {
            *((recebe + cpo10_d(cpo10_clube(cpo10_d(e + 8)))) as *mut u8) = 1;
            *((paga + cpo10_d(cpo10_clube(cpo10_d(e + 4)))) as *mut u8) = 1;
        }
    }
    let mut i = 0usize;
    while (i as i32) < cpo10_i(0xae235c) {
        let f = financas(cpo10_d(0xae2a38), cpo10_d(0xae23b4).wrapping_add(i * 0x245));
        if f != 0 {
            let (r, p) = (cpo10_b(recebe + i) != 0, cpo10_b(paga + i) != 0);
            let v = cpo10_i(f + 0x1c);
            let zerar = match (r, p) {
                (false, false) => true,
                (false, true) => v > 0,
                (true, false) => v < 0,
                (true, true) => false,
            };
            if zerar {
                cpo10_w(f + 0x1c, 0);
            }
        }
        i += 1;
    }
    if recebe != 0 {
        livrar(recebe);
    }
    if paga != 0 {
        livrar(paga);
    }
}

/// { 0x8d0f60 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::remove_offer(oferta)`: a oferta ligada
/// (+0x28 ≠ −1, em +0xc) é destruída (0x8e5710, 0x944c94), a vaga zera e a conta +0x14 baixa; com o +0x1f em 1, sai da
/// lista de empréstimos (+0x946) o par (clube +8, jogador +0xc); para cada jogador da troca (+4): a proposta de contrato
/// ligada à vaga +0x20 sai (0x4ccb30, se o jogador da oferta tem contrato) e o +2 do registro de 8 bytes dele (+0x86e)
/// baixa; sai a proposta de contrato do jogador com o comprador (0x4cca70), o +1 do registro do jogador baixa, a menor
/// vaga livre (+0x85e) acompanha, a vaga +0x20 do vetor (+0) zera, a oferta é destruída e a conta +8 baixa.
unsafe extern "thiscall" fn remove_offer(this: usize, oferta: usize) {
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8e5710usize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    crate::comparacao::ramo("cpo_10 tirar oferta");
    if cpo10_i(oferta + 0x28) != -1 {
        let x = cpo10_d(cpo10_d(this + 0xc).wrapping_add((cpo10_i(oferta + 0x28) as usize).wrapping_mul(4)));
        if x != 0 {
            destruir(x);
            apagar(x);
        }
        cpo10_w(cpo10_d(this + 0xc).wrapping_add((cpo10_i(oferta + 0x28) as usize).wrapping_mul(4)), 0);
        cpo10_w(this + 0x14, cpo10_i(this + 0x14).wrapping_sub(1) as u32);
    }
    if cpo10_b(oferta + 0x1f) == 1 && cpo10_d(this + 0x946) != 0 {
        let l = cpo10_d(this + 0x946);
        cpo10_w(l + 0xa, cpo10_d(l + 2) as u32);
        loop {
            let e = proximo(cpo10_d(this + 0x946));
            if e == 0 {
                break;
            }
            if cpo10_d(e + 4) == cpo10_d(oferta + 8) && cpo10_d(e) == cpo10_d(oferta + 0xc) {
                let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
                tirar(cpo10_d(this + 0x946), e);
                break;
            }
        }
    }
    if cpo10_d(oferta + 4) != 0 {
        let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let tirar_proposta: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(0x4ccb30usize);
        let l = cpo10_d(oferta + 4);
        cpo10_w(l + 0xa, cpo10_d(l + 2) as u32);
        loop {
            let e = proximo(cpo10_d(oferta + 4));
            if e == 0 {
                break;
            }
            let c = contract_of(cpo10_d(0xae1050), cpo10_pessoa(cpo10_d(oferta + 0xc)));
            *(0xae1080 as *mut usize) = c;
            if c != 0 && cpo10_clube(cpo10_d(c + 4)) != 0 {
                tirar_proposta(cpo10_d(0xae1050), cpo10_pessoa(cpo10_d(e)), cpo10_i(oferta + 0x20));
            }
            let q = (cpo10_d(this + 0x86e).wrapping_add(cpo10_d(e).wrapping_mul(8)) + 2) as *mut u8;
            *q = (*q).wrapping_sub(1);
        }
    }
    let tirar_do_clube: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cca70usize);
    tirar_do_clube(cpo10_d(0xae1050), cpo10_pessoa(cpo10_d(oferta + 0xc)), cpo10_clube(cpo10_d(oferta + 8)));
    let q = (cpo10_d(this + 0x86e).wrapping_add(cpo10_d(oferta + 0xc).wrapping_mul(8)) + 1) as *mut u8;
    *q = (*q).wrapping_sub(1); // as ofertas pela pessoa
    if cpo10_i(oferta + 0x20) < cpo10_i(this + 0x85e) {
        cpo10_w(this + 0x85e, cpo10_d(oferta + 0x20) as u32);
    }
    cpo10_w(cpo10_d(this).wrapping_add((cpo10_i(oferta + 0x20) as usize).wrapping_mul(4)), 0);
    if oferta != 0 {
        destruir(oferta);
        apagar(oferta);
    }
    cpo10_w(this + 8, cpo10_i(this + 8).wrapping_sub(1) as u32);
}

/// Os dados dos nós de uma LLIST (+2 o primeiro nó; nó: +0 o dado, +6 o próximo).
unsafe fn cpo10_itens(lista: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = cpo10_d(lista + 2);
    while no != 0 {
        v.push(cpo10_d(no));
        no = cpo10_d(no + 6);
    }
    v
}

/// Comparador: lado a lado pelas duas saídas, o sorteio, [0xae1080] e [0xb67a34] (a caixa da falta de ficha).
unsafe extern "thiscall" fn comparar_get_offer_response_delay(this: usize, oferta: usize, dias: *mut u8, fatias: *mut u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(%%I0%%);
    let mut reg = vec![(dias as usize, 1usize), (fatias as usize, 1), (0xae1080, 4), (0xb67a34, 4)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(
        &reg,
        || o(this, oferta, dias, fatias),
        || crate::aleatorio::sem_contar(|| get_offer_response_delay(this, oferta, dias, fatias)),
    );
    PLACAR.registrar(igual, || format!("get_offer_response_delay({:#x}) {:x?} x {:x?}", oferta, mo, mr));
}

/// Comparador: só a original, conferindo o plano (feito antes): saem da lista as prestações que tinham 1 a pagar.
// ponytail: a original paga (finanças), tira itens da lista (libera) e aloca as marcas; não roda duas vezes. A cópia
// tem os ramos e a prova é pela carreira.
unsafe extern "thiscall" fn comparar_pay_transfer_instalments(this: usize) {
    let lista = this + 0x872;
    let antes = cpo10_itens(lista);
    let acabam = antes.iter().filter(|&&e| cpo10_b(e + 0xc) == 1).count();
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let depois = cpo10_itens(lista).len();
    PLACAR.registrar(depois + acabam == antes.len(), || format!("pay_transfer_instalments: {} prestações, {} acabando, {} depois", antes.len(), acabam, depois));
}

/// Comparador: só a original, conferindo o plano (feito antes): a vaga zerada, as contas +8 e +0x14, a menor vaga livre
/// e o +1 do registro do jogador.
// ponytail: a original destrói e libera a oferta e mexe nas listas; não roda duas vezes.
unsafe extern "thiscall" fn comparar_remove_offer(this: usize, oferta: usize) {
    let vaga = cpo10_i(oferta + 0x20);
    let ligada = cpo10_i(oferta + 0x28) != -1;
    let (n8, n14, livre) = (cpo10_i(this + 8), cpo10_i(this + 0x14), cpo10_i(this + 0x85e));
    let q = cpo10_d(this + 0x86e).wrapping_add(cpo10_d(oferta + 0xc).wrapping_mul(8)) + 1;
    let ofertas = cpo10_b(q);
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, oferta);
    let ok = cpo10_d(cpo10_d(this).wrapping_add((vaga as usize).wrapping_mul(4))) == 0
        && cpo10_i(this + 8) == n8 - 1
        && cpo10_i(this + 0x14) == n14 - ligada as i32
        && cpo10_i(this + 0x85e) == livre.min(vaga)
        && cpo10_b(q) == ofertas.wrapping_sub(1);
    PLACAR.registrar(ok, || format!("remove_offer: vaga {} contas {}/{} -> {}/{} livre {} -> {} ofertas {} -> {}",
        vaga, n8, n14, cpo10_i(this + 8), cpo10_i(this + 0x14), livre, cpo10_i(this + 0x85e), ofertas, cpo10_b(q)));
}
