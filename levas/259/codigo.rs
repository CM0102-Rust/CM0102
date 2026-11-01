// ---------- 259a leva: a satisfação do jogador com o contrato, a notícia de que ele quer contrato novo e a do pedido de transferência retirado (staff_contracts.cpp, contract_manager.cpp) ----------

unsafe fn cpw1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpw1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpw1_clube(id: usize) -> usize {
    cpw1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpw1_pessoa(id: usize) -> usize {
    cpw1_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O valor de um contrato para a pessoa (STAFF_CONTRACT::calc_contract_value 0x866830: thiscall(contrato, pessoa,
/// oferta), ret 8).
unsafe fn cpw1_valor(contrato: usize, pessoa: usize, oferta: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x866830usize);
    f(contrato, pessoa, oferta)
}

/// { 0x866620 } (thiscall, ret 4, staff_contracts.cpp, quadro SEH sem destrutor): `STAFF_CONTRACT::
/// get_contract_satisfaction(pessoa)`, pelo passe diário do pessoal (0x5ce2d0, com sorteio(50) zero) e pela tela do
/// contrato: 1 a 3. Sem pessoa, 1. A situação no elenco que ela merece (0x4ca470) vale se for menor que a do contrato
/// (+0x4f >> 4) e não for 0 nem 7; senão, a do contrato. O contrato desejado (0x85fc50 num contrato local, com o clube
/// do contrato, −1, a situação e 0xb) e a razão 100 × valor do contrato atual (0x866830, em f32) / valor do desejado
/// (valor zero: razão 0). Contratado há menos de 15 × adaptabilidade (+0x59) + 100 dias (de +0x25, teto 500): razão +
/// 50 + 7,5 × adaptabilidade − dias / 2. Razão abaixo de 50: 1; abaixo de 80: 3; senão 2.
unsafe extern "thiscall" fn satisfacao_com_o_contrato(this: usize, pessoa: usize) -> u8 {
    if pessoa == 0 {
        return 1;
    }
    let categoria: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(0x4ca470usize);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, u32, u32) = core::mem::transmute(0x85fc50usize);
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    // o contrato local (0x50 bytes) como a original o deixa antes da 0x85fc50; o resto ela preenche
    let mut local = [0u8; 0x50];
    let l = local.as_mut_ptr() as usize;
    data_padrao(l + 0x25);
    data_padrao(l + 0x2d);
    core::ptr::write_unaligned((l + 4) as *mut i32, -1);
    core::ptr::write_unaligned((l + 0x4a) as *mut i32, -1);
    let merece = categoria(pessoa, this);
    let atual = cpw1_b(this + 0x4f) >> 4;
    // ponytail: a original empurra o dword inteiro da vaga onde guardou o byte (no primeiro caso, o resto é o ponteiro
    // da pessoa; no segundo, sobra de pilha); a 0x85fc50 só lê o byte.
    let situacao = if (merece as i8) < (atual as i8) && merece != 0 && merece != 7 {
        (pessoa as u32 & !0xff) | merece as u32
    } else {
        atual as u32
    };
    desejado(l, pessoa, cpw1_clube(cpw1_d(this + 4)), -1, situacao, 0xb, 0, 0);
    let v1 = cpw1_valor(this, pessoa, l) as f32;
    let v2 = cpw1_valor(l, pessoa, l) as f64;
    let mut razao = if v2 == f32_em(0x968970) as f64 {
        f32_em(0x968970) as f64
    } else {
        v1 as f64 * f32_em(0x968960) as f64 / v2
    };
    let anos = (cpw1_w(0xae2c92) as u16 as i32).wrapping_sub(cpw1_w(this + 0x27) as u16 as i32);
    let dias = anos.wrapping_mul(365).wrapping_sub(cpw1_w(this + 0x25) as i32).wrapping_add(cpw1_w(0xae2c90) as i32).clamp(0, 500);
    let adapt = cpw1_b(pessoa + 0x59) as i8 as i32;
    if dias < adapt.wrapping_mul(15).wrapping_add(100) {
        crate::comparacao::ramo("cpw_1 contratado ha pouco");
        razao = razao + f32_em(0x969560) as f64 + adapt as f64 * f64_em(0x968eb0) - (dias / 2) as f64; // cpw1: o desconto do recém-contratado
    }
    crate::comparacao::ramo("cpw_1 satisfacao avaliada");
    if !(razao >= f32_em(0x969560) as f64) { // cpw1: o limite da insatisfação
        return 1;
    }
    if razao < f32_em(0x96ff70) as f64 { // cpw1: o limite da satisfação
        3
    } else {
        2
    }
}

/// Comparador: a original e a cópia a partir do mesmo estado (o sorteio, o último contrato pedido [0xae1080] que a
/// 0x85fc50 grava e o contrato); a devolução e a memória.
unsafe extern "thiscall" fn comparar_satisfacao_com_o_contrato(this: usize, pessoa: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1272);
    let mut regioes = vec![(0xae1080usize, 4usize), (this, 0x50)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(this, pessoa),
        || crate::aleatorio::sem_contar(|| satisfacao_com_o_contrato(this, pessoa)),
    );
    PLACAR.registrar(igual && r == n, || {
        format!("satisfacao_com_o_contrato(pessoa {:#x}) original={} {:x?} rust={} {:x?}", if pessoa != 0 { cpw1_d(pessoa) } else { 0 }, r, mo, n, mr)
    });
    r
}

/// Os dados da pessoa numa notícia: o id na vaga 2 e as três chaves (+0x33 de +4, +8 e +0xc) nas vagas 3 a 5.
unsafe fn cpw1_dados_da_pessoa(n: usize, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    gravar(n, 2, cpw1_d(p) as u32);
    gravar(n, 3, cpw1_d(cpw1_d(p + 4) + 0x33) as u32);
    gravar(n, 4, cpw1_d(cpw1_d(p + 8) + 0x33) as u32);
    gravar(n, 5, cpw1_d(cpw1_d(p + 0xc) + 0x33) as u32);
}

/// { 0x4d9f80 } (thiscall, ret 4, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_wants_new_contract_news(contrato)`, pelo passe diário do pessoal (0x5ce2d0). Sem contrato: a caixa "Error"
/// (linha 0x2d91). Pessoa na lista de transferências (0x8c1870), com ofertas (0x8c5cf0) ou oferecida em troca (0x8c5e40):
/// nada. Senão a notícia 0xbc2 ao técnico do clube do contrato (0x7861e0): o clube, a pessoa, a situação no elenco (+0x4f
/// >> 4), 0, o fim do contrato (+0x2d dia com sinal, +0x2f ano), o salário (+0xc) e, na vaga 11, 1 se a adaptabilidade
/// (+0x59) for menor que 10, a +0x5b menor que 5 e a reputação da ficha (+0xd) passar de 4750.
unsafe extern "thiscall" fn noticia_quer_contrato_novo(this: usize, contrato: usize) {
    let _ = this;
    if contrato == 0 {
        return aviso_em(0x9a8190, 0x2d91);
    }
    let na_lista: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8c1870usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let clube = cpw1_clube(cpw1_d(contrato + 4));
    let p = cpw1_pessoa(cpw1_d(contrato));
    let tm = cpw1_d(0xde1f54);
    if na_lista(tm, p) != 0 || ofertas(tm, p, 0, 0) != 0 || trocas(tm, p, 0) != 0 {
        return;
    }
    crate::comparacao::ramo("cpw_1 noticia do contrato novo");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbc2, 0);
    gravar(n, 0, cpw1_d(clube) as u32);
    cpw1_dados_da_pessoa(n, p);
    gravar(n, 6, (cpw1_b(contrato + 0x4f) >> 4) as u32);
    gravar(n, 7, 0);
    gravar(n, 8, cpw1_w(contrato + 0x2d) as i32 as u32);
    gravar(n, 9, cpw1_w(contrato + 0x2f) as u16 as u32);
    gravar(n, 10, cpw1_d(contrato + 0xc) as u32);
    let ficha = cpw1_d(p + 0x61);
    let reputado = (cpw1_b(p + 0x59) as i8) < 0xa && (cpw1_b(p + 0x5b) as i8) < 5 && ficha != 0 && cpw1_w(ficha + 0xd) > 0x128e;
    gravar(n, 11, reputado as u32);
    entregar(cpw1_d(0xb7431c), n, clube);
}

// ponytail: a notícia vai à caixa do técnico (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_quer_contrato_novo(this: usize, contrato: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1273)(this, contrato)
}

/// { 0x4d9460 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_transfer_request_withdrawn_news(contrato, motivo)`. Sem contrato: a caixa "Error" (linha 0x2ba6). Senão a
/// notícia 0xbc0 ao técnico do clube do contrato (0x7861e0): o clube, a pessoa, o motivo e, na vaga 7, 1 se +0x3f do
/// contrato passar de 1.
unsafe extern "thiscall" fn noticia_pedido_de_transferencia_retirado(this: usize, contrato: usize, motivo: u32) {
    let _ = this;
    if contrato == 0 {
        return aviso_em(0x9a8190, 0x2ba6);
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparacao::ramo("cpw_1 pedido retirado");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbc0, 0);
    let clube = cpw1_clube(cpw1_d(contrato + 4));
    let p = cpw1_pessoa(cpw1_d(contrato));
    gravar(n, 0, cpw1_d(clube) as u32);
    cpw1_dados_da_pessoa(n, p);
    gravar(n, 6, motivo);
    gravar(n, 7, ((cpw1_b(contrato + 0x3f) as i8) > 1) as u32);
    entregar(cpw1_d(0xb7431c), n, clube);
}

// ponytail: a notícia vai à caixa do técnico (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_pedido_de_transferencia_retirado(this: usize, contrato: usize, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1274)(this, contrato, motivo)
}

// ---------- 259a leva: a aprovação final da oferta pelos clubes, o fechamento da oferta aprovada e o acerto das cláusulas das transferências anteriores (transfer_manager.cpp) ----------

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
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1275);
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
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1276)(this, oferta, no_prazo)
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
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1277);
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

// ---------- 259a leva: a oferta de empréstimo pelo jogador da lista, a notícia do contrato acabando e a da saída pela lei Bosman (shortlist_manager.cpp, contract_manager.cpp) ----------

unsafe fn cpw3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpw3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpw3_clube(id: usize) -> usize {
    cpw3_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpw3_pessoa(id: usize) -> usize {
    cpw3_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0).
unsafe fn cpw3_contrato(p: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    contrato_de(cpw3_d(0xae1050), p)
}

/// O interesse da pessoa em ir para o clube (Shortlist_Manager::staff_interest_in_joining_club 0x846920: thiscall(lista,
/// pessoa, clube, salário, modo, situação, motivos), ret 0x18, float no st0).
unsafe fn cpw3_interesse(lista: usize, pessoa: usize, clube: usize, salario: u32, modo: u32, situacao: u32) -> f64 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, usize) -> f64 = core::mem::transmute(0x846920usize);
    f(lista, pessoa, clube, salario, modo, situacao, 0)
}

/// O ensaio do comparador: a cópia roda sem a oferta de empréstimo (0x8bed60) nem a atualização da lista (0x843e10),
/// que alocam e mexem nas listas; devolve −2 onde faria a oferta e −3 onde atualizaria a lista.
static mut CPW3_ENSAIO: bool = false;

/// A data (dia i16, ano u16) de 8 bytes em `a`, como a original a compara (o ano sem sinal, o dia com sinal).
unsafe fn cpw3_antes(a: usize, b: usize) -> bool {
    let (aa, ab) = (cpw3_w(a + 2) as u16, cpw3_w(b + 2) as u16);
    aa < ab || (aa == ab && cpw3_w(a) < cpw3_w(b))
}

/// { 0x842740 } (thiscall, ret 8, shortlist_manager.cpp, quadro SEH sem destrutor): `Shortlist_Manager::
/// try_loan_offer_for_player(jogador, clube)`, pela avaliação das listas dos clubes (0x84a020): a vaga da oferta de
/// empréstimo, ou −1. Sem dono (contrato 0x4cf5a0, guardado em [0xae1080]): a vaga da lista (0x849ce0) volta ao estado 1
/// (0x843e10) e −1. O interesse (0x846920, com a situação da vaga) abaixo de 3,5: −1. O período do empréstimo (0x8d0590):
/// os meses até o fim e até o início (no mínimo 1); início depois do fim: −1; clube da nação 0x9cf408 fora de março a
/// setembro: −1. O interesse sem salário de 2,75 (fora das nações 0x9cf488, 0x9cf434, 0x9cf360) conta do início; senão do
/// fim. A data de volta (dia de hoje, no máximo 28; 0x549f70); sem poder encerrar o empréstimo (0x8d0530), depois do fim
/// do contrato, antes de 4 semanas do começo da janela da nação-chave (0x669360, [0xb63d8a]) ou a menos de 3 semanas de
/// hoje: −1. A porcentagem do salário (100 com reputação 2500 acima do dono, 0 com 1500 abaixo, senão 50) e a oferta
/// (0x8bed60).
unsafe extern "thiscall" fn tentar_emprestimo_pelo_jogador(this: usize, jogador: usize, clube: usize) -> i32 {
    let entrada_da_lista: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let entrada = entrada_da_lista(this, clube, jogador, 0, 0);
    let c = cpw3_contrato(jogador);
    *(0xae1080 as *mut usize) = c;
    let dono = if c != 0 { cpw3_clube(cpw3_d(c + 4)) } else { 0 };
    if dono == 0 {
        if entrada != 0 {
            if CPW3_ENSAIO {
                return -3;
            }
            let estado: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x843e10usize);
            estado(this, jogador, clube, 1, 0);
        }
        return -1;
    }
    // ponytail: a original empurra a situação da vaga num registrador cujo resto é o endereço do clube dono (base +
    // id); a cópia reproduz o dword.
    let situacao = (cpw3_d(0xae23b4).wrapping_add(cpw3_d(c + 4)) as u32 & !0xff) | ((cpw3_b(entrada + 0x24) >> 1) & 0xf) as u32;
    let f1 = cpw3_interesse(cpw3_d(cpw3_d(0xde1f54) + 0x856), jogador, clube, 0, 1, situacao);
    crate::comparacao::ramo("cpw_3 interesse no emprestimo");
    if !(f1 >= f64_em(0x968e38)) { // cpw3: o interesse mínimo
        return -1;
    }
    let mes_de: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x54cfb0usize);
    let m = mes_de(0xae2c90) as u8;
    let c = cpw3_contrato(jogador);
    *(0xae1080 as *mut usize) = c;
    let a2 = if c != 0 { cpw3_clube(cpw3_d(c + 4)) } else { 0 };
    let periodo: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i8, *mut u16, *mut i8, *mut u16) = core::mem::transmute(0x8d0590usize);
    // ponytail: o mês do fim vai para a vaga do argumento do clube (o byte baixo), o resto fica; os outros são locais
    let (mut ini_mes, mut ini_ano, mut mes, mut ano) = (0i8, 0u16, clube as u8 as i8, 0u16);
    periodo(cpw3_d(0xde1f54), clube, a2, jogador, &mut ini_mes, &mut ini_ano, &mut mes, &mut ano);
    let hoje_ano = cpw3_w(0xae2c92) as u16;
    let m16 = m as i8 as i16;
    let ate_fim = (ano.wrapping_sub(hoje_ano) as i16).wrapping_mul(12).wrapping_sub(m16).wrapping_add(mes as i16);
    let ate_ini = (ini_ano.wrapping_sub(hoje_ano) as i16).wrapping_mul(12).wrapping_sub(m16).wrapping_add(ini_mes as i16);
    let inicio = if ate_ini >= 1 { ate_ini } else { 1 };
    if inicio > ate_fim {
        return -1;
    }
    let nacao = cpw3_d(clube + 0x53);
    if nacao != 0 && cpw3_d(nacao) == cpw3_d(0x9cf408) && (m as i8 >= 0xa || m as i8 <= 2) {
        return -1;
    }
    let f2 = cpw3_interesse(this, jogador, clube, 0, 0, 1);
    let mut meses = ate_fim as u8;
    crate::comparacao::ramo("cpw_3 conta do inicio");
    if f2 >= f64_em(0x9690a8) { // cpw3: o interesse para contar do início
        let n = cpw3_d(cpw3_d(clube + 0x53));
        if n != cpw3_d(0x9cf488) && n != cpw3_d(0x9cf434) && n != cpw3_d(0x9cf360) {
            meses = inicio as u8;
        }
    }
    let mut mes_volta = meses.wrapping_add(m);
    // ponytail: o ano vai na vaga do argumento do jogador (a palavra baixa; o resto é o endereço do jogador), e a
    // porcentagem depois no byte baixo dela; a cópia reproduz o dword.
    let mut vaga = (jogador as u32 & 0xffff_0000) | hoje_ano as u32;
    if mes_volta as i8 > 0xb {
        let q = (mes_volta as i8 as i32 as u32) / 12;
        mes_volta = mes_volta.wrapping_add((q as u8).wrapping_mul(0xf4));
        vaga = vaga.wrapping_add(q);
    }
    let dia_do_mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x54cf70usize);
    let dia = dia_do_mes(0xae2c90) as i16;
    // a original relê o dia (segunda chamada) quando ele não passa de 28
    let dia = if dia > 0x1c { 0x1c } else { dia_do_mes(0xae2c90) as i16 as i32 };
    let data_dma: unsafe extern "thiscall" fn(*mut [u16; 4], i32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut volta = [0u16; 4];
    data_dma(&mut volta, dia, mes_volta as u32, vaga, 0xffff_ffff);
    let v = volta.as_ptr() as usize;
    let encerrar: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(0x8d0530usize);
    if encerrar(cpw3_d(0xde1f54), clube, jogador, 0, v, 1) == 0 {
        return -1;
    }
    let c = cpw3_contrato(jogador);
    if c == 0 || cpw3_antes(c + 0x2d, v) {
        return -1;
    }
    let chave: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669360usize);
    let k = chave(clube) as u8 as i8 as i32;
    let janela = (0xb63d8a + k * 72) as usize;
    let menos_semanas: unsafe extern "thiscall" fn(usize, *mut [u16; 4], u32) -> usize = core::mem::transmute(0x54cab0usize);
    let mais_semanas: unsafe extern "thiscall" fn(usize, *mut [u16; 4], u32) -> usize = core::mem::transmute(0x54c770usize);
    let mut t = [0u16; 4];
    let r = menos_semanas(v, &mut t, 4);
    if cpw3_antes(r, janela) {
        return -1;
    }
    crate::comparacao::ramo("cpw_3 tres semanas");
    let r = mais_semanas(0xae2c90, &mut t, 3); // cpw3: as 3 semanas a partir de hoje
    if cpw3_antes(v, r) {
        return -1;
    }
    let dif = cpw3_w(clube + 0x80) as i32 - cpw3_w(dono + 0x80) as i32;
    let pct = if dif > 0x9c4 { 100 } else if dif <= -0x5dc { 0 } else { 50 };
    let vaga = (vaga & !0xff) | pct;
    if CPW3_ENSAIO {
        return -2;
    }
    let oferta: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x8bed60usize);
    oferta(cpw3_d(0xde1f54), jogador, clube, v, vaga, 1, 1) as i32
}

/// O comparador: a cópia em ensaio e a original a partir do mesmo estado ([0xae1080], o sorteio e as estáticas das
/// operações de data). A cópia recusou (−1): a original também, com a mesma memória. A cópia atualizaria a lista (−3):
/// a original devolve −1. A cópia faria a oferta (−2): a original não devolve −1.
unsafe extern "thiscall" fn comparar_tentar_emprestimo_pelo_jogador(this: usize, jogador: usize, clube: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(1278);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    regioes.extend(CPQ5_ESTATICAS);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW3_ENSAIO = true;
    let n = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| tentar_emprestimo_pelo_jogador(this, jogador, clube)));
    CPW3_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let r = o(this, jogador, clube);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = match n {
        -1 => r == -1 && mo == mr,
        -3 => r == -1,
        _ => r != -1,
    };
    PLACAR.registrar(igual, || format!("tentar_emprestimo_pelo_jogador(jogador {:#x}, clube {:#x}) original={} {:x?} rust={} {:x?}", cpw3_d(jogador), cpw3_d(clube), r, mo, n, mr));
    r
}

