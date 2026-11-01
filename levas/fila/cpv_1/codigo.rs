// ---------- %%LEVA%%a leva: a oferta passada aos clubes, a notícia da oferta aceita e a do empréstimo oferecido (transfer_manager.cpp) ----------

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
    orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, oferta)
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_da_oferta_aceita(this: usize, oferta: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, oferta)
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_do_emprestimo_oferecido(this: usize, oferta: usize, termos: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%)(this, oferta, termos)
}
