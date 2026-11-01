// ---------- 258a leva: a oferta passada aos clubes, a notícia da oferta aceita e a do empréstimo oferecido (transfer_manager.cpp) ----------

unsafe fn cpv1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpv1_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpv1_clube(id: usize) -> usize {
    cpv1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpv1_pessoa(id: usize) -> usize {
    cpv1_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz depois de cada chamada.
unsafe fn cpv1_contrato(p: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpv1_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato (id em +4); com `filtro`, nenhum para o contrato de tipo 5 (+0x35 & 0x3f).
unsafe fn cpv1_clube_do_contrato(c: usize, filtro: bool) -> usize {
    if c != 0 && (!filtro || cpv1_b(c + 0x35) & 0x3f != 5) {
        cpv1_clube(cpv1_d(c + 4))
    } else {
        0
    }
}

/// Volta a lista ao começo (o cursor +0xa recebe a cabeça +2).
unsafe fn cpv1_inicio(l: usize) {
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpv1_d(l + 2));
}

/// O próximo item da lista (0x68d1b0), ou 0.
unsafe fn cpv1_proximo(l: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    proximo(l)
}

/// Os dados da pessoa numa notícia, a partir da vaga `k`: o id e as três chaves (+0x33 de +4, +8 e +0xc).
unsafe fn cpv1_dados_da_pessoa(n: usize, k: i8, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    gravar(n, k, cpv1_d(p) as u32);
    gravar(n, k + 1, cpv1_d(cpv1_d(p + 4) + 0x33) as u32);
    gravar(n, k + 2, cpv1_d(cpv1_d(p + 8) + 0x33) as u32);
    gravar(n, k + 3, cpv1_d(cpv1_d(p + 0xc) + 0x33) as u32);
}

/// { 0x8c2f50 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::advance_offer_to_club_approval(oferta)`:
/// comprador = o clube +8; vendedor = o clube do contrato do jogador (+0xc), nenhum no contrato de tipo 5 (0x4cf5a0
/// duas vezes, [0xae1080]). Fora do status 0xa (+0x2c): só com +0x10 = 2 e todos os jogadores da troca (+4) com +4 = 2.
/// As outras ofertas pelo mesmo jogador (lista +0, índices no vetor [this]; menos a +0x20) caem: status 0xf, +0x2e = 3,
/// reagendadas em 0x14 dias (0x8c6220), e as marcas de lance (0x84aeb0 no gerente de listas [this+0x856]) do comprador
/// e do dono de cada jogador da troca saem. Sem a marca +0x30 & 0xf, com permissão de trabalho a pedir (0x8c7470) e
/// +0x2e = 0: a notícia da espera (0x8e2a00), status 0xa e reagendada em 18 + sorteio(7) dias. Senão status 0xb, a
/// notícia da transferência (0x8da5e0) se um dos clubes é humano, e a decisão do comprador do computador (0x600a50):
/// com restrição (0x8cfc90) recusa (0x8c36b0, +0x2e < 3) ou retira (0x8bfcb0, que pode trocar a oferta); senão
/// confirma (0x8c3310). Depois o vendedor: com +0x2f ou sem vendedor confirma direto; do computador, recusa com
/// restrição ou confirma.
unsafe extern "thiscall" fn passar_a_oferta_aos_clubes(this: usize, oferta: usize) {
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    let limpar_marcas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    // thiscall (ecx = this, o gerente de transferências; corrigido em 31/10/2026: antes stdcall, o this ia com o ecx da vez e às vezes 0)
    let pede_permissao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8c7470usize);
    let noticia_da_espera: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8e2a00usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let noticia_da_transferencia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8da5e0usize);
    let clube_humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let restricoes: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, u32) -> u32 = core::mem::transmute(0x8cfc90usize);
    let recusar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x8c36b0usize);
    let retirar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(0x8bfcb0usize);
    let confirmar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8c3310usize);
    let comprador = cpv1_clube(cpv1_d(oferta + 8));
    let jogador = cpv1_pessoa(cpv1_d(oferta + 0xc));
    let mut vendedor = cpv1_clube_do_contrato(cpv1_contrato(jogador), true);
    let c = cpv1_contrato(jogador);
    if c != 0 && cpv1_b(c + 0x35) & 0x3f == 5 {
        vendedor = 0;
    }
    if cpv1_b(oferta + 0x2c) != 0xa {
        if cpv1_b(oferta + 0x10) != 2 {
            return;
        }
        let troca = cpv1_d(oferta + 4);
        if troca != 0 {
            cpv1_inicio(troca);
            loop {
                let x = cpv1_proximo(troca);
                if x == 0 {
                    break;
                }
                if cpv1_b(x + 4) != 2 {
                    return;
                }
            }
        }
        let rivais = cpv1_d(oferta);
        if rivais != 0 {
            cpv1_inicio(rivais);
            let mut x = cpv1_proximo(rivais);
            while x != 0 {
                let outra = cpv1_d(cpv1_d(this) + cpv1_d(x) * 4);
                if outra != 0 && cpv1_d(x) != cpv1_d(oferta + 0x20) {
                    crate::comparacao::ramo("cpv_1 ofertas rivais canceladas");
                    cpv1_pb(outra + 0x2c, 0xf);
                    cpv1_pb(cpv1_d(cpv1_d(this) + cpv1_d(x) * 4) + 0x2e, 3);
                    agendar(this, cpv1_d(cpv1_d(this) + cpv1_d(x) * 4), 0x14, 0); // cpv1: as rivais em 20 dias
                    let o = cpv1_d(cpv1_d(this) + cpv1_d(x) * 4);
                    let p = cpv1_pessoa(cpv1_d(o + 0xc));
                    let dono = cpv1_clube_do_contrato(cpv1_contrato(p), false);
                    limpar_marcas(cpv1_d(this + 0x856), cpv1_clube(cpv1_d(o + 8)), p);
                    let l = cpv1_d(o + 4);
                    if l != 0 {
                        cpv1_inicio(l);
                        loop {
                            let y = cpv1_proximo(l);
                            if y == 0 {
                                break;
                            }
                            limpar_marcas(cpv1_d(this + 0x856), dono, cpv1_pessoa(cpv1_d(y)));
                        }
                    }
                }
                x = cpv1_proximo(rivais);
            }
        }
        if cpv1_b(oferta + 0x30) & 0xf == 0 && pede_permissao(this, oferta) != 0 && cpv1_b(oferta + 0x2e) == 0 {
            noticia_da_espera(this, oferta);
            cpv1_pb(oferta + 0x2c, 0xa);
            let r = sorteio(7);
            crate::comparacao::ramo("cpv_1 permissao de trabalho");
            // a original soma 0x12 só no byte baixo do eax do sorteio (0..6: em cima, zero)
            let dias = (r as u32 & !0xff) | (r as u8).wrapping_add(0x12) as u32; // cpv1: a espera da permissao
            agendar(this, oferta, dias, 0);
            return;
        }
    }
    cpv1_pb(oferta + 0x2c, 0xb);
    if humano(cpv1_d(0xb7431c), comprador) != 0 || (vendedor != 0 && humano(cpv1_d(0xb7431c), vendedor) != 0) {
        noticia_da_transferencia(this, oferta);
    }
    let mut vaga = oferta;
    // ponytail: a original passa um short* para um dword local sem valor e empurra o dword inteiro; os chamados leem a
    // palavra. A cópia começa em zero.
    let mut motivo: u32 = 0;
    if clube_humano(comprador, 0, 1, 0, 0) == 0 {
        if restricoes(this, oferta, comprador, &mut motivo, 1) as u8 == 1 {
            if cpv1_b(oferta + 0x2e) < 3 {
                recusar(this, oferta, comprador, motivo);
            } else {
                retirar(this, &mut vaga, 0, motivo);
            }
        } else {
            confirmar(this, oferta, comprador);
        }
    }
    // relida da vaga, como a original ([esp+0x20], que a 0x8bfcb0 pode trocar)
    let o = vaga;
    if cpv1_b(o + 0x2f) != 0 || vendedor == 0 {
        return confirmar(this, o, vendedor);
    }
    if clube_humano(vendedor, 0, 1, 0, 0) != 0 {
        return;
    }
    if restricoes(this, vaga, vendedor, &mut motivo, 1) as u8 == 1 {
        return recusar(this, vaga, vendedor, 0);
    }
    crate::comparacao::ramo("cpv_1 confirmada pelo vendedor");
    confirmar(this, vaga, vendedor); // cpv1: o vendedor confirma
}

/// { 0x8d76d0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor): `TRANSFER_MANAGER::
/// post_offer_accepted_news(oferta)`: o dono do jogador (+0xc; contrato 0x4cf5a0, nenhum no tipo 5), os destinatários
/// (0x8d1130 com 0x2d, até 30), a notícia 0xfa3 (local): 0..3 o jogador, 4 = o id do dono (ou −1), 5 = +8 (comprador),
/// 6 = +4, 7 = +0x20, 8 = +0x24, 9 = +0x1f; entregue a cada destinatário (0x7861e0).
unsafe extern "thiscall" fn noticia_da_oferta_aceita(this: usize, oferta: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u32 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparacao::ramo("cpv_1 noticia da oferta aceita");
    let p = cpv1_pessoa(cpv1_d(oferta + 0xc));
    let dono = cpv1_clube_do_contrato(cpv1_contrato(p), true);
    let mut clubes = [0usize; 30];
    let q = destinatarios(this, oferta, 0x2d, clubes.as_mut_ptr(), 0x1e) as i8;
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xfa3, 0);
    cpv1_dados_da_pessoa(n, 0, p);
    gravar(n, 4, if dono != 0 { cpv1_d(dono) as u32 } else { u32::MAX });
    gravar(n, 5, cpv1_d(oferta + 8) as u32);
    gravar(n, 6, cpv1_d(oferta + 4) as u32);
    gravar(n, 7, cpv1_d(oferta + 0x20) as u32);
    gravar(n, 8, cpv1_d(oferta + 0x24) as u32);
    gravar(n, 9, cpv1_b(oferta + 0x1f) as i8 as i32 as u32);
    for &c in clubes.iter().take(if q > 0 { q as usize } else { 0 }) {
        entregar(cpv1_d(0xb7431c), n, c);
    }
}

/// { 0x8e1780 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor): `TRANSFER_MANAGER::
/// post_loan_offer_news(oferta, termos)`: os destinatários (0x8d1130 com 0x2e, até 30); sem nenhum, nada. A notícia
/// 0xfb5 (local): 0..3 o jogador (+0xc), 4 = +8, 5 = o id do clube do contrato (nenhum no tipo 5: a original lê o
/// endereço 0), 6 = +0x20, 7 = +0x24, 8 = termos +0xc, 9 = +0x2c, 10 = termos +0x2d (palavra com sinal), 11 = termos
/// +0x2f (palavra); sem contrato, a caixa "Error" (0x5e4c) e nada; senão 12 = contrato +0xc e a entrega a cada um.
unsafe extern "thiscall" fn noticia_do_emprestimo_oferecido(this: usize, oferta: usize, termos: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u32 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let p = cpv1_pessoa(cpv1_d(oferta + 0xc));
    let mut clubes = [0usize; 30];
    let q = destinatarios(this, oferta, 0x2e, clubes.as_mut_ptr(), 0x1e) as i8;
    if q == 0 {
        return;
    }
    crate::comparacao::ramo("cpv_1 noticia do emprestimo");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xfb5, 0);
    cpv1_dados_da_pessoa(n, 0, p);
    gravar(n, 4, cpv1_d(oferta + 8) as u32);
    // ponytail: sem clube (contrato de tipo 5) a original lê o endereço 0 e cai; a cópia faz o mesmo.
    let clube = cpv1_clube_do_contrato(cpv1_contrato(p), true);
    gravar(n, 5, cpv1_d(clube) as u32);
    gravar(n, 6, cpv1_d(oferta + 0x20) as u32);
    gravar(n, 7, cpv1_d(oferta + 0x24) as u32);
    gravar(n, 8, cpv1_d(termos + 0xc) as u32);
    gravar(n, 9, cpv1_b(oferta + 0x2c) as i8 as i32 as u32);
    gravar(n, 10, core::ptr::read_unaligned((termos + 0x2d) as *const i16) as i32 as u32);
    gravar(n, 11, core::ptr::read_unaligned((termos + 0x2f) as *const u16) as u32);
    let c = cpv1_contrato(p);
    if c == 0 {
        return aviso_em(0xa8d514, 0x5e4c);
    }
    gravar(n, 12, cpv1_d(c + 0xc) as u32);
    for &d in clubes.iter().take(if q > 0 { q as usize } else { 0 }) {
        entregar(cpv1_d(0xb7431c), n, d);
    }
}

// ponytail: reagenda, cancela rivais, sorteia e decide pelos clubes (as chamadas mexem nas ofertas e alocam): não roda
// duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e pelas mutações.
unsafe extern "thiscall" fn comparar_passar_a_oferta_aos_clubes(this: usize, oferta: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1233)(this, oferta)
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_da_oferta_aceita(this: usize, oferta: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1234)(this, oferta)
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_do_emprestimo_oferecido(this: usize, oferta: usize, termos: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1235)(this, oferta, termos)
}

// ---------- 258a leva: a recusa dos termos pelo jogador, a notícia da confiança da diretoria e a das expectativas da diretoria (transfer_manager.cpp, manager_manager.cpp) ----------

unsafe fn cpv2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpv2_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cpv2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpv2_clube(id: usize) -> usize {
    cpv2_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpv2_pessoa(id: usize) -> usize {
    cpv2_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz depois de cada chamada.
unsafe fn cpv2_contrato(p: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpv2_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato (id em +4); com `filtro`, nenhum para o contrato de tipo 5 (+0x35 & 0x3f).
unsafe fn cpv2_clube_do_contrato(c: usize, filtro: bool) -> usize {
    if c != 0 && (!filtro || cpv2_b(c + 0x35) & 0x3f != 5) {
        cpv2_clube(cpv2_d(c + 4))
    } else {
        0
    }
}

/// Volta a lista ao começo (o cursor +0xa recebe a cabeça +2).
unsafe fn cpv2_inicio(l: usize) {
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpv2_d(l + 2));
}

/// O próximo item da lista (0x68d1b0), ou 0.
unsafe fn cpv2_proximo(l: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    proximo(l)
}

/// A oferta já resolvida pelo status (+0x2c): 2, 3, 4, 0x10 ou 0x14, ou 0xf com +0x2e ≥ 3.
unsafe fn cpv2_resolvida(o: usize) -> bool {
    match cpv2_b(o + 0x2c) {
        2 | 3 | 4 | 0x10 | 0x14 => true,
        0xf => cpv2_b(o + 0x2e) >= 3,
        _ => false,
    }
}

/// { 0x8c28c0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::handle_player_rejected_terms(pessoa,
/// índice, silencioso)`: índice fora do vetor [this] (conta +4): a caixa "Error" (0xf49, 0xf4e). Vaga vazia: nada.
/// Se a pessoa é o jogador da oferta (+0xc): termos recusados (+0x10 = 4), status 0xf; +0x2e sobe com comprador humano
/// (0x600a50 sem as férias) e +0x1f ≠ 1, senão 3; saem as marcas de lance (0x84aeb0) do comprador e do dono de cada
/// jogador da troca; o gerente de listas anota a recusa (0x843e10) se o comprador é do computador; reagendada em 10
/// dias (0x8c6220); resolvida com todas as rivais resolvidas, o bit 0 do registro de 8 bytes do jogador ([this+0x86e])
/// sai; saem as propostas de contrato (0x4cca70) dos jogadores da troca com o vendedor. Sem `silencioso`: com comprador
/// mais famoso que o vendedor (+0x80), a reputação do jogador (+0xb da ficha +0x61) sobe a diferença × 0,075 (acima de
/// 7750 com ficha abaixo de 4750), × 0,05 (acima de 4250) ou × 0,025, até 10000; a notícia da resposta (0x8d8e90).
/// Se não, a pessoa é um jogador da troca (+4; lista vazia: a caixa 0x101f): sai da lista (0x68d1f0; a lista vazia é
/// liberada), o valor dela (0x4d0c60) passa ao preço (+0x1b), o registro de 8 bytes dela perde uma oferta (+2), a
/// notícia (sem `silencioso`), e a reação do vendedor do computador (0x8c8090) e do comprador (0x8c0810), parando se a
/// oferta ficou resolvida. No fim, a oferta segue (0x8c2f50).
unsafe extern "thiscall" fn recusa_dos_termos(this: usize, pessoa: usize, indice: i32, silencioso: u32) {
    let clube_humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let limpar_marcas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    let anotar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x843e10usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    let tirar_proposta: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cca70usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, usize, u32, u32, u32) = core::mem::transmute(0x8d8e90usize);
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let resposta_do_vendedor: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c8090usize);
    let resposta_do_comprador: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c0810usize);
    let seguir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c2f50usize);
    if indice < 0 {
        return aviso_em(0xa8d514, 0xf49);
    }
    if indice >= cpv2_d(this + 4) as i32 {
        return aviso_em(0xa8d514, 0xf4e);
    }
    let vaga = || cpv2_d(this).wrapping_add(indice as usize * 4);
    let o = cpv2_d(vaga());
    if o == 0 {
        return;
    }
    let preco = cpv2_d(o + 0x1b);
    if cpv2_d(pessoa) == cpv2_d(o + 0xc) {
        cpv2_pb(o + 0x10, 4);
        let comprador = cpv2_clube(cpv2_d(o + 8));
        let vendedor = cpv2_clube_do_contrato(cpv2_contrato(pessoa), true);
        cpv2_pb(o + 0x2c, 0xf);
        if silencioso == 0 && clube_humano(comprador, 1, 1, 0, 0) != 0 && cpv2_b(o + 0x1f) != 1 {
            cpv2_pb(o + 0x2e, cpv2_b(o + 0x2e).wrapping_add(1));
        } else {
            cpv2_pb(o + 0x2e, 3);
        }
        let p = cpv2_pessoa(cpv2_d(o + 0xc));
        let dono = cpv2_clube_do_contrato(cpv2_contrato(p), false);
        limpar_marcas(cpv2_d(this + 0x856), cpv2_clube(cpv2_d(o + 8)), p);
        let l = cpv2_d(o + 4);
        if l != 0 {
            cpv2_inicio(l);
            loop {
                let y = cpv2_proximo(l);
                if y == 0 {
                    break;
                }
                limpar_marcas(cpv2_d(this + 0x856), dono, cpv2_pessoa(cpv2_d(y)));
            }
        }
        if clube_humano(comprador, 0, 1, 0, 0) == 0 {
            crate::comparacao::ramo("cpv_2 recusa anotada na lista");
            // a original testa o byte com o eax zerado pela pergunta de antes (em cima, zero)
            anotar(cpv2_d(this + 0x856), pessoa, comprador, 1, (cpv2_b(o + 0x11) != 0xb) as u32); // cpv2: a recusa na lista
        }
        crate::comparacao::ramo("cpv_2 recusa do jogador");
        agendar(this, o, 0xa, 0); // cpv2: de volta em 10 dias
        if cpv2_resolvida(o) {
            let rivais = cpv2_d(o);
            let mut todas = true;
            if rivais != 0 {
                cpv2_inicio(rivais);
                loop {
                    let x = cpv2_proximo(rivais);
                    if x == 0 {
                        break;
                    }
                    if !cpv2_resolvida(cpv2_d(cpv2_d(this) + cpv2_d(x) * 4)) {
                        todas = false;
                        break;
                    }
                }
            }
            if todas {
                let m = cpv2_d(this + 0x86e).wrapping_add(cpv2_d(o + 0xc) * 8);
                if cpv2_b(m) & 1 != 0 {
                    cpv2_pb(m, cpv2_b(m) & 0xfe);
                }
            }
        }
        let l = cpv2_d(o + 4);
        if l != 0 {
            cpv2_inicio(l);
            loop {
                let y = cpv2_proximo(l);
                if y == 0 {
                    break;
                }
                tirar_proposta(cpv2_d(0xae1050), cpv2_pessoa(cpv2_d(y)), vendedor);
            }
        }
        if silencioso == 0 {
            let ficha = cpv2_d(pessoa + 0x61);
            if comprador != 0 && vendedor != 0 && ficha != 0 {
                let rc = cpv2_w(comprador + 0x80);
                let rv = cpv2_w(vendedor + 0x80);
                if rc > rv {
                    let dif = rc.wrapping_sub(rv) as f64;
                    let k = if rc > 0x1e46 && cpv2_w(ficha + 0xb) < 0x128e {
                        core::ptr::read_unaligned(0x96a730 as *const f64)
                    } else if rc > 0x109a {
                        core::ptr::read_unaligned(0x967888 as *const f64)
                    } else {
                        core::ptr::read_unaligned(0x967878 as *const f64)
                    };
                    crate::comparacao::ramo("cpv_2 reputacao do jogador");
                    let soma = ftol(dif * k) as i16; // cpv2: a reputação sobe
                    let r = (ficha + 0xb) as *mut i16;
                    core::ptr::write_unaligned(r, core::ptr::read_unaligned(r).wrapping_add(soma));
                    let f2 = cpv2_d(pessoa + 0x61);
                    if cpv2_w(f2 + 0xb) > 0x2710 {
                        core::ptr::write_unaligned((f2 + 0xb) as *mut i16, 0x2710);
                    }
                }
            }
            noticia(this, pessoa, comprador, vendedor, 4, o, 0, 0, preco as u32);
        }
        return seguir(this, o);
    }
    let troca = cpv2_d(o + 4);
    if troca == 0 {
        aviso_em(0xa8d514, 0x101f);
        return seguir(this, o);
    }
    cpv2_inicio(troca);
    let mut x = cpv2_proximo(troca);
    while x != 0 && cpv2_d(x) != cpv2_d(pessoa) {
        x = cpv2_proximo(troca);
    }
    if x == 0 {
        return seguir(this, o);
    }
    crate::comparacao::ramo("cpv_2 jogador saiu da troca");
    let comprador = cpv2_clube(cpv2_d(o + 8));
    let novo_preco = valor(cpv2_d(0xae1050), pessoa, comprador, 0, -1, 0).wrapping_add(cpv2_d(o + 0x1b) as i32);
    let vendedor = cpv2_clube_do_contrato(cpv2_contrato(cpv2_pessoa(cpv2_d(o + 0xc))), false);
    let comprador = cpv2_clube(cpv2_d(o + 8));
    tirar(cpv2_d(o + 4), x);
    let l = cpv2_d(o + 4);
    if core::ptr::read_unaligned(l as *const u16) == 0 {
        if l != 0 {
            destruir(l);
            liberar(l);
        }
        core::ptr::write_unaligned((o + 4) as *mut usize, 0);
    }
    let m = cpv2_d(this + 0x86e).wrapping_add(cpv2_d(pessoa) * 8 + 2);
    cpv2_pb(m, cpv2_b(m).wrapping_sub(1));
    if silencioso == 0 {
        noticia(this, pessoa, vendedor, comprador, 4, o, 0, 0, novo_preco as u32);
    }
    core::ptr::write_unaligned((o + 0x1b) as *mut i32, novo_preco);
    if vendedor != 0 && clube_humano(vendedor, 0, 1, 0, 0) == 0 {
        resposta_do_vendedor(this, vaga(), 1);
        if cpv2_resolvida(o) {
            return;
        }
    }
    if clube_humano(comprador, 0, 1, 0, 0) != 0 {
        return seguir(this, o);
    }
    resposta_do_comprador(cpv2_d(0xde1f54), vaga(), 1);
    if cpv2_resolvida(o) {
        return;
    }
    seguir(this, o);
}

