// ---------- %%LEVA%%a leva: a aprovação final da oferta pelos clubes, o fechamento da oferta aprovada e o acerto das cláusulas das transferências anteriores (transfer_manager.cpp) ----------

unsafe fn cpw2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw2_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpw2_clube(id: usize) -> usize {
    cpw2_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpw2_pessoa(id: usize) -> usize {
    cpw2_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// Volta a lista ao começo (o cursor +0xa recebe a cabeça +2).
unsafe fn cpw2_inicio(l: usize) {
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpw2_d(l + 2));
}

/// O próximo item da lista (0x68d1b0), ou 0.
unsafe fn cpw2_proximo(l: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    proximo(l)
}

/// Tira a pessoa das listas de troca das outras ofertas (TRANSFER_MANAGER::remove_exchange_player 0x8c41c0 com a
/// oferta, 0, 0).
unsafe fn cpw2_tirar_da_troca(this: usize, pessoa: usize, oferta: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x8c41c0usize);
    f(this, pessoa, oferta, 0, 0);
}

/// Agenda a oferta (0x8c6220: thiscall(this, oferta, dias, fatias)).
unsafe fn cpw2_agendar(this: usize, oferta: usize, dias: u32, fatias: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    f(this, oferta, dias, fatias);
}

/// O estado (+0x2c) da oferta depois da aprovação do `clube`: o comprador (+8) leva 0xb a 0xd e 0xc a 0xe; o outro
/// lado (ou sem clube) leva 0xb a 0xc e 0xd a 0xe. Os outros estados ficam.
unsafe fn cpw2_estado_aprovado(oferta: usize, clube: usize) -> u8 {
    let s = cpw2_b(oferta + 0x2c);
    if clube != 0 && cpw2_d(clube) == cpw2_d(oferta + 8) {
        match s {
            0xb => 0xd,
            0xc => 0xe,
            _ => s,
        }
    } else {
        match s {
            0xb => 0xc,
            0xd => 0xe,
            _ => s,
        }
    }
}

/// { 0x8c3310 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da lista local): `TRANSFER_MANAGER::
/// confirm_offer_by_club(oferta, clube)`, pela oferta passada aos clubes (0x8c2f50), pelas notícias e pela tela: a
/// aprovação do clube muda o estado (`cpw2_estado_aprovado`); com os dois de acordo (0xe): sem prazo (+0x30 & 0xf), a
/// oferta é agendada para hoje (0x8c6220 com 0, 1); com prazo, a data de conclusão (0x8cee70) ainda por vir faz a
/// transferência futura: uma LLIST local que não usa (0x68d020/0x68d040), o jogador (+0xc) e os da troca (lista +4)
/// saem das outras ofertas (0x8c41c0, e 0x8c47d0 para os da troca), as ofertas guardadas pelo jogador saem (0x4cf560),
/// a notícia (0x8e05d0) e a transferência futura (0x8cb9a0) com a data; data já chegada: agendada para hoje.
unsafe extern "thiscall" fn confirmar_oferta_pelo_clube(this: usize, oferta: usize, clube: usize) {
    let data_padrao: unsafe extern "thiscall" fn(*mut [u32; 2]) -> usize = core::mem::transmute(0x54a110usize);
    let mut data = [0u32; 2];
    data_padrao(&mut data);
    let s = cpw2_estado_aprovado(oferta, clube);
    *((oferta + 0x2c) as *mut u8) = s;
    if s != 0xe {
        return;
    }
    'futura: {
        if cpw2_b(oferta + 0x30) & 0xf == 0 {
            break 'futura;
        }
        let conclusao: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16 = core::mem::transmute(0x8cee70usize);
        conclusao(this, oferta, &mut data);
        let hoje = core::ptr::read_unaligned(0xae2c90 as *const [u16; 4]);
        let d = [data[0] as u16, (data[0] >> 16) as u16];
        let antes = hoje[1] < d[1] || (hoje[1] == d[1] && (hoje[0] as i16) < (d[0] as i16));
        if !antes { // cpw2: a data de conclusão ainda por vir
            break 'futura;
        }
        crate::comparacao::ramo("cpw_2 transferencia futura");
        let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let desfazer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c47d0usize);
        let guardadas: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x4cf560usize);
        let noticia: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) = core::mem::transmute(0x8e05d0usize);
        let futura: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) = core::mem::transmute(0x8cb9a0usize);
        let mut lista = [0u8; 0xe];
        let l = lista.as_mut_ptr() as usize;
        lista_nova(l);
        cpw2_tirar_da_troca(this, cpw2_pessoa(cpw2_d(oferta + 0xc)), oferta);
        if cpw2_d(oferta + 4) != 0 {
            cpw2_inicio(cpw2_d(oferta + 4));
            let mut x = cpw2_proximo(cpw2_d(oferta + 4));
            while x != 0 {
                cpw2_tirar_da_troca(this, cpw2_pessoa(cpw2_d(x)), oferta);
                desfazer(this, cpw2_pessoa(cpw2_d(x)));
                x = cpw2_proximo(cpw2_d(oferta + 4));
            }
        }
        destruir(l);
        guardadas(cpw2_d(0xae1050), cpw2_d(oferta + 0xc) as u32);
        noticia(this, oferta, &mut data);
        futura(this, oferta, &mut data);
        return;
    }
    crate::comparacao::ramo("cpw_2 agendar conclusao");
    cpw2_agendar(this, oferta, 0, 1); // cpw2: a conclusão agendada para hoje
}

