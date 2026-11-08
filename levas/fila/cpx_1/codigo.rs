// ---------- %%LEVA%%a leva: a oferta de transferência recusada pelo clube vendedor e a retirada pelo comprador, com a notícia da oferta recusada (transfer_manager.cpp) ----------

unsafe fn cpx1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpx1_clube(id: usize) -> usize {
    cpx1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpx1_pessoa(id: usize) -> usize {
    cpx1_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (CONTRACT_MANAGER::get_contract 0x4cf5a0 em [0xae1050]), guardado em [0xae1080] como
/// a original faz depois de cada chamada.
unsafe fn cpx1_contrato(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cpx1_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    c
}

/// Os itens de uma LLIST do começo (o cursor +0xa volta a +2), como os laços da original.
unsafe fn cpx1_itens(l: usize) -> Vec<usize> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpx1_d(l + 2));
    let mut v = Vec::new();
    let mut e = proximo(l);
    while e != 0 {
        v.push(e);
        e = proximo(l);
    }
    v
}

/// O ensaio do comparador: a cópia roda sem as chamadas que mexem no resto do jogo (o desassossego com sorteio, a
/// notícia, o agendamento, as listas de observação e as ofertas de contrato), que não podem rodar duas vezes.
static mut CPX1_ENSAIO: bool = false;

/// A gerência da lista de observação (Shortlist_Manager em +0x856 da gerência de transferências).
unsafe fn cpx1_listas(this: usize) -> usize {
    cpx1_d(this + 0x856)
}

/// Shortlist_Manager::clear_bid_markers_for_player (0x84aeb0: thiscall(gerência, clube, pessoa), ret 8); no ensaio,
/// nada.
unsafe fn cpx1_desmarcar(this: usize, clube: usize, pessoa: usize) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    f(cpx1_listas(this), clube, pessoa);
}

/// Shortlist_Manager::update_staff_status (0x843e10: thiscall(gerência, pessoa, clube, evento, tipo), ret 0x10); no
/// ensaio, nada.
unsafe fn cpx1_situacao_na_lista(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x843e10usize);
    f(cpx1_listas(this), pessoa, clube, evento, tipo);
}

/// Uma chamada thiscall(gerência de transferências, oferta) com ret 4 (0x8c1290 desassossegar, 0x8cbf70 tirar a
/// transferência futura, 0x8c6db0 fechar as ofertas de contrato da oferta); no ensaio, nada.
unsafe fn cpx1_com_a_oferta(f: usize, this: usize, o: usize) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
    f(this, o);
}

/// O fim comum da recusa e da retirada, a partir do teste da oferta encerrada: sem oferta rival (+0) em aberto, a marca
/// de oferta em andamento (bit 0 do registro de 8 bytes da pessoa em +0x86e) sai.
unsafe fn cpx1_encerrada(o: usize) -> bool {
    match cpx1_b(o + 0x2c) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => cpx1_b(o + 0x2e) >= 3,
        _ => false,
    }
}

unsafe fn cpx1_tirar_marca(this: usize, o: usize) {
    if !cpx1_encerrada(o) {
        return;
    }
    let rivais = cpx1_d(o);
    if rivais != 0 {
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        core::ptr::write_unaligned((rivais + 0xa) as *mut usize, cpx1_d(rivais + 2));
        let mut e = proximo(cpx1_d(o));
        while e != 0 {
            let r = cpx1_d(cpx1_d(this).wrapping_add(cpx1_d(e).wrapping_mul(4)));
            if !cpx1_encerrada(r) {
                return;
            }
            e = proximo(cpx1_d(o));
        }
    }
    let m = (cpx1_d(this + 0x86e).wrapping_add(cpx1_d(o + 0xc).wrapping_mul(8))) as *mut u8;
    if *m & 1 != 0 {
        *m &= 0xfe;
    }
}

/// O começo da lista de observação do comprador (0x849ce0) e as ofertas de contrato (0x4cca70 e, pela vaga +0x20, a
/// 0x4ccb30 de cada jogador da troca, cuja marca +4 zera), com +0x10 zerado entre elas; e, com troca, vendedor e
/// vendedor do computador (0x600a50), a situação na lista do comprador de cada jogador da troca que estava nela
/// (0x842080, 0x843e10 evento 1).
unsafe fn cpx1_lista_do_comprador(this: usize, o: usize) {
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let e = entrada(cpx1_listas(this), cpx1_clube(cpx1_d(o + 8)), cpx1_pessoa(cpx1_d(o + 0xc)), (cpx1_b(o + 0x11) != 0xb) as u32, 0);
    if e != 0 {
        // ponytail: a original empurra o registrador do ponteiro da entrada com o setne no byte baixo; a 0x843e10 só lê
        // o byte
        let tipo = (e as u32 & !0xff) | (cpx1_b(e) != 0xb) as u32;
        cpx1_situacao_na_lista(this, cpx1_pessoa(cpx1_d(o + 0xc)), cpx1_clube(cpx1_d(o + 8)), 1, tipo);
    }
}