/// Antes da data das notícias ([0xdd6ec8]: dia +0 com sinal, ano +2) em relação a hoje ([0xae2c90])?
unsafe fn cpv2_antes_das_noticias() -> bool {
    let dia = *(0xdd6ec8 as *const i16);
    let ano = *(0xdd6eca as *const u16);
    let hoje_ano = *(0xae2c92 as *const u16);
    ano > hoje_ano || (ano == hoje_ano && dia > *(0xae2c90 as *const i16))
}

/// { 0x6a4c40 } (thiscall, ret 4, manager_manager.cpp, quadro SEH sem destrutor): `MANAGER_MANAGER::
/// post_board_confidence_update_news(clube)`: antes da data das notícias, nada. A notícia 0x1772 (local): 0 = o id do
/// clube, 1 = os dias do técnico (+0xcf) no cargo (desde a data +0x3e, 365 dias por ano) ou 100 sem técnico; pelo
/// alcance (0x786090) com o clube.
unsafe extern "thiscall" fn noticia_da_confianca_da_diretoria(_this: usize, clube: usize) {
    if cpv2_antes_das_noticias() {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let por_alcance: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x786090usize);
    let mut local = [0u32; 0x3a]; // a notícia local (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1772, 0);
    gravar(n, 0, cpv2_d(clube) as u32);
    let tecnico = cpv2_d(clube + 0xcf);
    let dias = if tecnico != 0 {
        let anos = (*(0xae2c92 as *const u16) as i32).wrapping_sub(core::ptr::read_unaligned((tecnico + 0x40) as *const u16) as i32);
        anos.wrapping_mul(365).wrapping_sub(cpv2_w(tecnico + 0x3e) as i32).wrapping_add(*(0xae2c90 as *const i16) as i32)
    } else {
        100
    };
    gravar(n, 1, dias as u32);
    por_alcance(cpv2_d(0xb7431c), n, clube, 0, 0, 0, 0);
}

/// { 0x6a4a20 } (thiscall, ret 8, manager_manager.cpp, quadro SEH sem destrutor): `MANAGER_MANAGER::
/// post_board_expectations_news(clube, expectativas)`: antes da data das notícias, nada. A notícia 0x1771 (local): 0 =
/// o id do clube, 1 = as expectativas (palavra com sinal); ao técnico do clube (0x7861e0).
unsafe extern "thiscall" fn noticia_das_expectativas_da_diretoria(_this: usize, clube: usize, expectativas: u32) {
    if cpv2_antes_das_noticias() {
        return;
    }
    crate::comparacao::ramo("cpv_2 noticia das expectativas");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x3a]; // a notícia local (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1771, 0);
    gravar(n, 0, cpv2_d(clube) as u32);
    gravar(n, 1, expectativas as i16 as i32 as u32);
    entregar(cpv2_d(0xb7431c), n, clube);
}

// ponytail: reagenda, mexe nas listas e na reputação, pode liberar a lista da troca e chama a resposta dos clubes e a
// oferta seguinte (0x8c2f50): não roda duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e
// pelas mutações.
unsafe extern "thiscall" fn comparar_recusa_dos_termos(this: usize, pessoa: usize, indice: i32, silencioso: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, i32, u32)>(1236)(this, pessoa, indice, silencioso)
}

// ponytail: a notícia vai pelo alcance (aloca): só a original. Quente (40 mil chamadas): sem contador.
unsafe extern "thiscall" fn comparar_noticia_da_confianca_da_diretoria(this: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1237)(this, clube)
}

// ponytail: a notícia vai à caixa do técnico (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_das_expectativas_da_diretoria(this: usize, clube: usize, expectativas: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1238)(this, clube, expectativas)
}

// ---------- 258a leva: as ofertas em espera no fechamento da janela, a notícia das expectativas da federação e a do protesto da torcida (transfer_manager.cpp, manager_manager.cpp) ----------

unsafe fn cpv3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpv3_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpv3_clube(id: usize) -> usize {
    cpv3_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpv3_pessoa(id: usize) -> usize {
    cpv3_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz depois de cada chamada.
unsafe fn cpv3_contrato(p: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpv3_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato (id em +4); com `filtro`, nenhum para o contrato de tipo 5 (+0x35 & 0x3f).
unsafe fn cpv3_clube_do_contrato(c: usize, filtro: bool) -> usize {
    if c != 0 && (!filtro || cpv3_b(c + 0x35) & 0x3f != 5) {
        cpv3_clube(cpv3_d(c + 4))
    } else {
        0
    }
}

/// A oferta já resolvida pelo status (+0x2c): 2, 3, 4, 0x10 ou 0x14, ou 0xf com +0x2e ≥ 3.
unsafe fn cpv3_resolvida(o: usize) -> bool {
    match cpv3_b(o + 0x2c) {
        2 | 3 | 4 | 0x10 | 0x14 => true,
        0xf => cpv3_b(o + 0x2e) >= 3,
        _ => false,
    }
}

/// Põe a oferta em espera: status 1, a marca +0x30 (bits 1..3 limpos, bit 0 ligado), sai a proposta de contrato do
/// jogador com o comprador (0x4cca70), termos zerados (+0x10) e, para cada jogador da troca (+4), sai a proposta pela
/// vaga +0x20 (0x4ccb30) e o estado dele (+4) zera.
unsafe fn cpv3_em_espera(o: usize) {
    let tirar_proposta: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cca70usize);
    let tirar_da_troca: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4ccb30usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let marca = (cpv3_b(o + 0x30) & 0xf1) | 1;
    cpv3_pb(o + 0x2c, 1);
    cpv3_pb(o + 0x30, marca);
    tirar_proposta(cpv3_d(0xae1050), cpv3_pessoa(cpv3_d(o + 0xc)), cpv3_clube(cpv3_d(o + 8)));
    cpv3_pb(o + 0x10, 0);
    let l = cpv3_d(o + 4);
    if l != 0 {
        core::ptr::write_unaligned((l + 0xa) as *mut usize, cpv3_d(l + 2));
        loop {
            let y = proximo(cpv3_d(o + 4));
            if y == 0 {
                break;
            }
            tirar_da_troca(cpv3_d(0xae1050), cpv3_pessoa(cpv3_d(y)), cpv3_d(o + 0x20));
            cpv3_pb(y + 4, 0);
        }
    }
}

/// { 0x8c4e10 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::hold_offers_at_window_close(nação)`: as
/// regras da nação (das 28 em [this+0x8ac], a de +0xa igual à nação +0x53 da seleção 0x53b3d0; senão nenhuma). Para
/// cada oferta do vetor [this] (conta +4): status 0xe completa (0x8c34e0 no prazo). Comprador da nação: não resolvida,
/// jogador com ficha (+0x61), sem a marca +0x30 & 0xf, fora do status 0x13 e aceita pelas regras (virtual +0xc com o
/// comprador e hoje; resposta 0 = segura): em espera; a notícia da espera (0x8e1150) se o comprador ou o vendedor é
/// humano, senão retirada (0x8bfcb0); comprador do computador (0x600a50): status 3 e reagendada em 1 dia. Senão, com
/// jogadores na troca e vendedor da nação, pelos mesmos testes com o vendedor nas regras: em espera; a notícia, ou
/// recusada (0x8bf680); e o mesmo reagendamento.
unsafe extern "thiscall" fn ofertas_em_espera_na_janela(this: usize, nacao: usize) {
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let completar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c34e0usize);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let noticia_da_espera: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8e1150usize);
    let retirar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8bfcb0usize);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8bf680usize);
    let clube_humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    let da_nacao = cpv3_d(selecao(nacao) + 0x53);
    let mut regras = 0usize;
    for k in 0..0x1c {
        let r = cpv3_d(this + 0x8ac + 4 * k);
        if cpv3_d(r + 0xa) == da_nacao {
            regras = r;
            break;
        }
    }
    let aceita = |o: usize, clube: usize| -> bool {
        if regras == 0 {
            return true;
        }
        let segurar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(cpv3_d(cpv3_d(regras) + 0xc));
        segurar(regras, o, clube, 0xae2c90, 0) == 0
    };
    let mut i = 0usize;
    while (i as i32) < cpv3_d(this + 4) as i32 {
        let vaga = cpv3_d(this).wrapping_add(i * 4);
        let o = cpv3_d(vaga);
        i += 1;
        if o == 0 {
            continue;
        }
        let st = cpv3_b(o + 0x2c);
        if st == 0xe {
            completar(this, o, 1);
            continue;
        }
        let comprador = cpv3_clube(cpv3_d(o + 8));
        if cpv3_d(comprador + 0x53) == nacao
            && !cpv3_resolvida(o)
            && cpv3_d(cpv3_pessoa(cpv3_d(o + 0xc)) + 0x61) != 0
            && cpv3_b(o + 0x30) & 0xf == 0
            && st != 0x13
            && aceita(o, comprador)
        {
            cpv3_em_espera(o);
            let humano_vende = || {
                let c = cpv3_contrato(cpv3_pessoa(cpv3_d(o + 0xc)));
                if c == 0 || cpv3_b(c + 0x35) & 0x3f == 5 || cpv3_clube(cpv3_d(c + 4)) == 0 {
                    return false;
                }
                let v = cpv3_clube_do_contrato(cpv3_contrato(cpv3_pessoa(cpv3_d(o + 0xc))), true);
                humano(cpv3_d(0xb7431c), v) != 0
            };
            if humano(cpv3_d(0xb7431c), cpv3_clube(cpv3_d(o + 8))) != 0 || humano_vende() {
                noticia_da_espera(this, o, nacao);
            } else {
                crate::comparacao::ramo("cpv_3 retirada na janela");
                // a vaga relida de [this] como na original
                retirar(this, cpv3_d(this).wrapping_add((i - 1) * 4), 0, 0); // cpv3: o comprador retira
            }
            if clube_humano(cpv3_clube(cpv3_d(o + 8)), 0, 1, 0, 0) == 0 {
                crate::comparacao::ramo("cpv_3 reagendada na janela");
                cpv3_pb(o + 0x2c, 3); // cpv3: status 3
                agendar(this, o, 1, 0); // cpv3: de volta amanhã
            }
            continue;
        }
        // os outros: oferta com troca cujo vendedor é da nação
        if cpv3_d(o + 4) == 0 {
            continue;
        }
        let c = cpv3_contrato(cpv3_pessoa(cpv3_d(o + 0xc)));
        if c == 0 || cpv3_clube(cpv3_d(c + 4)) == 0 {
            continue;
        }
        let vendedor = cpv3_clube_do_contrato(cpv3_contrato(cpv3_pessoa(cpv3_d(o + 0xc))), false);
        if cpv3_d(vendedor + 0x53) != nacao || cpv3_resolvida(o) || cpv3_b(o + 0x2c) == 0x13 {
            continue;
        }
        if regras != 0 {
            let dono = cpv3_clube_do_contrato(cpv3_contrato(cpv3_pessoa(cpv3_d(o + 0xc))), false);
            if !aceita(o, dono) {
                continue;
            }
        }
        cpv3_em_espera(o);
        let humano_vende = || {
            let c = cpv3_contrato(cpv3_pessoa(cpv3_d(o + 0xc)));
            if c == 0 || cpv3_clube(cpv3_d(c + 4)) == 0 {
                return false;
            }
            let v = cpv3_clube_do_contrato(cpv3_contrato(cpv3_pessoa(cpv3_d(o + 0xc))), false);
            humano(cpv3_d(0xb7431c), v) != 0
        };
        if humano(cpv3_d(0xb7431c), cpv3_clube(cpv3_d(o + 8))) != 0 || humano_vende() {
            noticia_da_espera(this, o, nacao);
        } else {
            crate::comparacao::ramo("cpv_3 recusada na janela");
            recusar(this, cpv3_d(this).wrapping_add((i - 1) * 4), 0, 0); // cpv3: o vendedor recusa
        }
        if clube_humano(cpv3_clube(cpv3_d(o + 8)), 0, 1, 0, 0) == 0 {
            cpv3_pb(o + 0x2c, 3);
            agendar(this, o, 1, 0);
        }
    }
}

/// { 0x6a44a0 } (thiscall, ret 0xc, manager_manager.cpp, quadro SEH sem destrutor): `MANAGER_MANAGER::
/// post_fa_expectations_news(seleção, expectativa, competição)`: a notícia 0x177e (local): 0 = o id da seleção, 1 = a
/// expectativa (palavra com sinal), 2 = a competição; pelo alcance (0x786090) com a seleção.
unsafe extern "thiscall" fn noticia_das_expectativas_da_federacao(_this: usize, selecao: usize, expectativa: u32, competicao: u32) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let por_alcance: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x786090usize);
    crate::comparacao::ramo("cpv_3 noticia da federacao");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x177e, 0);
    gravar(n, 0, cpv3_d(selecao) as u32);
    gravar(n, 1, expectativa as i16 as i32 as u32);
    gravar(n, 2, competicao);
    por_alcance(cpv3_d(0xb7431c), n, selecao, 0, 0, 0, 0);
}

/// { 0x6a5d10 } (thiscall, ret 4, manager_manager.cpp, quadro SEH sem destrutor): `MANAGER_MANAGER::
/// post_supporter_protest_news(clube)`: antes da data das notícias ([0xdd6ec8]), nada. A notícia 0x1782 (local): 0 = o
/// id do clube, 1..4 o técnico (+0xcf: o id e as três chaves +0x33 de +4, +8, +0xc); pelo alcance (0x786090).
unsafe extern "thiscall" fn noticia_do_protesto_da_torcida(_this: usize, clube: usize) {
    let dia = *(0xdd6ec8 as *const i16);
    let ano = *(0xdd6eca as *const u16);
    let hoje_ano = *(0xae2c92 as *const u16);
    if ano > hoje_ano || (ano == hoje_ano && dia > *(0xae2c90 as *const i16)) {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let por_alcance: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x786090usize);
    crate::comparacao::ramo("cpv_3 noticia do protesto");
    let mut local = [0u32; 0x3a]; // a notícia local (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1782, 0);
    gravar(n, 0, cpv3_d(clube) as u32);
    gravar(n, 1, cpv3_d(cpv3_d(clube + 0xcf)) as u32);
    gravar(n, 2, cpv3_d(cpv3_d(cpv3_d(clube + 0xcf) + 4) + 0x33) as u32);
    gravar(n, 3, cpv3_d(cpv3_d(cpv3_d(clube + 0xcf) + 8) + 0x33) as u32);
    gravar(n, 4, cpv3_d(cpv3_d(cpv3_d(clube + 0xcf) + 0xc) + 0x33) as u32);
    por_alcance(cpv3_d(0xb7431c), n, clube, 0, 0, 0, 0);
}

// ponytail: completa, retira, recusa e reagenda ofertas (as chamadas liberam e alocam): não roda duas vezes. Só a
// original; a cópia leva os ramos e se prova pela carreira e pelas mutações.
unsafe extern "thiscall" fn comparar_ofertas_em_espera_na_janela(this: usize, nacao: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1239)(this, nacao)
}

// ponytail: a notícia vai pelo alcance (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_das_expectativas_da_federacao(this: usize, selecao: usize, expectativa: u32, competicao: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(1240)(this, selecao, expectativa, competicao)
}

// ponytail: a notícia vai pelo alcance (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_do_protesto_da_torcida(this: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1241)(this, clube)
}

// ---------- 258a leva: a negociação guardada da oferta, o boato do técnico em outro clube e a recusa do jogador em se mudar (transfer_manager.cpp, manager_manager.cpp) ----------