/// Comparador: sem chegar ao acordo dos dois (estado final diferente de 0xe), só o estado muda: lado a lado sobre a
/// oferta. Com o acordo, a oferta é agendada ou vira transferência futura (mexe nas outras ofertas e aloca): só a
/// original.
unsafe extern "thiscall" fn comparar_confirmar_oferta_pelo_clube(this: usize, oferta: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    if cpw2_estado_aprovado(oferta, clube) == 0xe {
        return o(this, oferta, clube);
    }
    let regioes = [(oferta, 0x31usize)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta, clube), || confirmar_oferta_pelo_clube(this, oferta, clube));
    PLACAR.registrar(igual, || format!("confirmar_oferta_pelo_clube(oferta {:#x}, clube {:#x}) original={:x?} rust={:x?}", oferta, clube, mo, mr));
}

/// { 0x8c34e0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::try_complete_offer(oferta, no_prazo)`, pela
/// oferta agendada (0x8c63a0) e pelo fim da janela (0x8c4e10). O vendedor é o clube do contrato do jogador (+0xc;
/// 0x4cf5a0, guardado em [0xae1080]; tipo 5 não conta); o comprador (+8) reconfirma (0x8cfc90, que dá o motivo): fora do
/// prazo com motivo 9, não conta; comprador do computador com resposta 1: a recusa (0x8c36b0); resposta 1 ou 2: a oferta
/// se desfaz (0x8c44e0 com o motivo, 1). O mesmo com o vendedor. Os dois de acordo: o jogador e os da troca saem das
/// outras ofertas (0x8c6ae0), as ofertas guardadas pelo jogador saem (0x4cf560), a notícia (0x8dbef0); com confirmação
/// (+0x2f), vendedor e prazo do fim do contrato (+0x30 & 0xf = 3), a transferência futura no fim do contrato (0x8cb9a0
/// com +0x2d do contrato); senão a transferência se conclui (0x8c9390).
unsafe extern "thiscall" fn tentar_concluir_oferta(this: usize, oferta: usize, no_prazo: u32) {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let reconfirmar: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, u32) -> u8 = core::mem::transmute(0x8cfc90usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let recusa: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x8c36b0usize);
    let desfazer: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x8c44e0usize);
    let p = cpw2_pessoa(cpw2_d(oferta + 0xc));
    let c = contrato_de(cpw2_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let vendedor = if c != 0 && cpw2_b(c + 0x35) & 0x3f != 5 { cpw2_clube(cpw2_d(c + 4)) } else { 0 };
    let comprador = cpw2_clube(cpw2_d(oferta + 8));
    // ponytail: a 0x8cfc90 grava só a palavra baixa do motivo; a original empurra o dword (a palavra alta é sobra de
    // pilha). A cópia parte de zero, um valor que o jogo também produz.
    let mut motivo: u32 = 0;
    let r = reconfirmar(this, oferta, comprador, &mut motivo, 0);
    if no_prazo == 0 || motivo as u16 != 9 {
        if humano(comprador, 0, 1, 0, 0) == 0 && r == 1 {
            return recusa(this, oferta, comprador, 0);
        }
        if r == 2 || r == 1 {
            return desfazer(this, oferta, comprador, motivo, 1);
        }
    }
    if vendedor != 0 {
        let r = reconfirmar(this, oferta, vendedor, &mut motivo, 0);
        if humano(vendedor, 0, 1, 0, 0) == 0 && r == 1 {
            return recusa(this, oferta, vendedor, 0);
        }
        if r == 2 || r == 1 {
            return desfazer(this, oferta, vendedor, motivo, 1);
        }
    }
    let soltar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c6ae0usize);
    let guardadas: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x4cf560usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8dbef0usize);
    soltar(this, oferta);
    guardadas(cpw2_d(0xae1050), cpw2_d(oferta + 0xc) as u32);
    noticia(this, oferta);
    if cpw2_b(oferta + 0x2f) != 0 && vendedor != 0 && cpw2_b(oferta + 0x30) & 0xf == 3 {
        let c = contrato_de(cpw2_d(0xae1050), p);
        if c != 0 {
            crate::comparacao::ramo("cpw_2 futura no fim do contrato");
            let futura: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8cb9a0usize);
            return futura(this, oferta, c + 0x2d);
        }
    }
    let concluir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c9390usize);
    crate::comparacao::ramo("cpw_2 concluir agora");
    concluir(this, oferta); // cpw2: a transferência se conclui
}

