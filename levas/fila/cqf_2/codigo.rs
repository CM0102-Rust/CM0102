// ---------- %%LEVA%%a leva: os termos aceitos pelo jogador e o empréstimo concluído na hora (transfer_manager.cpp) ----------

unsafe fn cqf2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf2_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqf2_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

unsafe fn cqf2_pessoa(id: usize) -> usize {
    cqf2_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqf2_clube(id: usize) -> usize {
    cqf2_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080].
unsafe fn cqf2_contrato(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cqf2_d(0xae1050), pessoa);
    cqf2_pd(0xae1080, c);
    c
}

/// O status da oferta deixa seguir: 2, 3, 0x14, 0x10, 4, ou 0xf com +0x2e ≥ 3.
unsafe fn cqf2_segue(e: usize) -> bool {
    match cqf2_b(e + 0x2c) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => cqf2_b(e + 0x2e) >= 3,
        _ => false,
    }
}

/// O bit 0 do registro de 8 bytes do jogador da oferta ([this+0x86e]) apaga quando ela e todas as ofertas pelo jogador
/// (a lista +0) seguem (`cqf2_segue`).
unsafe fn clear_under_offer_mark(this: usize, oferta: usize) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    if !cqf2_segue(oferta) {
        return;
    }
    let lista = cqf2_d(oferta);
    if lista != 0 {
        cqf2_pd(lista + 0xa, cqf2_d(lista + 2));
        let mut n = proximo(cqf2_d(oferta));
        while n != 0 {
            if !cqf2_segue(cqf2_d(cqf2_d(this).wrapping_add(4 * cqf2_d(n)))) {
                return;
            }
            n = proximo(cqf2_d(oferta));
        }
    }
    let r = cqf2_d(this + 0x86e).wrapping_add(cqf2_d(oferta + 0xc).wrapping_mul(8));
    if cqf2_b(r) & 1 != 0 {
        cqf2_pb(r, cqf2_b(r) & 0xfe);
    }
}

/// A regra de transferência da nação (os 28 objetos em [this+0x8ac], o +0xa de cada um comparado com `nacao`; a
/// original lê o +0xa sem conferir o ponteiro); None sem nenhuma.
unsafe fn nation_transfer_rules_opt(this: usize, nacao: usize) -> Option<usize> {
    for k in 0..0x1c {
        let p = this + 0x8ac + 4 * k;
        if cqf2_d(cqf2_d(p) + 0xa) == nacao {
            return Some(cqf2_d(p));
        }
    }
    None
}