/// Os dados da pessoa numa notícia: o id na vaga 2 e as três chaves (+0x33 de +4, +8 e +0xc) nas vagas 3 a 5.
unsafe fn cpw3_dados_da_pessoa(n: usize, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    gravar(n, 2, cpw3_d(p) as u32);
    gravar(n, 3, cpw3_d(cpw3_d(p + 4) + 0x33) as u32);
    gravar(n, 4, cpw3_d(cpw3_d(p + 8) + 0x33) as u32);
    gravar(n, 5, cpw3_d(cpw3_d(p + 0xc) + 0x33) as u32);
}

/// A nação está ativa (0x544670, cdecl)?
unsafe fn cpw3_nacao_ativa(nacao: usize) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x544670usize);
    f(nacao) != 0
}

/// { 0x4d6790 } (thiscall, ret 0x10, contract_manager.cpp, quadro SEH da notícia e da lista locais): `CONTRACT_MANAGER::
/// post_contract_expiry_news(contrato, período, bosman, avisar_interessados)`, pelo passe dos contratos. Sem contrato: a
/// caixa "Error" (linha 0x24f0). Senão a notícia 0xbbd ao técnico do clube (0x7861e0): o clube, a pessoa, o período (com
/// sinal), `bosman` e, na vaga 8, se a pessoa tem ficha (+0x61). Com `avisar_interessados` (anulado se a nação do clube,
/// +0x53, não estiver ativa, 0x544670): os clubes interessados no jogador (0x842180 numa LLIST local, com 1, 1, 0) de
/// outra nação ativa também a recebem.
unsafe extern "thiscall" fn noticia_contrato_acabando(this: usize, contrato: usize, periodo: u32, bosman: u32, avisar: u32) {
    let _ = this;
    if contrato == 0 {
        return aviso_em(0x9a8190, 0x24f0);
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbbd, 0);
    let clube = cpw3_clube(cpw3_d(contrato + 4));
    let p = cpw3_pessoa(cpw3_d(contrato));
    gravar(n, 0, cpw3_d(clube) as u32);
    cpw3_dados_da_pessoa(n, p);
    gravar(n, 6, periodo as u8 as i8 as i32 as u32);
    gravar(n, 7, bosman);
    gravar(n, 8, (cpw3_d(p + 0x61) != 0) as u32);
    let nacao = cpw3_d(clube + 0x53);
    let avisar = if nacao != 0 && !cpw3_nacao_ativa(nacao) { 0 } else { avisar };
    entregar(cpw3_d(0xb7431c), n, clube);
    if avisar == 0 {
        return;
    }
    crate::comparacao::ramo("cpw_3 avisar interessados");
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x842180usize);
    let mut lista = [0u8; 0xe];
    let l = lista.as_mut_ptr() as usize;
    lista_nova(l);
    interessados(cpw3_d(cpw3_d(0xde1f54) + 0x856), p, l, 1, 1, 0);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpw3_d(l + 2));
    let mut x = proximo(l);
    while x != 0 {
        let id = cpw3_d(x + 4);
        if id != cpw3_d(clube) {
            let nac = cpw3_d(cpw3_clube(id) + 0x53);
            if nac != 0 && cpw3_nacao_ativa(nac) {
                entregar(cpw3_d(0xb7431c), n, cpw3_clube(cpw3_d(x + 4)));
            }
        }
        x = proximo(l);
    }
    destruir(l);
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_contrato_acabando(this: usize, contrato: usize, periodo: u32, bosman: u32, avisar: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(1279)(this, contrato, periodo, bosman, avisar)
}

/// { 0x4d6d70 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_bosman_departure_news(contrato, destino)`. Sem contrato: a caixa "Error" (linha 0x2611). Senão a notícia 0xbbe ao
/// técnico do clube (0x7861e0): o clube, a pessoa e o clube de destino (−1 sem ele).
unsafe extern "thiscall" fn noticia_saida_pela_lei_bosman(this: usize, contrato: usize, destino: usize) {
    let _ = this;
    if contrato == 0 {
        return aviso_em(0x9a8190, 0x2611);
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparacao::ramo("cpw_3 saida pela lei bosman");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbbe, 0);
    let clube = cpw3_clube(cpw3_d(contrato + 4));
    let p = cpw3_pessoa(cpw3_d(contrato));
    gravar(n, 0, cpw3_d(clube) as u32);
    cpw3_dados_da_pessoa(n, p);
    gravar(n, 6, if destino != 0 { cpw3_d(destino) as u32 } else { 0xffff_ffff });
    entregar(cpw3_d(0xb7431c), n, clube);
}

// ponytail: a notícia vai à caixa do técnico (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_saida_pela_lei_bosman(this: usize, contrato: usize, destino: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1280)(this, contrato, destino)
}

// ---------- 259a leva: a oferta de contrato acrescentada ou atualizada, a oferta de contrato tirada pela vaga e o menor salário pedido (contract_manager.cpp) ----------

unsafe fn cpw4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpw4_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// O contador de ofertas de contrato da pessoa (byte +8 do registro de 79 bytes em [0xae2d0c]).
unsafe fn cpw4_contador(pessoa: usize) -> usize {
    cpw4_d(0xae2d0c).wrapping_add(cpw4_d(pessoa).wrapping_mul(0x4f)).wrapping_add(8)
}

/// A atribuição de data (0x417c10: thiscall(destino, temporária, origem)).
unsafe fn cpw4_copiar_data(destino: usize, origem: usize) {
    let copia: unsafe extern "thiscall" fn(usize, *mut [u32; 2], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u32; 2];
    copia(destino, &mut t, origem);
}

/// A oferta de contrato da pessoa (id em +0) pelo clube (id em +4) na lista `l` (nós: +0 o item, +6 o próximo), sem
/// mexer no cursor; nenhuma: 0.
unsafe fn cpw4_achar(l: usize, id: usize, clube: usize) -> usize {
    let mut no = cpw4_d(l + 2);
    let mut n = 0;
    while no != 0 && n < 100000 {
        let x = cpw4_d(no);
        if cpw4_d(x) == id && cpw4_d(x + 4) == clube {
            return x;
        }
        no = cpw4_d(no + 6);
        n += 1;
    }
    0
}

/// O ensaio do comparador: a cópia não cria nem acrescenta à lista (0x944cf1, 0x68d020, 0x68d0d0) e não chama o que vem
/// depois (0x8c1f80, 0x8c2060, 0x4cd230), que alocam, sorteiam e mexem nas ofertas.
static mut CPW4_ENSAIO: bool = false;

/// { 0x4cc490 } (thiscall, ret 0x1c, contract_manager.cpp, quadro SEH sem destrutor): `CONTRACT_MANAGER::
/// add_contract_offer(termos, vaga, valor, marca, a5, decidir, acertado)`, pela oferta de empréstimo (0x8bed60), pelas
/// ofertas guardadas (0x4cf4a0), pelas listas (0x84a020), pela negociação (0x4cda70) e pela tela. Calcula hoje + 14 (0x54c5b0,
/// descartado) e monta um registro local de 0x6f bytes (contrato 0x85e410, data 0x54a110). Com ofertas contadas (+8 do
/// registro de 79 bytes da pessoa em [0xae2d0c]) e lista (+0x34), a oferta do mesmo clube é atualizada: os termos (o
/// operator= 0x4cc810 embutido), +0x50 = 2 se `acertado` (senão 0), +0x51 = hoje, +0x59 a vaga, +0x5d o valor, +0x61 a
/// marca, +0x62 = 1 fora do contrato de tipo 6. Senão o registro local recebe o mesmo (+0x63 = −1, o resto fica como a
/// original o deixa) e entra na lista (criada se faltar; 0x68d0d0), e o contador sobe. Com vaga, a oferta (0x8c61e0) do
/// mesmo jogador marca os termos oferecidos (0x8c1f80), senão o jogador da troca (0x8c2060); com `decidir`, agenda a
/// decisão (0x4cd230).
unsafe extern "thiscall" fn acrescentar_oferta_de_contrato(this: usize, termos: usize, vaga: i32, valor: u32, marca: u32, a5: u32, decidir: u32, acertado: u32) {
    let _ = a5;
    let mais: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut quinzena = [0u32; 2];
    mais(0xae2c90, &mut quinzena, 0xe);
    // ponytail: os bytes +0x67..+0x6e do registro local a original não preenche (sobra de pilha); a cópia usa zero
    let mut registro = [0u8; 0x6f];
    let r = registro.as_mut_ptr() as usize;
    construir(r);
    data_padrao(r + 0x51);
    let id = cpw4_d(termos);
    let mut item = 0;
    if cpw4_b(cpw4_contador(termos)) != 0 && cpw4_d(this + 0x34) != 0 {
        let clube = cpw4_d(termos + 4);
        let l = cpw4_d(this + 0x34);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, cpw4_d(l + 2));
        let mut x = proximo(cpw4_d(this + 0x34));
        while x != 0 {
            if cpw4_d(x) == id && cpw4_d(x + 4) == clube {
                item = x;
                break;
            }
            x = proximo(cpw4_d(this + 0x34));
        }
    }
    let tipo6 = cpw4_b(termos + 0x35) & 0x3f == 6;
    if item != 0 {
        crate::comparacao::ramo("cpw_4 oferta atualizada");
        // ponytail: a original embute o operator= (0x4cc810) aqui; a cópia chama a versão Rust dele, as mesmas gravações
        copiar_contrato_de_pessoal(item, termos);
        *((item + 0x50) as *mut u8) = if acertado != 0 { 2 } else { 0 };
        cpw4_copiar_data(item + 0x51, 0xae2c90);
        cpw4_pd(item + 0x59, vaga as u32);
        cpw4_pd(item + 0x5d, valor);
        *((item + 0x61) as *mut u8) = marca as u8;
        *((item + 0x62) as *mut u8) = !tipo6 as u8;
    } else {
        let operador: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cc810usize);
        operador(r, termos);
        *((r + 0x50) as *mut u8) = 0;
        cpw4_copiar_data(r + 0x51, 0xae2c90);
        cpw4_pd(r + 0x5d, valor);
        cpw4_pd(r + 0x59, vaga as u32);
        *((r + 0x61) as *mut u8) = marca as u8;
        cpw4_pd(r + 0x63, 0xffff_ffff);
        *((r + 0x62) as *mut u8) = !tipo6 as u8;
        if CPW4_ENSAIO {
            core::ptr::copy_nonoverlapping(r as *const u8, core::ptr::addr_of_mut!(CPW4_REGISTRO) as *mut u8, 0x6f);
        } else {
            if cpw4_d(this + 0x34) == 0 {
                let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
                let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
                let p = novo(0xe);
                let l = if p != 0 { lista_nova(p) } else { 0 };
                cpw4_pd(this + 0x34, l as u32);
            }
            let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
            acrescentar(cpw4_d(this + 0x34), r, 0x6f);
        }
        crate::comparacao::ramo("cpw_4 oferta nova");
        let c = cpw4_contador(termos) as *mut u8;
        *c = (*c).wrapping_add(1); // cpw4: o contador de ofertas da pessoa
    }
    if CPW4_ENSAIO {
        return;
    }
    if vaga != -1 {
        let vaga_da: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x8c61e0usize);
        let tm = cpw4_d(0xde1f54);
        let v = vaga_da(tm, vaga);
        if cpw4_d(cpw4_d(v) + 0xc) == cpw4_d(termos) {
            crate::comparacao::ramo("cpw_4 termos ao jogador");
            let marcar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x8c1f80usize);
            marcar(cpw4_d(0xde1f54), vaga); // cpw4: os termos oferecidos ao jogador
        } else {
            let troca: unsafe extern "thiscall" fn(usize, i32, u32) = core::mem::transmute(0x8c2060usize);
            troca(cpw4_d(0xde1f54), vaga, cpw4_d(termos) as u32);
        }
    }
    if decidir != 0 {
        crate::comparacao::ramo("cpw_4 agendar decisao");
        let decisao: unsafe extern "thiscall" fn(usize, u32, i32) = core::mem::transmute(0x4cd230usize);
        decisao(this, cpw4_d(termos) as u32, vaga); // cpw4: a decisão agendada
    }
}

/// O registro que a cópia em ensaio acrescentaria à lista.
static mut CPW4_REGISTRO: [u8; 0x6f] = [0; 0x6f];

/// Comparador. Oferta já na lista (achada antes, sem mexer no cursor): a cópia em ensaio e a original a partir do mesmo
/// estado, comparando a oferta (0x63 bytes), o contador e a cabeça da lista (o cursor). Oferta nova (acrescentada, aloca):
/// a cópia em ensaio monta o registro; a original acrescenta, e o nó novo (o último do jogador pelo clube) tem de ter os
/// mesmos 0x63 primeiros bytes, com o contador + 1.
unsafe extern "thiscall" fn comparar_acrescentar_oferta_de_contrato(this: usize, termos: usize, vaga: i32, valor: u32, marca: u32, a5: u32, decidir: u32, acertado: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, u32, u32)>(1281);
    let id = cpw4_d(termos);
    let clube = cpw4_d(termos + 4);
    let cont = cpw4_contador(termos);
    let l = cpw4_d(this + 0x34);
    let item = if cpw4_b(cont) != 0 && l != 0 { cpw4_achar(l, id, clube) } else { 0 };
    let mut todas = vec![(cont, 1usize)];
    if item != 0 {
        todas.push((item, 0x63));
        todas.push((l, 0xe));
    }
    let n_cmp = todas.len();
    todas.extend(CPQ5_ESTATICAS);
    todas.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&todas);
    CPW4_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| acrescentar_oferta_de_contrato(this, termos, vaga, valor, marca, a5, decidir, acertado)));
    CPW4_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&todas[..n_cmp]);
    let registro = CPW4_REGISTRO;
    antes.escrever();
    o(this, termos, vaga, valor, marca, a5, decidir, acertado);
    let mo = crate::comparacao::Memoria::ler(&todas[..n_cmp]);
    let igual = if item != 0 {
        mo == mr
    } else {
        // o nó novo: o último do jogador pelo clube na lista (criada pela original se faltava)
        let l = cpw4_d(this + 0x34);
        let mut novo = 0;
        if l != 0 {
            let mut no = cpw4_d(l + 2);
            let mut n = 0;
            while no != 0 && n < 100000 {
                let x = cpw4_d(no);
                if cpw4_d(x) == id && cpw4_d(x + 4) == clube {
                    novo = x;
                }
                no = cpw4_d(no + 6);
                n += 1;
            }
        }
        mo == mr && novo != 0 && core::slice::from_raw_parts(novo as *const u8, 0x63) == &registro[..0x63]
    };
    PLACAR.registrar(igual, || format!("acrescentar_oferta_de_contrato(pessoa {:#x}, clube {:#x}, vaga {}, atualizada {}) original={:x?} rust={:x?}", id, clube, vaga, item != 0, mo, mr));
}

/// { 0x4ccb30 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract_offer_for_transfer(pessoa,
/// vaga)`, pelas ofertas retiradas e desfeitas (0x8bf680, 0x8bfcb0, 0x8c4950, 0x8c4e10, 0x8d0f70, 0x8df9b0): na lista das
/// ofertas de contrato (+0x34; cursor recomeçado) a da pessoa com a vaga (+0x59). Achada: com +0x63 marcado, a original
/// 0x7a81a0 de [0xb7434c] com +0x63, +0x67 e +0x6b; acertada (+0x50 = 2), desliga a marca de contrato acertado (0x8c5430);
/// tira da lista (0x68d1f0) e o contador de ofertas da pessoa (+8 do registro de 79 bytes) desce.
unsafe extern "thiscall" fn tirar_oferta_de_contrato_da_vaga(this: usize, pessoa: usize, vaga: i32) {
    let l = cpw4_d(this + 0x34);
    if l == 0 {
        return;
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpw4_d(l + 2));
    let mut x = proximo(cpw4_d(this + 0x34));
    while x != 0 {
        if cpw4_d(x) == cpw4_d(pessoa) && cpw4_d(x + 0x59) as i32 == vaga {
            break;
        }
        x = proximo(cpw4_d(this + 0x34));
    }
    if x == 0 {
        return;
    }
    crate::comparacao::ramo("cpw_4 oferta tirada pela vaga");
    if cpw4_d(x + 0x63) as i32 != -1 {
        let f: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(0x7a81a0usize);
        f(cpw4_d(0xb7434c), cpw4_d(x + 0x63) as u32, cpw4_d(x + 0x67) as u32, cpw4_d(x + 0x6b) as u32);
    }
    if cpw4_b(x + 0x50) == 2 {
        let desmarcar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c5430usize);
        desmarcar(cpw4_d(0xde1f54), pessoa);
    }
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    tirar(cpw4_d(this + 0x34), x);
    let c = cpw4_contador(pessoa) as *mut u8;
    *c = (*c).wrapping_sub(1);
}

/// Comparador: a original tira o item da lista (libera): só ela, conferindo o plano feito antes: com a oferta da pessoa
/// pela vaga na lista, o contador desce 1; sem ela, fica.
unsafe extern "thiscall" fn comparar_tirar_oferta_de_contrato_da_vaga(this: usize, pessoa: usize, vaga: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i32)>(1282);
    let l = cpw4_d(this + 0x34);
    let mut achou = false;
    if l != 0 {
        let mut no = cpw4_d(l + 2);
        let mut n = 0;
        while no != 0 && n < 100000 {
            let x = cpw4_d(no);
            if cpw4_d(x) == cpw4_d(pessoa) && cpw4_d(x + 0x59) as i32 == vaga {
                achou = true;
                break;
            }
            no = cpw4_d(no + 6);
            n += 1;
        }
    }
    let antes = cpw4_b(cpw4_contador(pessoa));
    o(this, pessoa, vaga);
    let depois = cpw4_b(cpw4_contador(pessoa));
    let esperado = if achou { antes.wrapping_sub(1) } else { antes };
    PLACAR.registrar(depois == esperado, || format!("tirar_oferta_de_contrato_da_vaga(pessoa {:#x}, vaga {}) achou {} contador {} -> {}", cpw4_d(pessoa), vaga, achou, antes, depois));
}

/// { 0x4ce450 } (stdcall, ret 8, contract_manager.cpp, quadro SEH sem destrutor): o menor salário que a pessoa pede ao
/// clube: num contrato local (0x85e410), o desejado (0x85fc50 com −1, a situação 1 e depois 2, 0xb), no máximo 10 milhões.
/// Pela tela do contrato.
unsafe extern "stdcall" fn menor_salario_pedido(pessoa: usize, clube: usize) -> i32 {
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x85fc50usize);
    let mut local = [0u8; 0x50];
    let c = local.as_mut_ptr() as usize;
    construir(c);
    let mut menor = 0x989680;
    let a = desejado(c, pessoa, clube, -1, 1, 0xb, 0, 0);
    if a < menor {
        menor = a;
    }
    let b = desejado(c, pessoa, clube, -1, 2, 0xb, 0, 0);
    if b < menor { b } else { menor }
}

/// Comparador: a original e a cópia a partir do mesmo estado (o sorteio e o último contrato pedido [0xae1080]); a
/// devolução e a memória.
unsafe extern "stdcall" fn comparar_menor_salario_pedido(pessoa: usize, clube: usize) -> i32 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize) -> i32>(1283);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(pessoa, clube), || crate::aleatorio::sem_contar(|| menor_salario_pedido(pessoa, clube)));
    PLACAR.registrar(igual && r == n, || format!("menor_salario_pedido(pessoa {:#x}) original={} {:x?} rust={} {:x?}", cpw4_d(pessoa), r, mo, n, mr));
    r
}

// ---------- 259a leva: as notas do mês: as notas da partida, a nota do jogador no mês e o melhor do mês (month_ratings.cpp) ----------

unsafe fn cpw5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw5_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpw5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpw5_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// A pessoa `id` da tabela de pessoas ([0xae23bc], 0x6e bytes cada).
unsafe fn cpw5_pessoa(id: i32) -> usize {
    cpw5_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e))
}