unsafe fn cpv4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpv4_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpv4_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpv4_clube(id: usize) -> usize {
    cpv4_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpv4_pessoa(id: usize) -> usize {
    cpv4_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// { 0x8be9d0 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH só dos `new`): `TRANSFER_MANAGER::
/// add_transfer_negotiation(oferta, termos, marcar)`: com `marcar`, +0x2d = 1 nas duas e o jogador marcado sob oferta
/// (0x84b580, com a lista +0). Sem vaga (+0x28 = −1): o vetor das negociações (+0xc, capacidade +0x10, conta +0x14)
/// cresce de 50 quando cheio (0x945667 zerando as novas; 0x945501 sem vetor), a primeira vaga vazia vai para +0x28 e a
/// oferta para o status 1 (sem vaga vazia: a caixa "Error" 0x60f e nada). A negociação antiga da vaga é destruída
/// (0x8e5710, 0x944c94), ou a conta sobe; uma nova (0x41 bytes, 0x944cf1 e 0x8e5680) recebe os termos campo a campo
/// (a data +0x12 por 0x417c10; de +0x30 e +0x31 só os 4 bits baixos), +0x20 da oferta e o id seguinte (+0x85a); a
/// lista +4 dos termos é copiada numa nova (0xe bytes, 0x68d020, 0x68d2f0). Comprador (termos +8) humano: a notícia da
/// negociação (0x8e4610 no empréstimo +0x1f = 1, senão 0x8d6bf0) e a oferta agendada em 14 dias; senão em 2 (0x8c6220).
unsafe extern "thiscall" fn guardar_a_negociacao(this: usize, oferta: usize, termos: usize, marcar: u32) {
    let marcar_sob_oferta: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x84b580usize);
    let realocar: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945667usize);
    let alocar_zerado: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945501usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8e5710usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8e5680usize);
    let atribuir_data: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let nova_lista: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let copiar_lista: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d2f0usize);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let noticia_do_emprestimo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8e4610usize);
    let noticia_da_negociacao: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8d6bf0usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    if marcar != 0 {
        cpv4_pb(oferta + 0x2d, 1);
        cpv4_pb(termos + 0x2d, 1);
        marcar_sob_oferta(cpv4_d(this + 0x856), cpv4_pessoa(cpv4_d(oferta + 0xc)), cpv4_clube(cpv4_d(oferta + 8)), cpv4_d(oferta));
    }
    let mut k = cpv4_d(oferta + 0x28);
    if k == usize::MAX {
        if cpv4_d(this + 0x14) == cpv4_d(this + 0x10) {
            let v = cpv4_d(this + 0xc);
            let cap = cpv4_d(this + 0x10).wrapping_add(0x32);
            cpv4_pd(this + 0x10, cap);
            if v != 0 {
                let nv = realocar(v, cap * 4);
                cpv4_pd(this + 0xc, nv);
                core::ptr::write_bytes((nv + cpv4_d(this + 0x10) * 4 - 0xc8) as *mut u32, 0, 0x32);
            } else {
                cpv4_pd(this + 0xc, alocar_zerado(cap, 4));
            }
        }
        let cap = cpv4_d(this + 0x10) as i32;
        let v = cpv4_d(this + 0xc);
        match (0..cap.max(0) as usize).find(|&j| cpv4_d(v + 4 * j) == 0) {
            Some(j) => k = j,
            None => return aviso_em(0xa8d514, 0x60f),
        }
        crate::comparacao::ramo("cpv_4 vaga nova");
        cpv4_pd(oferta + 0x28, k);
        cpv4_pb(oferta + 0x2c, 1); // cpv4: a oferta em negociação
    }
    let antiga = cpv4_d(cpv4_d(this + 0xc) + k * 4);
    if antiga != 0 {
        destruir(antiga);
        liberar(antiga);
    } else {
        cpv4_pd(this + 0x14, cpv4_d(this + 0x14).wrapping_add(1));
    }
    let mut nova = novo(0x41);
    nova = if nova != 0 { construir(nova) } else { 0 };
    cpv4_pd(cpv4_d(this + 0xc) + k * 4, nova);
    let neg = cpv4_d(cpv4_d(this + 0xc) + k * 4);
    cpv4_pd(neg, cpv4_d(termos));
    cpv4_pd(neg + 4, cpv4_d(termos + 4));
    cpv4_pd(neg + 8, cpv4_d(termos + 8));
    cpv4_pd(neg + 0xc, cpv4_d(termos + 0xc));
    cpv4_pb(neg + 0x10, cpv4_b(termos + 0x10));
    let b11 = cpv4_b(termos + 0x11);
    let mut temporaria = [0u32; 2];
    cpv4_pb(neg + 0x11, b11);
    atribuir_data(neg + 0x12, temporaria.as_mut_ptr() as usize, termos + 0x12);
    cpv4_pb(neg + 0x1a, cpv4_b(termos + 0x1a));
    cpv4_pd(neg + 0x1b, cpv4_d(termos + 0x1b)); // cpv4: o preço dos termos
    cpv4_pb(neg + 0x1f, cpv4_b(termos + 0x1f));
    cpv4_pd(neg + 0x20, cpv4_d(termos + 0x20));
    cpv4_pd(neg + 0x24, cpv4_d(termos + 0x24));
    cpv4_pd(neg + 0x28, cpv4_d(termos + 0x28));
    for o in 0x2c..0x30 {
        cpv4_pb(neg + o, cpv4_b(termos + o));
    }
    cpv4_pb(neg + 0x30, (cpv4_b(neg + 0x30) & 0xf0) | (cpv4_b(termos + 0x30) & 0xf));
    cpv4_pd(neg + 0x31, (cpv4_d(neg + 0x31) & !0xf) | (cpv4_d(termos + 0x31) & 0xf));
    cpv4_pd(neg + 0x35, cpv4_d(termos + 0x35));
    cpv4_pb(neg + 0x39, cpv4_b(termos + 0x39));
    cpv4_pd(neg + 0x3a, cpv4_d(termos + 0x3a));
    cpv4_pb(neg + 0x3e, cpv4_b(termos + 0x3e));
    cpv4_pb(neg + 0x3f, cpv4_b(termos + 0x3f));
    cpv4_pb(neg + 0x40, cpv4_b(termos + 0x40));
    let neg = || cpv4_d(cpv4_d(this + 0xc) + k * 4);
    cpv4_pd(neg() + 0x20, cpv4_d(oferta + 0x20));
    cpv4_pd(neg() + 0x24, cpv4_d(this + 0x85a));
    cpv4_pd(this + 0x85a, cpv4_d(this + 0x85a).wrapping_add(1));
    if cpv4_d(termos + 4) != 0 {
        let mut l = novo(0xe);
        l = if l != 0 { nova_lista(l) } else { 0 };
        cpv4_pd(neg() + 4, l);
        copiar_lista(cpv4_d(neg() + 4), cpv4_d(termos + 4));
    }
    if humano(cpv4_d(0xb7431c), cpv4_clube(cpv4_d(termos + 8))) != 0 {
        if cpv4_b(oferta + 0x1f) == 1 {
            noticia_do_emprestimo(this, neg(), oferta);
        } else {
            noticia_da_negociacao(this, neg(), oferta);
        }
        agendar(this, oferta, 0xe, 0);
    } else {
        crate::comparacao::ramo("cpv_4 negociacao agendada");
        agendar(this, oferta, 2, 0); // cpv4: a oferta em 2 dias
    }
}

/// { 0x69fd50 } (thiscall, ret 8, manager_manager.cpp, quadro SEH sem destrutor): `MANAGER_MANAGER::
/// post_job_rumour_news(pessoa, clube)`: sem pessoa ou sem clube dela (+0x39), nada; um sorteio(4) = 0, nada; antes da
/// data das notícias ([0xdd6ec8]), nada. A notícia 0x1776 (local): 0..3 a pessoa, 4 = o cargo (+0x3d), 5 = o id do
/// clube dela, 6 = o id do clube do boato, 7 = sorteio(4); aos destinatários (0x8d1da0 do gerente de transferências
/// [0xde1f54], com 0x52, até 30; sem gerente, nenhum) pela entrega ao técnico (0x7861e0).
unsafe extern "thiscall" fn noticia_do_boato_do_tecnico(_this: usize, pessoa: usize, clube: usize) {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    if pessoa == 0 || cpv4_d(pessoa + 0x39) == 0 || sorteio(4) == 0 {
        return;
    }
    let dia = *(0xdd6ec8 as *const i16);
    let ano = *(0xdd6eca as *const u16);
    let hoje_ano = *(0xae2c92 as *const u16);
    if ano > hoje_ano || (ano == hoje_ano && dia > *(0xae2c90 as *const i16)) {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u32 = core::mem::transmute(0x8d1da0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparacao::ramo("cpv_4 boato do tecnico");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1776, 0);
    gravar(n, 0, cpv4_d(pessoa) as u32);
    gravar(n, 1, cpv4_d(cpv4_d(pessoa + 4) + 0x33) as u32);
    gravar(n, 2, cpv4_d(cpv4_d(pessoa + 8) + 0x33) as u32);
    gravar(n, 3, cpv4_d(cpv4_d(pessoa + 0xc) + 0x33) as u32);
    gravar(n, 4, cpv4_b(pessoa + 0x3d) as i8 as i32 as u32);
    gravar(n, 5, cpv4_d(cpv4_d(pessoa + 0x39)) as u32);
    gravar(n, 6, cpv4_d(clube) as u32);
    let s = sorteio(4) as i16 as i32; // cpv4: a variante do boato
    gravar(n, 7, s as u32);
    let mut clubes = [0usize; 30];
    let gerente = cpv4_d(0xde1f54);
    let q = if gerente != 0 { destinatarios(gerente, pessoa, 0x52, clubes.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    for &c in clubes.iter().take(if q > 0 { q as usize } else { 0 }) {
        entregar(cpv4_d(0xb7431c), n, c);
    }
}

/// { 0x8d8310 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor): `TRANSFER_MANAGER::
/// post_player_rejects_move_news(oferta, motivo)`: o dono do jogador (+0xc; contrato 0x4cf5a0, nenhum no tipo 5), os
/// destinatários (0x8d1130 com 0x2f, até 30), a notícia 0xfae (local): 0..3 o jogador, 4 = o id do dono (ou −1), 5 =
/// +8, 6 = +0x1f, 7 = o motivo, 8 = o técnico do dono, 9 = o técnico do comprador (+8 > −1); a cada destinatário.
unsafe extern "thiscall" fn noticia_da_recusa_em_se_mudar(this: usize, oferta: usize, motivo: u32) {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u32 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparacao::ramo("cpv_4 recusa em se mudar");
    let p = cpv4_pessoa(cpv4_d(oferta + 0xc));
    let c = contrato_de(cpv4_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let dono = if c != 0 && cpv4_b(c + 0x35) & 0x3f != 5 { cpv4_clube(cpv4_d(c + 4)) } else { 0 };
    let mut clubes = [0usize; 30];
    let q = destinatarios(this, oferta, 0x2f, clubes.as_mut_ptr(), 0x1e) as i8;
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xfae, 0);
    gravar(n, 0, cpv4_d(p) as u32);
    gravar(n, 1, cpv4_d(cpv4_d(p + 4) + 0x33) as u32);
    gravar(n, 2, cpv4_d(cpv4_d(p + 8) + 0x33) as u32);
    gravar(n, 3, cpv4_d(cpv4_d(p + 0xc) + 0x33) as u32);
    gravar(n, 4, if dono != 0 { cpv4_d(dono) as u32 } else { u32::MAX });
    gravar(n, 5, cpv4_d(oferta + 8) as u32);
    gravar(n, 6, cpv4_b(oferta + 0x1f) as i8 as i32 as u32);
    gravar(n, 7, motivo);
    if dono != 0 {
        let t = cpv4_d(dono + 0xcf);
        if t != 0 {
            gravar(n, 8, cpv4_d(t) as u32);
        }
    }
    let comprador = cpv4_d(oferta + 8) as i32;
    if comprador > -1 {
        let t = cpv4_d(cpv4_clube(comprador as usize) + 0xcf);
        if t != 0 {
            gravar(n, 9, cpv4_d(t) as u32);
        }
    }
    for &d in clubes.iter().take(if q > 0 { q as usize } else { 0 }) {
        entregar(cpv4_d(0xb7431c), n, d);
    }
}

// ponytail: aloca e libera negociações e listas, agenda a oferta e manda notícias: não roda duas vezes. Só a original;
// a cópia leva os ramos e se prova pela carreira e pelas mutações.
unsafe extern "thiscall" fn comparar_guardar_a_negociacao(this: usize, oferta: usize, termos: usize, marcar: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1242)(this, oferta, termos, marcar)
}

// ponytail: sorteia e a notícia vai às caixas (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_do_boato_do_tecnico(this: usize, pessoa: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1243)(this, pessoa, clube)
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_da_recusa_em_se_mudar(this: usize, oferta: usize, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1244)(this, oferta, motivo)
}

// ---------- 258a leva: as receitas da semana e o centro de treino: o merchandising, os ingressos da temporada e a obra do centro de treino (finance.cpp) ----------

unsafe fn cpv5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv5_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpv5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpv5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpv5_somar(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, cpv5_i(a).wrapping_add(v));
}

/// O saldo (i64 em +0 do registro de finanças) mais `v` (estendido com sinal, como o cdq/adc da original).
unsafe fn cpv5_somar_saldo(fin: usize, v: i32) {
    let s = core::ptr::read_unaligned(fin as *const i64);
    core::ptr::write_unaligned(fin as *mut i64, s.wrapping_add(v as i64));
}

unsafe fn cpv5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O registro de finanças (0x167 bytes) do clube na tabela `this` (FinanceTable), pelo id (+0) do clube.
unsafe fn cpv5_financas(this: usize, clube: usize) -> usize {
    cpv5_d(this).wrapping_add((cpv5_d(clube) as u32).wrapping_mul(0x167) as usize)
}

/// { 0x59b7e0 } (thiscall, ret 4, finance.cpp): `FinanceTable::add_weekly_merchandising_income(clube)`, pelo passe
/// diário das finanças (0x59b9c0, no dia 6 da semana, clubes de liga). A base: com o clube de +0xbf e o estádio dele
/// (+0x69), o nível (+0xf) × 25 + a reputação (+0x80), × 0,65 truncado se o byte +0x6d for zero, escolhe o fator por
/// 5500 e 4000; sem eles, a reputação escolhe por 5250 e 3750 (um degrau acima na nação do Japão 0x9cf36c). Público
/// (+0x73) × reputação × fator, truncado; × (sorteio(11) + 95) × 0,01, truncado, soma em +0xbc, +0x15c e no saldo.
unsafe extern "thiscall" fn receita_do_merchandising(this: usize, clube: usize) {
    let outro = cpv5_d(clube + 0xbf);
    let est = if outro != 0 { cpv5_d(outro + 0x69) } else { 0 };
    let rep = cpv5_w(clube + 0x80) as i32;
    let fator = if est != 0 {
        let s = (cpv5_b(est + 0xf) as i8 as i32).wrapping_mul(25);
        let mut v = rep.wrapping_add(s);
        if cpv5_b(clube + 0x6d) == 0 {
            v = ftol(v as f64 * f64_em(0x96a5d0));
        }
        if v > 0x157c {
            0x96a6d0
        } else if v > 0xfa0 {
            0x96a6c8
        } else {
            0x96a6b8
        }
    } else {
        let nac = cpv5_d(clube + 0x53);
        let japao = nac != 0 && cpv5_i(nac) == cpv5_i(0x9cf36c);
        if rep > 0x1482 {
            if japao { 0x96a6c0 } else { 0x96a6d0 }
        } else if rep > 0xea6 {
            if japao { 0x96a6d0 } else { 0x96a6c8 }
        } else if japao {
            0x96a6c8
        } else {
            0x96a6b8
        }
    };
    let base = ftol(cpv5_i(clube + 0x73) as f64 * rep as f64 * f64_em(fator));
    crate::comparacao::ramo("cpv_5 merchandising");
    let r = cpv5_sortear(0xb); // cpv5: o sorteio do merchandising
    let v = ftol((r + 0x5f).wrapping_mul(base) as f64 * f64_em(0x967898));
    let fin = cpv5_financas(this, clube);
    cpv5_somar(fin + 0x15c, v);
    cpv5_somar(fin + 0xbc, v);
    cpv5_somar_saldo(fin, v);
}

/// Comparador: lado a lado no registro de finanças do clube e no estado do sorteio.
unsafe extern "thiscall" fn comparar_receita_do_merchandising(this: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1245);
    let mut regioes = vec![(cpv5_financas(this, clube), 0x167)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, clube), || crate::aleatorio::sem_contar(|| receita_do_merchandising(this, clube)));
    PLACAR.registrar(igual, || format!("receita_do_merchandising(clube {:#x}) original={:x?} rust={:x?}", clube, mo, mr));
}

/// { 0x599a20 } (thiscall, ret 4, finance.cpp): `Finance::sell_season_tickets(aplicar)`, pelo passe diário das finanças
/// (0x59b9c0, no dia 6 da semana, quando finance_get_days_to_season_start 0x59b4c0 diz que a temporada ainda vai
/// começar). O clube pelo id (+8, registro de 0x245 bytes); estádio (+0x69) com menos de 1500 lugares (+0x3c) passa a
/// reputação × 2,5 (mínimo 1500). A procura: o valor do clube no gerente de técnicos ([[0xb67a44]] + 73·id + 8) × 0,0001
/// (mínimo 0,15) × público (+0x73), × reputação × 0,00025 (mínimo 0,5), limitada a 87,5% dos lugares (1250 sem estádio).
/// Acima dos já vendidos (+0x18): o passo (+0x164, até 8) escolhe a fração (0x9c8758) da diferença, × (sorteio(11) +
/// 95) / 100 = os novos; a razão vendidos/lugares vai a +0x160 (f32, até 0,875). A receita (preço pela reputação × novos;
/// /4 ou /10 pelo tipo da nação +0x7e; × 1,25 na série A, Chelsea 1,5, Arsenal e Tottenham 1,3, Everton 0,75, Liverpool
/// 0,9, Alemanha 1,2) entra só com `aplicar`; depois (sorteio(rep) + sorteio(rep) + 1) × 0,001 + 1, × novos, em +0x15c.
unsafe extern "thiscall" fn ingressos_da_temporada(this: usize, aplicar: i32) {
    let id = cpv5_d(this + 8);
    let clube = cpv5_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245));
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, i32)>(1246)(this, aplicar);
    }
    let rep = cpv5_w(clube + 0x80) as i32;
    let est = cpv5_d(clube + 0x69);
    if est != 0 && cpv5_i(est + 0x3c) < 0x5dc {
        core::ptr::write_unaligned((est + 0x3c) as *mut i32, ftol(rep as f64 * f64_em(0x968e18)));
        let est = cpv5_d(clube + 0x69);
        if cpv5_i(est + 0x3c) < 0x5dc {
            core::ptr::write_unaligned((est + 0x3c) as *mut i32, 0x5dc);
        }
    }
    let gerente = cpv5_d(cpv5_d(0xb67a44));
    let valor = cpv5_w(gerente.wrapping_add((cpv5_d(clube) as u32).wrapping_mul(73) as usize) + 8);
    let mut f = valor as f64 * f64_em(0x96a5c0);
    if !(f > f64_em(0x968f20)) {
        f = f64_em(0x968f20);
    }
    let t = ftol(cpv5_i(clube + 0x73) as f64 * f);
    let mut g = rep as f64 * f64_em(0x96a680);
    if !(g > f64_em(0x968968)) {
        g = f64_em(0x968968);
    }
    let u = ftol(t as f64 * g);
    let est = cpv5_d(clube + 0x69);
    let teto = |e: usize| if e != 0 { cpv5_i(e + 0x3c) as f64 * f64_em(0x96a678) } else { f64_em(0x96a5e8) };
    let n = ftol(if (u as f64) < teto(est) { u as f64 } else { teto(est) });
    if n == 0 {
        return;
    }
    let ja = cpv5_i(this + 0x18);
    if ja >= n {
        return;
    }
    let mut passo = (cpv5_b(this + 0x164) as i8).wrapping_add(1);
    if passo < 0 || passo > 8 {
        passo = 8;
    }
    *((this + 0x164) as *mut i8) = passo;
    let b = ftol(n.wrapping_sub(ja) as f64 * f32_em(0x9c8758 + passo as usize * 4) as f64);
    crate::comparacao::ramo("cpv_5 ingressos vendidos");
    let r = cpv5_sortear(0xb);
    let novos = (r + 0x5f).wrapping_mul(b) / 100;
    let total = ja.wrapping_add(novos);
    core::ptr::write_unaligned((this + 0x18) as *mut i32, total); // cpv5: os ingressos vendidos
    let preco = ftol((rep as f64 * f64_em(0x969048) + f64_em(0x968938)) * f64_em(0x968ea8));
    let mut receita = preco.wrapping_mul(novos);
    let est = cpv5_d(clube + 0x69);
    let lugares = if est != 0 { cpv5_i(est + 0x3c) } else { 0 };
    let den = if lugares != 0 { lugares as f64 * f64_em(0x96a678) } else { f64_em(0x96a5e8) };
    let razao = total as f64 / den;
    core::ptr::write_unaligned((this + 0x160) as *mut f32, razao as f32);
    if razao > f64_em(0x96a678) {
        core::ptr::write_unaligned((this + 0x160) as *mut u32, 0x3f600000);
    }
    let nac = cpv5_d(clube + 0x53);
    let tipo = if nac != 0 { cpv5_b(nac + 0x7e) as i8 } else { 3 };
    match tipo {
        1 => {}
        2 => receita /= 4,
        _ => receita /= 10,
    }
    let liga = cpv5_d(clube + 0x57);
    let cid = cpv5_i(clube);
    let fator = if liga != 0 && cpv5_i(liga) == cpv5_i(0x9cf570) {
        0x968928
    } else if cid == cpv5_i(0x9d0250) {
        0x968940
    } else if cid == cpv5_i(0x9d0110) {
        0x968de8
    } else if cid == cpv5_i(0x9d0248) {
        0x969040
    } else if cid == cpv5_i(0x9d0244) {
        0x9689d8
    } else if cid == cpv5_i(0x9d024c) {
        0x968de8
    } else if nac != 0 && cpv5_i(nac) == cpv5_i(0x9cf314) {
        0x968978
    } else {
        0
    };
    if fator != 0 {
        receita = ftol(receita as f64 * f64_em(fator));
    }
    if aplicar != 0 { // cpv5: a receita dos ingressos
        cpv5_somar(this + 0xac, receita);
        cpv5_somar(this + 0x14c, receita);
        cpv5_somar_saldo(this, receita);
    }
    let r1 = cpv5_sortear(rep);
    let r2 = cpv5_sortear(rep); // cpv5: o segundo sorteio da loja
    let k = r1.wrapping_add(r2).wrapping_add(1);
    let v = ftol((k as f64 * f64_em(0x96a350) + f64_em(0x967890)) * novos as f64);
    cpv5_somar(this + 0x15c, v);
    cpv5_somar(this + 0xbc, v);
    cpv5_somar_saldo(this, v);
}