unsafe fn cpx1_ofertas_de_contrato(this: usize, o: usize, vendedor: usize, comprador: usize) {
    if !CPX1_ENSAIO {
        let tirar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cca70usize);
        tirar(cpx1_d(0xae1050), cpx1_pessoa(cpx1_d(o + 0xc)), cpx1_clube(cpx1_d(o + 8)));
    }
    *((o + 0x10) as *mut u8) = 0;
    let troca = cpx1_d(o + 4);
    if troca != 0 {
        let tirar_da_vaga: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4ccb30usize);
        for e in cpx1_itens(troca) {
            if !CPX1_ENSAIO {
                tirar_da_vaga(cpx1_d(0xae1050), cpx1_pessoa(cpx1_d(e)), cpx1_d(o + 0x20));
            }
            *((e + 4) as *mut u8) = 0;
        }
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if cpx1_d(o + 4) != 0 && vendedor != 0 && humano(vendedor, 0, 1, 0, 0) == 0 {
        let in_list: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x842080usize);
        for e in cpx1_itens(cpx1_d(o + 4)) {
            if in_list(cpx1_listas(this), cpx1_pessoa(cpx1_d(e)), comprador) != 0 {
                cpx1_situacao_na_lista(this, cpx1_pessoa(cpx1_d(e)), comprador, 1, 0);
            }
        }
    }
}

/// Desmarca os lances do comprador pelo jogador (0x84aeb0) e os do clube do contrato (`vendedor`, ou 0) por cada
/// jogador da troca.
unsafe fn cpx1_desmarcar_lances(this: usize, o: usize, vendedor: usize) {
    let p = cpx1_pessoa(cpx1_d(o + 0xc));
    cpx1_desmarcar(this, cpx1_clube(cpx1_d(o + 8)), p); // cpx1: desmarcar os lances do comprador
    let troca = cpx1_d(o + 4);
    if troca != 0 {
        for e in cpx1_itens(troca) {
            cpx1_desmarcar(this, vendedor, cpx1_pessoa(cpx1_d(e)));
        }
    }
}

/// O clube do contrato da pessoa (ou 0), com o contrato guardado em [0xae1080].
unsafe fn cpx1_clube_do_contrato(pessoa: usize) -> usize {
    let c = cpx1_contrato(pessoa);
    if c == 0 {
        0
    } else {
        cpx1_clube(cpx1_d(c + 4))
    }
}

/// A notícia da oferta recusada (0x8d5170: thiscall(gerência, oferta, motivo), ret 8); no ensaio, nada.
unsafe fn cpx1_noticia(this: usize, o: usize, motivo: u32) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8d5170usize);
    f(this, o, motivo);
}

/// TRANSFER_MANAGER::schedule_offer (0x8c6220: thiscall(gerência, oferta, dias, fatias), ret 0xc); no ensaio, nada.
unsafe fn cpx1_agendar(this: usize, o: usize, dias: u32) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    f(this, o, dias, 0);
}

/// { 0x8bf680 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::reject_transfer_offer(vaga, marcar,
/// motivo)`, pelo clube vendedor que recusa (0x8c36b0 e a avaliação dos lances). Com `marcar`, +0x2d da oferta = 1; o
/// jogador pode se desassossegar (0x8c1290); transferência futura (estado 0x13): sai da lista (0x8cbf70). Estado 2
/// (recusada); os lances do comprador pelo jogador e os do clube do contrato pelos da troca saem (0x84aeb0); as ofertas
/// de contrato ligadas fecham (0x8c6db0); a notícia (0x8d5170); a oferta volta em 25 dias (0x8c6220) para sair. Sem
/// rival em aberto, a marca de oferta da pessoa sai (+0x86e). A lista de observação do comprador (0x849ce0, 0x843e10),
/// as ofertas de contrato (0x4cca70, 0x4ccb30) e +0x10 = 0; com troca e vendedor do computador, os jogadores da troca
/// na lista do comprador (0x842080, 0x843e10).
unsafe extern "thiscall" fn reject_transfer_offer(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = *vaga;
    let vendedor = cpx1_clube_do_contrato(cpx1_pessoa(cpx1_d(o + 0xc)));
    let comprador = cpx1_clube(cpx1_d(o + 8));
    if marcar != 0 {
        *((*vaga + 0x2d) as *mut u8) = 1;
    }
    crate::comparacao::ramo("cpx_1 recusa desassossega");
    cpx1_com_a_oferta(0x8c1290, this, o); // cpx1: o desassossego da recusa
    let atual = *vaga;
    if cpx1_b(atual + 0x2c) == 0x13 {
        cpx1_com_a_oferta(0x8cbf70, this, atual);
    }
    *((o + 0x2c) as *mut u8) = 2;
    let vendedor2 = cpx1_clube_do_contrato(cpx1_pessoa(cpx1_d(o + 0xc)));
    crate::comparacao::ramo("cpx_1 recusa desmarca");
    cpx1_desmarcar_lances(this, o, vendedor2);
    cpx1_com_a_oferta(0x8c6db0, this, o);
    cpx1_noticia(this, o, motivo);
    cpx1_agendar(this, o, 0x19);
    cpx1_tirar_marca(this, o);
    cpx1_lista_do_comprador(this, o);
    cpx1_ofertas_de_contrato(this, o, vendedor, comprador);
}