// ponytail: conclui transferências, desfaz ofertas e dá notícias (aloca, não roda duas vezes): só a original; a cópia
// leva os ramos e se prova pela carreira e pelas mutações.
unsafe extern "thiscall" fn comparar_tentar_concluir_oferta(this: usize, oferta: usize, no_prazo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I1%%)(this, oferta, no_prazo)
}

/// As cláusulas antigas de uma pessoa pelas marcas do registro de 8 bytes em [this+0x86e]: o percentual da próxima
/// venda (bit 8: 0x8cb0f0 paga, 0x8cb4f0 tira, este da pessoa `tirar`), o prêmio por jogo no time (bit 2: 0x8cb340 com
/// −1, 2 e tira 0x8cb590) e o pela seleção (bit 4: 0x8cb340 com −1, 5 e tira 0x8cb630).
unsafe fn cpw2_clausulas(this: usize, pessoa: usize, tirar: usize, oferta: usize) {
    let marca = |p: usize| cpw2_b(cpw2_d(this + 0x86e).wrapping_add(cpw2_d(p).wrapping_mul(8)));
    let pagar_venda: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8cb0f0usize);
    let tirar_venda: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb4f0usize);
    let pagar_jogos: unsafe extern "thiscall" fn(usize, usize, i32, u32, usize) -> u32 = core::mem::transmute(0x8cb340usize);
    let tirar_jogos: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb590usize);
    let tirar_selecao: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb630usize);
    if marca(pessoa) & 8 != 0 {
        crate::comparacao::ramo("cpw_2 percentual da venda anterior");
        pagar_venda(this, pessoa, oferta);
        tirar_venda(this, tirar);
    }
    if marca(pessoa) & 2 != 0 {
        crate::comparacao::ramo("cpw_2 premio por jogo anterior");
        pagar_jogos(this, pessoa, -1, 2, oferta);
        tirar_jogos(this, pessoa);
    }
    if marca(pessoa) & 4 != 0 {
        pagar_jogos(this, pessoa, -1, 5, oferta);
        tirar_selecao(this, pessoa);
    }
}

/// { 0x8c91e0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::settle_previous_transfer_clauses(oferta)`,
/// pela transferência concluída (0x8c9390): as cláusulas antigas do jogador (+0xc) e das pessoas da troca (lista +4,
/// cursor recomeçado; a original tira o percentual da próxima venda do jogador da oferta, não o da pessoa da troca).
unsafe extern "thiscall" fn acertar_clausulas_anteriores(this: usize, oferta: usize) {
    let p = cpw2_pessoa(cpw2_d(oferta + 0xc));
    cpw2_clausulas(this, p, p, oferta);
    let l = cpw2_d(oferta + 4);
    if l == 0 {
        return;
    }
    cpw2_inicio(l);
    let mut x = cpw2_proximo(cpw2_d(oferta + 4));
    while x != 0 {
        cpw2_clausulas(this, cpw2_pessoa(cpw2_d(x)), p, oferta);
        x = cpw2_proximo(cpw2_d(oferta + 4));
    }
}

/// Comparador: sem marca de cláusula (bits 2, 4 e 8) no jogador nem nas pessoas da troca, só o cursor da lista muda:
/// lado a lado sobre a lista e as marcas. Com alguma, paga (finanças, notícias que alocam): só a original.
unsafe extern "thiscall" fn comparar_acertar_clausulas_anteriores(this: usize, oferta: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    let marcas = cpw2_d(this + 0x86e);
    let tem = |id: usize| cpw2_b(marcas.wrapping_add(id.wrapping_mul(8))) & 0xe != 0;
    let l = cpw2_d(oferta + 4);
    let mut regioes = vec![(marcas.wrapping_add(cpw2_d(oferta + 0xc).wrapping_mul(8)), 8usize)];
    let mut alguma = tem(cpw2_d(oferta + 0xc));
    if l != 0 {
        regioes.push((l, 0xe));
        // o percurso sem mexer no cursor da lista: os nós (+0 o item, +6 o próximo) a partir da cabeça +2
        let mut no = cpw2_d(l + 2);
        let mut n = 0;
        while no != 0 && n < 64 {
            let item = cpw2_d(no);
            if item != 0 && tem(cpw2_d(item)) {
                alguma = true;
            }
            no = cpw2_d(no + 6);
            n += 1;
        }
        if n >= 64 {
            alguma = true;
        }
    }
    if alguma {
        return o(this, oferta);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta), || acertar_clausulas_anteriores(this, oferta));
    PLACAR.registrar(igual, || format!("acertar_clausulas_anteriores(oferta {:#x}) original={:x?} rust={:x?}", oferta, mo, mr));
}