/// Comparador: lado a lado no registro de finanças (this), nos lugares do estádio (+0x3c) e no estado do sorteio
/// (clube nulo: só a original).
unsafe extern "thiscall" fn comparar_ingressos_da_temporada(this: usize, aplicar: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(1246);
    let clube = cpv5_d(0xae23b4).wrapping_add(cpv5_d(this + 8).wrapping_mul(0x245));
    if clube == 0 {
        return o(this, aplicar);
    }
    let mut regioes = vec![(this, 0x167)];
    let est = cpv5_d(clube + 0x69);
    if est != 0 {
        regioes.push((est + 0x3c, 4));
    }
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, aplicar), || crate::aleatorio::sem_contar(|| ingressos_da_temporada(this, aplicar)));
    PLACAR.registrar(igual, || format!("ingressos_da_temporada(fin {:#x}, {}) original={:x?} rust={:x?}", this, aplicar, mo, mr));
}

/// O custo da obra (+0x8c e +0x12c, menos no saldo) no registro de finanças do clube.
unsafe fn cpv5_cobrar_obra(this: usize, clube: usize, custo: i32) {
    let fin = cpv5_financas(this, clube);
    cpv5_somar(fin + 0x12c, custo);
    cpv5_somar(fin + 0x8c, custo);
    cpv5_somar_saldo(fin, custo.wrapping_neg());
}

/// { 0x59b630 } (thiscall, ret 4, finance.cpp): `FinanceTable::upgrade_training_facilities(clube)`, pelo fim da
/// temporada do clube (0x59cfa0, com o pedido de treino aprovado pela diretoria). O nível (+0x7f) sobe 2 (1 a partir de
/// 19) e cobra nível × 50000 + 75000. Depois, abaixo de 20: com reputação (+0x80) acima de 8250, vai a 20 e cobra
/// (22 − nível) × 250000 (mínimo 2 milhões); abaixo de 14 com reputação acima de 5250, sobe 4 (mínimo 16) e cobra 3,5
/// milhões − nível × 150000 (mínimo 1 milhão); abaixo de 16 com reputação acima de 4250, ou abaixo de 20 com reputação
/// acima de 7250: acima de 7250 vai a 20, senão sobe 2 (mínimo 12), e cobra o novo nível × 50000 + 75000. Teto 20.
unsafe extern "thiscall" fn melhorar_o_centro_de_treino(this: usize, clube: usize) {
    let nivel = (clube + 0x7f) as *mut i8;
    let custo = (*nivel as i32).wrapping_mul(50000).wrapping_add(75000);
    *nivel = if *nivel < 0x13 { (*nivel).wrapping_add(2) } else { (*nivel).wrapping_add(1) };
    cpv5_cobrar_obra(this, clube, custo);
    let n = *nivel;
    let rep = cpv5_w(clube + 0x80);
    let segundo = if n < 0x14 && rep > 0x203a {
        *nivel = 0x14;
        Some((0x16 - n as i32).wrapping_mul(250000).max(2000000))
    } else if n < 0xe && rep > 0x1482 {
        let c = 3500000i32.wrapping_sub((n as i32).wrapping_mul(150000)).max(1000000);
        *nivel = n.wrapping_add(4);
        if *nivel < 0x10 {
            *nivel = 0x10;
        }
        Some(c)
    } else if (n < 0x10 && rep > 0x109a) || (n < 0x14 && rep > 0x1c52) {
        if rep > 0x1c52 {
            *nivel = 0x14;
        } else {
            *nivel = n.wrapping_add(2);
            if *nivel < 0xc {
                *nivel = 0xc;
            }
        }
        Some((*nivel as i32).wrapping_mul(50000).wrapping_add(75000))
    } else {
        None
    };
    if let Some(c) = segundo {
        cpv5_cobrar_obra(this, clube, c);
    }
    if *nivel > 0x14 {
        *nivel = 0x14;
    }
}

/// Comparador: lado a lado no nível do centro de treino (+0x7f) e no registro de finanças do clube.
unsafe extern "thiscall" fn comparar_melhorar_o_centro_de_treino(this: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1247);
    let regioes = [(clube + 0x7f, 1), (cpv5_financas(this, clube), 0x167)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, clube), || melhorar_o_centro_de_treino(this, clube));
    PLACAR.registrar(igual, || format!("melhorar_o_centro_de_treino(clube {:#x}) original={:x?} rust={:x?}", clube, mo, mr));
}

// ---------- 258a leva: os eventos financeiros da semana: o passe dos eventos do clube, a diretoria que paga a dívida e a injeção de dinheiro (finance.cpp) ----------

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

unsafe fn cpv6_saldo(fin: usize) -> i64 {
    core::ptr::read_unaligned(fin as *const i64)
}

/// O saldo (i64 em +0) mais `v` estendido com sinal (o cdq/adc da original).
unsafe fn cpv6_somar_saldo(fin: usize, v: i32) {
    core::ptr::write_unaligned(fin as *mut i64, cpv6_saldo(fin).wrapping_add(v as i64));
}

unsafe fn cpv6_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

unsafe fn cpv6_faixa(fin: usize) -> i8 {
    let f: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    f(fin) as i8
}

/// Um evento da semana (0x59e050, 0x59dd40, 0x59e5a0, 0x59e940): thiscall(tabela, clube, finanças), 1 se aconteceu.
unsafe fn cpv6_evento(rotina: usize, this: usize, clube: usize, fin: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(rotina);
    f(this, clube, fin)
}

/// O ensaio do comparador: a cópia roda sem a notícia (0x59f300) e sem a verba (0x598800), que alocam e não podem
/// rodar duas vezes.
static mut CPV6_ENSAIO: bool = false;

/// A notícia do evento (0x59f300: thiscall(tabela, clube, valor, verba, tipo), ret 0x10); no ensaio, nada.
unsafe fn cpv6_noticia(this: usize, clube: usize, valor: i32, verba: u32, tipo: u32) {
    if CPV6_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) = core::mem::transmute(0x59f300usize);
    f(this, clube, valor, verba, tipo);
}

/// A verba de transferências (Finance::get_transfer_budget 0x598800 com 0, 0, 0, 1, 0) que vai na notícia; no
/// ensaio, 0 (a notícia não sai).
unsafe fn cpv6_verba(fin: usize) -> u32 {
    if CPV6_ENSAIO {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x598800usize);
    f(fin, 0, 0, 0, 1, 0)
}

/// O fim da semana do clube no estado de crise (0x5a02f0: thiscall(tabela, id)).
unsafe fn cpv6_fim_da_crise(this: usize, clube: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x5a02f0usize);
    f(this, cpv6_d(clube) as u32);
}

/// O estado (+0x165) volta a 0; se era 1 (crise), a notícia do fim da crise (0x5a02f0) antes.
unsafe fn cpv6_sair_da_crise(this: usize, clube: usize, fin: usize) {
    let s = cpv6_b(fin + 0x165);
    if s == 1 {
        cpv6_fim_da_crise(this, clube);
    }
    if s == 1 || s == 2 {
        *((fin + 0x165) as *mut u8) = 0;
    }
}

/// O centro de treino no máximo (20) cai um nível com sorteio(10) zero, se a reputação for menor que 7250.
unsafe fn cpv6_desgaste_do_centro(clube: usize) {
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
unsafe extern "thiscall" fn eventos_financeiros_da_semana(this: usize, clube: usize, semana: u32) {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1248)(this, clube, semana);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut eh_a: u32 = 0;
    let p = parceiro(clube, &mut eh_a, 1);
    let c = if p != 0 && eh_a == 0 { p } else { clube };
    let fin = cpv6_d(this).wrapping_add((cpv6_d(c) as u32).wrapping_mul(0x167) as usize);
    let aperto = (fin + 0x166) as *mut i8;
    match cpv6_faixa(fin) {
        -1 => {
            let s = cpv6_b(fin + 0x165);
            if s != 2 {
                if s == 1 {
                    cpv6_fim_da_crise(this, clube);
                }
                *((fin + 0x165) as *mut u8) = 2;
            }
            if semana == 0 {
                return;
            }
            if cpv6_b(clube + 0x6d) != 0 {
                *aperto = (*aperto).wrapping_add(1);
            }
            cpv6_desgaste_do_centro(clube);
            if cpv6_evento(0x59dd40, this, clube, fin) == 0 && cpv6_evento(0x59e5a0, this, clube, fin) == 0 {
                cpv6_evento(0x59e940, this, clube, fin);
            }
        }
        -2 => {
            if cpv6_b(fin + 0x165) != 1 {
                let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x59fc60usize);
                let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
                let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
                let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
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
                    let k = contrato_de(g, j);
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
            if cpv6_evento(0x59e940, this, clube, fin) == 0 && cpv6_evento(0x59e5a0, this, clube, fin) == 0 {
                cpv6_evento(0x59e050, this, clube, fin);
            }
            cpv6_desgaste_do_centro(clube);
            if cpv6_b(clube + 0x6d) != 0 {
                *aperto = (*aperto).wrapping_add(3);
            }
        }
        1 => {
            cpv6_sair_da_crise(this, clube, fin);
            if semana == 0 {
                return;
            }
            if *aperto > 0 {
                *aperto -= 1;
            }
            if cpv6_evento(0x59e050, this, clube, fin) == 0 && cpv6_evento(0x59dd40, this, clube, fin) == 0 {
                cpv6_evento(0x59e5a0, this, clube, fin);
            }
        }
        2 => {
            cpv6_sair_da_crise(this, clube, fin);
            if semana == 0 {
                return;
            }
            if *aperto > 5 {
                *aperto -= 1;
            }
        }
        _ => {
            cpv6_sair_da_crise(this, clube, fin);
            if semana == 0 {
                return;
            }
            if *aperto > 10 {
                *aperto -= 1;
            }
            crate::comparacao::ramo("cpv_6 impulso na faixa 0");
            let ok = cpv6_evento(0x59e050, this, clube, fin) != 0; // cpv6: o impulso na faixa 0
            if !ok && cpv6_evento(0x59dd40, this, clube, fin) == 0 {
                cpv6_evento(0x59e5a0, this, clube, fin);
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
    let t = ftol(lugares as f64 * f64_em(0x967880)) as i16 as i32;
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
unsafe extern "thiscall" fn comparar_eventos_financeiros_da_semana(this: usize, clube: usize, semana: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1248)(this, clube, semana)
}

/// { 0x59dd40 } (thiscall, ret 8, finance.cpp): `FinanceTable::try_board_debt_payment(clube, finanças)`, pelo passe dos
/// eventos (0x59ed70). Faixa (0x5986b0) −2, −1 (divisor 2, ou 0,4 com reputação de 3500), ou 0 com reputação até 3500 e
/// saldo abaixo de 25 × reputação (0,4); as outras: 0. Sorteio(7000) + 3500 acima da reputação; sorteio(6 / divisor)
/// zero; sorteio(nível +0x20 do estádio do clube de +0xbf, ou 10) não zero. A diretoria paga 450 × reputação (abaixo de
/// 1250 com saldo abaixo de −450 × reputação), 300 × (saldo abaixo de −300 × reputação) ou 150 ×: em +0x154, +0xb4 e no
/// saldo; a notícia (0x59f300, tipo 4); devolve 1.
unsafe extern "thiscall" fn diretoria_paga_a_divida(this: usize, clube: usize, fin: usize) -> u32 {
    let rep = cpv6_w(clube + 0x80) as i32;
    let div = match cpv6_faixa(fin) {
        -2 => 2.0f64,
        -1 => {
            if rep >= 0xdac {
                0.4
            } else {
                2.0
            }
        }
        0 => {
            if rep > 0xdac || cpv6_saldo(fin) >= rep.wrapping_mul(25) as i64 {
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
    let n = ftol(f64_em(0x969000) / div);
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
    let saldo = cpv6_saldo(fin);
    let valor = if rep < 0x4e2 && saldo < rep.wrapping_mul(-450) as i64 {
        rep.wrapping_mul(450)
    } else if saldo >= rep.wrapping_mul(-300) as i64 {
        rep.wrapping_mul(150)
    } else {
        rep.wrapping_mul(300)
    };
    cpv6_somar(fin + 0x154, valor);
    cpv6_somar(fin + 0xb4, valor);
    cpv6_somar_saldo(fin, valor);
    cpv6_noticia(this, clube, valor, 0, 4);
    1
}

/// O comparador dos eventos: a cópia em ensaio (sem notícia nem verba) e a original a partir do mesmo estado (registro de
/// finanças e sorteio). Sem evento: tudo igual, inclusive o sorteio; com evento: a devolução e os campos que o evento
/// grava (saldo, +0xb4, +0x154, +0x165), porque a notícia e a verba da original podem sortear e gravar.
unsafe fn cpv6_comparar(nome: &str, fin: usize, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
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

unsafe extern "thiscall" fn comparar_diretoria_paga_a_divida(this: usize, clube: usize, fin: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1249);
    cpv6_comparar("diretoria_paga_a_divida", fin, || o(this, clube, fin), || diretoria_paga_a_divida(this, clube, fin))
}

/// { 0x59e5a0 } (thiscall, ret 8, finance.cpp): `FinanceTable::try_cash_injection(clube, finanças)`, pelo passe dos
/// eventos (0x59ed70). Reputação (+0x80) de 4750, sem a marca +0x82, fora do País de Gales e das Irlandas. Divisor pela
/// faixa (−2: 0,25; −1: 0,1; outras: 0,001). Com o estádio do clube de +0xbf: sorteio(30 / divisor) abaixo do nível
/// (+0xf) e o nível +0x20 acima de sorteio(5) + 10: (20 × nível + 150) × reputação + 750000; sem ele: sorteio(4 /
/// divisor) zero: (5 × reputação + 15000) × 50. Dívida acima de 10 × o valor: 10% da dívida. Teto: reputação² + 1,5
/// milhão abaixo de 2000; senão 4 milhões (+1 se o valor passar de reputação² / 4). Em +0x154, +0xb4 e no saldo; a
/// notícia (0x59f300, tipo 3, com a verba 0x598800); encerra a crise (+0x165 = 0); devolve 1.
unsafe extern "thiscall" fn injecao_de_dinheiro(this: usize, clube: usize, fin: usize) -> u32 {
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
    let div = match cpv6_faixa(fin) {
        -2 => f64_em(0x9689e0),
        -1 => f64_em(0x967880),
        _ => f64_em(0x96a350),
    };
    let rep = cpv6_w(clube + 0x80) as i32;
    let outro = cpv6_d(clube + 0xbf);
    let mut valor;
    if outro != 0 && cpv6_d(outro + 0x69) != 0 && cpv6_b(clube + 0x82) == 0 {
        let r = cpv6_sortear(ftol(f64_em(0x968dd8) / div));
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
        if cpv6_sortear(ftol(f64_em(0x968938) / div)) != 0 {
            return 0;
        }
        if cpv6_b(clube + 0x82) != 0 {
            return 0;
        }
        valor = rep.wrapping_mul(5).wrapping_add(0x3a98).wrapping_mul(50);
    }
    let saldo = cpv6_saldo(fin);
    if (valor.wrapping_mul(10) as i64) < saldo.wrapping_neg() {
        valor = ftol(-(saldo as f64 * f64_em(0x967880)));
    }
    if cpv6_w(clube + 0x80) < 0x7d0 {
        let teto = rep.wrapping_mul(rep).wrapping_add(0x16e360);
        if valor > teto {
            valor = teto;
        }
    } else {
        let maior = (valor as f64 > rep.wrapping_mul(rep) as f64 * f64_em(0x9689e0)) as i32;
        if valor > 0x3d0900 + maior {
            valor = 0x3d0900 + maior;
        }
    }
    cpv6_somar(fin + 0x154, valor);
    cpv6_somar(fin + 0xb4, valor);
    cpv6_somar_saldo(fin, valor);
    let verba = cpv6_verba(fin);
    cpv6_noticia(this, clube, valor, verba, 3);
    *((fin + 0x165) as *mut u8) = 0;
    1
}

unsafe extern "thiscall" fn comparar_injecao_de_dinheiro(this: usize, clube: usize, fin: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1250);
    cpv6_comparar("injecao_de_dinheiro", fin, || o(this, clube, fin), || injecao_de_dinheiro(this, clube, fin))
}

// ---------- 258a leva: os salários do pessoal e os bônus: o teto salarial do pessoal, o pagamento dos bônus e o ajuste do teto salarial (finance.cpp) ----------

unsafe fn cpv7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv7_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpv7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpv7_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x597db0 } (thiscall, ret 4, finance.cpp): `Finance::get_max_staff_wage(cargo)`, o maior salário semanal que o
/// clube oferece a um membro do pessoal (renovação 0x4cda70, interesse pelo cargo 0x844780, telas de contrato). Clube
/// pelo id (+8); tipo 3 (+0x64): 0. O padrão pelo cargo (5 a 7: 35000; 8: 20000; 9: 1500; 10: 1000; outros: 750);
/// seleção (0x53b900): o padrão. Sem nação (+0x53): padrão × 0,25. Com nação: o nível (+0x85) / (20 × tipo +0x7e
/// entre 1 e 3) × padrão; no tipo 1, Inglaterra, Itália e Espanha × 1,05 com reputação acima de 7250, Japão nada, EUA
/// acima de 1750 com 0,75 do excedente, e os demais grupos com o excedente acima de 2500/1250/750 pelo fator (0,75/0,75/
/// 0,3, ou reputação/50 / nível × 0,1 × ele, até 1); com liga (+0x57), × peso da liga (+0x69) × 0,2. Pela faixa das
/// finanças (0x5986b0): −2 × 0,75, −1 × 0,95, 1 × 1,5, 2 × 2. Limitado ao teto do clube (+0x14) e à faixa do cargo.
unsafe extern "thiscall" fn teto_salarial_do_pessoal(this: usize, cargo: i8) -> i32 {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    let id = cpv7_d(this + 8);
    let clube = cpv7_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245));
    if cpv7_b(clube + 0x64) == 3 {
        return 0;
    }
    let r50 = cpv7_w(clube + 0x80) as i32 / 50;
    let idx = (cargo as i32).wrapping_sub(5);
    let padrao: i32 = match idx {
        0..=2 => 35000,
        3 => 20000,
        4 => 1500,
        5 => 1000,
        _ => 750,
    };
    if selecao(clube) != 0 {
        return padrao;
    }
    let nac = cpv7_d(clube + 0x53);
    let mut w;
    if nac == 0 {
        w = ftol(padrao as f64 * f64_em(0x9689e0));
    } else {
        let tipo = (cpv7_b(nac + 0x7e) as i8).clamp(1, 3);
        let k = cpv7_b(nac + 0x85) as i8 as i32;
        w = ftol(k as f64 / (tipo as i32 * 20) as f64 * padrao as f64);
        if tipo == 1 {
            let n = cpv7_i(nac);
            let e = |a: usize| n == cpv7_i(a);
            if e(0x9cf2e4) || e(0x9cf360) || e(0x9cf488) {
                if cpv7_w(clube + 0x80) > 0x1c52 {
                    crate::comparacao::ramo("cpv_7 clube grande");
                    w = ftol(w as f64 * f64_em(0x9689b8)); // cpv7: o clube grande da Inglaterra, Itália e Espanha
                }
            } else if e(0x9cf36c) {
            } else if e(0x9cf4f8) {
                if w > 0x6d6 {
                    w = ftol(w.wrapping_sub(0x6d6) as f64 * f64_em(0x969040) + f64_em(0x96a5f8));
                }
            } else {
                let (base, limiar, soma) = if e(0x9cf314) || e(0x9cf300) || e(0x9cf45c) || e(0x9cf338) {
                    (0x969040, 0x9c4, 0x9690b0)
                } else if e(0x9cf4a8) || e(0x9cf2c0) || e(0x9cf244) || e(0x9cf434) {
                    (0x969040, 0x4e2, 0x96a5e8)
                } else {
                    (0x968e80, 0x2ee, 0x96a5f0)
                };
                let mut f = f64_em(base);
                if r50 > k.wrapping_mul(10) {
                    let d = if k >= 1 { k } else { 1 };
                    f = r50 as f64 / d as f64 * f64_em(0x967880) * f64_em(base);
                }
                if w > limiar {
                    if f > f64_em(0x967890) {
                        f = f64_em(0x967890);
                    }
                    w = ftol(w.wrapping_sub(limiar) as f64 * f + f64_em(soma));
                }
            }
        }
        let liga = cpv7_d(clube + 0x57);
        if liga != 0 {
            crate::comparacao::ramo("cpv_7 peso da liga");
            w = ftol(cpv7_w(liga + 0x69) as f64 * w as f64 * f64_em(0x968918)); // cpv7: o peso da liga
        }
    }
    match faixa(this) as i8 {
        -2 => w = ftol(w as f64 * f64_em(0x969040)),
        -1 => w = ftol(w as f64 * f64_em(0x9689d0)),
        1 => w = ftol(w as f64 * f64_em(0x968940)),
        2 => w = w.wrapping_add(w),
        _ => {}
    }
    let teto = cpv7_i(this + 0x14);
    if teto < w {
        w = teto;
    }
    let (min, max) = match idx {
        0 => (0x6d6, 0x88b8),
        1 | 2 => (0x1f4, 0x4e20),
        3 => (0xfa, 0x1d4c),
        4 => (0x7d, 0x7d0),
        5 => (0x7d, 0x5dc),
        _ => (0x7d, 0x3e8),
    };
    if w > max {
        max
    } else if w < min {
        min
    } else {
        w.min(0x3d090)
    }
}

/// Comparador: pelo valor.
unsafe extern "thiscall" fn comparar_teto_salarial_do_pessoal(this: usize, cargo: i8) -> i32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8) -> i32>(1251)(this, cargo);
    let n = teto_salarial_do_pessoal(this, cargo);
    PLACAR.registrar(r == n, || format!("teto_salarial_do_pessoal(fin {:#x}, cargo {}) original={} rust={}", this, cargo, r, n));
    r
}