/// { 0x767ee0 } (thiscall, ret 8, month_ratings.cpp): `MONTH_RATINGS::record_match_ratings(jogo, partida)`, pelo
/// 0x7683a0 a cada partida das ligas com prêmio do mês. Os dois lados da partida (+0x190 e +0x190 + 0x625), 20 vagas de
/// 0x49 bytes cada: fora a vaga marcada 0xfb em +0x1d e, da 12ª em diante, a reserva que não entrou (+0x32 = 0xff), a
/// pessoa (id em +0) abaixo da conta de pessoas [0xae2364] vai para as notas do mês (0x767fa0).
unsafe extern "thiscall" fn guardar_notas_da_partida(this: usize, jogo: usize, partida: usize) {
    let _ = jogo;
    let nota: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x767fa0usize);
    let mut lado = partida.wrapping_add(0x190);
    for l in 0..2 { // cpw5: os dois lados
        for k in 0..0x14usize {
            let e = lado + k * 0x49;
            if cpw5_b(e + 0x1d) == 0xfb {
                continue;
            }
            if k >= 0xb && cpw5_b(e + 0x32) == 0xff {
                continue;
            }
            let p = cpw5_pessoa(cpw5_i(e));
            if p != 0 && cpw5_i(p) >= cpw5_i(0xae2364) {
                continue;
            }
            if l == 1 {
                crate::comparacao::ramo("cpw_5 notas do visitante");
            }
            nota(this, e);
        }
        lado += 0x625;
    }
}

/// O comparador das notas da partida: lado a lado (cabeçalho e as vagas da tabela que podem mudar) quando cabem as 40
/// vagas sem crescer a tabela; senão só a original (a 0x767fa0 realoca e a original não pode rodar duas vezes).
unsafe extern "thiscall" fn comparar_guardar_notas_da_partida(this: usize, jogo: usize, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1284);
    let (tab, n, cap) = (cpw5_d(this), cpw5_i(this + 4), cpw5_i(this + 8));
    if tab == 0 || n < 0 || cap.wrapping_sub(n) < 40 {
        // ponytail: a tabela pode crescer (realloc): só a original; a 0x767fa0 compara cada nota
        return o(this, jogo, partida);
    }
    let regioes = [(this, 12), (tab, (n as usize + 40) * 12)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, jogo, partida), || guardar_notas_da_partida(this, jogo, partida));
    PLACAR.registrar(igual, || format!("guardar_notas_da_partida(this {:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// A nota da partida (+0x35 da vaga, sem sinal) soma na palavra +8 do registro de 12 bytes e o byte +0xa conta mais um
/// jogo.
unsafe fn cpw5_somar(r: usize, e: usize) {
    let s = (r + 8) as *mut u16;
    core::ptr::write_unaligned(s, core::ptr::read_unaligned(s).wrapping_add(cpw5_b(e + 0x35) as u16));
    *((r + 0xa) as *mut u8) = cpw5_b(r + 0xa).wrapping_add(1);
}

/// { 0x767fa0 } (thiscall, ret 4, month_ratings.cpp): `MONTH_RATINGS::add_player_rating(vaga)`. A tabela em [this]
/// (conta em +4, capacidade em +8) tem registros de 12 bytes: +0 pessoa, +4 liga do clube (ou −1), +8 soma das notas,
/// +0xa jogos, +0xb idade. Acha a pessoa da vaga (+0) ou acrescenta (cresce 300 com malloc 0x944e46 ou realloc
/// 0x945667 quando cheia; a liga pelo clube +0x39 da pessoa e o +0x57 dele; a idade pelo +0x18 da pessoa) e soma a nota.
unsafe extern "thiscall" fn nota_do_jogador_no_mes(this: usize, e: usize) {
    let id = cpw5_i(e);
    let n = cpw5_i(this + 4);
    let mut tab = cpw5_d(this);
    for k in 0..n.max(0) as usize {
        let r = tab + k * 12;
        if cpw5_i(r) == id {
            return cpw5_somar(r, e);
        }
    }
    let cap = cpw5_i(this + 8);
    if n == cap {
        let novo = cap.wrapping_add(300);
        cpw5_pd(this + 8, novo as u32);
        let bytes = (novo as u32).wrapping_mul(12);
        tab = if tab == 0 {
            let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
            malloc(bytes)
        } else {
            let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
            realloc(tab, bytes)
        };
        if tab == 0 {
            // ponytail: sem memória (a caixa "Error" da original): volta a capacidade e deixa a original refazer
            cpw5_pd(this + 8, cap as u32);
            return orig::<unsafe extern "thiscall" fn(usize, usize)>(1285)(this, e);
        }
        cpw5_pd(this, tab as u32);
    }
    let p = cpw5_pessoa(id);
    let clube = cpw5_d(p + 0x39);
    let mut liga = -1i32;
    if clube != 0 {
        let l = cpw5_d(clube + 0x57);
        if l != 0 {
            liga = cpw5_i(l);
        }
    }
    let r = tab + (n as usize) * 12;
    cpw5_pd(r, id as u32);
    cpw5_pd(r + 4, liga as u32);
    core::ptr::write_unaligned((r + 8) as *mut u16, 0);
    *((r + 0xb) as *mut u8) = cpw5_b(p + 0x18);
    crate::comparacao::ramo("cpw_5 jogador novo no mes");
    *((r + 0xa) as *mut u8) = 0; // cpw5: começa sem jogos
    cpw5_pd(this + 4, n.wrapping_add(1) as u32);
    cpw5_somar(r, e);
}

/// O comparador da nota do mês: lado a lado (cabeçalho e os registros até o novo) quando a tabela não precisa crescer;
/// senão só a original (realloc).
unsafe extern "thiscall" fn comparar_nota_do_jogador_no_mes(this: usize, e: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1285);
    let (tab, n, cap) = (cpw5_d(this), cpw5_i(this + 4), cpw5_i(this + 8));
    let achou = tab != 0 && (0..n.max(0) as usize).any(|k| cpw5_i(tab + k * 12) == cpw5_i(e));
    if tab == 0 || n < 0 || (n >= cap && !achou) {
        // ponytail: a tabela cresce (malloc/realloc): só a original
        return o(this, e);
    }
    let vagas = if n < cap { n as usize + 1 } else { n as usize };
    let regioes = [(this, 12), (tab, vagas * 12)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, e), || nota_do_jogador_no_mes(this, e));
    PLACAR.registrar(igual, || format!("nota_do_jogador_no_mes(pessoa {}) original={:x?} rust={:x?}", cpw5_i(e), mo, mr));
}

/// { 0x768130 } (thiscall, ret 8, month_ratings.cpp): `MONTH_RATINGS::find_best_player(liga, idade)`, pelo prêmio do
/// mês (0x767150). Ordena a tabela pela média (qsort 0x945333 com 0x768200: quem tem menos de 3 jogos vai para o fim) e
/// devolve o primeiro registro da liga (−1: qualquer) com pelo menos 3 jogos (depois 2, 1 e 0); com idade (não 0xff),
/// os acima dela só contam e o mais novo só vale depois de 3 contados; com liga, a liga atual do clube da pessoa (+0x39,
/// +0x57) também tem de ser a pedida. Nenhum: 0.
unsafe extern "thiscall" fn melhor_do_mes(this: usize, liga: i32, idade: u32) -> usize {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    crate::comparacao::ramo("cpw_5 ordenou as notas");
    ordenar(cpw5_d(this), cpw5_d(this + 4) as u32, 0xc, 0x768200); // cpw5: ordena pela média
    let tab = cpw5_d(this);
    let n = cpw5_i(this + 4);
    let idade = idade as u8;
    let mut contados = 0i32;
    let mut minimo = 3i8;
    loop {
        for k in 0..n.max(0) as usize {
            let r = tab + k * 12;
            if liga != -1 && cpw5_i(r + 4) != liga {
                continue;
            }
            if (cpw5_b(r + 0xa) as i8) < minimo {
                continue;
            }
            if idade != 0xff {
                if cpw5_b(r + 0xb) as i8 > idade as i8 {
                    contados += 1;
                    continue;
                }
                if contados < 3 {
                    continue;
                }
            }
            if liga == -1 {
                return r;
            }
            let c = cpw5_d(cpw5_pessoa(cpw5_i(r)) + 0x39);
            if c != 0 {
                let l = cpw5_d(c + 0x57);
                if l != 0 && cpw5_i(l) == liga {
                    return r;
                }
            }
        }
        minimo -= 1;
        if minimo < 0 {
            return 0;
        }
    }
}

/// O comparador do melhor do mês: lado a lado pela tabela ordenada e pelo registro devolvido.
unsafe extern "thiscall" fn comparar_melhor_do_mes(this: usize, liga: i32, idade: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, u32) -> usize>(1286);
    let (tab, n) = (cpw5_d(this), cpw5_i(this + 4));
    let regioes: Vec<(usize, usize)> = if tab != 0 && n > 0 { vec![(tab, n as usize * 12)] } else { Vec::new() };
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, liga, idade), || melhor_do_mes(this, liga, idade));
    PLACAR.registrar(igual && r == m, || format!("melhor_do_mes(liga {}, idade {}) original={:#x} {:x?} rust={:#x} {:x?}", liga, idade as u8 as i8, r, mo, m, mr));
    r
}

// ---------- 259a leva: o prêmio do mês: o melhor das notas, a vaga do vencedor e a reputação do premiado (month_award.cpp, award_manager.cpp) ----------

unsafe fn cpw6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw6_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpw6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpw6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpw6_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cpw6_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// A pessoa `id` ([0xae23bc], 0x6e bytes cada).
unsafe fn cpw6_pessoa(id: i32) -> usize {
    cpw6_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e))
}

/// O clube `id` ([0xae23b4], 0x245 bytes cada).
unsafe fn cpw6_clube(id: i32) -> usize {
    cpw6_d(0xae23b4).wrapping_add((id as usize).wrapping_mul(0x245))
}

/// O ensaio dos comparadores: a cópia roda sem a notícia (0x767330), sem o registro do prêmio (0x86c8c0) e sem as
/// listas dos clubes (0x842c00), que alocam e não podem rodar duas vezes.
static mut CPW6_ENSAIO: bool = false;

/// { 0x767150 } (thiscall, ret 8, month_award.cpp): `MONTH_AWARD::award_best_rated_player(notas, idade)`, na virada do
/// mês pelos prêmios das nações (os da Inglaterra em 0x579a37..0x579adc). O melhor das notas (0x768130) na liga do
/// prêmio ([+9], ou −1) e com a idade pedida; achou: grava o vencedor (0x7671c0) com o clube da pessoa (+0x39, ou −1),
/// o mês (+0xc) e o ano (+0xd) das notas. Os bytes altos do mês e do ano vão como a original os empurra (da tabela de
/// pessoas no registrador).
unsafe extern "thiscall" fn premiar_o_melhor_do_mes(this: usize, notas: usize, idade: u32) {
    let liga = {
        let l = cpw6_d(this + 9);
        if l != 0 {
            cpw6_i(l)
        } else {
            -1
        }
    };
    let melhor: unsafe extern "thiscall" fn(usize, i32, u32) -> usize = core::mem::transmute(0x768130usize);
    let r = melhor(notas, liga, idade);
    if r == 0 {
        return;
    }
    crate::comparacao::ramo("cpw_6 premiou o melhor do mes");
    let id = cpw6_i(r);
    let tabela = cpw6_d(0xae23bc) as u32;
    let clube = cpw6_d(cpw6_pessoa(id) + 0x39);
    let ano = (tabela & 0xffff0000) | cpw6_w(notas + 0xd) as u16 as u32;
    let mes = cpw6_b(notas + 0xc) as u32;
    let gravar: unsafe extern "thiscall" fn(usize, i32, i32, u32, u32) = core::mem::transmute(0x7671c0usize);
    if clube != 0 {
        gravar(this, id, cpw6_i(clube), (ano & 0xffffff00) | mes, ano);
    } else {
        gravar(this, id, -1, mes, ano);
    }
}

// ponytail: o trabalho dela é chamar o melhor do mês e a vaga do vencedor, que têm comparador próprio (lado a lado e em
// ensaio); a vaga chama a notícia e o registro do prêmio, que alocam: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_premiar_o_melhor_do_mes(this: usize, notas: usize, idade: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1287)(this, notas, idade)
}

/// A reputação do premiado: no ensaio, a cópia direto (sem as listas dos clubes); senão pelo endereço (0x412f00).
unsafe fn cpw6_reputacao(pessoa: usize, clube: usize, nac: u32, mundo: u32, jogador: u32) {
    if CPW6_ENSAIO {
        return reputacao_do_premiado(pessoa, clube, nac, mundo, jogador);
    }
    let f: unsafe extern "C" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x412f00usize);
    f(pessoa, clube, nac, mundo, jogador);
}

/// { 0x7671c0 } (thiscall, ret 0x10, month_award.cpp): `MONTH_AWARD::set_winner(pessoa, clube, mês, ano)`. Acha a vaga
/// do mês (12 vagas de 11 bytes: mês em +0x25, ano em +0x26); sem ela, nada. Grava a pessoa (+0x1d) e o clube (+0x21);
/// com pessoa, a reputação do premiado (0x412f00, com as reputações +0x15 e +0x17 do prêmio e a marca de jogador
/// +0x19); a notícia (0x767330) e o registro do prêmio (0x86c8c0 em [0xdd7f00], com o +5 do prêmio).
unsafe extern "thiscall" fn gravar_vencedor_do_mes(this: usize, id: i32, clube_id: i32, mes: u32, ano: u32) {
    let mut vaga = None;
    for k in 0..12usize {
        let v = this + k * 11;
        if cpw6_b(v + 0x25) == mes as u8 && cpw6_w(v + 0x26) == ano as u16 as i16 {
            vaga = Some(k);
            break;
        }
    }
    let k = match vaga {
        Some(k) => k,
        None => return,
    };
    let v = this + k * 11;
    cpw6_pd(v + 0x1d, id as u32);
    cpw6_pd(v + 0x21, clube_id as u32);
    if id > -1 {
        let clube = if clube_id > -1 { cpw6_clube(clube_id) } else { 0 };
        // os bytes altos das reputações vêm do dword +0x19 no registrador, como a original empurra
        let marca = cpw6_d(this + 0x19) as u32;
        let alto = marca & 0xffff0000;
        crate::comparacao::ramo("cpw_6 vencedor do mes");
        cpw6_reputacao(cpw6_pessoa(id), clube, alto | cpw6_w(this + 0x15) as u16 as u32, alto | cpw6_w(this + 0x17) as u16 as u32, marca);
    }
    if CPW6_ENSAIO {
        return;
    }
    let noticia: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x767330usize);
    noticia(this, (this as u32 & 0xffffff00) | k as u32);
    let cid = cpw6_i(v + 0x21);
    let clube = if cid != -1 { cpw6_clube(cid) } else { 0 };
    let registro: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c8c0usize);
    registro(cpw6_d(0xdd7f00), cpw6_pessoa(cpw6_i(v + 0x1d)), clube, cpw6_d(this + 5) as u32, mes, ano);
}

/// As regiões que a reputação do premiado grava: o registro do jogador (+0x61: +9..+0xe), o do não jogador (+0x69:
/// +8..+0xd) e a linha da diretoria do clube (73 bytes em [[0xb67a44]], palavra +6).
unsafe fn cpw6_regioes_da_reputacao(regioes: &mut Vec<(usize, usize)>, pessoa: usize, clube: usize) {
    let p = cpw6_d(pessoa + 0x61);
    if p != 0 {
        regioes.push((p + 9, 6));
    }
    let np = cpw6_d(pessoa + 0x69);
    if np != 0 {
        regioes.push((np + 8, 6));
    }
    if clube != 0 {
        let linhas = cpw6_d(cpw6_d(0xb67a44));
        regioes.push((linhas.wrapping_add((cpw6_i(clube) as usize).wrapping_mul(73)).wrapping_add(6), 2));
    }
}

/// O comparador em ensaio: a cópia sem as chamadas que alocam e a original a partir do mesmo estado das `regioes`.
unsafe fn cpw6_comparar(nome: &str, regioes: &[(usize, usize)], original: impl FnOnce(), copia: impl FnOnce()) {
    let antes = crate::comparacao::Memoria::ler(regioes);
    CPW6_ENSAIO = true;
    crate::aleatorio::pausar_rastro(copia);
    CPW6_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(regioes);
    antes.escrever();
    original();
    let mo = crate::comparacao::Memoria::ler(regioes);
    PLACAR.registrar(mo == mr, || format!("{} original={:x?} rust={:x?}", nome, mo, mr));
}

unsafe extern "thiscall" fn comparar_gravar_vencedor_do_mes(this: usize, id: i32, clube_id: i32, mes: u32, ano: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i32, u32, u32)>(1288);
    let mut regioes = vec![(this + 0x1d, 12 * 11)];
    if id > -1 {
        cpw6_regioes_da_reputacao(&mut regioes, cpw6_pessoa(id), if clube_id > -1 { cpw6_clube(clube_id) } else { 0 });
    }
    cpw6_comparar("gravar_vencedor_do_mes", &regioes, || o(this, id, clube_id, mes, ano), || gravar_vencedor_do_mes(this, id, clube_id, mes, ano));
}

/// A palavra `a` mais o valor do _ftol (16 bits baixos, como o `add word` da original).
unsafe fn cpw6_somar(a: usize, v: i32) {
    cpw6_pw(a, cpw6_w(a).wrapping_add(v as i16));
}

/// Teto de 10000 e piso `piso` na palavra `a` (as comparações com sinal da original).
unsafe fn cpw6_teto_e_piso(a: usize, piso: i16) {
    if cpw6_w(a) > 10000 {
        cpw6_pw(a, 10000);
    }
    if cpw6_w(a) <= piso {
        cpw6_pw(a, piso);
    }
}

/// { 0x412f00 } (cdecl, award_manager.cpp): `award_raise_winner_reputation(pessoa, clube, rep_nacional, rep_mundial,
/// de_jogador)`, pela vaga do vencedor do mês (0x7671c0) e pelos prêmios do ano e das seleções. Jogador (+0x61) num
/// prêmio de jogador: mundial (+0xd) + 5% da mundial do prêmio, até 10000 e pelo menos ela; nacional (+9): do clube da
/// nação dele (+0x53 = +0x1a da pessoa), + 10% da nacional e pelo menos ela; senão + 2,5% da soma e pelo menos a média;
/// atual (+0xb) + 10% da nacional, pelo menos ela; põe o jogador nas listas dos clubes (0x842c00, com 1) se houver o
/// gerente das listas. Não jogador (+0x69) num prêmio que não é de jogador: as três sobem até os pisos (+0xc mundial,
/// +8 pela nação do clube, +0xa nacional) e, com clube, a confiança da diretoria (+6 da linha de 73 bytes) + 1000, entre
/// 6500 e 10000. Só as palavras baixas das reputações valem.
unsafe extern "C" fn reputacao_do_premiado(pessoa: usize, clube: usize, nac: u32, mundo: u32, jogador: u32) {
    let nac = nac as u16 as i16;
    let mundo = mundo as u16 as i16;
    let p = cpw6_d(pessoa + 0x61);
    if p != 0 && jogador != 0 {
        crate::comparacao::ramo("cpw_6 premio de jogador");
        cpw6_somar(p + 0xd, ftol(mundo as i32 as f64 * f64_em(0x967888))); // cpw6: o aumento mundial
        if cpw6_w(p + 0xd) > 10000 {
            cpw6_pw(p + 0xd, 10000);
        }
        if cpw6_w(p + 0xd) <= mundo { // cpw6: o piso mundial
            cpw6_pw(p + 0xd, mundo);
        }
        if clube != 0 && cpw6_d(clube + 0x53) == cpw6_d(pessoa + 0x1a) { // cpw6: o clube da nação do jogador
            cpw6_somar(p + 9, ftol(nac as i32 as f64 * f64_em(0x967880)));
            cpw6_teto_e_piso(p + 9, nac);
        } else {
            cpw6_somar(p + 9, ftol((nac as i32 + mundo as i32) as f64 * f64_em(0x967878)));
            if cpw6_w(p + 9) > 10000 {
                cpw6_pw(p + 9, 10000);
            }
            let piso = nac as i32 / 2 + mundo as i32 / 2;
            if (cpw6_w(p + 9) as i32) <= piso {
                cpw6_pw(p + 9, piso as i16);
            }
        }
        cpw6_somar(p + 0xb, ftol(nac as i32 as f64 * f64_em(0x967880)));
        cpw6_teto_e_piso(p + 0xb, nac);
        let listas = cpw6_d(cpw6_d(0xde1f54) + 0x856);
        if listas != 0 && !CPW6_ENSAIO {
            let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842c00usize);
            f(listas, pessoa, 1);
        }
        return;
    }
    let np = cpw6_d(pessoa + 0x69);
    if np == 0 || jogador != 0 {
        return;
    }
    if cpw6_w(np + 0xc) <= mundo {
        cpw6_pw(np + 0xc, mundo);
    }
    if clube != 0 && cpw6_d(clube + 0x53) == cpw6_d(pessoa + 0x1a) {
        if cpw6_w(np + 8) <= nac {
            cpw6_pw(np + 8, nac);
        }
    } else {
        let piso = nac as i32 / 2 + mundo as i32 / 2;
        if (cpw6_w(np + 8) as i32) <= piso {
            cpw6_pw(np + 8, piso as i16);
        }
    }
    if cpw6_w(np + 0xa) <= nac {
        cpw6_pw(np + 0xa, nac);
    }
    if clube == 0 {
        return;
    }
    let linha = cpw6_d(cpw6_d(0xb67a44)).wrapping_add((cpw6_i(clube) as usize).wrapping_mul(73)).wrapping_add(6);
    let v = cpw6_w(linha).wrapping_add(1000);
    let v = if v < 1 {
        1
    } else if v > 10000 {
        10000
    } else {
        v
    };
    cpw6_pw(linha, if v < 6500 { 6500 } else { v });
}