/// { 0x8bfcb0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::withdraw_transfer_offer(vaga, marcar,
/// motivo)`, pelo comprador que desiste (a revisão dos lances e as ofertas desfeitas). Com `marcar`, +0x2d = 1;
/// transferência futura: sai da lista (0x8cbf70); estado de 0xb para cima: as ofertas de contrato guardadas voltam
/// (0x4cf4a0). Estado 3 (retirada); as ofertas de contrato ligadas fecham (0x8c6db0); os lances saem (0x84aeb0); +0x10 =
/// 4; a oferta volta em 10 dias (0x8c6220). Sem rival em aberto, a marca de oferta da pessoa sai. Motivo 2: o jogador
/// pode se desassossegar (0x8c1290). A lista de observação do comprador, a notícia (0x8d5170), as ofertas de contrato e
/// +0x10 = 0; com troca e vendedor do computador, os jogadores da troca na lista do comprador.
unsafe extern "thiscall" fn withdraw_transfer_offer(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = *vaga;
    let vendedor = cpx1_clube_do_contrato(cpx1_pessoa(cpx1_d(o + 0xc)));
    let comprador = cpx1_clube(cpx1_d(o + 8));
    if marcar != 0 {
        *((*vaga + 0x2d) as *mut u8) = 1;
    }
    let atual = *vaga;
    if cpx1_b(atual + 0x2c) == 0x13 {
        cpx1_com_a_oferta(0x8cbf70, this, atual);
    }
    if cpx1_b(o + 0x2c) as i8 >= 0xb && !CPX1_ENSAIO {
        let reenviar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4cf4a0usize);
        reenviar(cpx1_d(0xae1050), cpx1_d(o + 0xc));
    }
    *((o + 0x2c) as *mut u8) = 3;
    cpx1_com_a_oferta(0x8c6db0, this, o);
    let vendedor2 = cpx1_clube_do_contrato(cpx1_pessoa(cpx1_d(o + 0xc)));
    cpx1_desmarcar_lances(this, o, vendedor2);
    *((o + 0x10) as *mut u8) = 4;
    cpx1_agendar(this, o, 0xa);
    cpx1_tirar_marca(this, o);
    if motivo as u16 == 2 { // cpx1: só o motivo 2 desassossega
        cpx1_com_a_oferta(0x8c1290, this, o);
    } else {
        crate::comparacao::ramo("cpx_1 retirada sem desassossego");
    }
    cpx1_lista_do_comprador(this, o);
    cpx1_noticia(cpx1_d(0xde1f54), o, motivo);
    cpx1_ofertas_de_contrato(this, o, vendedor, comprador);
}

/// As vagas de uma LLIST (ou nenhuma), sem mexer no cursor.
unsafe fn cpx1_nos(l: usize) -> Vec<usize> {
    if l == 0 {
        return Vec::new();
    }
    let cursor = cpx1_d(l + 0xa);
    let v = cpx1_itens(l);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cursor);
    v
}