/// { 0x599960 } (thiscall, ret 4, finance.cpp): `Finance::pay_bonuses(valor)`, pelo fim do jogo dos contratos
/// (CONTRACT_MANAGER::update_after_match 0x4ca6d0, três chamadas): soma o valor nas despesas de bônus (+0x138 e +0x98)
/// e o tira do saldo (i64 em +0, estendido com sinal).
unsafe extern "thiscall" fn pagar_os_bonus(this: usize, valor: i32) {
    for o in [0x138usize, 0x98] {
        core::ptr::write_unaligned((this + o) as *mut i32, cpv7_i(this + o).wrapping_add(valor));
    }
    crate::comparacao::ramo("cpv_7 bonus pagos");
    let s = core::ptr::read_unaligned(this as *const i64);
    core::ptr::write_unaligned(this as *mut i64, s.wrapping_sub(valor as i64)); // cpv7: o bônus sai do saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn comparar_pagar_os_bonus(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(1252);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pagar_os_bonus(this, valor));
    PLACAR.registrar(igual, || format!("pagar_os_bonus(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// { 0x598340 } (thiscall, ret 4, finance.cpp): `Finance::adjust_max_wage(percentual)`, pela resposta da diretoria
/// (MANAGER_MANAGER::handle_board_request 0x6a91b0) e pela 0x6acee8: o teto salarial (+0x14) vira (percentual + 100) ×
/// 0,01 × teto, truncado.
unsafe extern "thiscall" fn ajustar_o_teto_salarial(this: usize, percentual: i8) {
    let v = ftol((percentual as i32 + 100) as f64 * f64_em(0x967898) * cpv7_i(this + 0x14) as f64);
    core::ptr::write_unaligned((this + 0x14) as *mut i32, v);
}

/// Comparador: lado a lado no teto salarial (+0x14).
unsafe extern "thiscall" fn comparar_ajustar_o_teto_salarial(this: usize, percentual: i8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8)>(1253);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x14, 4)], || o(this, percentual), || ajustar_o_teto_salarial(this, percentual));
    PLACAR.registrar(igual, || format!("ajustar_o_teto_salarial(fin {:#x}, {}) original={:x?} rust={:x?}", this, percentual, mo, mr));
}

// ---------- 258a leva: o físico depois da partida: o ritmo de recuperação de cada jogador e as batidas que viram lesão, a batida do jogador na partida e o jogador de férias (injury.cpp) ----------

unsafe fn cpv8_d(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpv8_b(a: usize) -> i8 {
    *(a as *const i8)
}

/// A pessoa da vaga de 0x49 bytes da partida (id em −0x30): sem id (−1) ou com id fora de [0xae2364], nenhuma. A
/// original confere o ponteiro nulo antes do id (ponteiro nulo passa).
unsafe fn cpv8_pessoa_da_vaga(vaga: usize) -> Option<(i32, usize)> {
    let id = cpv8_d(vaga - 0x30);
    if id == -1 {
        return None;
    }
    let pessoa = (cpv8_d(0xae23bc) as u32).wrapping_add((id as u32).wrapping_mul(0x6e)) as usize;
    if pessoa != 0 && cpv8_d(pessoa) >= cpv8_d(0xae2364) {
        return None;
    }
    Some((id, pessoa))
}

/// O nível da vaga para o ritmo: +4 se não for negativo, senão +0, senão o padrão da partida (+0x4d).
unsafe fn cpv8_nivel(partida: usize, vaga: usize) -> i8 {
    let b = cpv8_b(vaga + 4);
    if b >= 0 {
        return b;
    }
    let b = cpv8_b(vaga);
    if b >= 0 {
        b
    } else {
        cpv8_b(partida + 0x4d)
    }
}

/// { 0x62c080 } (thiscall, ret 8, injury.cpp): `Injury::apply_match_effects(jogo, partida)`, pela fechar_partida_do_dia
/// (0x6b5f00) e pelas estatísticas simuladas (0x84be20), em toda partida: para os dois lados da partida (0x625 bytes a
/// partir de +0x1c0), as 20 vagas de 0x49 bytes com pessoa (id em −0x30). O ritmo: o nível (+0x5a nas onze primeiras
/// com a marca +0x4c da partida; senão o da vaga, menos o +2 nas reservas, que com 0xff dá 0) mais (10000 − condição
/// +6) / 100, em 32 bits (o nível entra só nos 16 de baixo). Jogo sem a marca +0x40: a metade (16 bits). Com ela: acima
/// de 33 + sorteio(3) anos (+0x18), × 1,25 (0x968928, _ftol); senão abaixo de 23 − sorteio(5), o dobro. Antes do começo
/// do jogo ([0xdd6ec8] depois de hoje, 0x456740): 0. A recuperação (0x62bd10) com a condição e o ritmo. Com a batida
/// (−0x11) e a lista das batidas da partida (+0x28): cada batida da pessoa (o tipo 7 só com a ficha +0xf de 15 ou
/// mais) tem a gravidade por +5 (acima de 95: 2; de 66: 1; de 51: 0; senão a da batida anterior) e o tipo da lesão
/// (0x62d140); tipo válido: fim da lesão atual (0x62e700) e a lesão nova (0x62d250 com o jogo, ou 0 no jogo −2).
unsafe extern "thiscall" fn efeitos_fisicos_da_partida(this: usize, jogo: usize, partida: usize) {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let recuperar: unsafe extern "thiscall" fn(usize, usize, i16, i16) = core::mem::transmute(0x62bd10usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tipo_de_lesao: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> u8 = core::mem::transmute(0x62d140usize);
    let fim_da_lesao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x62e700usize);
    let nova_lesao: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize, u32, i32, u32, u32) = core::mem::transmute(0x62d250usize);
    // ponytail: a gravidade é uma local da original que passa de uma batida para a outra; sem batida acima de 51 antes,
    // a original lê sobra de pilha e a cópia usa 0 (o comparador deixa toda partida com batida para a original).
    let mut gravidade: i8 = 0;
    for lado in 0..2usize {
        let base = partida + 0x1c0 + 0x625 * lado;
        for k in 0..0x14i8 {
            let vaga = base + 0x49 * k as usize;
            let Some((id, pessoa)) = cpv8_pessoa_da_vaga(vaga) else { continue };
            let marca = cpv8_b(partida + 0x4c) != 0;
            let nivel: i8 = if k < 0xb {
                if marca { 0x5a } else { cpv8_nivel(partida, vaga) }
            } else if *((vaga + 2) as *const u8) == 0xff {
                0
            } else {
                cpv8_nivel(partida, vaga).wrapping_sub(cpv8_b(vaga + 2))
            };
            let c = 0x2710i32.wrapping_sub(core::ptr::read_unaligned((vaga + 6) as *const i16) as i32);
            // ponytail: a original soma o ecx inteiro: o nível nos 16 de baixo, os 16 de cima de 10000 − condição
            let mut v = (c / 100).wrapping_add(((c as u32 & 0xffff_0000) | nivel as i16 as u16 as u32) as i32); // cpv8: o ritmo
            crate::comparacao::ramo("cpv_8 ritmo");
            if cpv8_b(jogo + 0x40) == 0 {
                crate::comparacao::ramo("cpv_8 metade");
                v = (v as i16 as i32) / 2; // cpv8: sem a marca do jogo, a metade
            } else if cpv8_b(pessoa + 0x18) as i32 > sorteio(3) + 0x21 {
                v = ((v as i16 as i32) as f64 * f64_em(0x968928)) as i32;
            } else if (cpv8_b(pessoa + 0x18) as i32) < 0x17 - sorteio(5) {
                crate::comparacao::ramo("cpv_8 jovem");
                v = v.wrapping_add(v); // cpv8: o jovem em dobro
            }
            if depois(0xdd6ec8, cpv8_d(0xae2c90) as u32, cpv8_d(0xae2c94) as u32) != 0 {
                v = 0;
            }
            recuperar(this, pessoa, core::ptr::read_unaligned((vaga + 6) as *const i16), v as i16);
            if *((vaga - 0x11) as *const u8) == 0 || cpv8_d(this + 0x28) == 0 {
                continue;
            }
            let lista = cpv8_d(this + 0x28) as usize;
            core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
            let mut e = proximo(cpv8_d(this + 0x28) as usize);
            while e != 0 {
                let tipo = cpv8_b(e + 4);
                if cpv8_d(e) == id && (tipo != 7 || cpv8_b(cpv8_d(pessoa + 0x61) as usize + 0xf) >= 0xf) {
                    let g = cpv8_b(e + 5);
                    if g > 0x5f {
                        gravidade = 2;
                    } else if g > 0x41 {
                        gravidade = 1;
                    } else if g > 0x32 {
                        gravidade = 0;
                    }
                    let t = tipo_de_lesao(this, tipo, gravidade, pessoa);
                    if (t as i8) >= 0 {
                        crate::comparacao::ramo("cpv_8 batida vira lesao");
                        fim_da_lesao(this, pessoa, 0);
                        let j = if cpv8_d(jogo) == -2 { 0 } else { jogo };
                        nova_lesao(this, t as u32, 0, pessoa, j, 1, -1, 0, 0);
                    }
                }
                e = proximo(cpv8_d(this + 0x28) as usize);
            }
        }
    }
}

/// A pessoa tem batida na lista da partida que chega à lesão (a mesma conferência da 0x62c080)? O cursor (+0xa) volta
/// ao que era.
unsafe fn cpv8_tem_batida(this: usize, id: i32, pessoa: usize) -> bool {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let lista = cpv8_d(this + 0x28) as usize;
    let cursor = core::ptr::read_unaligned((lista + 0xa) as *const u32);
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let mut achou = false;
    let mut e = proximo(lista);
    while e != 0 && !achou {
        achou = cpv8_d(e) == id && (cpv8_b(e + 4) != 7 || cpv8_b(cpv8_d(pessoa + 0x61) as usize + 0xf) >= 0xf);
        e = proximo(lista);
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, cursor);
    achou
}

/// Comparador: lado a lado no estado do sorteio, no cursor da lista das batidas e na recuperação (+0xa, 4 bytes, do
/// registro de 31 bytes em [this]) de cada pessoa das vagas. Partida com batida que chega ao tipo da lesão: só a original (a lesão nova aloca e manda a
/// notícia; a gravidade da primeira batida pode ser sobra de pilha).
unsafe extern "thiscall" fn comparar_efeitos_fisicos_da_partida(this: usize, jogo: usize, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1254);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let mut batida = false;
    for lado in 0..2usize {
        for k in 0..0x14usize {
            let vaga = partida + 0x1c0 + 0x625 * lado + 0x49 * k;
            let Some((id, pessoa)) = cpv8_pessoa_da_vaga(vaga) else { continue };
            if pessoa == 0 {
                return o(this, jogo, partida);
            }
            reg.push(((cpv8_d(this) as u32).wrapping_add((id as u32).wrapping_mul(31)).wrapping_add(0xa) as usize, 4));
            if *((vaga - 0x11) as *const u8) != 0 && cpv8_d(this + 0x28) != 0 && cpv8_tem_batida(this, id, pessoa) {
                batida = true;
            }
        }
    }
    if batida {
        crate::comparacao::ramo("efeitos_fisicos_da_partida com batida");
        return o(this, jogo, partida);
    }
    if cpv8_d(this + 0x28) != 0 {
        reg.push((cpv8_d(this + 0x28) as usize + 0xa, 4)); // o cursor da lista das batidas
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo, partida), || crate::aleatorio::sem_contar(|| efeitos_fisicos_da_partida(this, jogo, partida)));
    PLACAR.registrar(igual, || format!("efeitos_fisicos_da_partida({:x}, {:x}) {:x?} x {:x?}", jogo, partida, mo, mr));
}

/// { 0x62bf60 } (thiscall, ret 4, injury.cpp): `Injury::find_match_injury(pessoa)`: o item da lista das batidas da
/// partida (+0x28; cursor +0xa recomeça de +2, 0x68d1b0) com o id da pessoa, ou 0 (sem lista também). Pelas telas do
/// pessoal (0x870650, 0x885820).
unsafe extern "thiscall" fn batida_do_jogador(this: usize, pessoa: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let lista = cpv8_d(this + 0x28) as usize;
    if lista == 0 {
        return 0;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let mut e = proximo(cpv8_d(this + 0x28) as usize);
    while e != 0 {
        if cpv8_d(e) == cpv8_d(pessoa) {
            return e;
        }
        e = proximo(cpv8_d(this + 0x28) as usize);
    }
    0
}

/// Comparador: lado a lado no cursor da lista e na devolução.
unsafe extern "thiscall" fn comparar_batida_do_jogador(this: usize, pessoa: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1255);
    let lista = cpv8_d(this + 0x28) as usize;
    let reg: &[(usize, usize)] = if lista == 0 { &[] } else { &[(lista + 0xa, 4)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg, || o(this, pessoa), || batida_do_jogador(this, pessoa));
    PLACAR.registrar(igual && r == n, || format!("batida_do_jogador({:x}) original={:x} {:x?} rust={:x} {:x?}", pessoa, r, mo, n, mr));
    r
}

/// { 0x631550 } (ret 4, injury.cpp; `this` não é lido): `Injury::is_player_on_holiday(pessoa)`: 1 se a lesão atual
/// da pessoa (0x62ed70 no [0xb63d28]) é de férias (tipo 0x51). Pela tela do clube (0x4849f0).
unsafe extern "thiscall" fn jogador_de_ferias(_this: usize, pessoa: usize) -> u8 {
    let atual: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let e = atual(cpv8_d(0xb63d28) as usize, pessoa);
    (e != 0 && *(e as *const u8) == 0x51) as u8
}

/// Comparador: a devolução (só o al).
unsafe extern "thiscall" fn comparar_jogador_de_ferias(this: usize, pessoa: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1256);
    let r = o(this, pessoa);
    let n = jogador_de_ferias(this, pessoa);
    PLACAR.registrar(r == n, || format!("jogador_de_ferias({:x}) original={} rust={}", pessoa, r, n));
    r
}

// ---------- 258a leva: o regen no clube: a vaga no elenco, o valor e o contrato do jovem; a aposentadoria pela lesão e a notícia da aposentadoria anunciada (player_regen.cpp) ----------