unsafe extern "C" fn comparar_reputacao_do_premiado(pessoa: usize, clube: usize, nac: u32, mundo: u32, jogador: u32) {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, u32, u32)>(1289);
    let mut regioes = Vec::new();
    cpw6_regioes_da_reputacao(&mut regioes, pessoa, clube);
    cpw6_comparar("reputacao_do_premiado", &regioes, || o(pessoa, clube, nac, mundo, jogador), || reputacao_do_premiado(pessoa, clube, nac, mundo, jogador));
}

// ---------- 259a leva: o físico do dia do jogador (a forma e a condição de jogo até hoje), a faixa de confiança da diretoria na liga e as notas da diretoria (injury.cpp, manager_manager.cpp) ----------

unsafe fn cpw8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpw8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpw8_uw(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cpw8_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cpw8_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A data `destino` recebe `origem` (0x417c10, com a devolução numa local).
unsafe fn cpw8_copiar_data(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    copiar(destino, t.as_mut_ptr(), origem);
}

/// A diferença entre hoje ([0xae2c90]) e a data `d` (0x54cc30): os dias (+0, com sinal) mais 365 × os anos (+2).
unsafe fn cpw8_dias_desde(d: usize) -> i32 {
    let diferenca: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> usize = core::mem::transmute(0x54cc30usize);
    let mut r = [0u8; 8];
    diferenca(0xae2c90, r.as_mut_ptr(), cpw8_d(d) as u32, cpw8_d(d + 4) as u32);
    let p = r.as_ptr() as usize;
    (cpw8_uw(p + 2) as i32).wrapping_mul(365).wrapping_add(cpw8_w(p) as i32)
}

/// O ensaio do comparador: a cópia roda sem o fim da lesão (0x62e700: histórico em arquivo e notícia), que não pode
/// rodar duas vezes; chegando lá, marca `CPW8_TERMINOU` e o comparador fica com a original sozinha.
static mut CPW8_ENSAIO: bool = false;
static mut CPW8_TERMINOU: bool = false;

unsafe fn cpw8_terminar_lesao(this: usize, pessoa: usize) {
    if CPW8_ENSAIO {
        CPW8_TERMINOU = true;
        return;
    }
    let terminar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x62e700usize);
    terminar(this, pessoa, 1);
}

/// A estática local da guarda [0xb63d10] (bit `bit`): na primeira vez, Data() (0x54a110) e o destrutor vazio no atexit.
unsafe fn cpw8_estatica(bit: u8, data: usize, destrutor: usize) {
    let g = 0xb63d10 as *mut u8;
    if *g & bit == 0 {
        *g |= bit;
        let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
        padrao(data);
        let atexit: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x94627ausize);
        atexit(destrutor);
    }
}

/// O trecho da forma acima de `base`: (x − base) × a constante `k` (f64), truncado a 16 bits com sinal, mais `base`.
unsafe fn cpw8_trecho(x: i32, base: i32, k: usize) -> i32 {
    (ftol(x.wrapping_sub(base) as f64 * f64_em(k)) as i16 as i32).wrapping_add(base)
}

/// { 0x62c680 } (thiscall, ret 4, injury.cpp, quadro SEH só das estáticas locais): `Injury::update_player_fitness(
/// pessoa)`: uma vez por dia (a data +0 do registro de 31 bytes da pessoa em [this] já é hoje: nada), pela condição de
/// jogo (0x62c510), pelo estado da lesão (0x62c400), pelo dia das lesões (0x62b7b0) e pelas notícias da volta (0x62e010):
/// toda pessoa que joga, todo dia em que alguém pergunta. T = dias desde a última atualização (0x54cc30). Lesionada
/// (+0xe na tabela de 0x30 bytes em [this+4]): pelos dias desde o começo (+0x20) contra +0x2a e +0x28, os dias parados
/// (perda), em recuperação ou livres, com o fim da lesão (0x62e700) quando passa de +0x28. Sem lesão: dias livres, salvo
/// na seleção com data futura (0x76d780/0x76e040) ou no clube fora da temporada (0x669bd0, guardado em 0xb63d2c/0xb63d30
/// com as datas 0xb63d18/0xb63d20): férias (o byte +0xa do registro de 79 bytes sobe com sorteio(+0x5b / 3 + 2) zero, até
/// 50) e a volta da pré-temporada (15 dias, ou o meio de um sorteio(2) com forma × condição abaixo de 80%). A forma (+8):
/// recuperação de 1000 − 12 × (+0x1e − natural (+0x34 da ficha), 0..100) por dia livre, perda de 66 por dia parado, teto
/// pela forma natural, trechos acima de 7000/9000/9850 encolhidos (0x96af30, 0x96bad0, 0x968968, 0x968918, 0x967880) e −55
/// por dia de férias; a condição de jogo (+0xa) sobe 500 × dias livres e 250 × em recuperação pelo fator do cansaço
/// (+0xc).
/// Grava as duas e a data de hoje.
unsafe extern "thiscall" fn fisico_do_dia(this: usize, pessoa: usize) {
    cpw8_estatica(1, 0xb63d08, 0x62ce20);
    cpw8_estatica(2, 0xb63d18, 0x62ce10);
    cpw8_estatica(4, 0xb63d20, 0x62ce00);
    if pessoa == 0 {
        return;
    }
    let id = cpw8_i(pessoa);
    if id >= cpw8_i(0xae2364) {
        return;
    }
    let ficha = cpw8_d(pessoa + 0x61);
    if ficha == 0 {
        return;
    }
    let rec = cpw8_d(this).wrapping_add((id as u32).wrapping_mul(31) as usize);
    if cpw8_uw(rec + 2) == cpw8_uw(0xae2c92) && cpw8_w(rec) == cpw8_w(0xae2c90) {
        return; // já atualizado hoje
    }
    let t = cpw8_dias_desde(rec);
    let atual = cpw8_i(rec + 0xe);
    if atual > cpw8_i(this + 8) || atual < -1 {
        core::ptr::write_unaligned((rec + 0xe) as *mut i32, -1);
    }
    // os dias livres (l38), em recuperação (l3c), parados (l34) e de férias (l30)
    let (mut l3c, mut l38, mut l34, mut l30) = (0i32, 0i32, 0i32, 0i32);
    let i = cpw8_i(rec + 0xe);
    if i != -1 {
        let inj = cpw8_d(this + 4).wrapping_add((i as u32).wrapping_mul(0x30) as usize);
        let e = cpw8_dias_desde(inj + 0x20);
        let s = cpw8_w(inj + 0x2a) as i32;
        if e < s {
            l34 = t;
        } else {
            let f = cpw8_w(inj + 0x28) as i32;
            if e < f {
                if t < s.wrapping_add(e) {
                    l3c = e - s;
                    l34 = t - (e - s);
                } else {
                    l3c = t;
                }
            } else if t > e - s {
                l38 = t - f;
                l3c = (e - s) - l38;
                l34 = f - l3c;
                cpw8_terminar_lesao(this, pessoa);
            } else {
                l38 = t;
                cpw8_terminar_lesao(this, pessoa);
            }
        }
    } else {
        l38 = t;
        let selecao = cpw8_d(pessoa + 0x24);
        let mut ocupado = false;
        if selecao != 0 {
            let classe: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(0x76d780usize);
            let k = classe(selecao, (*((pessoa + 0x5e) as *const u8) & 8) as u32);
            let campo: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x76e040usize);
            // ponytail: a original empurra o argumento com o al por cima (o resto é o ponteiro da pessoa); só o byte conta
            let d = campo(cpw8_d(0xb74318), cpw8_d(selecao + 0x53), (pessoa as u32 & 0xffff_ff00) | k as u32);
            let (ano, hoje) = (cpw8_uw(d + 2), cpw8_uw(0xae2c92));
            ocupado = if ano > hoje {
                true
            } else if ano < hoje {
                false
            } else {
                cpw8_w(d) > cpw8_w(0xae2c90)
            };
        }
        let clube = cpw8_d(pessoa + 0x39);
        if clube != 0 && !ocupado {
            if clube != cpw8_d(0xb63d2c) || cpw8_uw(0xb63d0a) != cpw8_uw(0xae2c92) || cpw8_w(0xb63d08) != cpw8_w(0xae2c90) {
                let na_temporada: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x669bd0usize);
                let r = na_temporada(clube, 0xb63d18, 0xb63d20);
                *(0xb63d30 as *mut u32) = (r == 0) as u32;
                *(0xb63d2c as *mut usize) = cpw8_d(pessoa + 0x39);
                cpw8_copiar_data(0xb63d08, 0xae2c90);
            }
            if cpw8_i(0xb63d30) != 0 {
                l38 = 0;
                let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
                let u6 = dias_ate(0xae2c90, cpw8_d(0xb63d20) as u32, cpw8_d(0xb63d24) as u32) as u16;
                let u7 = dias_ate(0xb63d18, cpw8_d(0xae2c90) as u32, cpw8_d(0xae2c94) as u32) as u16;
                if u6 < u7 {
                    let lim = if (u6 as i32) < t { u6 as i32 } else { t };
                    l30 = lim;
                    l3c = t - lim;
                    for _ in 0..lim {
                        let g = cpw8_d(0xae2d0c).wrapping_add((cpw8_i(pessoa) as u32).wrapping_mul(0x4f) as usize) + 0xa;
                        if *(g as *const u8) >= 0x32 {
                            break;
                        }
                        if cpw8_sortear(cpw8_b(pessoa + 0x5b) as i32 / 3 + 2) == 0 {
                            *(g as *mut u8) = (*(g as *const u8)).wrapping_add(1);
                        }
                    }
                } else {
                    let mut x = u6 as i32 - (u7 as i32 + u6 as i32) / 2;
                    if x < 0 {
                        x = 0;
                    }
                    if t < x {
                        x = t;
                    }
                    l3c = x;
                    l34 = t - x;
                }
                if u7 < 0xf {
                    l38 = 0xf - u7 as i32;
                    l3c -= l38;
                    if l3c < 0 {
                        l3c = 0;
                    }
                } else {
                    let (f, mc) = (cpw8_w(rec + 8) as i32, cpw8_w(rec + 0xa) as i32);
                    if mc.wrapping_mul(f) / 10000 < 8000 { // cpw8: a forma abaixo de 80% na pré-temporada
                        crate::comparacao::ramo("cpw_8 sorteio da forma");
                        let r = cpw8_sortear(2);
                        l38 = (r + l3c) / 2;
                        l3c -= l38;
                    }
                }
            }
        }
    }
    // a forma
    let f = cpw8_w(rec + 8) as i32;
    let natural = cpw8_b(ficha + 0x34);
    let mut v = cpw8_b(rec + 0x1e).wrapping_sub(natural);
    if v < 0 {
        v = 0;
    } else if v > 100 {
        v = 100;
    }
    let s = (1000 - 12 * v as i32) as i16 as i32; // cpw8: a recuperação por dia livre
    let metade = s / 2;
    let n = natural as i32;
    let piso = ((n * 25 / 20 + 15) * 100) as i16 as i32;
    if l34 != 0 {
        crate::comparacao::ramo("cpw_8 perda");
    }
    let mut x = f.wrapping_sub(66i32.wrapping_mul(l34)); // cpw8: a perda por dia parado
    if x < piso {
        x = 4000;
    }
    let m = n * 16 / 20 + 0x54;
    let mut teto = m.wrapping_mul(100);
    if (teto as i16 as i32) < x {
        teto = x;
    }
    x = x.wrapping_add((metade as i16 as i32).wrapping_mul(l3c));
    let teto16 = teto as i16 as i32;
    if x > teto16 {
        x = teto16;
    }
    if l38 != 0 {
        crate::comparacao::ramo("cpw_8 recuperacao");
    }
    x = x.wrapping_add(s.wrapping_mul(l38));
    if f > 0x267a {
        x = cpw8_trecho(x, f, 0x967880);
    } else if f > 0x2328 {
        x = cpw8_trecho(x, f, 0x968918);
        if x > 0x267a {
            x = cpw8_trecho(x, 0x267a, 0x968968);
        }
    } else if x > 0x1b58 {
        x = cpw8_trecho(x, 0x1b58, 0x96af30);
        if x > 0x2328 {
            x = cpw8_trecho(x, 0x2328, 0x96bad0);
        }
        if x > 0x267a {
            x = cpw8_trecho(x, 0x267a, 0x968968);
        }
    }
    if x > 10000 {
        x = 10000;
    }
    let mut forma = x.wrapping_sub(55i32.wrapping_mul(l30));
    if forma < piso {
        forma = 5000;
    }
    let mut e2 = m;
    if (m as i16 as i32) < forma {
        e2 = forma;
    }
    let e16 = e2 as i16 as i32;
    if forma > e16 {
        forma = e16;
    }
    // a condição de jogo, pelo fator do cansaço (+0xc)
    let cansaco = cpw8_w(rec + 0xc);
    let mc = cpw8_w(rec + 0xa) as i32;
    let teto_k = f64_em(0x9689d0);
    let k = if cansaco > 1000 {
        let kv = f64_em(0x968dc0) - cansaco as f64 * f64_em(0x96a608);
        if !(kv > teto_k) {
            kv
        } else if l3c > 0 {
            teto_k
        } else {
            kv
        }
    } else if l3c > 0 {
        teto_k
    } else {
        f64_em(0x967890)
    };
    let r1 = ftol(l38.wrapping_mul(500) as f64 * k);
    let r2 = ftol(l3c.wrapping_mul(250) as f64 * k);
    let mut cond = r2.wrapping_add(mc).wrapping_add(r1);
    if cond > 10000 {
        cond = 10000;
    }
    core::ptr::write_unaligned((rec + 8) as *mut i16, forma as i16);
    core::ptr::write_unaligned((rec + 0xa) as *mut i16, cond as i16);
    cpw8_copiar_data(rec, 0xae2c90);
}

/// Comparador lado a lado: o registro de 31 bytes da pessoa, o byte +0xa do registro de 79 bytes, as estáticas e o
/// guardado do clube (0xb63d08..0xb63d34) e o sorteio. A cópia roda em ensaio, sem o fim da lesão (0x62e700): quando
/// ela chega lá, fica a original sozinha (já rodou primeiro). Na primeira vez (estáticas por criar, com atexit) e sem
/// registro (pessoa nula, id fora da tabela, sem ficha): só a original.
unsafe extern "thiscall" fn comparar_fisico_do_dia(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1290);
    if *(0xb63d10 as *const u8) & 7 != 7 || pessoa == 0 || cpw8_i(pessoa) < 0 || cpw8_i(pessoa) >= cpw8_i(0xae2364) || cpw8_d(pessoa + 0x61) == 0 {
        return o(this, pessoa); // ponytail: as estáticas e o atexit da primeira vez não podem rodar duas vezes
    }
    let id = cpw8_i(pessoa) as usize;
    let rec = cpw8_d(this).wrapping_add(id.wrapping_mul(31));
    let geral = cpw8_d(0xae2d0c).wrapping_add(id.wrapping_mul(0x4f));
    let e = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(rec, 31), (geral + 0xa, 1), (0xb63d08, 0x2c), e[0], e[1]];
    CPW8_TERMINOU = false;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa), || {
        CPW8_ENSAIO = true;
        crate::aleatorio::sem_contar(|| fisico_do_dia(this, pessoa));
        CPW8_ENSAIO = false;
    });
    if CPW8_TERMINOU {
        crate::comparacao::ramo("cpw_8 fim da lesao so na original");
        return;
    }
    PLACAR.registrar(igual, || format!("fisico_do_dia(pessoa {:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

/// { 0x690bd0 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::get_league_confidence_band(clube)`: a
/// confiança da diretoria na liga (palavra com sinal +0x12 do registro de 0x49 bytes do clube em [this]) em faixas:
/// abaixo de −2000: 0; −1000: 1; −500: 2; 500: 3; 1000: 4; 2000: 5; senão 6. Pelo texto da confiança (0x690d80).
/// Só o al conta (o resto do eax é o que sobrou da conta do endereço).
unsafe extern "thiscall" fn faixa_da_confianca_na_liga(this: usize, clube: usize) -> u8 {
    let v = cpw8_w(cpw8_d(this).wrapping_add((cpw8_i(clube) as u32).wrapping_mul(0x49) as usize) + 0x12);
    if v < -2000 {
        0
    } else if v < -1000 {
        1
    } else if v < -500 {
        2
    } else if v < 500 {
        3
    } else if v < 1000 {
        4
    } else if v < 2000 {
        5
    } else {
        6
    }
}

unsafe extern "thiscall" fn comparar_faixa_da_confianca_na_liga(this: usize, clube: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1291)(this, clube);
    let n = faixa_da_confianca_na_liga(this, clube);
    PLACAR.registrar(r == n, || format!("faixa_da_confianca_na_liga(clube {:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x69f7c0 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::set_board_ratings(clube, valor)`: as quatro
/// notas da diretoria (+6, +8, +0xc e +0xa do registro de 0x49 bytes do clube em [this], palavras, nessa ordem) recebem
/// o valor (16 bits). Pela configuração inicial dos jogadores (0x7a46a6).
unsafe extern "thiscall" fn gravar_notas_da_diretoria(this: usize, clube: usize, valor: u32) {
    for off in [6usize, 8, 0xc, 0xa] {
        let r = cpw8_d(this).wrapping_add((cpw8_i(clube) as u32).wrapping_mul(0x49) as usize);
        core::ptr::write_unaligned((r + off) as *mut u16, valor as u16);
    }
}

unsafe extern "thiscall" fn comparar_gravar_notas_da_diretoria(this: usize, clube: usize, valor: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1292);
    let r = cpw8_d(this).wrapping_add((cpw8_i(clube) as u32).wrapping_mul(0x49) as usize);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 0x49)], || o(this, clube, valor), || gravar_notas_da_diretoria(this, clube, valor));
    PLACAR.registrar(igual, || format!("gravar_notas_da_diretoria(clube {:x}, {:x}) {:x?} x {:x?}", clube, valor, mo, mr));
}

// ---------- 259a leva: o dia das lesões: as lesões que envelhecem e as lesões no treino, as notícias da volta aos treinos e o jogador a um cartão da suspensão (injury.cpp, discipline.cpp) ----------