/// O comparador da recusa e da retirada: a cópia em ensaio (sem as chamadas que mexem no resto do jogo) e a original a
/// partir do mesmo estado (a oferta, as duas listas dela, as vagas da troca, o registro da pessoa em +0x86e e
/// [0xae1080]); iguais o estado (+0x2c), +0x10, a marca +4 de cada jogador da troca e a marca de oferta da pessoa, que
/// a original grava sem depender das chamadas.
unsafe fn cpx1_comparar(nome: &str, this: usize, vaga: *mut usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let o = *vaga;
    let marca = cpx1_d(this + 0x86e).wrapping_add(cpx1_d(o + 0xc).wrapping_mul(8));
    let nos = cpx1_nos(cpx1_d(o + 4));
    let mut regioes = vec![(o, 0x30usize), (marca, 8), (0xae1080, 4)];
    for l in [cpx1_d(o), cpx1_d(o + 4)] {
        if l != 0 {
            regioes.push((l, 0xe));
        }
    }
    for &e in &nos {
        regioes.push((e, 5));
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPX1_ENSAIO = true;
    crate::aleatorio::pausar_rastro(copia);
    CPX1_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    original();
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let (a, b) = (&mo.0, &mr.0);
    let mut igual = a[0].1[0x2c] == b[0].1[0x2c] && a[0].1[0x10] == b[0].1[0x10] && a[1].1[0] == b[1].1[0];
    for k in regioes.len() - nos.len()..regioes.len() {
        igual &= a[k].1[4] == b[k].1[4];
    }
    PLACAR.registrar(igual, || format!("{}(oferta {:#x}) original={:x?} rust={:x?}", nome, o, mo, mr));
}

unsafe extern "thiscall" fn comparar_reject_transfer_offer(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(%%I0%%);
    cpx1_comparar("reject_transfer_offer", this, vaga, || o(this, vaga, marcar, motivo), || reject_transfer_offer(this, vaga, marcar, motivo))
}

unsafe extern "thiscall" fn comparar_withdraw_transfer_offer(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(%%I1%%);
    cpx1_comparar("withdraw_transfer_offer", this, vaga, || o(this, vaga, marcar, motivo), || withdraw_transfer_offer(this, vaga, marcar, motivo))
}

/// { 0x8d5170 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_offer_rejected_news(oferta, motivo)`, pela recusa, pela retirada e pela recusa do empréstimo. Os clubes humanos
/// que recebem (0x8d1130, com 0x2d se a oferta estiver recusada, senão 0x2e; até 30); sem nenhum, nada. A notícia 0xfa1:
/// a pessoa (o id e as três chaves), o clube do contrato (sem contrato de tipo 5; nenhum no motivo 6 com +0x2f), o
/// comprador, a lista da troca (+4), o estado, o motivo, +0x1f, o valor total (0x8e7a70 com 0, 0, 0, 1) e a divisão do
/// clube do contrato (+0x57); a cada destinatário, na vaga 12, 1 se for o comprador ou o clube do contrato.
unsafe extern "thiscall" fn post_offer_rejected_news(this: usize, o: usize, motivo: u32) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x8e7a70usize);
    let p = cpx1_pessoa(cpx1_d(o + 0xc));
    let comprador = cpx1_clube(cpx1_d(o + 8));
    let tipo5 = |c: usize| cpx1_b(c + 0x35) & 0x3f == 5;
    let c = cpx1_contrato(p);
    let do_contrato = if c == 0 || tipo5(c) { 0 } else { cpx1_clube(cpx1_d(c + 4)) };
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, (cpx1_b(o + 0x2c) != 2) as u32 + 0x2d, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    let c = cpx1_contrato(p);
    let mut clube = if c == 0 || tipo5(c) { 0 } else { cpx1_clube(cpx1_d(c + 4)) };
    let c = cpx1_contrato(p);
    if c != 0 && tipo5(c) {
        clube = 0;
    }
    if motivo as i16 == 6 && cpx1_b(o + 0x2f) != 0 {
        clube = 0;
    }
    crate::comparacao::ramo("cpx_1 noticia da recusa");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfa1, 0);
    gravar(nt, 0, cpx1_d(p) as u32);
    gravar(nt, 1, cpx1_d(cpx1_d(p + 4) + 0x33) as u32);
    gravar(nt, 2, cpx1_d(cpx1_d(p + 8) + 0x33) as u32);
    gravar(nt, 3, cpx1_d(cpx1_d(p + 0xc) + 0x33) as u32);
    gravar(nt, 4, if clube != 0 { cpx1_d(clube) as u32 } else { u32::MAX });
    gravar(nt, 5, cpx1_d(o + 8) as u32);
    gravar(nt, 6, cpx1_d(o + 4) as u32);
    gravar(nt, 7, cpx1_b(o + 0x2c) as i8 as u32);
    gravar(nt, 8, motivo as i16 as u32);
    gravar(nt, 9, cpx1_b(o + 0x1f) as i8 as u32);
    gravar(nt, 10, total(o, 0, 0, 0, 1));
    let divisao = if clube != 0 { cpx1_d(clube + 0x57) } else { 0 };
    gravar(nt, 11, if divisao != 0 { cpx1_d(divisao) as u32 } else { u32::MAX });
    for &d in lista.iter().take(n.max(0) as usize) {
        gravar(nt, 12, (d == comprador || d == do_contrato) as u32);
        entregar(cpx1_d(0xb7431c), nt, d);
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_post_offer_rejected_news(this: usize, o: usize, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%)(this, o, motivo)
}