unsafe fn cpv9_d(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpv9_por(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

/// A primeira vaga livre (dword 0) das 50 do elenco do clube (+0xd7); 0x32 sem vaga.
unsafe fn cpv9_vaga_livre(clube: usize) -> usize {
    (0..0x32usize).find(|&k| cpv9_d(clube + 0xd7 + 4 * k) == 0).unwrap_or(0x32)
}

/// { 0x7a6f10 } (stdcall, ret 8, player_regen.cpp, quadro SEH só das datas locais): `player_regen_sign_for_club(pessoa,
/// clube)`, pela PLAYER_REGEN::regenerate_as_youth_player (0x7abac0) em todo regen jovem: na primeira vaga livre das 50
/// do elenco (+0xd7; sem vaga, 0) a pessoa, o clube dela (+0x39) e +0x4e = 0; o valor (+0x52) pela 0x4d0c60 (clube, 1,
/// −1, 0), o elenco (0x540980 com 1), uma vez em quatro (sorteio(4) = 0) a 0x547d90 (pessoa, clube, 0); clube com +0x64
/// = 3: valor até 5000. O fim do contrato (+0x46): abaixo de 19 anos (+0x18), o ano de hoje + 2; senão + sorteio(6) +
/// 1; no mês do fim da temporada do clube (0x4d45f0) e no dia sorteio(28) + 1 (0x549f70). O contrato (0x4c7950 com o
/// valor); clube humano (0x600a50 com 1, 1): para os reservas (0x4cc0f0) e os bits +0x4f (0x60 em cima) e +0x35 (1 em
/// baixo) do contrato. Com contrato: o salário padrão (0x8626e0), +0x3a = 0 e, fora do humano, o fim (+0x2f) até o ano de
/// hoje + 5 (com o dia +0x2d = 0). No fim os números do elenco ([0xdd7ef0], 0x85add0 e 0x85a470). Devolve 1.
unsafe extern "stdcall" fn contratar_o_regen(pessoa: usize, clube: usize) -> u8 {
    let k = cpv9_vaga_livre(clube);
    if k == 0x32 {
        return 0;
    }
    let gerente = cpv9_d(0xae1050) as usize;
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
    let elenco: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x540980usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let f547d90: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547d90usize);
    let mes_do_fim: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x4d45f0usize);
    let data_dma: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> *mut u8 = core::mem::transmute(0x549f70usize);
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, *const u8) -> *mut u8 = core::mem::transmute(0x417c10usize);
    let contrato_novo: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x4c7950usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let reservas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x4cc0f0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let salario: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8626e0usize);
    let usa_numeros: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x85add0usize);
    let dar_numero: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x85a470usize);
    crate::comparacao::ramo("cpv_9 regen contratado");
    cpv9_por(clube + 0xd7 + 4 * k, pessoa as i32);
    cpv9_por(pessoa + 0x39, clube as i32);
    cpv9_por(pessoa + 0x4e, 0);
    let v = valor(gerente, pessoa, clube, 1, -1, 0);
    cpv9_por(pessoa + 0x52, v); // cpv9: o valor do regen
    elenco(pessoa, 1);
    if sorteio(4) == 0 { // cpv9: uma vez em quatro
        crate::comparacao::ramo("cpv_9 uma em quatro");
        f547d90(pessoa, clube, 0);
    }
    if *((clube + 0x64) as *const u8) == 3 && cpv9_d(pessoa + 0x52) > 0x1388 {
        cpv9_por(pessoa + 0x52, 0x1388);
    }
    // ponytail: a original grava o ano na palavra de baixo do argumento `clube` e empurra o dword; a 0x549f70 lê 16 bits
    let ano = *(0xae2c92 as *const u16) as u32;
    let (mut d, mut ret) = ([0u8; 8], [0u8; 8]);
    let dia_e_ano = if *((pessoa + 0x18) as *const i8) < 0x13 {
        crate::comparacao::ramo("cpv_9 contrato do jovem");
        let a = ano.wrapping_add(2); // cpv9: o jovem por dois anos
        let mes = mes_do_fim(clube);
        (sorteio(0x1c), mes, a)
    } else {
        let a = (sorteio(6) as u32).wrapping_add(ano).wrapping_add(1);
        let mes = mes_do_fim(clube);
        (sorteio(0x1c), mes, a)
    };
    let dia = (dia_e_ano.0 as u8).wrapping_add(1) as i8 as i16 as u16 as u32;
    let p = data_dma(d.as_mut_ptr(), dia, dia_e_ano.1, dia_e_ano.2, 0xffff_ffff);
    copia(pessoa + 0x46, ret.as_mut_ptr(), p);
    contrato_novo(gerente, pessoa, cpv9_d(pessoa + 0x52) as u32);
    if humano(clube, 1, 1, 0, 0) != 0 {
        reservas(cpv9_d(0xae1050) as usize, pessoa, clube);
        let c = contrato(cpv9_d(0xae1050) as usize, pessoa);
        *((c + 0x4f) as *mut u8) = *((c + 0x4f) as *const u8) & 0xf | 0x60;
        let c = contrato(cpv9_d(0xae1050) as usize, pessoa);
        *((c + 0x35) as *mut u8) = *((c + 0x35) as *const u8) & 0xc1 | 1;
    }
    let c = contrato(cpv9_d(0xae1050) as usize, pessoa);
    if c != 0 {
        salario(c, pessoa);
        *((c + 0x3a) as *mut u8) = 0;
        if humano(clube, 1, 1, 0, 0) == 0 {
            let a = (*(0xae2c92 as *const u16)).wrapping_add(5);
            if core::ptr::read_unaligned((c + 0x2f) as *const u16) > a {
                core::ptr::write_unaligned((c + 0x2f) as *mut u16, a);
                core::ptr::write_unaligned((c + 0x2d) as *mut u16, 0);
            }
        }
    }
    if usa_numeros(cpv9_d(0xdd7ef0) as usize, clube, 0) != 0 {
        dar_numero(cpv9_d(0xdd7ef0) as usize, clube, 0);
    }
    1
}

/// Comparador: o contrato novo aloca e o valor sorteia: só a original, conferindo o plano (a vaga, o clube da pessoa, a
/// devolução e o +0x3a do contrato).
unsafe extern "stdcall" fn comparar_contratar_o_regen(pessoa: usize, clube: usize) -> u8 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize) -> u8>(1257);
    let k = cpv9_vaga_livre(clube);
    let r = o(pessoa, clube);
    if k == 0x32 {
        PLACAR.registrar(r == 0, || format!("contratar_o_regen({:x}, {:x}) sem vaga original={}", pessoa, clube, r));
        return r;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpv9_d(0xae1050) as usize, pessoa);
    let ok = r == 1 && cpv9_d(clube + 0xd7 + 4 * k) == pessoa as i32 && cpv9_d(pessoa + 0x39) == clube as i32 && (c == 0 || *((c + 0x3a) as *const u8) == 0);
    PLACAR.registrar(ok, || format!("contratar_o_regen({:x}, {:x}) vaga {} original={}", pessoa, clube, k, r));
    r
}

/// { 0x7a7f20 } (thiscall, ret 8, player_regen.cpp, quadro SEH só das datas locais):
/// `PLAYER_REGEN::schedule_injury_retirement(pessoa, dias)`, pela lesão nova (0x62d250) que encerra a carreira: tira a
/// aposentadoria pendente da pessoa (a primeira com o id dela na tabela de 26 bytes [this+4], conta +8; 0x7a8690) e
/// acrescenta (0x7a8510) a nova: id, id do clube (+0x39, ou −1), a data de hoje + sorteio(dias / 2) + 1 (0x54c5b0) em
/// +8 e de novo em +0x10, o motivo 8 (+0x18) e 0 (+0x19).
unsafe extern "thiscall" fn aposentar_pela_lesao(this: usize, pessoa: usize, dias: i16) {
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7a8690usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> *mut u8 = core::mem::transmute(0x54c5b0usize);
    let copia: unsafe extern "thiscall" fn(*mut u8, *mut u8, *const u8) -> *mut u8 = core::mem::transmute(0x417c10usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x7a8510usize);
    let n = cpv9_d(this + 8);
    if pessoa != 0 && n > 0 {
        let base = cpv9_d(this + 4) as usize;
        let id = cpv9_d(pessoa);
        if let Some(i) = (0..n as usize).find(|&i| cpv9_d(base + 0x1a * i) == id) {
            crate::comparacao::ramo("cpv_9 tira a aposentadoria pendente");
            tirar(this, base + 0x1a * i);
        }
    }
    let mut r = [0u8; 0x1a];
    let clube = cpv9_d(pessoa + 0x39) as usize;
    let rp = r.as_mut_ptr();
    core::ptr::write_unaligned(rp as *mut i32, cpv9_d(pessoa));
    core::ptr::write_unaligned(rp.add(4) as *mut i32, if clube == 0 { -1 } else { cpv9_d(clube) });
    let q = sorteio(dias as i32 / 2);
    let (mut t, mut t2) = ([0u8; 8], [0u8; 8]);
    let d = mais(0xae2c90, t.as_mut_ptr(), q.wrapping_add(1) as u32);
    copia(rp.add(8), t2.as_mut_ptr(), d);
    copia(rp.add(0x10), t2.as_mut_ptr(), rp.add(8));
    r[0x18] = 8;
    r[0x19] = 0;
    acrescentar(this, r.as_ptr());
}

/// Comparador: lado a lado no estado do sorteio, na tabela (ponteiro, conta e capacidade, e as entradas até a
/// capacidade). Com a tabela cheia (conta = capacidade − 1, a 0x7a8510 realoca): só a original.
unsafe extern "thiscall" fn comparar_aposentar_pela_lesao(this: usize, pessoa: usize, dias: i16) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i16)>(1258);
    let (n, cap) = (cpv9_d(this + 8), cpv9_d(this + 0xc));
    if pessoa == 0 || n >= cap.wrapping_sub(1) || cap <= 0 {
        return o(this, pessoa, dias);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((this + 4, 12));
    reg.push((cpv9_d(this + 4) as usize, cap as usize * 0x1a));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, dias), || crate::aleatorio::sem_contar(|| aposentar_pela_lesao(this, pessoa, dias)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("aposentar_pela_lesao({:x}, {}) diferencas em {:x?}", pessoa, dias, dif)
    });
}

/// { 0x7aa340 } (stdcall, ret 8, player_regen.cpp, quadro SEH sem destrutor): `player_regen_post_retirement_announcement_news(
/// pessoa, motivo)`, pela PLAYER_REGEN::process_retirements (0x7a59e0): fora do jogador (+0x61) com motivo +0x3d 8 ou
/// 5, a notícia 0xbbb numa local (0x77bbe0 com 0) com os dados 0 = id, 1 a 3 = os nomes (+0x33 de +4, +8 e +0xc), 4 =
/// motivo, 5 = o id do clube do contrato (0x4cf5a0, guardado em [0xae1080]; clube do registro de 0x245 bytes em
/// [0xae23b4] pelo +4; contrato de tipo +0x35 & 0x3f = 5 ou sem clube: −1) e 6 = a idade (+0x18); vai para os técnicos
/// dos até 30 clubes interessados ([0xde1f54], 0x8d1da0 com 10).
unsafe extern "stdcall" fn noticia_da_aposentadoria_anunciada(pessoa: usize, motivo: i8) {
    if pessoa == 0 {
        return;
    }
    if cpv9_d(pessoa + 0x61) != 0 && matches!(*((pessoa + 0x3d) as *const u8), 8 | 5) {
        return;
    }
    crate::comparacao::ramo("noticia_da_aposentadoria_anunciada");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let interessados: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1da0usize);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbbb, 0);
    gravar(n, 0, cpv9_d(pessoa) as u32);
    for (k, o) in [(1i8, 4usize), (2, 8), (3, 0xc)] {
        gravar(n, k, cpv9_d(cpv9_d(pessoa + o) as usize + 0x33) as u32);
    }
    gravar(n, 4, motivo as i32 as u32);
    let c = contrato(cpv9_d(0xae1050) as usize, pessoa);
    cpv9_por(0xae1080, c as i32);
    let mut clube = -1i32;
    if c != 0 && *((c + 0x35) as *const u8) & 0x3f != 5 {
        let idx = cpv9_d(c + 4) as u32;
        let registro = (cpv9_d(0xae23b4) as u32).wrapping_add(idx.wrapping_mul(0x245)) as usize;
        if registro != 0 {
            clube = cpv9_d(registro);
        }
    }
    gravar(n, 5, clube as u32);
    gravar(n, 6, *((pessoa + 0x18) as *const i8) as i32 as u32);
    let mut lista = [0usize; 0x1e];
    let q = if cpv9_d(0xde1f54) != 0 { interessados(cpv9_d(0xde1f54) as usize, pessoa, 0xa, lista.as_mut_ptr(), 0x1e) } else { 0 };
    for i in 0..(q as i8).max(0) as usize {
        ao_tecnico(cpv9_d(0xb7431c) as usize, n, lista[i]);
    }
}

/// Comparador: a notícia vai para as caixas dos técnicos: só a original (a prova é a carreira e o modo Rust).
unsafe extern "stdcall" fn comparar_noticia_da_aposentadoria_anunciada(pessoa: usize, motivo: i8) {
    orig::<unsafe extern "stdcall" fn(usize, i8)>(1259)(pessoa, motivo)
}

// ---------- 258a leva: o fim da lesão do jogador, a notícia do pedido de desculpas e a caixa de entrada esvaziada (injury.cpp, media.cpp, news.cpp) ----------

unsafe fn cpv10_d(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// O registro de 31 bytes da pessoa `id` em [lesoes] (o físico e a lesão atual em +0xe).
unsafe fn cpv10_registro(lesoes: usize, id: i32) -> usize {
    (cpv10_d(lesoes) as u32).wrapping_add((id as u32).wrapping_mul(31)) as usize
}

/// { 0x62e700 } (thiscall, ret 8, injury.cpp): `Injury::end_player_injury(pessoa, noticia)`, pelo físico do dia
/// (0x62c680, com notícia), pelas batidas da partida (0x62c080, sem) e pelo recomeço do físico (0x62bfa0, sem): com a
/// lesão atual (+0xe do registro, índice na tabela de 0x30 bytes em [this+4]; −1 nenhuma): fora das férias (tipo
/// 0x51), o histórico ([this+0xc], 0x58e340 com +0x14 e +0x18 do registro); com `noticia` e o clube da pessoa (+0x39)
/// humano (0x600a50), a notícia da volta (0x630ee0 com 1 e o tipo). O tipo vira 0xff e o +0xe −1. Sem pessoa: a caixa
/// de erro (linha 0xc87).
unsafe extern "thiscall" fn fim_da_lesao(this: usize, pessoa: usize, noticia: u32) {
    if pessoa == 0 {
        aviso_em(0x9e7f28, 0xc87);
        return;
    }
    let id = cpv10_d(pessoa);
    if id >= cpv10_d(0xae2364) { // cpv10: o id da pessoa
        return;
    }
    if id >= cpv10_d(0xae2364) / 2 {
        crate::comparacao::ramo("cpv_10 metade de cima");
    }
    let r = cpv10_registro(this, id);
    let i = cpv10_d(r + 0xe);
    if i == -1 { // cpv10: a lesão atual
        return;
    }
    if i == 0 {
        crate::comparacao::ramo("cpv_10 entrada zero");
    }
    let e = (cpv10_d(this + 4) as u32).wrapping_add((i as u32).wrapping_mul(0x30)) as usize;
    if *(e as *const u8) != 0x51 {
        let historico: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(0x58e340usize);
        historico(cpv10_d(this + 0xc) as usize, r + 0x14, r + 0x18, e as *const u8);
    }
    let clube = cpv10_d(pessoa + 0x39) as usize;
    if noticia != 0 && clube != 0 {
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
        if humano(clube, 0, 1, 0, 0) != 0 {
            let volta: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x630ee0usize);
            // ponytail: a original empurra o ecx inteiro com o tipo no byte de baixo; a 0x630ee0 lê o byte
            volta(this, pessoa, 1, *(e as *const u8) as u32, 0, 0);
        }
    }
    crate::comparacao::ramo("cpv_10 fim da lesao");
    *(e as *mut u8) = 0xff; // cpv10: a entrada fica livre
    core::ptr::write_unaligned((cpv10_registro(this, cpv10_d(pessoa)) + 0xe) as *mut i32, -1); // cpv10: sem lesão atual
}

/// Comparador: lado a lado na entrada (tipo) e no +0xe do registro quando nada sai da rotina (férias e sem notícia);
/// com o histórico (arquivo) ou a notícia: só a original, conferindo o plano (tipo 0xff e +0xe −1). Sem pessoa: a
/// original (caixa).
unsafe extern "thiscall" fn comparar_fim_da_lesao(this: usize, pessoa: usize, noticia: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1260);
    if pessoa == 0 || cpv10_d(pessoa) >= cpv10_d(0xae2364) {
        return o(this, pessoa, noticia);
    }
    let r = cpv10_registro(this, cpv10_d(pessoa));
    let i = cpv10_d(r + 0xe);
    if i == -1 {
        return o(this, pessoa, noticia);
    }
    let e = (cpv10_d(this + 4) as u32).wrapping_add((i as u32).wrapping_mul(0x30)) as usize;
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let clube = cpv10_d(pessoa + 0x39) as usize;
    let fora = *(e as *const u8) != 0x51 || (noticia != 0 && clube != 0 && humano(clube, 0, 1, 0, 0) != 0);
    if fora {
        o(this, pessoa, noticia);
        let ok = *(e as *const u8) == 0xff && cpv10_d(cpv10_registro(this, cpv10_d(pessoa)) + 0xe) == -1;
        PLACAR.registrar(ok, || format!("fim_da_lesao({:x}, {}) plano: tipo {:x} e +0xe {}", pessoa, noticia, *(e as *const u8), cpv10_d(cpv10_registro(this, cpv10_d(pessoa)) + 0xe)));
        return;
    }
    let reg = [(e, 1), (r + 0xe, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, noticia), || fim_da_lesao(this, pessoa, noticia));
    PLACAR.registrar(igual, || format!("fim_da_lesao({:x}, {}) {:x?} x {:x?}", pessoa, noticia, mo, mr));
}

/// { 0x72bbf0 } (stdcall, ret 0xc, media.cpp, quadro SEH sem destrutor): `media_queue_player_apology_news(jogador,
/// clube, historia)`, pelos efeitos da história (0x736c00) e pela resposta à história do jogador (0x741020): com o
/// jogador, a ficha (+0x61) e o clube, a notícia 0xbdb numa local (0x77bbe0 com 0) com os dados 0 = id, 1 a 3 = os
/// nomes (+0x33 de +4, +8 e +0xc; sem o terceiro, −1), 4 = id do clube, 5 = a história, 6 = 0 e 7 = a variante:
/// sorteio(4) nas histórias 0, 2 a 5, 13, 14, 17 e 35 (tabela 0x72bd2c), senão 0; vai para o técnico do clube
/// (0x7861e0).
unsafe extern "stdcall" fn noticia_do_pedido_de_desculpas(jogador: usize, clube: usize, historia: i16) {
    if jogador == 0 || cpv10_d(jogador + 0x61) == 0 || clube == 0 {
        return;
    }
    crate::comparacao::ramo("noticia_do_pedido_de_desculpas");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbdb, 0);
    gravar(n, 0, cpv10_d(jogador) as u32);
    gravar(n, 1, cpv10_d(cpv10_d(jogador + 4) as usize + 0x33) as u32);
    gravar(n, 2, cpv10_d(cpv10_d(jogador + 8) as usize + 0x33) as u32);
    let terceiro = cpv10_d(jogador + 0xc) as usize;
    gravar(n, 3, if terceiro == 0 { u32::MAX } else { cpv10_d(terceiro + 0x33) as u32 });
    gravar(n, 4, cpv10_d(clube) as u32);
    let h = historia as i32;
    gravar(n, 5, h as u32);
    gravar(n, 6, 0);
    let v = if (h as u32) <= 0x23 && *((0x72bd2c + h as usize) as *const u8) == 0 { sorteio(4) } else { 0 };
    gravar(n, 7, v as u32);
    ao_tecnico(cpv10_d(0xb7431c) as usize, n, clube);
}

/// Comparador: a notícia sorteia e vai para a caixa do técnico: só a original (a prova é a carreira e o modo Rust).
unsafe extern "stdcall" fn comparar_noticia_do_pedido_de_desculpas(jogador: usize, clube: usize, historia: i16) {
    orig::<unsafe extern "stdcall" fn(usize, usize, i16)>(1261)(jogador, clube, historia)
}