unsafe fn cpw9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpw9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpw9_uw(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cpw9_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cpw9_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// Soma `delta` ao byte `a` (o `add`/`dec` de 8 bits da original).
unsafe fn cpw9_somar_byte(a: usize, delta: u8) {
    *(a as *mut u8) = (*(a as *const u8)).wrapping_add(delta);
}

/// O registro de 31 bytes da pessoa de índice `idx` em [this].
unsafe fn cpw9_registro(this: usize, idx: i32) -> usize {
    cpw9_d(this).wrapping_add((idx as u32).wrapping_mul(31) as usize)
}

/// A categoria da recorrência (+0x12 do registro, com sinal) é a da tabela de tipos (0x9e7ad9, 13 bytes por tipo)?
unsafe fn cpw9_recorrente(this: usize, idx: i32, categoria: u8) -> bool {
    let r = cpw9_b(cpw9_registro(this, idx) + 0x12);
    r > -1 && *((0x9e7ad9 + (r as isize * 13) as usize) as *const u8) == categoria
}

/// O ensaio do comparador: a cópia roda só a fase do treino (sem soltar a lista das batidas, sem o físico do dia 180,
/// sem a lesão nova nem as rotinas do fim); `CPW9_PAROU` marca que ela chegou a uma lesão nova (0x62d250, que aloca e
/// manda notícia) ou ao físico de todos.
static mut CPW9_ENSAIO: bool = false;
static mut CPW9_PAROU: bool = false;
/// O que a cópia deixou nas regiões da fase do treino, à espera de a original chegar às notícias da volta (0x62e010).
static mut CPW9_ESPERA: Option<(Vec<(usize, usize)>, crate::comparacao::Memoria)> = None;

/// O switch dos tipos de lesão (tipo − 0xc, byte da tabela 0x62bcd8): 0 os do +0x1b/+0x36, 1 nenhuma lesão (0x14), 2 o
/// 0x1c, 3 o 0x2d, 4 os 0x3b e 0x3c, 5 só a lesão.
const CPW9_CASOS: [u8; 0x31] = [
    0, 5, 5, 5, 5, 5, 5, 5, 1, 5, 5, 0, 5, 5, 0, 0, 2, 5, 5, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 3, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 4, 4,
];

/// { 0x62b7b0 } (thiscall, ret 0, injury.cpp): `Injury::daily_update()`, pelo laço do dia (0x5cccd0) e pela ida até a
/// data do começo (0x5cc850), todo dia: solta a lista das batidas da partida (+0x28, 0x68d040 e free); os dois contadores
/// (+0x2c, +0x2e) de cada lesão da tabela (0x30 bytes em [+4], conta +8) descem um; no dia 180 da primeira temporada
/// (0x456770 com a data do começo), o físico de todos (0x62c680). Fora do sábado e do domingo (0x54a3f0: 5 e 6), as lesões
/// no treino: [0xae2364] / 150 sorteios (pelo menos 10) de uma pessoa: jogador (+0x61) — senão o sorteio não conta —
/// nos cargos 11, 13 e 15, presente (0x418f60), sem lesão (+0xe = −1), passando pelos sorteios de risco (o +0x2d da ficha,
/// a propensão +0x1e e a recorrência +0x12/+0x13 do registro de 31 bytes; nota 20: sorteio(5) contra sorteio(2)), um em
/// três e propensão de 20: a categoria (a da recorrência ou sorteada pelas faixas 0x9e7f05/0x9e7f06), o tipo (0x62d140,
/// grave nos primeiros 1/14 dos sorteios), os atributos que a lesão gasta (+0x1b, +0x36, +0x1d da ficha) e a lesão nova
/// (0x62d250); fora da temporada (0x62ce30) nada; categoria 7 com nota abaixo de 15 não conta. No fim as notícias da volta
/// (0x62e010) e os surtos de doença (0x62e9e0).
unsafe extern "thiscall" fn dia_das_lesoes(this: usize) {
    let lista = cpw9_d(this + 0x28);
    if lista != 0 {
        if !CPW9_ENSAIO {
            let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
            let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
            destruir(lista);
            soltar(lista);
        }
        core::ptr::write_unaligned((this + 0x28) as *mut u32, 0);
    }
    let base = cpw9_d(this + 4);
    let mut k = 0i32;
    while k < cpw9_i(this + 8) {
        let e = base.wrapping_add((k as u32).wrapping_mul(0x30) as usize) + 0x2e;
        core::ptr::write_unaligned((e - 2) as *mut u16, cpw9_uw(e - 2).wrapping_sub(1));
        core::ptr::write_unaligned(e as *mut u16, cpw9_uw(e).wrapping_sub(1));
        k += 1;
    }
    let anterior: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    if anterior(0xae2c90, cpw9_d(0xdd6ec8) as u32, cpw9_d(0xdd6ecc) as u32) != 0 && cpw9_w(0xae2c90) == 0xb4 {
        if CPW9_ENSAIO {
            CPW9_PAROU = true;
            return;
        }
        let fisico: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x62c680usize);
        let mut p = cpw9_d(0xae23bc);
        let mut i = 0i32;
        while i < cpw9_i(0xae2364) {
            fisico(this, p);
            i += 1;
            p = p.wrapping_add(0x6e);
        }
    }
    let semana: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54a3f0usize);
    if semana(0xae2c90) as i16 != 6 && semana(0xae2c90) as i16 != 5 { // cpw9: sábado e domingo sem treino
        crate::comparacao::ramo("cpw_9 dia de treino");
        cpw9_treino(this);
    }
    if CPW9_ENSAIO {
        return;
    }
    let noticias: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x62e010usize);
    let surtos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x62e9e0usize);
    noticias(this);
    surtos(this);
}

/// As lesões no treino do dia (o meio da 0x62b7b0).
unsafe fn cpw9_treino(this: usize) {
    let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x418f60usize);
    let fora_da_temporada: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x62ce30usize);
    let escolher: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> u8 = core::mem::transmute(0x62d140usize);
    let mut quantos = (cpw9_i(0xae2364) / 150) as i16; // cpw9: os sorteios do treino
    if quantos < 10 {
        quantos = 10;
    }
    let total = quantos as i32;
    let graves = (total / 14) as i16 as i32;
    let mut i = 0i32;
    while i < total {
        let idx = cpw9_sortear(cpw9_i(0xae2364));
        let p = cpw9_d(0xae23bc).wrapping_add((idx as u32).wrapping_mul(0x6e) as usize);
        'proximo: {
            if cpw9_d(p + 0x61) == 0 {
                i -= 1; // não jogador: o sorteio não conta
                break 'proximo;
            }
            let cargo = *((p + 0x3d) as *const u8);
            if cargo != 0xb && cargo != 0xf && cargo != 0xd {
                break 'proximo;
            }
            if ausente(cpw9_d(0xad9c6c), p, 0) != 0 {
                break 'proximo;
            }
            if cpw9_b(cpw9_d(p + 0x61) + 0xf) == 0x14 {
                let r5 = cpw9_sortear(5);
                let r2 = cpw9_sortear(2);
                if r5 > r2 {
                    break 'proximo;
                }
            }
            let rec = cpw9_registro(this, idx);
            if cpw9_i(rec + 0xe) != -1 {
                break 'proximo;
            }
            if cpw9_b(rec + 0x12) > -1 {
                let rec = cpw9_registro(this, idx);
                let r = cpw9_sortear(100);
                let lim = ((cpw9_b(cpw9_d(p + 0x61) + 0x2d) as i32 - 3) * 5 + cpw9_b(rec + 0x1e) as i32 + cpw9_b(rec + 0x13) as i32) * 2;
                if r > lim {
                    break 'proximo;
                }
            } else {
                let r = cpw9_sortear(0xa0);
                let lim = ((cpw9_b(cpw9_d(p + 0x61) + 0x2d) as i32 - 3) * 5 + cpw9_b(cpw9_registro(this, idx) + 0x1e) as i32) * 2; // cpw9: o risco sem recorrência
                if r > lim {
                    break 'proximo;
                }
                crate::comparacao::ramo("cpw_9 risco sem recorrencia");
            }
            if cpw9_sortear(3) != 0 {
                break 'proximo;
            }
            let rec = cpw9_registro(this, idx);
            if cpw9_b(rec + 0x1e) < 0x14 {
                break 'proximo;
            }
            let mut categoria: u8 = 0;
            let mut sortear_categoria = true;
            if cpw9_b(rec + 0x12) > -1 {
                let r1 = cpw9_sortear(cpw9_b(rec + 0x13) as i32 + 2);
                let r2 = cpw9_sortear(3);
                if r1 > r2 {
                    let r = cpw9_b(cpw9_registro(this, idx) + 0x12) as isize;
                    categoria = *((0x9e7ad9 + (r * 13) as usize) as *const u8);
                    sortear_categoria = false;
                }
            }
            if sortear_categoria {
                loop {
                    categoria = cpw9_sortear(0xc) as u8;
                    if categoria == 0 && cpw9_sortear(100) != 0 {
                        continue;
                    }
                    let c = categoria as i8 as isize * 3;
                    let n = *((0x9e7f06 + c as usize) as *const i8) as i32 - *((0x9e7f05 + c as usize) as *const i8) as i32 + 2;
                    if cpw9_sortear(n) != 0 {
                        break;
                    }
                }
            }
            if cpw9_b(cpw9_d(p + 0x61) + 0xf) < 0xf && categoria == 7 {
                i -= 1;
                break 'proximo;
            }
            if fora_da_temporada(p) != 0 {
                break 'proximo;
            }
            let grave: i8 = if i < graves { -1 } else { 0 };
            let tipo = escolher(this, categoria as i8, grave, p);
            let k = (tipo as i8 as i32).wrapping_sub(0xc);
            let caso = if (k as u32) > 0x30 { 5 } else { CPW9_CASOS[k as usize] };
            if caso == 1 {
                break 'proximo; // o tipo 0x14: nenhuma lesão
            }
            if CPW9_ENSAIO {
                CPW9_PAROU = true;
                return;
            }
            cpw9_gasto_da_lesao(this, idx, p, categoria, caso);
            let nova: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x62d250usize);
            // ponytail: a original empurra o tipo num dword cujos bytes de cima não têm valor; a 0x62d250 lê um char
            nova(this, tipo as u32, 0, p, 0, 1, 0xffff_ffff, 0, 0);
        }
        i += 1;
    }
}

/// Os atributos que a lesão gasta (os casos 0, 2, 3 e 4 do switch da 0x62b7b0): a ficha (+0x61) é relida a cada vez.
unsafe fn cpw9_gasto_da_lesao(this: usize, idx: i32, p: usize, categoria: u8, caso: u8) {
    let ficha = || cpw9_d(p + 0x61);
    match caso {
        0 => {
            let a = cpw9_b(ficha() + 0x1b);
            if a == 0x14 {
                if cpw9_recorrente(this, idx, categoria) {
                    let f = ficha();
                    let r = cpw9_sortear(3) as u8;
                    cpw9_somar_byte(f + 0x1b, 0xffu8.wrapping_sub(r));
                } else if cpw9_sortear(3) != 0 {
                    let f = ficha();
                    let r = cpw9_sortear(2) as u8;
                    cpw9_somar_byte(f + 0x1b, 0xffu8.wrapping_sub(r));
                }
            } else if a > 0xf && cpw9_sortear(3) == 0 {
                cpw9_somar_byte(ficha() + 0x1b, 0xff);
            }
            if cpw9_b(ficha() + 0x36) == 0x14 {
                if cpw9_recorrente(this, idx, categoria) {
                    let f = ficha();
                    let r = cpw9_sortear(3) as u8;
                    cpw9_somar_byte(f + 0x36, 0xffu8.wrapping_sub(r));
                } else if cpw9_sortear(3) != 0 {
                    cpw9_somar_byte(ficha() + 0x36, 0xff);
                }
            } else if cpw9_b(ficha() + 0x1b) > 0x11 && cpw9_sortear(3) == 0 {
                cpw9_somar_byte(ficha() + 0x1b, 0xff);
            }
        }
        2 => {
            if cpw9_recorrente(this, idx, categoria) && cpw9_b(ficha() + 0x1b) == 0x14 && cpw9_sortear(3) == 0 {
                cpw9_somar_byte(ficha() + 0x1b, 0xff);
            }
        }
        3 => {
            let c = cpw9_b(ficha() + 0x1d);
            if c == 0x14 {
                *((ficha() + 0x1d) as *mut u8) = 0x13;
            } else if c > 0xf && cpw9_sortear(3) == 0 {
                cpw9_somar_byte(ficha() + 0x1d, 0xff);
            }
        }
        4 => {
            let c = cpw9_b(ficha() + 0x1d);
            if c == 0x14 {
                if cpw9_sortear(3) != 0 {
                    cpw9_somar_byte(ficha() + 0x1d, 0xff);
                }
            } else if c > 0xf && cpw9_sortear(3) == 0 {
                cpw9_somar_byte(ficha() + 0x1d, 0xff);
            }
        }
        _ => {}
    }
}

/// Comparador lado a lado da fase do treino: a cópia em ensaio (sem soltar a lista, sem a lesão nova e sem as rotinas
/// do fim) e a original a partir do mesmo estado (o cabeçalho do Injury, a tabela de lesões e o sorteio); a original é
/// conferida quando chega às notícias da volta (0x62e010, desviada no mesmo item), antes das rotinas do fim. Com lesão
/// nova (aloca e manda notícia) ou com o físico de todos (dia 180 da primeira temporada): só a original.
unsafe extern "thiscall" fn comparar_dia_das_lesoes(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1293);
    let mut reg = vec![(this, 0x2c)];
    let n = cpw9_i(this + 8);
    if n > 0 && cpw9_d(this + 4) != 0 {
        reg.push((cpw9_d(this + 4), n as usize * 0x30));
    }
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&reg);
    CPW9_ENSAIO = true;
    CPW9_PAROU = false;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| dia_das_lesoes(this)));
    CPW9_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    if CPW9_PAROU {
        crate::comparacao::ramo("cpw_9 lesao nova so na original");
        return o(this);
    }
    *core::ptr::addr_of_mut!(CPW9_ESPERA) = Some((reg, mr));
    o(this);
    // sem a 0x62e010 desviada (desligada), a original não passou pelo ponto de conferência: fica sem comparação
    *core::ptr::addr_of_mut!(CPW9_ESPERA) = None;
}

/// { 0x62e010 } (thiscall, ret 0, injury.cpp, quadro SEH só das datas locais): `Injury::post_due_recovery_news()`, pelo
/// dia das lesões (0x62b7b0), todo dia. Nos clubes dos 16 técnicos humanos (as últimas 16 pessoas de [0xae23bc]), cada
/// jogador do elenco (50 vagas +0xd7) lesionado (+0xe): no dia em que começo (+0x20) + dias parados (+0x2a, mais de 5)
/// chega, a notícia da volta ao treino leve (0x630ee0, uma vez: +0x1c do registro de 31 bytes) e o físico do dia
/// (0x62c680). Depois a fila das voltas ([+0x10], 22 bytes: +0 data A, +8 data B, +0x10 id, +0x14 e +0x15 o que a
/// notícia leva, conta +0x18): as de data A hoje (bsearch 0x946464 com 0x778e90, voltando enquanto não for antes de hoje,
/// 0x456740) dão a notícia com os dias entre B e A (0x54cc30); as de data B hoje (bsearch com 0x632040) a notícia da volta
/// completa; as de data B já passada saem do começo da fila.
unsafe extern "thiscall" fn noticias_de_volta_aos_treinos(this: usize) {
    crate::comparacao::ramo("cpw_9 noticias de volta");
    let volta: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x630ee0usize);
    let fisico: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x62c680usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    for k in 0..0x10i32 {
        let n = cpw9_i(0xae2364);
        let s = cpw9_d(0xae23bc).wrapping_add(((n - k - 1) as u32).wrapping_mul(0x6e) as usize);
        let clube = cpw9_d(s + 0x39);
        if clube == 0 {
            continue;
        }
        for j in 0..50usize {
            let p = cpw9_d(clube + 0xd7 + 4 * j);
            if p == 0 || cpw9_i(p) >= cpw9_i(0xae2364) {
                continue;
            }
            let i = cpw9_i(cpw9_registro(this, cpw9_i(p)) + 0xe);
            if i == -1 {
                continue;
            }
            let inj = cpw9_d(this + 4).wrapping_add((i as u32).wrapping_mul(0x30) as usize);
            let (dia, ano) = (cpw9_w(0xae2c90), cpw9_uw(0xae2c92));
            let mut t = [0u8; 8];
            let r = mais(inj + 0x20, t.as_mut_ptr(), cpw9_uw(inj + 0x2a) as u32);
            if ano == cpw9_uw(r + 2) && dia == cpw9_w(r) && cpw9_w(inj + 0x2a) > 5 {
                let marca = cpw9_registro(this, cpw9_i(p)) + 0x1c;
                if *(marca as *const u8) == 0 {
                    *(marca as *mut u8) = 1;
                    let dias = cpw9_uw(inj + 0x28).wrapping_sub(cpw9_uw(inj + 0x2a));
                    // ponytail: os bytes de cima dos argumentos de 8 e 16 bits não têm valor na original; a 0x630ee0 lê char/short
                    volta(this, p, 0, *(inj as *const u8) as u32, 0, dias as u32);
                }
            }
            fisico(this, p);
        }
    }
    if cpw9_d(this + 0x10) == 0 {
        return;
    }
    let padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    let copiar: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let bsearch: unsafe extern "C" fn(*const u8, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let depois: unsafe extern "thiscall" fn(*const u8, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let diferenca: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> usize = core::mem::transmute(0x54cc30usize);
    let (mut d1, mut d2, mut t) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    padrao(d1.as_mut_ptr());
    padrao(d2.as_mut_ptr());
    copiar(d1.as_mut_ptr(), t.as_mut_ptr(), 0xae2c90);
    copiar(d2.as_mut_ptr(), t.as_mut_ptr(), 0xae2c90);
    let pd1 = d1.as_ptr() as usize;
    let (d1dia, d1ano) = (cpw9_w(pd1), cpw9_uw(pd1 + 2));
    let pd2 = d2.as_ptr() as usize;
    let (d2dia, d2ano) = (cpw9_w(pd2), cpw9_uw(pd2 + 2));
    let ultimo = || cpw9_d(this + 0x10).wrapping_add((cpw9_i(this + 0x18) as u32).wrapping_mul(0x16) as usize).wrapping_sub(0x16);
    let pessoa = |e: usize| cpw9_d(0xae23bc).wrapping_add((cpw9_d(e) as u32).wrapping_mul(0x6e) as usize);
    // as de data A hoje
    let mut e = bsearch(d1.as_ptr(), cpw9_d(this + 0x10), cpw9_i(this + 0x18) as u32, 0x16, 0x778e90);
    if e != 0 {
        if e > cpw9_d(this + 0x10) {
            loop {
                e -= 0x16;
                let (dia, ano) = (cpw9_w(e), cpw9_uw(e + 2));
                if ano > d1ano || (ano == d1ano && dia > d1dia) || depois(d1.as_ptr(), cpw9_d(e) as u32, cpw9_d(e + 4) as u32) != 0 {
                    e += 0x16;
                    break;
                }
                if e <= cpw9_d(this + 0x10) {
                    break;
                }
            }
        }
        while e <= ultimo() {
            let (dia, ano) = (cpw9_w(e), cpw9_uw(e + 2));
            if ano > d1ano || (ano == d1ano && dia > d1dia) {
                break;
            }
            if depois(d1.as_ptr(), cpw9_d(e) as u32, cpw9_d(e + 4) as u32) != 0 {
                break;
            }
            let p = pessoa(e + 0x10);
            let mut dif = [0u8; 8];
            diferenca(e + 8, dif.as_mut_ptr(), cpw9_d(e) as u32, cpw9_d(e + 4) as u32);
            let pd = dif.as_ptr() as usize;
            let dias = (cpw9_uw(pd + 2) as u32).wrapping_mul(365).wrapping_add(cpw9_uw(pd) as u32) as u16;
            volta(this, p, 0, *((e + 0x15) as *const u8) as u32, *((e + 0x14) as *const u8) as u32, dias as u32);
            e += 0x16;
        }
    }
    // as de data B hoje: a volta completa
    let mut e = bsearch(d1.as_ptr(), cpw9_d(this + 0x10), cpw9_i(this + 0x18) as u32, 0x16, 0x632040);
    if e == 0 {
        return;
    }
    let diferente = |b: usize| {
        let (dia, ano) = (cpw9_w(b), cpw9_uw(b + 2));
        ano != d2ano || dia != d2dia
    };
    if e > cpw9_d(this + 0x10) {
        loop {
            e -= 0x16;
            if diferente(e + 8) {
                e += 0x16;
                break;
            }
            if e <= cpw9_d(this + 0x10) {
                break;
            }
        }
    }
    while e <= ultimo() {
        if diferente(e + 8) {
            break;
        }
        let p = pessoa(e + 0x10);
        volta(this, p, 1, *((e + 0x15) as *const u8) as u32, *((e + 0x14) as *const u8) as u32, 0);
        e += 0x16;
    }
    // as de data B já passada saem do começo da fila
    let n = cpw9_i(this + 0x18);
    let mut tirar: i16 = 0;
    if n > 0 {
        let (hd, ha) = (cpw9_w(0xae2c90), cpw9_uw(0xae2c92));
        let mut q = cpw9_d(this + 0x10) + 0xa;
        let mut i = 0i32;
        while i < n {
            let ano = cpw9_uw(q);
            if ha < ano || (ha == ano && !(hd > cpw9_w(q - 2))) {
                break;
            }
            tirar = tirar.wrapping_add(1);
            i += 1;
            q += 0x16;
        }
        if tirar != 0 && tirar as i32 != n {
            let base = cpw9_d(this + 0x10);
            let t = tirar as i32 as usize;
            core::ptr::copy((base + t * 0x16) as *const u8, base as *mut u8, (n as usize - t) * 0x16);
        }
    }
    core::ptr::write_unaligned((this + 0x18) as *mut i32, cpw9_i(this + 0x18).wrapping_sub(tirar as i32));
}

// ponytail: as notícias da volta (0x630ee0) alocam e o físico do dia mexe no sorteio: só a original; a cópia leva o
// ramo. É também o ponto de conferência da fase do treino do dia das lesões (comparar_dia_das_lesoes).
unsafe extern "thiscall" fn comparar_noticias_de_volta_aos_treinos(this: usize) {
    if let Some((reg, mr)) = (*core::ptr::addr_of_mut!(CPW9_ESPERA)).take() {
        let mo = crate::comparacao::Memoria::ler(&reg);
        PLACAR.registrar(mo == mr, || format!("dia_das_lesoes({:x}) {:x?} x {:x?}", this, mo, mr));
    }
    orig::<unsafe extern "thiscall" fn(usize)>(1294)(this)
}

/// { 0x55d360 } (thiscall, ret 8, discipline.cpp): `Discipline::is_one_booking_from_ban(pessoa, competicao)`: com id
/// válido, a competição ([[0xadadfc] + 4 × id]) e o órgão dela (+0x50, 0xff sem; [this] + 4 × órgão), a pergunta ao órgão
/// (0x7e8d60 com o id, 0, 0 e o +4 da competição); senão 0. Pela tela do clube (0x4849f0).
unsafe extern "thiscall" fn um_cartao_da_suspensao(this: usize, pessoa: usize, competicao: usize) -> u32 {
    if pessoa != 0 && cpw9_i(pessoa) >= cpw9_i(0xae2364) {
        return 0;
    }
    let comp = cpw9_d(cpw9_d(0xadadfc).wrapping_add((cpw9_i(competicao) as u32).wrapping_mul(4) as usize));
    if comp == 0 || pessoa == 0 {
        return 0;
    }
    let orgao = *((comp + 0x50) as *const u8);
    if orgao == 0xff {
        return 0;
    }
    let o = cpw9_d(cpw9_d(this).wrapping_add((orgao as i8 as i32 as u32).wrapping_mul(4) as usize));
    if o == 0 {
        return 0;
    }
    let perguntar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x7e8d60usize);
    perguntar(o, cpw9_d(pessoa) as u32, 0, 0, cpw9_d(comp + 4) as u32)
}