/// { 0x8c2180 } (thiscall, ret 0x14, transfer_manager.cpp, quadro SEH da data local sem destrutor):
/// `TRANSFER_MANAGER::handle_player_accepted_terms(pessoa, vaga, contrato, tipo, noticia)`, pela aceitação de um
/// contrato (0x4cf413). Vaga fora do vetor: a caixa "Error" (0xe0c, 0xe11) e nada; vaga vazia: nada. O +0x2e zera. O
/// alvo da oferta aceitou: +0x10 = 2 e +0x11 = tipo; A = o comprador, B = o clube do contrato do alvo (tipo 5 não
/// conta). Senão a pessoa é um jogador da troca: o nó dela na troca (+4) ganha 2 e o tipo (+4, +6); A = o clube do
/// contrato do alvo, B = o comprador (sem troca: a caixa 0xe4d; sem o nó, nada achado: A e B a original deixa sem
/// valor, a cópia 0). Fora do empréstimo (+0x1f ≠ 1): a notícia da resposta (0x8d8e90 com A, B, 2 e a oferta) e o
/// seguimento da oferta (0x8c2f50). No empréstimo: elenco de A (0x53c590) + 1 acima de 50, o motivo 4; senão a regra
/// da nação de A (+0x53) responde (vtable +0x20 com o comprador e o alvo) com o motivo. Com motivo: a recusa
/// (0x8e23d0), o status 0x10, o fim (0x8c6db0), o alvo sai das listas do comprador e os da troca das do clube do
/// contrato do alvo (0x84aeb0), 5 dias (0x8c6220) e a marca (`clear_under_offer_mark`). Sem motivo, o empréstimo na hora:
/// os termos (0x8e1f10), a regra da nação do comprador (vtable +8, ou +4 fora do empréstimo), o contrato assinado
/// (0x4d01e0), o +0x6d 2 da pessoa vira 0x10, o histórico (0x86da20), a lista de observação (0x842080/0x843e10 com 5),
/// 0x5563f0, os números (0x85a850, 0x540980, 0x85add0/0x85a470), 0x4196a0, o registro da transferência passada
/// (0x8cb810: a pessoa, os +0x33 de +4, +8 e +0xc dela, os ids de A e B, 0, 2 e hoje), o status 0x14 com 5 dias, a
/// marca, o fim (0x8c6db0) e a pessoa fora das listas (0x842b00 com 1).
unsafe extern "thiscall" fn handle_player_accepted_terms(this: usize, pessoa: usize, vaga: i32, contrato: usize, tipo: u32, noticia: u32) {
    let noticia_resposta: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, usize, u32, u32, u32) = core::mem::transmute(0x8d8e90usize);
    let seguir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c2f50usize);
    let elenco: unsafe extern "C" fn(usize, u32, u32) -> i8 = core::mem::transmute(0x53c590usize);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8e23d0usize);
    let fechar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c6db0usize);
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    let termos: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8e1f10usize);
    let assinar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4d01e0usize);
    let historico: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x86da20usize);
    let observado: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x842080usize);
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x843e10usize);
    let guardar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5563f0usize);
    let numero: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x85a850usize);
    let marcar_elenco: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x540980usize);
    let usa_numeros: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x85add0usize);
    let dar_numero: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x85a470usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let acabar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4196a0usize);
    let nova_data: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let copiar_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let registrar: unsafe extern "thiscall" fn(usize, *mut u8) = core::mem::transmute(0x8cb810usize);
    let suspender: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842b00usize);
    if vaga < 0 {
        show_error_at(0xa8d514, 0xe0c);
        return;
    }
    if vaga >= cqf2_d(this + 4) as i32 {
        show_error_at(0xa8d514, 0xe11);
        return;
    }
    let oferta = cqf2_d(cqf2_d(this).wrapping_add(4 * vaga as usize));
    if oferta == 0 {
        return;
    }
    cqf2_pb(oferta + 0x2e, 0);
    // a = edi da original, b = [esp+0x1c]
    let (mut a, mut b) = (0usize, 0usize);
    if cqf2_d(pessoa) == cqf2_d(oferta + 0xc) {
        cqf2_pb(oferta + 0x10, 2);
        cqf2_pb(oferta + 0x11, tipo as u8);
        a = cqf2_clube(cqf2_d(oferta + 8));
        let c = cqf2_contrato(pessoa);
        b = if c == 0 || cqf2_b(c + 0x35) & 0x3f == 5 { 0 } else { cqf2_clube(cqf2_d(c + 4)) };
    } else if cqf2_d(oferta + 4) == 0 {
        show_error_at(0xa8d514, 0xe4d);
    } else {
        let troca = cqf2_d(oferta + 4);
        cqf2_pd(troca + 0xa, cqf2_d(troca + 2));
        let mut n = proximo(cqf2_d(oferta + 4));
        while n != 0 {
            if cqf2_d(n) == cqf2_d(pessoa) {
                cqf2_pb(n + 4, 2);
                cqf2_pb(n + 6, tipo as u8);
                let c = cqf2_contrato(cqf2_pessoa(cqf2_d(oferta + 0xc)));
                a = if c != 0 { cqf2_clube(cqf2_d(c + 4)) } else { 0 };
                b = cqf2_clube(cqf2_d(oferta + 8));
                break;
            }
            n = proximo(cqf2_d(oferta + 4));
        }
    }
    if cqf2_b(oferta + 0x1f) != 1 {
        // ponytail: a resposta (2) vai num dword com bytes altos de sobra na original; a 0x8d8e90 lê o byte
        noticia_resposta(this, pessoa, a, b, 2, oferta, 0, noticia, 0);
        crate::comparacao::ramo("cqf_2 seguir a oferta");
        seguir(this, oferta); // cqf2: a oferta segue
        return;
    }
    crate::comparacao::ramo("cqf_2 empréstimo");
    let mut motivo: u32 = 0;
    if (elenco(a, 0, 0) as i32) + 1 > 0x32 { // cqf2: o elenco cheio
        motivo = 4;
    } else if let Some(r) = nation_transfer_rules_opt(this, cqf2_d(a + 0x53)) {
        if r != 0 {
            let pode: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u32) = core::mem::transmute(cqf2_d(cqf2_d(r) + 0x20));
            pode(r, cqf2_clube(cqf2_d(oferta + 8)), cqf2_pessoa(cqf2_d(oferta + 0xc)), 0, &mut motivo);
        }
    }
    if motivo as u16 != 0 {
        crate::comparacao::ramo("cqf_2 empréstimo recusado");
        recusar(this, oferta, motivo);
        cqf2_pb(oferta + 0x2c, 0x10);
        fechar(this, oferta);
        let alvo = cqf2_pessoa(cqf2_d(oferta + 0xc));
        let c = cqf2_contrato(alvo);
        let dono = if c != 0 { cqf2_clube(cqf2_d(c + 4)) } else { 0 };
        desmarcar(cqf2_d(this + 0x856), cqf2_clube(cqf2_d(oferta + 8)), alvo);
        let troca = cqf2_d(oferta + 4);
        if troca != 0 {
            cqf2_pd(troca + 0xa, cqf2_d(troca + 2));
            let mut n = proximo(cqf2_d(oferta + 4));
            while n != 0 {
                desmarcar(cqf2_d(this + 0x856), dono, cqf2_pessoa(cqf2_d(n)));
                n = proximo(cqf2_d(oferta + 4));
            }
        }
        agendar(this, oferta, 5, 0);
        clear_under_offer_mark(this, oferta);
        return;
    }
    crate::comparacao::ramo("cqf_2 empréstimo na hora");
    termos(this, oferta, contrato);
    if let Some(r) = nation_transfer_rules_opt(this, cqf2_d(cqf2_clube(cqf2_d(oferta + 8)) + 0x53)) {
        if r != 0 {
            let k = if cqf2_b(oferta + 0x1f) == 0 { 4 } else { 8 };
            let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(cqf2_d(cqf2_d(r) + k));
            f(r, oferta);
        }
    }
    assinar(cqf2_d(0xae1050), contrato);
    if cqf2_b(pessoa + 0x6d) == 2 {
        cqf2_pb(pessoa + 0x6d, 0x10);
    }
    historico(cqf2_d(0xdd7f00), pessoa, b, a);
    if observado(cqf2_d(this + 0x856), pessoa, a) != 0 {
        situacao(cqf2_d(this + 0x856), pessoa, a, 5, 0);
    }
    guardar(cqf2_d(0xae28f0), cqf2_pessoa(cqf2_d(oferta + 0xc)));
    numero(cqf2_d(0xdd7ef0), cqf2_pessoa(cqf2_d(oferta + 0xc)), a, 0);
    marcar_elenco(pessoa, 1);
    if humano(a, 0, 1, 0, 0) == 0 && usa_numeros(cqf2_d(0xdd7ef0), a, 0) != 0 {
        dar_numero(cqf2_d(0xdd7ef0), a, pessoa);
    }
    acabar(cqf2_d(0xad9c6c), pessoa);
    // o registro da transferência passada, 0x25 bytes: +0 a pessoa, +4/+8/+0xc os +0x33 dos ponteiros +4/+8/+0xc dela,
    // +0x10 e +0x14 os ids de A e B, +0x18 0, +0x1c 2, +0x1d hoje
    let mut reg = [0u8; 0x28];
    let r = reg.as_mut_ptr() as usize;
    nova_data(r + 0x1d);
    cqf2_pd(r + 0x10, cqf2_d(a));
    cqf2_pd(r + 0x14, cqf2_d(b));
    let mut tmp = [0u8; 8];
    copiar_data(r + 0x1d, tmp.as_mut_ptr(), 0xae2c90);
    cqf2_pd(r, cqf2_d(pessoa));
    cqf2_pd(r + 4, cqf2_d(cqf2_d(pessoa + 4) + 0x33));
    cqf2_pd(r + 8, cqf2_d(cqf2_d(pessoa + 8) + 0x33));
    cqf2_pd(r + 0xc, cqf2_d(cqf2_d(pessoa + 0xc) + 0x33));
    cqf2_pb(r + 0x1c, 2);
    cqf2_pd(r + 0x18, 0);
    registrar(this, reg.as_mut_ptr());
    cqf2_pb(oferta + 0x2c, 0x14);
    agendar(this, oferta, 5, 0); // cqf2: o empréstimo concluído em 5 dias
    clear_under_offer_mark(this, oferta);
    fechar(this, oferta);
    suspender(cqf2_d(this + 0x856), pessoa, 1);
}