/// { 0x787630 } (thiscall, ret 4, news.cpp): `News::clear_inbox(tecnico)`, pela 0x6b28b0 e pela carga do jogo
/// (0x82a9c0): as respostas padrão das notícias do clube (+0x39) e da seleção (+0x24) do técnico (0x7874b0); depois a
/// caixa do humano (registro de 12 bytes em [this+8], pelo id − [0xae2364] + 16): +4 = +0 e +8 = 0; com a primeira
/// notícia (+0, −1 nenhuma), as 100 notícias de 0xdf bytes a partir dela ([this]) com +0xd6 = 0 e +0xda = 1. Sem técnico
/// ou com índice negativo: a caixa de erro (linhas 0x24c1 e 0x24cc).
unsafe extern "thiscall" fn esvaziar_caixa_de_entrada(this: usize, tecnico: usize) {
    if tecnico == 0 {
        aviso_em(0xa693ec, 0x24c1);
        return;
    }
    let h = cpv10_d(tecnico).wrapping_sub(cpv10_d(0xae2364)).wrapping_add(0x10);
    if h < 0 {
        aviso_em(0xa693ec, 0x24cc);
        return;
    }
    let caixa = (cpv10_d(this + 8) as u32).wrapping_add((h as u32).wrapping_mul(12)) as usize;
    let responder: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7874b0usize);
    for o in [0x39usize, 0x24] {
        let c = cpv10_d(tecnico + o) as usize;
        if c != 0 {
            responder(this, tecnico, c);
        }
    }
    let primeira = cpv10_d(caixa);
    core::ptr::write_unaligned((caixa + 8) as *mut i32, 0);
    core::ptr::write_unaligned((caixa + 4) as *mut i32, primeira);
    if primeira != -1 {
        crate::comparacao::ramo("esvaziar_caixa_de_entrada");
        for k in 0..0x64u32 {
            let n = (cpv10_d(this) as u32).wrapping_add(k.wrapping_add(cpv10_d(caixa) as u32).wrapping_mul(0xdf)) as usize;
            core::ptr::write_unaligned((n + 0xd6) as *mut i32, 0);
            core::ptr::write_unaligned((n + 0xda) as *mut i32, 1);
        }
    }
}

/// As regiões que a 0x787630 grava (a caixa e, com a primeira notícia, +0xd6..+0xde das 100 notícias).
unsafe fn cpv10_regioes_da_caixa(this: usize, caixa: usize) -> Vec<(usize, usize)> {
    let mut reg = vec![(caixa, 12)];
    let primeira = cpv10_d(caixa);
    if primeira != -1 {
        for k in 0..0x64u32 {
            reg.push(((cpv10_d(this) as u32).wrapping_add(k.wrapping_add(primeira as u32).wrapping_mul(0xdf)) as usize + 0xd6, 8));
        }
    }
    reg
}

/// Comparador: lado a lado na caixa e nas 100 notícias; com resposta padrão a dar (clube ou seleção), só a original,
/// conferindo o plano depois dela (+4 = +0, +8 = 0 e os dois campos das 100 notícias). Erros: a original.
unsafe extern "thiscall" fn comparar_esvaziar_caixa_de_entrada(this: usize, tecnico: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1262);
    if tecnico == 0 {
        return o(this, tecnico);
    }
    let h = cpv10_d(tecnico).wrapping_sub(cpv10_d(0xae2364)).wrapping_add(0x10);
    if h < 0 {
        return o(this, tecnico);
    }
    let caixa = (cpv10_d(this + 8) as u32).wrapping_add((h as u32).wrapping_mul(12)) as usize;
    if cpv10_d(tecnico + 0x39) != 0 || cpv10_d(tecnico + 0x24) != 0 {
        o(this, tecnico);
        let primeira = cpv10_d(caixa);
        let noticias = cpv10_regioes_da_caixa(this, caixa);
        let ok = cpv10_d(caixa + 4) == primeira && cpv10_d(caixa + 8) == 0
            && noticias[1..].iter().all(|&(n, _)| cpv10_d(n) == 0 && cpv10_d(n + 4) == 1);
        PLACAR.registrar(ok, || format!("esvaziar_caixa_de_entrada({:x}) plano: caixa {:x?}", tecnico, (primeira, cpv10_d(caixa + 4), cpv10_d(caixa + 8))));
        return;
    }
    let reg = cpv10_regioes_da_caixa(this, caixa);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tecnico), || esvaziar_caixa_de_entrada(this, tecnico));
    PLACAR.registrar(igual, || format!("esvaziar_caixa_de_entrada({:x}) {:x?} x {:x?}", tecnico, mo, mr));
}

// ---------- 258a leva: o convocado machucado: tirar da seleção, a notícia ao clube dele e o pedido de substituto ao técnico da seleção (national_teams.cpp) ----------

unsafe fn cpv11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv11_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro da nação no gerenciador das seleções: [this + 4] + 0x6e × id.
unsafe fn cpv11_registro(this: usize, nacao: usize) -> usize {
    cpv11_d(this + 4).wrapping_add((cpv11_d(nacao) as i32).wrapping_mul(0x6e) as usize)
}

/// O tipo de convocação do jogador no clube da seleção `clube`: 2 no segundo conjunto de seleções (0x5409b0), senão o
/// bit 3 das marcas de elenco (+0x5e).
unsafe fn cpv11_tipo(clube: usize, p: usize) -> u8 {
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    if segunda(clube) != 0 {
        2
    } else {
        (cpv11_b(p + 0x5e) >> 3) & 1 // cpv11: o tipo pelas marcas
    }
}

/// Os dados 0..3 da notícia: o id do jogador e os de +4, +8 e +0xc dele (+0x33 de cada).
unsafe fn cpv11_quem(n: usize, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    gravar(n, 0, cpv11_d(p) as u32);
    for k in 0..3usize {
        gravar(n, 1 + k as i8, cpv11_d(cpv11_d(p + 4 + 4 * k) + 0x33) as u32);
    }
}

/// O dado 6 da notícia: a gravidade da lesão do jogador (0x62ec00 sobre o tipo do registro 0x62ed70), ou 0.
unsafe fn cpv11_gravidade(n: usize, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let gravidade: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x62ec00usize);
    let v = if lesao(cpv11_d(0xb63d28), p) != 0 {
        let t = cpv11_b(lesao(cpv11_d(0xb63d28), p)) as i8;
        gravidade(cpv11_d(0xb63d28), t) as i8 as i32
    } else {
        0
    };
    gravar(n, 6, v as u32);
}

/// O clube do contrato do jogador (0x4cf5a0, guardado em [0xae1080]; registros de 0x245 bytes), ou 0.
unsafe fn cpv11_clube_do_contrato(p: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpv11_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    if c == 0 {
        0
    } else {
        cpv11_d(0xae23b4).wrapping_add(cpv11_d(c + 4).wrapping_mul(0x245))
    }
}

/// { 0x775cd0 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::withdraw_injured_player(p, dias)`: com 14 dias
/// de lesão ou mais, o convocado (+0x28 = 0xb) do clube da seleção (+0x24) sai: com a lesão do tipo 0x51 só sai da
/// tática (0x896c60, duas vezes) e do elenco (0x7762f0). Senão, só antes da data do tipo (+0x3a do registro do tipo:
/// [this+4] + 0x6e × id da nação + 20 × tipo; 0x456740 sobre [0xae2c90]) e se a lista [this] não tem o clube com uma
/// data já passada: a notícia ao clube dele (0x76f7d0), a saída, a vaga do tipo (+0x45) sobe 1, o nível (0x76d810) e,
/// se o técnico da seleção é humano e decide (0x600a50, 0x5fd6b0), o pedido de substituto (0x7701f0); senão o
/// substituto (0x775f90) e as convocações repetidas apagadas (0x76c060). Sem clube: a original (caixa e 0).
unsafe extern "thiscall" fn tirar_convocado_machucado(this: usize, p: usize, dias_arg: u32) {
    let dias = dias_arg as u16 as i16;
    if dias < 0xe { // cpv11: o limite de dias
        return;
    }
    let clube = cpv11_d(p + 0x24);
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1263)(this, p, dias_arg);
    }
    if cpv11_b(p + 0x28) != 0xb {
        return;
    }
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let marcar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x896c60usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x7762f0usize);
    let maior: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x76f7d0usize);
    let nivel_de: unsafe extern "stdcall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x76d810usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8 = core::mem::transmute(0x5fd6b0usize);
    let pedido: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x7701f0usize);
    let substituto: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize) = core::mem::transmute(0x775f90usize);
    let repetidos: unsafe extern "stdcall" fn(usize) = core::mem::transmute(0x76c060usize);
    let nacao = cpv11_d(clube + 0x53);
    let rec = cpv11_registro(this, nacao);
    // ponytail: a original empurra o dword da local com o tipo só no byte de baixo (os outros são sobra de pilha); as
    // chamadas leem só o byte
    let tipo = cpv11_tipo(clube, p);
    let inj = lesao(cpv11_d(0xb63d28), p);
    if inj != 0 && cpv11_b(inj) == 0x51 {
        crate::comparacao::ramo("cpv_11 doença");
        marcar(cpv11_d(0xde1d08), clube, p, 1, 1);
        marcar(cpv11_d(0xde1d08), clube, p, 0, 1);
        tirar(this, nacao, tipo as u32, p);
        return;
    }
    let vaga = rec.wrapping_add((tipo as i8 as i32 * 20) as usize);
    if maior(vaga + 0x3a, cpv11_d(0xae2c90) as u32, cpv11_d(0xae2c94) as u32) == 0 {
        return;
    }
    let lista = cpv11_d(this);
    if lista != 0 {
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, cpv11_d(lista + 2) as u32);
        let mut e = proximo(cpv11_d(this));
        while e != 0 {
            if cpv11_d(e) == cpv11_d(clube) && maior(0xae2c90, cpv11_d(e + 8) as u32, cpv11_d(e + 0xc) as u32) != 0 {
                return;
            }
            e = proximo(cpv11_d(this));
        }
    }
    if dias < 0x1c {
        crate::comparacao::ramo("cpv_11 lesão de 14 a 27 dias");
    }
    noticia(this, p, nacao, tipo as u32);
    marcar(cpv11_d(0xde1d08), clube, p, 1, 1);
    marcar(cpv11_d(0xde1d08), clube, p, 0, 1);
    if tipo != 2 {
        crate::comparacao::ramo("cpv_11 tipo pelas marcas");
    }
    tirar(this, nacao, tipo as u32, p);
    let a = (vaga + 0x45) as *mut u8;
    *a = (*a).wrapping_add(1);
    // a base vai no eax da original: o endereço da vaga com o byte de baixo trocado por (tipo == 2)
    let nivel = nivel_de(p, rec, (vaga as u32 & !0xff) | (tipo == 2) as u32);
    // a local do nível é a do registro com o byte de baixo trocado (só o byte é lido)
    let nivel = (rec as u32 & !0xff) | nivel as u32;
    let pede = 'decide: {
        if humano(clube, 0, 1, 0, 0) == 0 {
            if cpv11_d(clube + 0xcf) != 0 || humano(principal(nacao), 0, 1, 0, 0) == 0 {
                break 'decide false;
            }
        }
        let t = cpv11_d(principal(nacao) + 0xcf);
        tipo == 0 || t9a06(cpv11_d(0xb63c98), t, core::ptr::null_mut()) != 0
    };
    if pede {
        crate::comparacao::ramo("cpv_11 pedido de substituto");
        pedido(this, p, nacao, tipo as u32, nivel);
        return;
    }
    crate::comparacao::ramo("cpv_11 substituto");
    substituto(this, nacao, tipo as u32, nivel, p); // cpv11: o substituto
    repetidos(nacao);
}

/// O caminho que a 0x775cd0 vai tomar, só lendo (a lista [this] percorrida pelos nós, sem mexer no cursor): 0 nada, 1
/// doença (só sai), 2 sai com notícia. O clube (+0x24) já conferido.
unsafe fn cpv11_plano(this: usize, p: usize, dias: u32) -> u8 {
    if (dias as u16 as i16) < 0xe || cpv11_b(p + 0x28) != 0xb {
        return 0;
    }
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let maior: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let clube = cpv11_d(p + 0x24);
    let tipo = cpv11_tipo(clube, p);
    let inj = lesao(cpv11_d(0xb63d28), p);
    if inj != 0 && cpv11_b(inj) == 0x51 {
        return 1;
    }
    let vaga = cpv11_registro(this, cpv11_d(clube + 0x53)).wrapping_add((tipo as i8 as i32 * 20) as usize);
    if maior(vaga + 0x3a, cpv11_d(0xae2c90) as u32, cpv11_d(0xae2c94) as u32) == 0 {
        return 0;
    }
    let lista = cpv11_d(this);
    if lista != 0 {
        let mut no = cpv11_d(lista + 2);
        while no != 0 {
            let e = cpv11_d(no);
            if e == 0 {
                break;
            }
            if cpv11_d(e) == cpv11_d(clube) && maior(0xae2c90, cpv11_d(e + 8) as u32, cpv11_d(e + 0xc) as u32) != 0 {
                return 0;
            }
            no = cpv11_d(no + 6);
        }
    }
    2
}

/// { 0x76f7d0 } (thiscall, ret 0xc, national_teams.cpp, quadro SEH sem destrutor; `this` não é lido):
/// `national_teams_post_player_withdrawn_news(p, nacao, tipo)`: se o clube do jogador (+0x39) ou o do contrato
/// (0x4cf5a0) é de técnico humano (0x7875e0), monta a notícia 0x20 numa local (0x77bbe0 com 0): dados 0..3 o jogador
/// e os de +4/+8/+0xc, 4 o id da seleção (principal 0x53b3d0 nos tipos 0 e 1, senão 0x53b4f0), 5 = (tipo == 1), 6 a
/// gravidade da lesão; entrega ao clube +0x39 e, se for outro, ao do contrato (0x7861e0).
unsafe extern "thiscall" fn noticia_da_saida_do_convocado(_this: usize, p: usize, nacao: usize, tipo: u32) {
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let selecao_b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    if cpv11_d(p + 0x39) == 0 || humano(cpv11_d(p + 0x39)) == 0 {
        if cpv11_clube_do_contrato(p) == 0 {
            return;
        }
        if humano(cpv11_clube_do_contrato(p)) == 0 {
            return;
        }
    }
    crate::comparacao::ramo("cpv_11 notícia da saída");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x20, 0);
    cpv11_quem(n, p);
    let (sel, um) = match tipo as u8 {
        0 => (principal(nacao), 0),
        1 => (principal(nacao), 1),
        _ => (selecao_b(nacao), 0),
    };
    gravar(n, 4, cpv11_d(sel) as u32);
    gravar(n, 5, um);
    cpv11_gravidade(n, p);
    let noticias = cpv11_d(0xb7431c);
    if cpv11_d(p + 0x39) != 0 {
        entregar(noticias, n, cpv11_d(p + 0x39));
    }
    if cpv11_clube_do_contrato(p) != cpv11_d(p + 0x39) {
        entregar(noticias, n, cpv11_clube_do_contrato(p));
    }
}

/// { 0x7701f0 } (thiscall, ret 0x10, national_teams.cpp, quadro SEH sem destrutor; `this` não é lido):
/// `national_teams_post_replacement_needed_news(p, nacao, tipo, nivel)`: a notícia 0x22 ao técnico da seleção: dados
/// 0..3 o jogador, 4 o id do time (tipo 1: principal com marca 8 e 5 = 1; tipo 0: principal, senão o sub-21 0x53b4f0,
/// marca 4 e 5 = 0), 6 a gravidade da lesão, 7 = 0, 8 os do elenco com a marca (0x53c590), 9 o nível, 0xa o tipo e 0xb
/// o id da nação. Entrega ao time se o técnico dele é humano (0x600a50), senão ao principal se o dele for.
unsafe extern "thiscall" fn noticia_do_pedido_de_substituto(_this: usize, p: usize, nacao: usize, tipo: u32, nivel: u32) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let selecao_b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    crate::comparacao::ramo("cpv_11 notícia do pedido");
    let (time, um, marca) = match tipo as u8 {
        0 => (principal(nacao), 0u32, 4u32),
        1 => (principal(nacao), 1, 8),
        _ => (selecao_b(nacao), 0, 4),
    };
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x22, 0);
    cpv11_quem(n, p);
    gravar(n, 4, cpv11_d(time) as u32);
    gravar(n, 5, um);
    cpv11_gravidade(n, p);
    gravar(n, 7, 0);
    // ponytail: a marca vai num dword da local com só o byte de baixo gravado; a 0x53c590 lê só o byte
    gravar(n, 8, contar(time, marca, 0) as i8 as i32 as u32);
    gravar(n, 9, nivel as u8 as i8 as i32 as u32);
    gravar(n, 0xa, tipo as u8 as i8 as i32 as u32);
    gravar(n, 0xb, cpv11_d(nacao) as u32);
    let noticias = cpv11_d(0xb7431c);
    if humano(time, 0, 1, 0, 0) != 0 {
        entregar(noticias, n, time);
    } else if humano(principal(nacao), 0, 1, 0, 0) != 0 {
        entregar(noticias, n, principal(nacao));
    }
}

/// Comparador: a retirada chama a notícia (aloca), a saída da tática e o substituto (originais com efeitos): só a
/// original roda. O caminho é decidido antes, só lendo (cpv11_plano), e conferido depois: sem retirada, o clube da
/// seleção (+0x24) e a marca de convocado (+0x28) ficam; com retirada, o jogador que estava numa das 50 vagas do time
/// sai delas (+0x24 zerado pela 0x7762f0). Sem clube (a caixa): só a original.
unsafe extern "thiscall" fn comparar_tirar_convocado_machucado(this: usize, p: usize, dias: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1263);
    let clube = cpv11_d(p + 0x24);
    if (dias as u16 as i16) < 0xe || clube == 0 {
        return o(this, p, dias);
    }
    let plano = cpv11_plano(this, p, dias);
    let time = cpd2_time(cpv11_d(clube + 0x53), cpv11_tipo(clube, p));
    let estava = time != 0 && (0..50usize).any(|c| cpv11_d(time + 0xd7 + 4 * c) == p);
    let antes = (clube, cpv11_b(p + 0x28));
    o(this, p, dias);
    let depois = (cpv11_d(p + 0x24), cpv11_b(p + 0x28));
    let ok = match plano {
        0 => depois == antes,
        _ => !estava || depois.0 == 0,
    };
    PLACAR.registrar(ok, || format!("tirar_convocado_machucado({:x}, {}) plano {} estava {} antes {:x?} depois {:x?}", p, dias as u16 as i16, plano, estava, antes, depois));
}

/// Comparador: a notícia aloca e vai para a caixa de entrada: só a original.
unsafe extern "thiscall" fn comparar_noticia_da_saida_do_convocado(this: usize, p: usize, nacao: usize, tipo: u32) {
    crate::comparacao::ramo("noticia_da_saida_do_convocado");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1264)(this, p, nacao, tipo)
}

/// Comparador: a notícia aloca e vai para a caixa de entrada: só a original.
unsafe extern "thiscall" fn comparar_noticia_do_pedido_de_substituto(this: usize, p: usize, nacao: usize, tipo: u32, nivel: u32) {
    crate::comparacao::ramo("noticia_do_pedido_de_substituto");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(1265)(this, p, nacao, tipo, nivel)
}

// ---------- 258a leva: a convocação pode mudar agora?, o jogador está na convocação e o adversário marcado no amistoso (national_teams.cpp, friendly.cpp) ----------

unsafe fn cpv12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv12_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O nome do time (0x542100: time, marca de segundo time, destino de 0x47 bytes, 2, 1) e o byte de gramática (+0x52).
unsafe fn cpv12_nome_do_time(time: usize, segundo: u32, buf: &mut [u8; 0x48]) -> i32 {
    let nome: unsafe extern "C" fn(usize, u32, *mut u8, u32, u32, u32) -> u32 = core::mem::transmute(0x542100usize);
    nome(time, segundo, buf.as_mut_ptr(), 0x47, 2, 1);
    cpv12_b(time + 0x52) as i8 as i32
}