unsafe extern "thiscall" fn comparar_um_cartao_da_suspensao(this: usize, pessoa: usize, competicao: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1295)(this, pessoa, competicao);
    let n = um_cartao_da_suspensao(this, pessoa, competicao);
    PLACAR.registrar(r == n, || format!("um_cartao_da_suspensao(pessoa {:x}, competicao {:x}) original={:x} rust={:x}", pessoa, competicao, r, n));
    r
}

// ---------- 259a leva: a fila dos regens: os regens jovens que vencem e os técnicos de reserva refeitos; o veículo da notícia pelo clube e pela pessoa (player_regen.cpp, media.cpp) ----------

unsafe fn cpw10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpw10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpw10_por16(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

unsafe fn cpw10_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A data da entrada `e` da fila dos regens (+4) é antes de hoje (0x456770)?
unsafe fn cpw10_vencida(e: usize) -> bool {
    let anterior: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    anterior(e + 4, cpw10_d(0xae2c90) as u32, cpw10_d(0xae2c94) as u32) != 0 // cpw10: a entrada vencida
}

/// Uma nota do técnico refeito (um dos blocos da 0x7a65d0): P = pow(+0x22 da pessoa, 0x968950) (vezes `fator`, quando
/// há), o maior entre P e o valor `v` da ficha, vezes `k`, no máximo `teto` (f64 em 0x...), truncado.
unsafe fn cpw10_nota(pessoa: usize, v: i16, fator: Option<usize>, k: usize, teto: usize) -> u16 {
    let mut p = pow_do_jogo(*((pessoa + 0x22) as *const u8) as f64, f64_em(0x968950));
    if let Some(f) = fator {
        p *= f64_em(f);
    }
    let v = v as f64;
    let m = if p > v { p } else { v };
    let mk = m * f64_em(k);
    let t = f64_em(teto);
    ftol(if !(mk >= t) { mk } else { t }) as u16
}

/// O cargo do técnico refeito no clube da máquina (0x7a6757..0x7a68d8): reputação do jogador (+0xb da ficha) abaixo da
/// do clube (+0x80) vezes 0x968968 e menos de 5 treinadores (0x53c8d0): as vagas de treinador vazias (+0x19f, 5) ficam
/// com ele (cargo 0xf); sem vaga, auxiliar (+0xd3, cargo 0xd) se o clube não tiver (0x53cb80). Senão, abaixo da do clube
/// vezes 0x969040, auxiliar do mesmo jeito.
unsafe fn cpw10_cargo_no_clube(s: usize) {
    let clube = cpw10_d(s + 0x39);
    if clube == 0 {
        return;
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 1, 1, 0, 0) != 0 {
        return;
    }
    let treinadores: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x53c8d0usize);
    let auxiliar: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53cb80usize);
    let rep = |k: usize| {
        let f = cpw10_w(cpw10_d(s + 0x61) + 0xb) as f64;
        let c = cpw10_w(cpw10_d(s + 0x39) + 0x80) as f64 * f64_em(k);
        c > f
    };
    if rep(0x968968) && (treinadores(cpw10_d(s + 0x39)) as i8) < 5 {
        for k in 0..5usize {
            let v = cpw10_d(s + 0x39) + 0x19f + 4 * k;
            if cpw10_d(v) == 0 {
                core::ptr::write_unaligned(v as *mut usize, s);
                *((s + 0x3d) as *mut u8) = 0xf;
            }
        }
        if *((s + 0x3d) as *const u8) == 0xf || auxiliar(cpw10_d(s + 0x39)) != 0 {
            return;
        }
        core::ptr::write_unaligned((cpw10_d(s + 0x39) + 0xd3) as *mut usize, s);
        *((s + 0x3d) as *mut u8) = 0xd;
        return;
    }
    if !rep(0x969040) || auxiliar(cpw10_d(s + 0x39)) != 0 {
        return;
    }
    core::ptr::write_unaligned((cpw10_d(s + 0x39) + 0xd3) as *mut usize, s);
    *((s + 0x3d) as *mut u8) = 0xd;
}

/// { 0x7a65d0 } (thiscall, ret 0, player_regen.cpp): `PLAYER_REGEN::process_regen_queue()`, pelo laço do dia (0x5cccd0),
/// todo dia. A fila dos regens jovens ([+0x10], 12 bytes: +0 id, +4 data; conta +0x14): da frente, enquanto a data da
/// primeira for antes de hoje (0x456770) e +0x40 não for zero, o regen jovem (0x7abac0) da pessoa (id fora de 0..[0xae2364]:
/// caixa "Invalid staff_id for regen" e fim), as seguintes sobem uma casa (0x7a8b70) e a última fica com id −1. Depois, com
/// mais de 2 reservas de técnico ([+0x1c], conta +0x20), para conta − 2 delas: a pessoa que empresta os dados (0x7a6e60
/// pela nação +4 da primeira) ganha o registro de não jogador da reserva (+0x69, [0xae23c0] + 0x44 × id); ex-jogador: a
/// troca de +0x10/+0x11 da nota 20, a normalização (0x539490), o cargo no clube da máquina e as três notas (+0xa, +8, +0xc)
/// pela reputação, +9 e +0xd da ficha (faixa acima de 7500, ou sorteio(+0x57) e as outras duas); não jogador: sorteio(1500)
/// e sorteio(500). A reserva sai (0x7a82b0).
unsafe extern "thiscall" fn fila_de_regens(this: usize) {
    let regen: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7abac0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7a8b70usize);
    let c0 = cpw10_i(this + 0x14);
    let primeira = cpw10_d(this + 0x10);
    let mut k = 0i32;
    while k < c0 {
        if !cpw10_vencida(primeira) || cpw10_i(this + 0x40) == 0 {
            break;
        }
        let id = cpw10_i(primeira);
        if id < 0 || id > cpw10_i(0xae2364) {
            let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
            formatar(0xde1f64, 0xa6bfcc, id);
            aviso_em(0xa6bf9c, 0x36e);
            return;
        }
        crate::comparacao::ramo("cpw_10 regen da fila");
        regen(this, cpw10_d(0xae23bc).wrapping_add((id as u32).wrapping_mul(0x6e) as usize));
        let mut j = 0i32;
        while j < cpw10_i(this + 0x14) - 1 {
            let e = cpw10_d(this + 0x10).wrapping_add((j as u32).wrapping_mul(0xc) as usize);
            copiar(e, e + 0xc);
            j += 1;
        }
        let n = cpw10_i(this + 0x14);
        let ultima = cpw10_d(this + 0x10).wrapping_add((n as u32).wrapping_mul(0xc) as usize).wrapping_sub(0xc);
        core::ptr::write_unaligned(ultima as *mut i32, -1);
        cpw10_por16(ultima + 4, 0);
        cpw10_por16(ultima + 6, 0);
        core::ptr::write_unaligned((this + 0x14) as *mut i32, n - 1); // cpw10: a entrada sai
        k += 1;
    }
    let total = cpw10_i(this + 0x20);
    if total <= 2 {
        return;
    }
    let doador: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7a6e60usize);
    let normalizar: unsafe extern "C" fn(usize) = core::mem::transmute(0x539490usize);
    let tirar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x7a82b0usize);
    for _ in 0..total - 2 {
        let s = doador(this, cpw10_d(cpw10_d(this + 0x1c) + 4));
        if s == 0 {
            return;
        }
        crate::comparacao::ramo("cpw_10 tecnico de reserva");
        let nj = cpw10_d(0xae23c0).wrapping_add((cpw10_d(cpw10_d(this + 0x1c)) as u32).wrapping_mul(0x44) as usize);
        core::ptr::write_unaligned((s + 0x69) as *mut usize, nj);
        let ficha = cpw10_d(s + 0x61);
        if ficha == 0 {
            let a = cpw10_sortear(0x5dc).wrapping_add(1) as u16;
            cpw10_por16(cpw10_d(s + 0x69) + 8, a);
            let b = cpw10_sortear(0x5dc).wrapping_add(1) as u16;
            cpw10_por16(cpw10_d(s + 0x69) + 0xa, b);
            let c = cpw10_sortear(0x1f4).wrapping_add(1) as u16;
            cpw10_por16(cpw10_d(s + 0x69) + 0xc, c);
        } else {
            if *((ficha + 0xf) as *const i8) == 0x14 {
                let (cl, dl) = (*((nj + 0x10) as *const i8), *((nj + 0x11) as *const i8));
                if cl > dl {
                    *((nj + 0x11) as *mut i8) = cl;
                    *((cpw10_d(s + 0x69) + 0x10) as *mut i8) = dl;
                    if cpw10_sortear(4) == 0 {
                        let r = cpw10_sortear(0x32) as u8;
                        *((cpw10_d(s + 0x69) + 0x11) as *mut u8) = r.wrapping_add(0x32);
                    }
                }
            }
            normalizar(s);
            cpw10_cargo_no_clube(s);
            let rep = cpw10_w(cpw10_d(s + 0x61) + 0xb);
            let (k, teto) = if rep > 0x1d4c {
                (0x9689d8, 0x9690b8)
            } else if cpw10_sortear(*((s + 0x57) as *const i8) as i32) != 0 {
                (0x969040, 0x96d300)
            } else {
                (0x968968, 0x96e288)
            };
            let a = cpw10_nota(s, rep, None, k, teto);
            cpw10_por16(cpw10_d(s + 0x69) + 0xa, a);
            let b = cpw10_nota(s, cpw10_w(cpw10_d(s + 0x61) + 9), None, k, teto);
            cpw10_por16(cpw10_d(s + 0x69) + 8, b);
            let c = cpw10_nota(s, cpw10_w(cpw10_d(s + 0x61) + 0xd), Some(0x969040), k, teto);
            cpw10_por16(cpw10_d(s + 0x69) + 0xc, c);
        }
        tirar(this, cpw10_i(cpw10_d(this + 0x1c)));
    }
}

/// O comparador: a 0x7abac0 (o regen jovem) aloca o contrato e manda notícia e as reservas de técnico refazem pessoas
/// achadas pelo caminho: só a original, conferindo o plano da fila. Sem entrada vencida e sem reservas a refazer, os dois
/// lado a lado no cabeçalho (nada muda); com vencidas, as k primeiras saem: as seguintes sobem k casas, as k últimas
/// ficam com id −1 e as palavras +4/+6 zeradas, a conta cai k. Com reservas (mais de 2) ou id inválido: só a original.
unsafe extern "thiscall" fn comparar_fila_de_regens(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1296);
    if cpw10_i(this + 0x20) > 2 {
        crate::comparacao::ramo("cpw_10 reservas so na original");
        return o(this);
    }
    let c0 = cpw10_i(this + 0x14);
    let base = cpw10_d(this + 0x10);
    let mut k = 0i32;
    if cpw10_i(this + 0x40) != 0 {
        while k < c0 {
            let e = base + k as usize * 0xc;
            if !cpw10_vencida(e) {
                break;
            }
            let id = cpw10_i(e);
            if id < 0 || id > cpw10_i(0xae2364) {
                return o(this); // ponytail: a caixa de erro só na original
            }
            k += 1;
        }
    }
    if k == 0 {
        let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x44)], || o(this), || fila_de_regens(this));
        PLACAR.registrar(igual, || format!("fila_de_regens({:x}) sem vencidas {:x?} x {:x?}", this, mo, mr));
        return;
    }
    let antes = core::slice::from_raw_parts(base as *const u8, c0 as usize * 0xc).to_vec();
    o(this);
    let (n, b2) = (cpw10_i(this + 0x14), cpw10_d(this + 0x10));
    let mut igual = n == c0 - k && b2 == base;
    if igual {
        let depois = core::slice::from_raw_parts(base as *const u8, c0 as usize * 0xc);
        let ficam = (c0 - k) as usize * 0xc;
        igual = depois[..ficam] == antes[k as usize * 0xc..];
        for j in (c0 - k) as usize..c0 as usize {
            let e = base + j * 0xc;
            igual &= cpw10_i(e) == -1 && cpw10_w(e + 4) == 0 && cpw10_w(e + 6) == 0;
        }
    }
    PLACAR.registrar(igual, || format!("fila_de_regens({:x}) {} vencidas de {}: conta {} base {:x}/{:x}", this, k, c0, n, base, b2));
}

/// O veículo pelo sorteio(100) dos grandes (0x757f30/0x757fa0): 0 → 6; até 7 → 3; até 14 → 5; até 39 → 2; até 59 → 0;
/// senão 1 (comparações do al com sinal).
fn cpw10_veiculo_grande(al: i8) -> u32 {
    if al == 0 {
        6
    } else if al < 8 {
        3
    } else if al < 0xf {
        5
    } else if al < 0x28 {
        2
    } else {
        (al >= 0x3c) as u32
    }
}

/// O veículo dos menores: sorteio(4) de 0 a 2 → 2, senão 4.
unsafe fn cpw10_veiculo_menor() -> u32 {
    let r = cpw10_sortear(4);
    if (0..=2).contains(&r) {
        2
    } else {
        4
    }
}

/// { 0x757f30 } (cdecl, media.cpp): `media_pick_outlet_type_for_club(clube)`: reputação (+0x80) acima de 6250: o
/// veículo dos grandes pelo sorteio(100); senão o dos menores (sorteio(4)). Pelas notícias do troféu (0x75c390) e da venda
/// (0x75cfc0).
unsafe extern "C" fn veiculo_pelo_clube(clube: usize) -> u32 {
    if cpw10_w(clube + 0x80) > 0x186a {
        cpw10_veiculo_grande(cpw10_sortear(100) as u8 as i8)
    } else {
        cpw10_veiculo_menor()
    }
}

unsafe extern "C" fn comparar_veiculo_pelo_clube(clube: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(1297);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(clube), || crate::aleatorio::sem_contar(|| veiculo_pelo_clube(clube)));
    PLACAR.registrar(igual && r == n, || format!("veiculo_pelo_clube({:x}) original={} rust={} sorteio igual={}", clube, r, n, igual));
    r
}

/// { 0x757fa0 } (cdecl, media.cpp): `media_pick_outlet_type_for_person(pessoa)`: como o do clube, pela reputação do
/// jogador (+0xb da ficha +0x61). Pelas histórias do jogador (0x72c1f0), do desabafo, do elogio, de querer jogar e da
/// comemoração da contratação (0x75b6e0, nas transferências).
unsafe extern "C" fn veiculo_pela_pessoa(pessoa: usize) -> u32 {
    if cpw10_w(cpw10_d(pessoa + 0x61) + 0xb) > 0x186a {
        cpw10_veiculo_grande(cpw10_sortear(100) as u8 as i8)
    } else {
        cpw10_veiculo_menor()
    }
}

unsafe extern "C" fn comparar_veiculo_pela_pessoa(pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(1298);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(pessoa), || crate::aleatorio::sem_contar(|| veiculo_pela_pessoa(pessoa)));
    PLACAR.registrar(igual && r == n, || format!("veiculo_pela_pessoa({:x}) original={} rust={} sorteio igual={}", pessoa, r, n, igual));
    r
}

// ---------- 259a leva: a tabela da liga: o resultado na tabela, a cópia da tabela e as posições previstas (league.cpp) ----------