/// Comparador da 0x8c2180: só a original (entrega notícias, conclui ou recusa o empréstimo, assina contrato, registra a
/// transferência, mexe nas listas).
unsafe extern "thiscall" fn comparar_handle_player_accepted_terms(this: usize, pessoa: usize, vaga: i32, contrato: usize, tipo: u32, noticia: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, usize, u32, u32)>(%%I0%%)(this, pessoa, vaga, contrato, tipo, noticia) // ponytail: só a original
}

/// { 0x4d71b0 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da lista local): `CONTRACT_MANAGER::
/// post_transfer_listed_news(jogador, clube)`, pelo pedido de transferência (0x4d8610), pelo jogador posto à venda
/// (0x842320), pela notícia (0x784100) e por 0x4dc3a0. Sem jogador, sem ficha (+0x61), sem clube ou sem a comissão
/// (+0xcf): nada. Sem contrato com o clube (0x4cf5f0): nada. A notícia 0xbea (local): 0 o clube, 1 o jogador, 2..4 os
/// +0x33 de +4, +8 e +0xc dele, 5 a comissão, 6..8 os +0x33 dela, 9 o contrato +0x45, 10 o nibble alto de +0x4f, 11 a
/// máscara de posições (0x5b7d20), 12 um sorteio de 2 (0x90cfc0). O preço: o valor (+0x52) com clube humano, quando o
/// jogador não está na lista do clube (0x5444a0) ou o contrato tem o nibble 1; senão o pedido (0x4d1600) × (sorteio de
/// 0x29 × [0x967898] + [0x9689b0]) truncado, arredondado para baixo em 10 mil (entre 10 mil e 200 mil, exclusive), 100 mil
/// (abaixo de 1 milhão), 250 mil (abaixo de 5 milhões) ou 1 milhão; 13 o preço. De 500 mil para cima: os interessados
/// (0x842180), até 30, os do computador e os humanos com verba (0x598800) ≥ preço × [0x9689c0]; ordenados pela reputação
/// (+0x80); 14 = até 3; 15.. os ids, com 1 (no lugar 13) se o 2º repete o 1º e 2 (no 12) se o 3º repete um deles; por
/// fim o "fora da lista" no lugar seguinte. Os destinatários (0x8d1da0 com 0x21a, até 30, com [0xde1f54] e ficha) e a
/// entrega (0x7861e0).
unsafe extern "thiscall" fn post_transfer_listed_news(this: usize, jogador: usize, clube: usize) {
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let lista_fim: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let mascara: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(0x5b7d20usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let pedido: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x842180usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let verba: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i64 = core::mem::transmute(0x598800usize);
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1da0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    if jogador == 0 || cqf2_d(jogador + 0x61) == 0 || clube == 0 || cqf2_d(clube + 0xcf) == 0 {
        return;
    }
    let c = contrato_com(this, jogador, clube);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    lista_nova(l);
    if c != 0 {
        let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
        let n = local.as_mut_ptr() as usize;
        construir(n, 0xbea, 0);
        gravar(n, 0, cqf2_d(clube) as u32);
        gravar(n, 1, cqf2_d(jogador) as u32);
        gravar(n, 2, cqf2_d(cqf2_d(jogador + 4) + 0x33) as u32);
        gravar(n, 3, cqf2_d(cqf2_d(jogador + 8) + 0x33) as u32);
        gravar(n, 4, cqf2_d(cqf2_d(jogador + 0xc) + 0x33) as u32);
        let com = cqf2_d(clube + 0xcf);
        gravar(n, 5, cqf2_d(com) as u32);
        gravar(n, 6, cqf2_d(cqf2_d(cqf2_d(clube + 0xcf) + 4) + 0x33) as u32);
        gravar(n, 7, cqf2_d(cqf2_d(cqf2_d(clube + 0xcf) + 8) + 0x33) as u32);
        gravar(n, 8, cqf2_d(cqf2_d(cqf2_d(clube + 0xcf) + 0xc) + 0x33) as u32);
        gravar(n, 9, cqf2_d(c + 0x45) as u32);
        gravar(n, 10, (cqf2_b(c + 0x4f) >> 4) as u32);
        gravar(n, 11, mascara(cqf2_d(jogador + 0x61)) as i32 as u32);
        crate::comparacao::ramo("cqf_2 notícia do listado");
        let s = sorteio(2); // cqf2: o sorteio do dado 12
        gravar(n, 12, s as u32);
        let fora_da_lista = (listado(jogador, clube) == 0) as u32;
        let preco = if humano(clube, 0, 1, 0, 0) != 0 && (fora_da_lista == 0 || cqf2_b(c + 0x4f) & 0xf0 == 0x10) {
            cqf2_d(jogador + 0x52) as i32
        } else {
            let p = pedido(this, clube, 0, jogador, 0, 1, 1);
            let r = sorteio(0x29);
            let v = match_pl_ftol((r as f64 * read_f64_at(0x967898) + read_f64_at(0x9689b0)) * p as f64); // cqf2: o preço da notícia
            if v > 0x2710 && v < 0x30d40 {
                v / 10000 * 10000
            } else if v < 1_000_000 {
                v / 100000 * 100000
            } else if v < 5_000_000 {
                v / 250000 * 250000
            } else {
                v / 1_000_000 * 1_000_000
            }
        };
        gravar(n, 0xd, preco as u32);
        if preco >= 500_000 {
            crate::comparacao::ramo("cqf_2 interessados no listado");
            let mut clubes = [0usize; 30];
            let mut q: i8 = 0;
            interessados(cqf2_d(cqf2_d(0xde1f54) + 0x856), jogador, l, 1, 1, 0);
            cqf2_pd(l + 0xa, cqf2_d(l + 2));
            let mut no = proximo(l);
            while no != 0 && q < 0x1e {
                let id = cqf2_d(no + 4);
                let pode = if humano(cqf2_clube(id), 0, 1, 0, 0) == 0 {
                    true
                } else {
                    let f = financas(cqf2_d(0xae2a38), cqf2_clube(cqf2_d(no + 4)));
                    f != 0 && preco as f64 * read_f64_at(0x9689c0) <= verba(f, 0, 0, 0, 1, 0) as f64
                };
                if pode && cqf2_d(no + 4) as i32 > -1 {
                    clubes[q as usize] = cqf2_clube(cqf2_d(no + 4));
                    q += 1;
                }
                no = proximo(l);
            }
            let total = q as i32;
            let rep = |c: usize| core::ptr::read_unaligned((c + 0x80) as *const i16);
            let mut i = 0i32;
            while i < total - 1 {
                let mut j = i + 1;
                while j < total {
                    if rep(clubes[i as usize]) < rep(clubes[j as usize]) {
                        clubes.swap(i as usize, j as usize);
                    }
                    j += 1;
                }
                i += 1;
            }
            let m = if q >= 3 { 3 } else { total };
            gravar(n, 0xe, m as u32);
            let mut lugar: i8 = 0xf;
            let mut k: i8 = 0;
            let fim = loop {
                if k as i32 >= m {
                    break lugar;
                }
                if k == 1 && clubes[1] == clubes[0] {
                    gravar(n, lugar - 2, 1);
                    break lugar;
                }
                if k == 2 && (clubes[2] == clubes[0] || clubes[2] == clubes[1]) {
                    gravar(n, lugar - 3, 2);
                    break lugar;
                }
                gravar(n, lugar, cqf2_d(clubes[k as usize]) as u32);
                lugar += 1;
                k += 1;
            };
            // ponytail: o lugar vai num dword com os bytes altos do preço na original; a 0x785790 lê o byte
            gravar(n, fim, fora_da_lista);
            let mut dest = clubes;
            let qd = if cqf2_d(0xde1f54) != 0 && cqf2_d(jogador + 0x61) != 0 {
                destinatarios(cqf2_d(0xde1f54), jogador, 0x21a, dest.as_mut_ptr(), 0x1e) as i8
            } else {
                0
            };
            for &d in dest.iter().take(if qd > 0 { qd as usize } else { 0 }) {
                entregar(cqf2_d(0xb7431c), n, d);
            }
        }
    }
    lista_fim(l);
}

/// Comparador da 0x4d71b0: só a original (sorteia, entrega a notícia, que aloca).
unsafe extern "thiscall" fn comparar_post_transfer_listed_news(this: usize, jogador: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%)(this, jogador, clube) // ponytail: só a original
}