/// { 0x76db00 } (thiscall, ret 0x18, national_teams.cpp, quadro SEH com a lista local da busca de jogos):
/// `national_teams_can_change_squad(nacao, tipo, p, entrando, msg, tam)`: o jogador (se houver) aposentado da seleção
/// (0x547d30) ou de clube de outra nação não muda (0). O time do tipo (0 e 1 a principal, o 1 com a marca 8 e o segundo
/// time; senão o sub-21 0x53b4f0, marca 4). Entrando: elenco cheio (0x53c590 contra +0x42 do registro do tipo) ou
/// jogador machucado (0x62c400 = 2) não muda. Pode mudar (1): entrando com vagas abertas (+0x45), o tipo sem janela
/// (+0x44 = 0), a data do tipo (+0x3a) já passada ou, no próprio dia, um jogo do time hoje (0x5acee0 numa lista local);
/// nos tipos 1 e 2 com técnico humano que decide (0x600a50), também quando a lista [this] não tem o time com data
/// passada e ele não deixa (0x5fd6b0). Os motivos vão para `msg` (0x66f4e0). Sem nação, ou `msg` com menos de 0x32
/// bytes: a original (caixa e 1).
unsafe extern "thiscall" fn pode_mudar_a_convocacao(this: usize, nacao: usize, tipo: u32, p: usize, entrando: u32, msg: usize, tam: i32) -> u32 {
    if nacao == 0 || (msg != 0 && tam < 0x32) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, i32) -> u32>(1266)(this, nacao, tipo, p, entrando, msg, tam);
    }
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let selecao_b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let aposentado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let nome_do_jogador: unsafe extern "C" fn(usize, *mut u8, u32, u32) -> u32 = core::mem::transmute(0x53dba0usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8 = core::mem::transmute(0x5fd6b0usize);
    if p != 0 {
        if aposentado(p, principal(cpv12_d(p + 0x1a))) != 0 { // cpv12: aposentado
            crate::comparacao::ramo("cpv_12 aposentado");
            if msg != 0 {
                let mut nome = [0u8; 0x12c];
                nome_do_jogador(p, nome.as_mut_ptr(), 0x12c, 1);
                texto(msg, 0xa5b4c8, nome.as_ptr());
            }
            return 0;
        }
        let c = cpv12_d(p + 0x24);
        if c != 0 && cpv12_d(c + 0x53) != nacao {
            if msg != 0 {
                let mut nome = [0u8; 0x12c];
                nome_do_jogador(p, nome.as_mut_ptr(), 0x12c, 1);
                let outra = cpv12_d(cpv12_d(p + 0x24) + 0x53);
                let g = cpv12_b(outra + 0x52) as i8 as i32;
                texto(msg, 0xa5b460, g, g, nome.as_ptr(), outra + 0x38);
            }
            return 0;
        }
    }
    let s = tipo as u8;
    // a marca vai na local [esp+0x1c] só no byte de baixo (o resto é sobra de pilha); a 0x53c590 lê só o byte
    let (time, segundo, marca) = match s {
        0 => (principal(nacao), 0u32, 4u32),
        1 => (principal(nacao), 1, 8),
        _ => (selecao_b(nacao), 0, 4),
    };
    let mut tecnico = cpv12_d(principal(nacao) + 0xcf);
    if humano(principal(nacao), 0, 1, 0, 0) == 0 {
        tecnico = 0;
    }
    let rec = cpv12_d(this + 4).wrapping_add((cpv12_d(nacao) as i32).wrapping_mul(0x6e) as usize);
    let e = rec.wrapping_add((s as i8 as i32 * 20) as usize);
    if entrando != 0 {
        let teto = cpv12_b(e + 0x42) as i8;
        if (contar(time, marca, 0) as i8) >= teto {
            if msg != 0 {
                let mut buf = [0u8; 0x48];
                let g = cpv12_nome_do_time(time, segundo, &mut buf);
                texto(msg, 0xa5b3e0, g, g, teto as i32, buf.as_ptr());
            }
            return 0;
        }
        if estado(cpv12_d(0xb63d28), p) == 2 {
            if msg != 0 {
                texto(msg, 0xa5b3bc);
            }
            return 0;
        }
    }
    if cpv12_b(e + 0x45) != 0 && entrando != 0 {
        return 1;
    }
    if cpv12_b(e + 0x44) == 0 {
        crate::comparacao::ramo("cpv_12 sem janela");
        return 1; // cpv12: sem janela
    }
    let hoje = cpv12_d(0xae2c90) as u32;
    let (ano, dia) = ((hoje >> 16) as u16, hoje as u16 as i16);
    if s != 0 && tecnico != 0 {
        let mut passou = false;
        let lista = cpv12_d(this);
        if lista != 0 {
            core::ptr::write_unaligned((lista + 0xa) as *mut u32, cpv12_d(lista + 2) as u32);
            let mut it = proximo(cpv12_d(this));
            while it != 0 {
                if cpv12_d(it) == cpv12_d(time) {
                    let d = cpv12_d(it + 8) as u32;
                    let (a, dd) = ((d >> 16) as u16, d as u16 as i16);
                    if a < ano || (a == ano && dd < dia) {
                        passou = true;
                        break;
                    }
                }
                it = proximo(cpv12_d(this));
            }
        }
        if !passou && t9a06(cpv12_d(0xb63c98), tecnico, core::ptr::null_mut()) == 0 {
            return 1;
        }
    }
    let d = cpv12_d(e + 0x3a) as u32;
    let (a, dd) = ((d >> 16) as u16, d as u16 as i16);
    if a < ano || (a == ano && dd < dia) {
        crate::comparacao::ramo("cpv_12 data passada");
        return 1; // cpv12: data passada
    }
    if a == ano && dd as u16 == dia as u16 {
        crate::comparacao::ramo("cpv_12 jogo hoje");
        let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let buscar: unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, i32, i32, i32, u32, i32, u32, u32, u32, u32, usize, u32, i32, u32, u32) -> u16 =
            core::mem::transmute(0x5acee0usize);
        // as vagas da data que a busca lê (a palavra do ano, o dword do dia, o byte da fatia), como a original monta
        let mut v_ano: u32 = ano as u32;
        let mut v_dia: u32 = hoje;
        let mut fatia: u8 = 0;
        let mut l = [0u32; 4];
        let lp = l.as_mut_ptr() as usize;
        nova(lp);
        let achou = buscar(cpv12_d(0xae2a58), &mut v_ano, &mut v_dia, &mut fatia, 1, -1, -1, -1, cpv12_d(time) as u32, -1, 0, 0, 0, 1, lp, 1, -2,
            (segundo == 0) as u32, 0);
        destruir(lp);
        if achou != 0 {
            return 1;
        }
    }
    if msg != 0 {
        let mut buf = [0u8; 0x48];
        let g = cpv12_nome_do_time(time, segundo, &mut buf);
        texto(msg, if entrando != 0 { 0xa5b34c } else { 0xa5b2d8 }, g, g, buf.as_ptr());
    }
    0
}

/// { 0x76da80 } (stdcall, ret 0xc, national_teams.cpp): `national_teams_player_in_squad(p, nacao, tipo)`: 1 se o
/// clube da seleção do jogador (+0x24) é o time do tipo (0 e 1 a principal 0x53b3d0, 2 o sub-21 0x53b4f0) e ele tem a
/// marca do tipo (+0x5e: 4 nos tipos 0 e 2, 8 no 1). Sem clube ou outro tipo: 0.
unsafe extern "stdcall" fn esta_na_convocacao(p: usize, nacao: usize, tipo: u32) -> u32 {
    if cpv12_d(p + 0x24) == 0 {
        return 0;
    }
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let selecao_b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let (time, marca) = match tipo as u8 {
        0 => (principal(nacao), 4u8),
        1 => (principal(nacao), 8),
        2 => (selecao_b(nacao), 4),
        _ => return 0,
    };
    (cpv12_d(p + 0x24) == time && cpv12_b(p + 0x5e) & marca != 0) as u32
}

/// { 0x5c57d0 } (thiscall, ret 8, friendly.cpp; `this` não é lido): `Friendly::schedule_assign_opponent(agenda,
/// adversario)`: na primeira das `[agenda+8]` (byte com sinal) vagas de 25 bytes (a partir de +0x19) sem adversário
/// (dword +0), grava o adversário e marca o pedido (+0x21 da vaga anterior, ou seja agenda + 25·i + 0x21).
unsafe extern "thiscall" fn marcar_adversario_do_amistoso(_this: usize, agenda: usize, adversario: usize) {
    let n = cpv12_b(agenda + 8) as i8;
    let mut i: i8 = 0;
    while i < n {
        let vaga = agenda + 25 * (i as usize + 1);
        if cpv12_d(vaga) == 0 {
            core::ptr::write_unaligned(vaga as *mut usize, adversario);
            *((agenda + 25 * i as usize + 0x21) as *mut u8) = 1;
            return;
        }
        i += 1;
    }
}

/// Comparador: lado a lado pela devolução e pelo motivo (a original no destino, a cópia num destino próprio do mesmo
/// tamanho). As duas andam o cursor da lista [this] do mesmo jeito; a busca de jogos do dia monta e libera uma lista
/// local (sem estado que fique). Sem nação ou destino curto (a caixa): só a original.
unsafe extern "thiscall" fn comparar_pode_mudar_a_convocacao(this: usize, nacao: usize, tipo: u32, p: usize, entrando: u32, msg: usize, tam: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, i32) -> u32>(1266);
    if nacao == 0 || (msg != 0 && tam < 0x32) {
        return o(this, nacao, tipo, p, entrando, msg, tam);
    }
    let r = o(this, nacao, tipo, p, entrando, msg, tam);
    let mut meu: Vec<u8> = if msg != 0 { vec![0u8; tam as usize] } else { Vec::new() };
    let n = pode_mudar_a_convocacao(this, nacao, tipo, p, entrando, if msg != 0 { meu.as_mut_ptr() as usize } else { 0 }, tam);
    let texto = |a: *const u8, m: usize| -> Vec<u8> { (0..m).map(|i| *a.add(i)).take_while(|&c| c != 0).collect() };
    let (to, tr) = if msg != 0 { (texto(msg as *const u8, tam as usize), texto(meu.as_ptr(), tam as usize)) } else { (Vec::new(), Vec::new()) };
    PLACAR.registrar(r == n && to == tr, || format!("pode_mudar_a_convocacao({:x}, {}, {:x}, {}) {}/{} {:?} x {:?}", nacao, tipo as u8, p, entrando, r, n,
        String::from_utf8_lossy(&to), String::from_utf8_lossy(&tr)));
    r
}

/// Comparador: pura.
unsafe extern "stdcall" fn comparar_esta_na_convocacao(p: usize, nacao: usize, tipo: u32) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize, u32) -> u32>(1267)(p, nacao, tipo);
    let n = esta_na_convocacao(p, nacao, tipo);
    PLACAR.registrar(r == n, || format!("esta_na_convocacao({:x}, {:x}, {}) original={} rust={}", p, nacao, tipo as u8, r, n));
    r
}

/// Comparador: lado a lado pelas vagas da agenda (até a última possível, 25 · (n + 1) + 4 bytes).
unsafe extern "thiscall" fn comparar_marcar_adversario_do_amistoso(this: usize, agenda: usize, adversario: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1268);
    let n = cpv12_b(agenda + 8) as i8;
    if n <= 0 {
        return o(this, agenda, adversario);
    }
    let reg = [(agenda, 25 * (n as usize + 1) + 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, agenda, adversario), || marcar_adversario_do_amistoso(this, agenda, adversario));
    PLACAR.registrar(igual, || format!("marcar_adversario_do_amistoso({:x}, {:x}) {:x?} x {:x?}", agenda, adversario, mo, mr));
}

// ---------- 258a leva: o prêmio do mês no histórico (o técnico conta, o jogador agradece ou renova), a faixa de reputação e a folga do amistoso (staff_records.cpp, friendly.cpp) ----------

unsafe fn cpv13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv13_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube do contrato do jogador (0x4cf5a0, guardado em [0xae1080]; registros de 0x245 bytes), ou 0 sem contrato ou
/// com o contrato do tipo 5 (+0x35 & 0x3f).
unsafe fn cpv13_clube_do_contrato(p: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpv13_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    if c == 0 || cpv13_b(c + 0x35) & 0x3f == 5 {
        0
    } else {
        cpv13_d(0xae23b4).wrapping_add(cpv13_d(c + 4).wrapping_mul(0x245))
    }
}

/// O corpo da 0x86c8c0; sem `gravar` (a cópia pelo comparador), sem a entrada no arquivo do histórico.
unsafe fn cpv13_premio(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32, gravar: bool) {
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0x33;
    cpq4_por(&mut e, 8, if clube != 0 { cpv13_d(clube) as i32 } else { -1 });
    cpq4_por(&mut e, 0xd, cpv13_d(premio) as i32);
    cpq4_por(&mut e, 0x11, (a4 & 0xff) as i32);
    cpq4_por(&mut e, 0x15, (a5 & 0xffff) as i32);
    cpq4_historico(gravar, this, cpv13_d(p), &e);
    let (t3d, t28) = (cpv13_b(p + 0x3d), cpv13_b(p + 0x28));
    if t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc {
        let r = cpq4_registro(this, p);
        if r != 0 {
            crate::comparacao::ramo("cpv_13 prêmio do técnico");
            let a = (r + 0x9e) as *mut u16;
            core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(1)); // cpv13: conta o prêmio
        }
        return;
    }
    if cpv13_d(p + 0x61) == 0 {
        return;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpv13_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    if c == 0 || cpv13_b(c + 0x35) & 0x3f == 5 {
        return;
    }
    if cpv13_d(0xae23b4).wrapping_add(cpv13_d(c + 4).wrapping_mul(0x245)) == 0 {
        return;
    }
    let do_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let k = do_clube(cpv13_d(0xae1050), p, cpv13_clube_do_contrato(p));
    if k == 0 {
        return;
    }
    let gt = cpv13_d(0xde1f54);
    if gt != 0 {
        let dias: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
        if dias(0xae2c90, cpv13_d(k + 0x25), cpv13_d(k + 0x29)) > 0x12c {
            let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
            let x = entrada(cpv13_d(cpv13_d(0xde1f54) + 0x856), cpv13_clube_do_contrato(p), p, 0, 0);
            if x != 0 && cpv13_d(x + 0xf) == cpv13_d(p) {
                crate::comparacao::ramo("cpv_13 renovação");
                cpq3_renovar(p);
            }
        }
    }
    if cpv13_d(k + 0x45) != 0 {
        return;
    }
    let v59 = cpv13_b(p + 0x59) as i8;
    if v59 <= 0xf { // cpv13: o +0x59
        return;
    }
    if cpv13_b(p + 0x5b) as i8 <= 0xa { // cpv13: o +0x5b
        return;
    }
    crate::comparacao::ramo("cpv_13 sorteio do agradecimento");
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    if sorteio(v59 as i32) <= 5 {
        return;
    }
    let agradece: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x58a070usize);
    agradece(p, clube, premio);
}

/// { 0x86c8c0 } (thiscall, ret 0x14, staff_records.cpp, quadro SEH sem destrutor): `Staff_Records::add_month_award_record(
/// p, clube, premio, a4, a5)`: a entrada 0x33 do histórico da pessoa (data do jogo, clube ou −1, o id do prêmio, a4 no
/// byte, a5 na palavra; 0x58e340). Técnico (+0x3d ou +0x28 = 5 ou 0xc): +1 na palavra +0x9e do registro dele
/// (0x86f5b0). Jogador com ficha (+0x61) e contrato (0x4cf5a0, fora do tipo 5) com o clube (0x4cf5f0): com o gerente de
/// transferências, contrato de mais de 300 dias e o jogador na lista do clube (0x849ce0), a renovação (0x4cda70); depois,
/// sem a marca +0x45 do contrato, com +0x59 acima de 15 e +0x5b acima de 10, sorteio(+0x59) acima de 5: a notícia do
/// agradecimento ao técnico (0x58a070). Sem pessoa ou sem prêmio: a original (caixa "Error").
unsafe extern "thiscall" fn registrar_premio_do_mes(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32) {
    if p == 0 || premio == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(1269)(this, p, clube, premio, a4, a5);
    }
    cpv13_premio(this, p, clube, premio, a4, a5, true)
}

/// { 0x5c52d0 } (stdcall, ret 4, friendly.cpp): `friendly_club_reputation_tier(clube)`: a faixa da reputação (+0x80):
/// 5 a partir de 8000, 4 de 6000, 3 de 4000, 2 de 2000, senão 1 (só o al vale).
unsafe extern "stdcall" fn faixa_de_reputacao_do_amistoso(clube: usize) -> u8 {
    let r = core::ptr::read_unaligned((clube + 0x80) as *const i16);
    if r >= 0x1f40 {
        5
    } else if r >= 0x1770 {
        4
    } else if r >= 0xfa0 {
        3
    } else {
        1 + (r >= 0x7d0) as u8
    }
}

/// { 0x5c74d0 } (stdcall, ret 4, friendly.cpp): `friendly_get_rest_days_for_kind(tipo)`: a folga em volta do amistoso:
/// 2 nos tipos 1 e 10, 3 no 2, 1 nos 0, 3, 4 e 11. Os tipos 5..9 e fora de 0..11: a original (caixa "Error" e 1).
unsafe extern "stdcall" fn folga_do_amistoso(tipo: u32) -> u8 {
    match tipo as u8 as i8 {
        1 | 10 => 2,
        2 => 3,
        0 | 3 | 4 | 11 => 1,
        _ => orig::<unsafe extern "stdcall" fn(u32) -> u8>(1271)(tipo),
    }
}

/// Comparador: o técnico lado a lado pelo registro de pontos dele (criado antes pela 0x86f5b0, como a original faria;
/// a cópia sem a entrada do histórico em arquivo). O jogador sorteia, renova contrato e manda notícia (não pode rodar
/// duas vezes): só a original. Sem pessoa ou sem prêmio (a caixa): só a original.
unsafe extern "thiscall" fn comparar_registrar_premio_do_mes(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(1269);
    if p == 0 || premio == 0 {
        return o(this, p, clube, premio, a4, a5);
    }
    let (t3d, t28) = (cpv13_b(p + 0x3d), cpv13_b(p + 0x28));
    if !(t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc) {
        crate::comparacao::ramo("registrar_premio_do_mes jogador");
        return o(this, p, clube, premio, a4, a5);
    }
    let r = cpq4_registro(this, p);
    if r == 0 {
        return o(this, p, clube, premio, a4, a5);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(r, 0xd2)], || o(this, p, clube, premio, a4, a5),
        || cpv13_premio(this, p, clube, premio, a4, a5, false));
    PLACAR.registrar(igual, || format!("registrar_premio_do_mes({:x}, prêmio {:x}) {:x?} x {:x?}", p, cpv13_d(premio), mo, mr));
}

/// Comparador: pura.
unsafe extern "stdcall" fn comparar_faixa_de_reputacao_do_amistoso(clube: usize) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u8>(1270)(clube);
    let n = faixa_de_reputacao_do_amistoso(clube);
    PLACAR.registrar(r == n, || format!("faixa_de_reputacao_do_amistoso({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparador: pura; os tipos da caixa, só a original.
unsafe extern "stdcall" fn comparar_folga_do_amistoso(tipo: u32) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(u32) -> u8>(1271)(tipo);
    if !matches!(tipo as u8 as i8, 0..=4 | 10 | 11) {
        return r;
    }
    let n = folga_do_amistoso(tipo);
    PLACAR.registrar(r == n, || format!("folga_do_amistoso({}) original={} rust={}", tipo as u8 as i8, r, n));
    r
}