unsafe fn cpw11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O ensaio do comparador da 0x685d30: a cópia roda sem a estatística da competição (0x49f570, que acha ou cria os
/// registros dos jogadores e não pode rodar duas vezes).
static mut CPW11_ENSAIO: bool = false;

/// { 0x685d30 } (thiscall, ret 8, league.cpp): `League::add_result(partida, jogo)`: grava a data do dia em +0xc9
/// (0x417c10 de [0xae2c90]); liga com fases (+0x44 = 0xff) e partida de uma fase (+0x42 ≠ 0xff): a fase
/// ([+0xc] + 4 × fase, até +0x30) recebe a partida pela virtual +4 e o que ela devolve sai; fase fora da faixa: as
/// caixas e 0. Partida de outra fase (+0x42 ≠ +0x44, que não é 0xfe): 0. Senão soma a partida na tabela (0x685f60 na
/// tabela +0xb1 com as +0x3e linhas, marcando) e, fora das fases 0xfe, com a competição ([0xadadfc] pelo id de +4) e a
/// estatística dela (+8) e com o jogo: 0x49f570 com os pontos dos dois; devolve 1. Partida nula: a caixa e −1.
unsafe extern "thiscall" fn somar_resultado_na_liga(liga: usize, p: usize, jogo: usize) -> i32 {
    if p == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(1299)(liga, p, jogo);
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    copia(liga + 0xc9, local.as_mut_ptr(), 0xae2c90);
    let fase_da_liga = *((liga + 0x44) as *const u8);
    let fase = *((p + 0x42) as *const u8);
    if fase_da_liga == 0xff && fase != 0xff {
        let k = fase as i8 as i32;
        if k > cpw11_d(liga + 0x30) as i32 || k < 0 {
            // as caixas da fase fora da faixa (linhas 0xc8e e 0xc8f): a original (a data já está gravada, igual)
            return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(1299)(liga, p, jogo);
        }
        let filha = cpw11_d(cpw11_d(liga + 0xc).wrapping_add(4 * k as usize));
        let somar: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(cpw11_d(cpw11_d(filha) + 4));
        return somar(filha, p, jogo);
    }
    if fase_da_liga != fase && fase_da_liga != 0xfe {
        return 0;
    }
    let aplicar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut i16, *mut i16) = core::mem::transmute(0x685f60usize);
    let (mut pts_casa, mut pts_fora) = (0i16, 0i16);
    // ponytail: a original empurra o +0x3e com o alto do ecx (sobra da chamada anterior); a 0x685f60 lê só o curto
    crate::comparacao::ramo("cpw_11 resultado aplicado");
    aplicar(liga, p, cpw11_d(liga + 0xb1), cpw11_w(liga + 0x3e) as u16 as u32, 1, &mut pts_casa, &mut pts_fora); // cpw11: a tabela
    if *((liga + 0x44) as *const u8) == 0xfe || cpw11_d(0xadadfc) == 0 {
        return 1;
    }
    let comp = cpw11_d(cpw11_d(0xadadfc).wrapping_add(cpw11_d(cpw11_d(liga + 4)).wrapping_mul(4)));
    if comp == 0 || cpw11_d(comp + 8) == 0 || jogo == 0 {
        return 1;
    }
    if !CPW11_ENSAIO {
        // ponytail: a original empurra os pontos como dwords das locais com o alto sem valor; a 0x49f570 lê os curtos
        let estatistica: unsafe extern "thiscall" fn(usize, usize, usize, i32, i32) -> u32 = core::mem::transmute(0x49f570usize);
        estatistica(cpw11_d(comp + 8), p, jogo, pts_casa as i32, pts_fora as i32);
    }
    1
}

/// Comparador: a partida nula e a da fase (a virtual da fase recebe a partida e compara por conta própria): só a
/// original. Nos outros casos, o ensaio: a cópia sem a estatística (0x49f570 acha ou cria registros) e a original a
/// partir do mesmo estado: a data em +0xc9, as linhas da tabela com os confrontos (as regiões da 0x685f60), o +0x3d/+0x3e
/// da partida e a devolução.
unsafe extern "thiscall" fn comparar_somar_resultado_na_liga(liga: usize, p: usize, jogo: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(1299);
    if p == 0 || (*((liga + 0x44) as *const u8) == 0xff && *((p + 0x42) as *const u8) != 0xff) {
        return o(liga, p, jogo); // ponytail: a caixa ou a fase, só a original
    }
    let n = cpw11_w(liga + 0x3e) as u16 as u32;
    let mut regioes = vec![(liga + 0xc9, 8)];
    regioes.extend(cpe2_regioes(p, cpw11_d(liga + 0xb1), n, core::ptr::null_mut(), core::ptr::null_mut()));
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW11_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| somar_resultado_na_liga(liga, p, jogo)));
    CPW11_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let ro = o(liga, p, jogo);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(ro == rr && mo == mr, || format!("somar_resultado_na_liga({:x}, partida {:x}) original={} {:x?} rust={} {:x?}", liga, p, ro, mo, rr, mr));
    ro
}

/// { 0x687710 } (thiscall, ret 0, league.cpp): `League::copy_table()`: sem tabela (+0xb1) ou sem linhas (+0x3e): 0.
/// Com +0xa7 = −1, grava nele (linhas − 1) × +0x3c (16 bits; mais de 50 linhas: a caixa da linha 0x1059 antes, pela
/// original). Aloca as linhas (0x944e46, 0x3b cada; sem memória: a caixa 0x1061 e 0) e copia; com os confrontos na
/// primeira linha (+5), cada linha ganha um vetor novo de (linhas − 1) × 14 bytes com a cópia dos dela; com a forma
/// (+9), um de +0xa7 bytes. Devolve a cópia. As alocações dos vetores não são conferidas (como na original).
unsafe extern "thiscall" fn copiar_tabela_da_liga(liga: usize) -> usize {
    let tab = cpw11_d(liga + 0xb1);
    let n = cpw11_w(liga + 0x3e);
    if tab == 0 || n == 0 {
        return 0;
    }
    if cpw11_w(liga + 0xa7) == -1 && n > 0 {
        if n > 0x32 {
            return orig::<unsafe extern "thiscall" fn(usize) -> usize>(1300)(liga); // ponytail: a caixa, só a original
        }
        let v = n.wrapping_sub(1).wrapping_mul(cpw11_w(liga + 0x3c));
        core::ptr::write_unaligned((liga + 0xa7) as *mut i16, v);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha = |i: i32| (i as usize).wrapping_mul(0x3b);
    let c = alocar((n as i32 * 0x3b) as u32);
    if c == 0 {
        aviso_em(0x9f15f4, 0x1061);
        return 0;
    }
    core::ptr::copy_nonoverlapping(tab as *const u8, c as *mut u8, n as i32 as usize * 0x3b);
    if cpw11_d(tab + 5) != 0 && n > 0 {
        let mut i: i16 = 0;
        while i < cpw11_w(liga + 0x3e) {
            let m = cpw11_w(liga + 0x3e) as i32;
            let h = alocar((14 * m - 14) as u32);
            core::ptr::write_unaligned((c + linha(i as i32) + 5) as *mut usize, h);
            let de = cpw11_d(cpw11_d(liga + 0xb1) + linha(i as i32) + 5);
            core::ptr::copy_nonoverlapping(de as *const u8, h as *mut u8, (14 * cpw11_w(liga + 0x3e) as i32 - 14) as usize);
            i += 1;
        }
    }
    if cpw11_d(cpw11_d(liga + 0xb1) + 9) != 0 && cpw11_w(liga + 0x3e) > 0 {
        let mut i: i16 = 0;
        while i < cpw11_w(liga + 0x3e) {
            let t = cpw11_w(liga + 0xa7) as i32;
            let h = alocar(t as u32);
            core::ptr::write_unaligned((c + linha(i as i32) + 9) as *mut usize, h);
            let de = cpw11_d(cpw11_d(liga + 0xb1) + linha(i as i32) + 9);
            core::ptr::copy_nonoverlapping(de as *const u8, h as *mut u8, cpw11_w(liga + 0xa7) as i32 as usize);
            i += 1;
        }
    }
    c
}

/// Comparador: aloca (não roda duas vezes): só a original, com o plano completo conferido depois: o +0xa7 esperado
/// (calculado antes) e a cópia devolvida igual à tabela, linha a linha (fora os ponteiros dos vetores novos), com os
/// vetores de confrontos e de forma iguais aos da tabela.
unsafe extern "thiscall" fn comparar_copiar_tabela_da_liga(liga: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(1300);
    let tab = cpw11_d(liga + 0xb1);
    let n = cpw11_w(liga + 0x3e);
    if tab == 0 || n <= 0 || n > 0x32 {
        return o(liga); // ponytail: nada a copiar ou a caixa: só a original
    }
    let a7 = if cpw11_w(liga + 0xa7) == -1 { n.wrapping_sub(1).wrapping_mul(cpw11_w(liga + 0x3c)) } else { cpw11_w(liga + 0xa7) };
    let c = o(liga);
    let mut erros = Vec::new();
    if cpw11_w(liga + 0xa7) != a7 {
        erros.push(format!("+0xa7 {} esperado {}", cpw11_w(liga + 0xa7), a7));
    }
    if c != 0 {
        let (h2h, forma) = (cpw11_d(tab + 5) != 0, cpw11_d(tab + 9) != 0);
        let n = n as usize;
        for i in 0..n {
            let (lt, lc) = (tab + 0x3b * i, c + 0x3b * i);
            for b in 0..0x3b {
                if (h2h && (5..9).contains(&b)) || (forma && (9..13).contains(&b)) {
                    continue;
                }
                if *((lt + b) as *const u8) != *((lc + b) as *const u8) {
                    erros.push(format!("linha {} +{:#x}", i, b));
                    break;
                }
            }
            let iguais = |o_: usize, t: usize| {
                let (a, b) = (cpw11_d(lt + o_), cpw11_d(lc + o_));
                b != 0 && core::slice::from_raw_parts(a as *const u8, t) == core::slice::from_raw_parts(b as *const u8, t)
            };
            if h2h && n > 1 && !iguais(5, 14 * (n - 1)) {
                erros.push(format!("linha {} confrontos", i));
            }
            if forma && a7 > 0 && !iguais(9, a7 as usize) {
                erros.push(format!("linha {} forma", i));
            }
        }
    }
    PLACAR.registrar(c != 0 && erros.is_empty(), || format!("copiar_tabela_da_liga({:x}) n {} copia {:x}: {}", liga, n, c, erros.join("; ")));
    c
}

/// { 0x68a850 } (thiscall, ret 0, league.cpp): `League::set_predicted_positions()`: sem estatística (+8): a caixa
/// (linha 0x189e). Copia os clubes das +0x3e linhas da tabela (+0xb1, 0x3b cada) para um vetor (0x944e46; sem memória:
/// a caixa 0x18a6, sem soltar), ordena pela força (0x4c08f0) e grava a posição i + 1 de cada clube não nulo na
/// estatística (0x4a2540); solta o vetor (0x9452ca).
unsafe extern "thiscall" fn posicoes_previstas_da_liga(liga: usize) {
    if cpw11_d(liga + 8) == 0 {
        return aviso_em(0x9f15f4, 0x189e);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, i32) = core::mem::transmute(0x4c08f0usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    let v = alocar(((cpw11_w(liga + 0x3e) as i32) << 2) as u32);
    if v == 0 {
        return aviso_em(0x9f15f4, 0x18a6);
    }
    let mut i = 0i32;
    while i < cpw11_w(liga + 0x3e) as i32 {
        *((v + 4 * i as usize) as *mut u32) = cpw11_d(cpw11_d(liga + 0xb1) + 0x3b * i as usize) as u32;
        i += 1;
    }
    ordenar(v, cpw11_w(liga + 0x3e) as i32);
    let mut i = 0i32;
    while i < cpw11_w(liga + 0x3e) as i32 {
        let c = cpw11_d(v + 4 * i as usize);
        if c != 0 {
            // ponytail: a original empurra o ecx com a posição no byte de baixo (o resto é sobra); a 0x4a2540 lê o byte
            gravar(cpw11_d(liga + 8), c, (i as u8).wrapping_add(1) as u32);
        }
        i += 1;
    }
    liberar(v);
}

/// Comparador: grava e pode criar registros na estatística (não roda duas vezes): o plano sai antes, sobre uma cópia
/// do vetor ordenada pelo núcleo da 0x4c08f0 (chaves num Vec); depois, o registro de cada clube (a última vez que ele
/// aparece) tem de ter em +0x72 a posição planejada.
unsafe extern "thiscall" fn comparar_posicoes_previstas_da_liga(liga: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1301);
    let n = cpw11_w(liga + 0x3e) as i32;
    if cpw11_d(liga + 8) == 0 || n <= 0 {
        return o(liga); // ponytail: a caixa ou nada a ordenar, só a original
    }
    let mut v: Vec<u32> = (0..n as usize).map(|i| cpw11_d(cpw11_d(liga + 0xb1) + 0x3b * i) as u32).collect();
    let mut k = vec![0i32; n as usize];
    cpt13_ordenar_com(v.as_mut_ptr(), k.as_mut_ptr(), n);
    o(liga);
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut erros = Vec::new();
    for (i, &c) in v.iter().enumerate() {
        if c == 0 || v[i + 1..].contains(&c) {
            continue;
        }
        let p = (i as u8).wrapping_add(1);
        let e = buscar(cpw11_d(liga + 8), c as usize);
        if e == 0 || *((e + 0x72) as *const u8) != p {
            erros.push(format!("clube {:x} esperado {} obtido {}", c, p, if e == 0 { -1 } else { *((e + 0x72) as *const u8) as i32 }));
        }
    }
    PLACAR.registrar(erros.is_empty(), || format!("posicoes_previstas_da_liga({:x}) {} clubes: {}", liga, n, erros.join("; ")));
}

// ---------- 259a leva: o jogo entre seleções: as convocações, os jogos e gols pela seleção, o nome do elenco e as rodadas conhecidas (national_teams.cpp, comp.cpp) ----------

unsafe fn cpw12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O ensaio do comparador da 0x76b600: a cópia roda sem o ranking da FIFA (0x58cf70, acrescenta pontos ao histórico),
/// sem a primeira convocação (0x86cc50, vai para o arquivo), sem o jogo pela seleção no registro de transferências
/// (0x8ce920) e sem a notícia aos clubes (0x7706c0, aloca).
static mut CPW12_ENSAIO: bool = false;

/// A vaga `k` (0..20) da escalação do lado `lado` no resumo do jogo `m`: m + 0x625 × lado + 0x190 + 0x49 × k.
fn cpw12_vaga(m: usize, lado: usize, k: usize) -> usize {
    m.wrapping_add(0x625 * lado + 0x190 + 0x49 * k)
}

/// A pessoa da vaga, se a vaga conta (id ≠ −1 e titular, ou reserva que entrou: +0x32 ≠ 0xff) e a pessoa está na
/// faixa (id < [0xae2364]; o ponteiro nulo passa sem o teste, como na original).
unsafe fn cpw12_pessoa(vaga: usize, k: usize) -> Option<usize> {
    let id = cpw12_d(vaga) as i32;
    if id == -1 || (k >= 11 && *((vaga + 0x32) as *const u8) == 0xff) {
        return None;
    }
    let p = cpw12_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    if p != 0 && cpw12_d(p) as i32 >= cpw12_d(0xae2364) as i32 {
        return None;
    }
    Some(p)
}

/// O registro geral da pessoa (79 bytes em [0xae2d0c]).
unsafe fn cpw12_geral(p: usize) -> usize {
    cpw12_d(0xae2d0c).wrapping_add(cpw12_d(p).wrapping_mul(0x4f))
}

/// O registro da nação do clube nas seleções: [this + 4] + 0x6e × id.
unsafe fn cpw12_nacao(this: usize, clube: usize) -> usize {
    cpw12_d(this + 4).wrapping_add(cpw12_d(cpw12_d(clube + 0x53)).wrapping_mul(0x6e))
}

/// Jogou pela nação do clube: se a segunda nação (+0x1e) é a dele, ela vira a primeira (+0x1a) e a antiga a segunda.
unsafe fn cpw12_trocar_nacao(p: usize, clube: usize) {
    if cpw12_d(clube + 0x53) == cpw12_d(p + 0x1e) {
        core::ptr::write_unaligned((p + 0x1e) as *mut usize, cpw12_d(p + 0x1a));
        core::ptr::write_unaligned((p + 0x1a) as *mut usize, cpw12_d(clube + 0x53));
    }
}

/// O jogo de nação por lado (o seu código de competição em [p + 0x14]): conta o jogo da nação (+2 do registro dela,
/// curto) e cada pessoa que conta na escalação.
unsafe fn cpw12_lados(this: usize, p: usize, m: usize, mut f: impl FnMut(usize, usize, usize, usize)) {
    for lado in 0..2usize {
        let clube = cpw12_d(p + if lado == 0 { 0x1c } else { 0x20 });
        let jogos = (cpw12_nacao(this, clube) + 2) as *mut i16;
        core::ptr::write_unaligned(jogos, core::ptr::read_unaligned(jogos).wrapping_add(1));
        for k in 0..20usize {
            let vaga = cpw12_vaga(m, lado, k);
            if let Some(pessoa) = cpw12_pessoa(vaga, k) {
                f(clube, vaga, k, pessoa);
            }
        }
    }
}

/// { 0x76b600 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::record_international_result(partida, resumo)`:
/// só entre duas seleções (0x53b900). Pela competição da partida ([+0x14]): a de [0x9cf794] marca +0xd = 1 no registro
/// geral de quem jogou; a de [0x9cf798] liga o bit 2 do +0xc e troca a nação (+0x1a/+0x1e); a de [0x9cf7a0] nada; as
/// outras: o ranking da FIFA (0x58cf70 em [this+0x1c]) e, de quem jogou, a primeira convocação (0x86cc50 com o
/// adversário, se +0x22 = 0, depois da troca da nação), um jogo (+0x22) e os gols (+0x23 += +0x21 da vaga) pela
/// seleção e o jogo no registro (0x8ce920 com o número de jogos da nação). Em todas, o jogo da nação (+2) sobe. Depois,
/// por lado: o tipo (1 se +0x40 = 0; senão 2 na segunda seleção 0x5409b0, senão 0); na data do tipo (+0x3a + 20 ×
/// tipo, 0x4a7f20 com a de hoje) zera +0x44 + 20 × tipo; a notícia aos clubes (0x7706c0) com o destaque (vaga +0x153) e
/// a lista de quem jogou. Sem SEH, sem caixa, sem sorteio.
unsafe extern "thiscall" fn resultado_entre_selecoes(this: usize, p: usize, m: usize) {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if e_selecao(cpw12_d(p + 0x1c)) == 0 || e_selecao(cpw12_d(p + 0x20)) == 0 {
        return;
    }
    let comp = cpw12_d(cpw12_d(p + 0x14));
    if comp == cpw12_d(0x9cf794) {
        cpw12_lados(this, p, m, |_, _, _, pessoa| *((cpw12_geral(pessoa) + 0xd) as *mut u8) = 1);
    } else if comp == cpw12_d(0x9cf798) {
        cpw12_lados(this, p, m, |clube, _, _, pessoa| {
            *((cpw12_geral(pessoa) + 0xc) as *mut u8) |= 2;
            cpw12_trocar_nacao(pessoa, clube);
        });
    } else if comp != cpw12_d(0x9cf7a0) {
        let fifa: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x58cf70usize);
        let primeira: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86cc50usize);
        let jogo_pela_selecao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8ce920usize);
        if !CPW12_ENSAIO {
            fifa(cpw12_d(this + 0x1c), p);
        }
        for lado in 0..2usize {
            let clube = cpw12_d(p + if lado == 0 { 0x1c } else { 0x20 });
            let jogos = (cpw12_nacao(this, clube) + 2) as *mut i16;
            core::ptr::write_unaligned(jogos, core::ptr::read_unaligned(jogos).wrapping_add(1));
            for k in 0..20usize {
                let vaga = cpw12_vaga(m, lado, k);
                let id = cpw12_d(vaga) as i32;
                if id == -1 {
                    continue;
                }
                if k >= 11 && *((vaga + 0x32) as *const u8) == 0xff { // cpw12: a reserva que não entrou
                    crate::comparacao::ramo("cpw_12 reserva sem entrar");
                    continue;
                }
                let pessoa = cpw12_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
                if pessoa != 0 && cpw12_d(pessoa) as i32 >= cpw12_d(0xae2364) as i32 {
                    continue;
                }
                if *((pessoa + 0x22) as *const u8) == 0 {
                    cpw12_trocar_nacao(pessoa, clube);
                    let adversario = if clube == cpw12_d(p + 0x1c) { cpw12_d(p + 0x20) } else { cpw12_d(p + 0x1c) };
                    if !CPW12_ENSAIO {
                        primeira(cpw12_d(0xdd7f00), pessoa, adversario);
                    }
                }
                crate::comparacao::ramo("cpw_12 jogo pela seleção");
                *((pessoa + 0x22) as *mut u8) = (*((pessoa + 0x22) as *const u8)).wrapping_add(1); // cpw12: um jogo
                *((pessoa + 0x23) as *mut u8) = (*((pessoa + 0x23) as *const u8)).wrapping_add(*((vaga + 0x21) as *const u8)); // cpw12: os gols
                if !CPW12_ENSAIO {
                    // a original empurra o edx com o curto dos jogos embaixo e o alto de 55 × id da nação
                    let id_nacao = cpw12_d(cpw12_d(clube + 0x53)) as u32;
                    let n = core::ptr::read_unaligned(jogos) as u16 as u32;
                    jogo_pela_selecao(cpw12_d(0xde1f54), pessoa, (id_nacao.wrapping_mul(0x37) & 0xffff_0000) | n);
                }
            }
        }
    }
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let mesma_data: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x4a7f20usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut usize, u32) = core::mem::transmute(0x7706c0usize);
    for lado in 0..2usize {
        let clube = cpw12_d(p + if lado == 0 { 0x1c } else { 0x20 });
        let b = *((p + 0x40) as *const u8) == 0;
        let tipo: usize = if segunda(clube) == 0 { if b { 1 } else { 0 } } else if b { 1 } else { 2 };
        let reg = cpw12_nacao(this, clube) + 20 * tipo;
        if mesma_data(reg + 0x3a, cpw12_d(0xae2c90) as u32, cpw12_d(0xae2c94) as u32) != 0 {
            *((reg + 0x44) as *mut u8) = 0;
        }
        let mut lista = [0usize; 20];
        let mut q = 0usize;
        for k in 0..20usize {
            if let Some(pessoa) = cpw12_pessoa(cpw12_vaga(m, lado, k), k) {
                lista[q] = pessoa;
                q += 1;
            }
        }
        let destaque_k = *((m + 0x625 * lado + 0x153) as *const i8) as isize;
        let destaque_id = cpw12_d((m + 0x625 * lado + 0x190) .wrapping_add((0x49 * destaque_k) as usize));
        let destaque = cpw12_d(0xae23bc).wrapping_add(destaque_id.wrapping_mul(0x6e));
        if !CPW12_ENSAIO {
            // ponytail: a original empurra a conta num dword da local com só o byte de baixo gravado; a 0x7706c0 lê o byte
            noticia(this, clube, p, m, destaque, lista.as_mut_ptr(), q as u32);
        }
    }
}

/// As regiões que a 0x76b600 grava por conta própria: de cada lado, o registro da nação (o +2 e os +0x44 dos tipos) e,
/// de cada vaga com pessoa na faixa, os campos +0x1a..+0x24 dela e o +0xc/+0xd do registro geral.
unsafe fn cpw12_regioes(this: usize, p: usize, m: usize) -> Vec<(usize, usize)> {
    let mut reg = Vec::new();
    for lado in 0..2usize {
        let clube = cpw12_d(p + if lado == 0 { 0x1c } else { 0x20 });
        let r = cpw12_nacao(this, clube);
        reg.extend([(r + 2, 2), (r + 0x44, 1), (r + 0x58, 1), (r + 0x6c, 1)]);
        for k in 0..20usize {
            let vaga = cpw12_vaga(m, lado, k);
            let id = cpw12_d(vaga) as i32;
            if id == -1 {
                continue;
            }
            let pessoa = cpw12_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
            if pessoa != 0 && (cpw12_d(pessoa) as i32) < cpw12_d(0xae2364) as i32 {
                reg.push((pessoa + 0x1a, 10));
                reg.push((cpw12_geral(pessoa) + 0xc, 2));
            }
        }
    }
    reg
}

/// Comparador: o ensaio (a cópia sem o ranking, a primeira convocação, o registro de transferências e a notícia, que
/// alocam, vão para o arquivo ou gravam fora) e a original a partir do mesmo estado, conferidos nas regiões que a
/// rotina grava por conta própria (cpw12_regioes). Fora de jogo entre seleções: só a original.
unsafe extern "thiscall" fn comparar_resultado_entre_selecoes(this: usize, p: usize, m: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1302);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if e_selecao(cpw12_d(p + 0x1c)) == 0 || e_selecao(cpw12_d(p + 0x20)) == 0 {
        return o(this, p, m); // ponytail: não grava nada, só a original
    }
    let regioes = cpw12_regioes(this, p, m);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW12_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| resultado_entre_selecoes(this, p, m)));
    CPW12_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(this, p, m);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("resultado_entre_selecoes(partida {:x}, resumo {:x}) original={:x?} rust={:x?}", p, m, mo, mr));
}

/// { 0x775610 } (thiscall, ret 0x14, national_teams.cpp; `this` não é lido): `NationalTeams::format_squad_label(tipo,
/// sub23, saida, tamanho, minusculo)`: tamanho abaixo de 20: a caixa (linha 0x1c98). Escreve (0x66f4e0) o nome do
/// elenco: tipo 0 "Main", 1 "'B'", outros "Under 23" (com sub23) ou "Under 21"; com minúsculo = 1, em minúsculas.
unsafe extern "thiscall" fn nome_do_elenco_da_selecao(this: usize, tipo: u32, sub23: u32, saida: usize, tamanho: i32, minusculo: u32) -> u32 {
    if tamanho < 0x14 {
        return orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, i32, u32) -> u32>(1303)(this, tipo, sub23, saida, tamanho, minusculo);
    }
    let texto: unsafe extern "C" fn(usize, usize, ...) -> u32 = core::mem::transmute(0x66f4e0usize);
    let s = match (minusculo as u8 == 1, tipo as u8, sub23 != 0) {
        (true, 0, _) => 0x9cd9b4,
        (true, 1, _) => 0xa62fa0,
        (true, _, true) => 0xa62f94,
        (true, _, false) => 0xa62f88,
        (false, 0, _) => 0xa62f80,
        (false, 1, _) => 0xa62fa0,
        (false, _, true) => 0xa62f74,
        (false, _, false) => 0xa62f68,
    };
    texto(saida, s)
}

/// Comparador: lado a lado no começo da saída (até 64 bytes) e na devolução. A caixa: só a original.
unsafe extern "thiscall" fn comparar_nome_do_elenco_da_selecao(this: usize, tipo: u32, sub23: u32, saida: usize, tamanho: i32, minusculo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, i32, u32) -> u32>(1303);
    if tamanho < 0x14 || saida == 0 {
        return o(this, tipo, sub23, saida, tamanho, minusculo); // ponytail: a caixa, só a original
    }
    let reg = [(saida, (tamanho as usize).min(0x40))];
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tipo, sub23, saida, tamanho, minusculo),
        || nome_do_elenco_da_selecao(this, tipo, sub23, saida, tamanho, minusculo));
    PLACAR.registrar(igual && ro == rr, || format!("nome_do_elenco_da_selecao({}, {}, {}) original={:x} {:x?} rust={:x} {:x?}", tipo as u8, sub23, minusculo as u8, ro, mo, rr, mr));
    ro
}

/// { 0x48d410 } (thiscall, ret 8, comp.cpp): `Comp::are_round_fixtures_known(fase, rodada)`: a fase (−1 = a própria
/// competição; acima de +0x30: 0; abaixo de −1: a caixa da linha 0x36e, pela original) é liga (+0x43 = 2): 1. Senão 1
/// se a rodada é antes da atual (+0xab, byte) ou é a atual, a última (+0x3c − 1) e com o código 0x8c (+7 do registro
/// de 0x68 bytes dela em +0xa3); senão 0.
unsafe extern "thiscall" fn rodada_com_jogos_conhecidos(this: usize, fase: i32, rodada: i32) -> u32 {
    if fase > cpw12_d(this + 0x30) as i32 {
        return 0;
    }
    if fase < -1 {
        return orig::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(1304)(this, fase, rodada);
    }
    let c = if fase == -1 { this } else { cpw12_d(cpw12_d(this + 0xc).wrapping_add(4 * fase as usize)) };
    if *((c + 0x43) as *const u8) == 2 {
        return 1;
    }
    let atual = *((c + 0xab) as *const i8) as i32;
    if rodada < atual {
        return 1;
    }
    if rodada != atual {
        return 0;
    }
    let n = core::ptr::read_unaligned((c + 0x3c) as *const i16) as i32;
    if atual != n - 1 {
        return 0;
    }
    let cod = core::ptr::read_unaligned((cpw12_d(c + 0xa3).wrapping_add((0x68 * (n - 1)) as usize) + 7) as *const i16);
    (cod == 0x8c) as u32
}

/// Comparador: pelo valor (só lê). A caixa: só a original.
unsafe extern "thiscall" fn comparar_rodada_com_jogos_conhecidos(this: usize, fase: i32, rodada: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(1304);
    if fase < -1 && fase <= cpw12_d(this + 0x30) as i32 {
        return o(this, fase, rodada); // ponytail: a caixa, só a original
    }
    let r = o(this, fase, rodada);
    let n = rodada_com_jogos_conhecidos(this, fase, rodada);
    PLACAR.registrar(r == n, || format!("rodada_com_jogos_conhecidos({:x}, {}, {}) original={} rust={}", this, fase, rodada, r, n));
    r
}

// ---------- 259a leva: os convocados por clube na data do jogo da seleção, a importância do jogo e a fase da competição (national_teams.cpp, comp.cpp) ----------

unsafe fn cpw13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O registro de convocações do clube `c` em [this + 0x20] (6 bytes: +0 vetor de entradas de 9 bytes, +4 capacidade,
/// +5 quantas), se o clube conta: id dentro dos [0xae235c] − 2 × [0xae2350] registros, com nação (+0x53) marcada
/// (+0x11c) e que não é a de [0x9cf260].
unsafe fn cpw13_registro(this: usize, c: usize) -> Option<usize> {
    let n = (cpw13_d(0xae235c) as i32).wrapping_sub((cpw13_d(0xae2350) as i32).wrapping_mul(2));
    let id = cpw13_d(c) as i32;
    if id >= n || id < 0 {
        return None;
    }
    let nacao = cpw13_d(c + 0x53);
    if nacao == 0 || *((nacao + 0x11c) as *const u8) == 0 || cpw13_d(nacao) == cpw13_d(0x9cf260) {
        return None;
    }
    Some(cpw13_d(this + 0x20).wrapping_add(6 * id as usize))
}

/// Soma `conta` convocados do clube `c` na data `d` (dia +0, ano +2): na entrada da mesma data, se houver; senão uma
/// entrada nova (com o vetor cheio, mais 3 vagas por 0x945667) com a data e a conta, e o vetor reordenado pela data
/// (qsort 0x945333 com 0x778e90). Sem memória: a caixa (linha 0x21c8) e false (a rotina para).
unsafe fn cpw13_somar(this: usize, c: usize, conta: u8, d: *mut u8) -> bool {
    let rec = match cpw13_registro(this, c) {
        Some(r) => r,
        None => return true,
    };
    let quantas = *((rec + 5) as *const i8);
    let (dia, ano) = (core::ptr::read_unaligned(d as *const u16), core::ptr::read_unaligned((d as usize + 2) as *const u16));
    let mut k: i8 = 0;
    while k < quantas {
        let e = cpw13_d(rec).wrapping_add(9 * k as i32 as usize);
        if core::ptr::read_unaligned((e + 2) as *const u16) == ano && core::ptr::read_unaligned(e as *const u16) == dia {
            crate::comparacao::ramo("cpw_13 soma na data");
            *((e + 8) as *mut u8) = (*((e + 8) as *const u8)).wrapping_add(conta); // cpw13: soma na data
            return true;
        }
        k += 1;
    }
    if quantas == *((rec + 4) as *const i8) {
        let cap = (*((rec + 4) as *const u8)).wrapping_add(3);
        *((rec + 4) as *mut u8) = cap;
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let v = realocar(cpw13_d(rec), (cap as i8 as i32 * 9) as u32);
        core::ptr::write_unaligned(rec as *mut usize, v);
        if v == 0 {
            aviso_em(0xa5b2a4, 0x21c8);
            return false;
        }
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, *mut u8) -> usize = core::mem::transmute(0x417c10usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let q = *((rec + 5) as *const i8) as i32;
    let e = cpw13_d(rec).wrapping_add(9 * q as usize);
    let mut t = [0u8; 8];
    copia(e, t.as_mut_ptr(), d);
    crate::comparacao::ramo("cpw_13 registro novo");
    *((e + 8) as *mut u8) = conta; // cpw13: a conta do registro novo
    let q = (*((rec + 5) as *const i8)).wrapping_add(1);
    *((rec + 5) as *mut i8) = q;
    qsort(cpw13_d(rec), q as i32 as u32, 9, 0x778e90);
    true
}

/// { 0x7771c0 } (thiscall, ret 0xc, national_teams.cpp; quadro SEH só do estado, as datas não têm destrutor):
/// `NationalTeams::record_club_call_ups(time, b, partida)`: ordena o elenco do time (+0xd7, 50 vagas) pelo clube (qsort
/// com 0x778ef0). Com a partida: a data dela (+0x2a, +0x28; um dia antes no segundo time 0x5409b0 ou com `b`); conta os
/// convocados seguidos de cada clube (+0x39), pulando as vagas vazias, as pessoas fora da faixa ([0xae2364]) e as de
/// outro elenco (+0x5e bit 3 sem `b`, bit 2 com), e ao mudar de clube soma a conta do anterior no registro dele
/// (cpw13_somar). O último clube da volta fica sem registro (como na original).
unsafe extern "thiscall" fn registrar_convocados_por_clube(this: usize, time: usize, b: u32, p: usize) {
    let padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let mut d = [0u8; 8];
    padrao(d.as_mut_ptr());
    qsort(time + 0xd7, 0x32, 4, 0x778ef0);
    if p == 0 {
        return;
    }
    let nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let copia: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let menos: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let (mut t1, mut t2) = ([0u8; 8], [0u8; 8]);
    // ponytail: a original empurra os curtos +0x2a e +0x28 com o alto do registro; a 0x549ef0 só lê os curtos
    let r = nova(t1.as_mut_ptr(), *((p + 0x2a) as *const u16) as u32, *((p + 0x28) as *const u16) as u32);
    copia(d.as_mut_ptr(), t2.as_mut_ptr(), r);
    if segunda(time) != 0 || b != 0 {
        let r = menos(d.as_mut_ptr(), t2.as_mut_ptr(), 1);
        copia(d.as_mut_ptr(), t1.as_mut_ptr(), r);
    }
    let mut anterior = 0usize;
    let mut conta: u8 = 0;
    for i in 0..0x32usize {
        let s = cpw13_d(time + 0xd7 + 4 * i);
        if s == 0 || cpw13_d(s) as i32 >= cpw13_d(0xae2364) as i32 {
            continue;
        }
        let marcas = *((s + 0x5e) as *const u8);
        if (b == 0 && marcas & 8 != 0) || (b != 0 && marcas & 4 != 0) {
            continue;
        }
        let c = cpw13_d(s + 0x39);
        if c == anterior && anterior != 0 {
            conta = conta.wrapping_add(1);
            continue;
        }
        if anterior != 0 && !cpw13_somar(this, anterior, conta, d.as_mut_ptr()) {
            return;
        }
        anterior = c;
        conta = 1;
    }
    if anterior != 0 && cpw13_registro(this, anterior).is_some() {
        crate::comparacao::ramo("cpw_13 último clube sem registro"); // cpw13: o último clube fica sem registro
    }
}

/// O registro de algum clube que a 0x7771c0 pode somar está com o vetor cheio (capacidade = quantas, inclusive sem
/// vetor)? Então ela pode realocar.
unsafe fn cpw13_pode_realocar(this: usize, time: usize) -> bool {
    (0..0x32usize).any(|i| {
        let s = cpw13_d(time + 0xd7 + 4 * i);
        if s == 0 || cpw13_d(s) as i32 >= cpw13_d(0xae2364) as i32 || cpw13_d(s + 0x39) == 0 {
            return false;
        }
        match cpw13_registro(this, cpw13_d(s + 0x39)) {
            Some(rec) => *((rec + 4) as *const i8) == *((rec + 5) as *const i8),
            None => false,
        }
    })
}

/// Comparador: lado a lado no elenco do time (50 vagas) e, de cada clube do elenco que conta, no registro (6 bytes) e
/// no vetor de entradas inteiro (capacidade × 9). Quando algum vetor pode crescer (0x945667 solta o velho; não roda
/// duas vezes): só a original, conferindo depois o elenco ordenado pelo clube (o mesmo qsort sobre uma cópia, antes).
unsafe extern "thiscall" fn comparar_registrar_convocados_por_clube(this: usize, time: usize, b: u32, p: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(1305);
    if p != 0 && cpw13_pode_realocar(this, time) {
        let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
        let mut esperado: Vec<u32> = (0..0x32usize).map(|i| cpw13_d(time + 0xd7 + 4 * i) as u32).collect();
        // o mesmo qsort da CRT (a ordem dos iguais é a dele), fora do rastro da original
        crate::aleatorio::pausar_rastro(|| qsort(esperado.as_mut_ptr() as usize, 0x32, 4, 0x778ef0));
        o(this, time, b, p);
        let obtido: Vec<u32> = (0..0x32usize).map(|i| cpw13_d(time + 0xd7 + 4 * i) as u32).collect();
        PLACAR.registrar(obtido == esperado, || format!("registrar_convocados_por_clube({:x}) elenco original={:x?} esperado={:x?}", time, obtido, esperado));
        return;
    }
    let mut reg = vec![(time + 0xd7, 0xc8)];
    let mut vistos = Vec::new();
    for i in 0..0x32usize {
        let s = cpw13_d(time + 0xd7 + 4 * i);
        if s == 0 || cpw13_d(s) as i32 >= cpw13_d(0xae2364) as i32 {
            continue;
        }
        let c = cpw13_d(s + 0x39);
        if c == 0 || vistos.contains(&c) {
            continue;
        }
        vistos.push(c);
        if let Some(rec) = cpw13_registro(this, c) {
            reg.push((rec, 6));
            let cap = *((rec + 4) as *const i8);
            if cpw13_d(rec) != 0 && cap > 0 {
                reg.push((cpw13_d(rec), 9 * cap as usize));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, time, b, p), || registrar_convocados_por_clube(this, time, b, p));
    PLACAR.registrar(igual, || format!("registrar_convocados_por_clube({:x}, b {}, partida {:x}) {:x?} x {:x?}", time, b, p, mo, mr));
}

/// { 0x48e190 } (thiscall, ret 0xc, comp.cpp): `Comp::get_fixture_match_importance(partida, a, b)`: a fase da partida
/// (+0x42; 0xff = a própria competição, senão [+0xc] + 4 × fase) responde pela virtual +0x54 com os mesmos argumentos.
unsafe extern "thiscall" fn importancia_do_jogo_na_competicao(this: usize, p: usize, a: u32, b: u32) -> u32 {
    let fase = *((p + 0x42) as *const u8);
    let c = if fase == 0xff { this } else { cpw13_d(cpw13_d(this + 0xc).wrapping_add(4 * (fase as i8 as i32) as usize)) };
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(cpw13_d(cpw13_d(c) + 0x54));
    f(c, p, a, b)
}

/// Comparador: pelo valor (a virtual da fase só lê; roda duas vezes).
unsafe extern "thiscall" fn comparar_importancia_do_jogo_na_competicao(this: usize, p: usize, a: u32, b: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(1306)(this, p, a, b);
    let n = importancia_do_jogo_na_competicao(this, p, a, b);
    PLACAR.registrar(r == n, || format!("importancia_do_jogo_na_competicao({:x}, partida {:x}, {}, {}) original={} rust={}", this, p, a, b, r, n));
    r
}

/// { 0x48d200 } (thiscall, ret 4, comp.cpp): `Comp::get_stage(k)`: −1 a própria competição, 0..+0x30 a fase
/// [+0xc] + 4 × k; fora disso a caixa (linha 0x36e) e 0, pela original.
unsafe extern "thiscall" fn fase_da_competicao(this: usize, k: i32) -> usize {
    if k < -1 || k > cpw13_d(this + 0x30) as i32 {
        return orig::<unsafe extern "thiscall" fn(usize, i32) -> usize>(1307)(this, k);
    }
    if k == -1 {
        return this;
    }
    cpw13_d(cpw13_d(this + 0xc).wrapping_add(4 * k as usize))
}

/// Comparador: pelo valor (só lê). A caixa: só a original.
unsafe extern "thiscall" fn comparar_fase_da_competicao(this: usize, k: i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> usize>(1307);
    if k < -1 || k > cpw13_d(this + 0x30) as i32 {
        return o(this, k); // ponytail: a caixa, só a original
    }
    let r = o(this, k);
    let n = fase_da_competicao(this, k);
    PLACAR.registrar(r == n, || format!("fase_da_competicao({:x}, {}) original={:x} rust={:x}", this, k, r, n));
    r
}
