// ---------- %%LEVA%%a leva: as ofertas de transferência: desfazer a oferta, fazer a oferta de empréstimo e concluir as transferências futuras do dia (transfer_manager.cpp) ----------

unsafe fn cpq8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq8_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpq8_clube(id: usize) -> usize {
    cpq8_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpq8_pessoa(id: usize) -> usize {
    cpq8_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato da pessoa da oferta (+0xc; 0x4cf5a0 com o gerente [0xae1050]), guardado em [0xae1080].
unsafe fn offer_target_contract(oferta: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contract_of(cpq8_d(0xae1050), cpq8_pessoa(cpq8_d(oferta + 0xc)));
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato da pessoa da oferta; contrato de tipo 5 (+0x35 & 0x3f) não conta.
unsafe fn offer_target_seller_club(oferta: usize) -> usize {
    let c = offer_target_contract(oferta);
    if c != 0 && cpq8_b(c + 0x35) & 0x3f != 5 {
        cpq8_clube(cpq8_d(c + 4))
    } else {
        0
    }
}

/// Recomeça o cursor da LLIST (+0xa = +2, o primeiro nó).
unsafe fn llist_restart(lista: usize) {
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, cpq8_d(lista + 2));
}

/// O estado da oferta (+0x2c) que deixa a marca 1 da pessoa sair: 2, 3, 0x14, 0x10, 4 e 0xf com +0x2e ≥ 3.
unsafe fn is_offer_state_closed(oferta: usize) -> bool {
    match cpq8_b(oferta + 0x2c) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => cpq8_b(oferta + 0x2e) >= 3,
        _ => false,
    }
}

/// { 0x8c44e0 } (thiscall, ret 0x10, transfer_manager.cpp): `TRANSFER_MANAGER::collapse_offer(oferta, clube, motivo,
/// tirar_futura)`: na oferta de estado 0x13 tira a marca 0x10 da pessoa (+0xc) e dos jogadores da troca (lista +4) e,
/// com `tirar_futura`, tira a transferência futura (0x8cbf70). Põe o estado 0x10, fecha as propostas (0x8c6db0), solta
/// os lances do comprador (+8) pela pessoa e do vendedor (clube do contrato, guardado em [0xae1080]) pelos da troca
/// (0x84aeb0), posta a notícia (0x8de790 com o clube e o motivo) e agenda a oferta (0x8c6220 com 0x14). Encerrada a
/// oferta e todas as da lista +0 (estados de `is_offer_state_closed`), tira a marca 1 da pessoa. Atualiza a lista do
/// comprador (0x849ce0 e 0x843e10 com 1), tira a oferta de contrato (0x4cca70), zera +0x10 e solta os contratos da troca
/// (0x4ccb30 com a vaga +0x20; +4 do item = 0).
unsafe extern "thiscall" fn collapse_offer(this: usize, oferta: usize, clube: usize, motivo: u32, tirar_futura: u32) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    let marca = |id: usize| (cpq8_d(this + 0x86e) + id * 8) as *mut u8;
    crate::comparison::branch("cpq_8 undo offer");
    if cpq8_b(oferta + 0x2c) == 0x13 {
        *marca(cpq8_d(oferta + 0xc)) &= 0xef;
        if cpq8_d(oferta + 4) != 0 {
            llist_restart(cpq8_d(oferta + 4));
            loop {
                let it = proximo(cpq8_d(oferta + 4));
                if it == 0 {
                    break;
                }
                *marca(cpq8_d(it)) &= 0xef;
            }
        }
        if tirar_futura != 0 {
            let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cbf70usize);
            tirar(this, oferta);
        }
    }
    *((oferta + 0x2c) as *mut u8) = 0x10;
    let fechar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c6db0usize);
    fechar(this, oferta);
    let pessoa = cpq8_pessoa(cpq8_d(oferta + 0xc));
    let c = offer_target_contract(oferta);
    let vendedor = if c != 0 { cpq8_clube(cpq8_d(c + 4)) } else { 0 };
    desmarcar(cpq8_d(this + 0x856), cpq8_clube(cpq8_d(oferta + 8)), pessoa);
    if cpq8_d(oferta + 4) != 0 {
        llist_restart(cpq8_d(oferta + 4));
        loop {
            let it = proximo(cpq8_d(oferta + 4));
            if it == 0 {
                break;
            }
            desmarcar(cpq8_d(this + 0x856), vendedor, cpq8_pessoa(cpq8_d(it)));
        }
    }
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x8de790usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    noticia(this, oferta, clube, motivo);
    agendar(this, oferta, 0x14, 0);
    if is_offer_state_closed(oferta) {
        let mut todas = true;
        if cpq8_d(oferta) != 0 {
            llist_restart(cpq8_d(oferta));
            loop {
                let it = proximo(cpq8_d(oferta));
                if it == 0 {
                    break;
                }
                if !is_offer_state_closed(cpq8_d(cpq8_d(this) + cpq8_d(it) * 4)) {
                    todas = false;
                    break;
                }
            }
        }
        if todas {
            let m = marca(cpq8_d(oferta + 0xc));
            if *m & 1 != 0 {
                *m &= 0xfe;
            }
        }
    }
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x843e10usize);
    let e = entrada(cpq8_d(this + 0x856), cpq8_clube(cpq8_d(oferta + 8)), cpq8_pessoa(cpq8_d(oferta + 0xc)), (cpq8_b(oferta + 0x11) != 0xb) as u32, 0);
    if e != 0 {
        // ponytail: a original empurra o tipo (setne) com lixo nos bytes altos; a 0x843e10 só lê o byte
        atualizar(cpq8_d(this + 0x856), cpq8_pessoa(cpq8_d(oferta + 0xc)), cpq8_clube(cpq8_d(oferta + 8)), 1, (cpq8_b(e) != 0xb) as u32);
    }
    let tirar_contrato: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cca70usize);
    tirar_contrato(cpq8_d(0xae1050), cpq8_pessoa(cpq8_d(oferta + 0xc)), cpq8_clube(cpq8_d(oferta + 8))); // a oferta de contrato sai
    *((oferta + 0x10) as *mut u8) = 0;
    if cpq8_d(oferta + 4) != 0 {
        let soltar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4ccb30usize);
        llist_restart(cpq8_d(oferta + 4));
        loop {
            let it = proximo(cpq8_d(oferta + 4));
            if it == 0 {
                break;
            }
            soltar(cpq8_d(0xae1050), cpq8_pessoa(cpq8_d(it)), cpq8_d(oferta + 0x20));
            *((it + 4) as *mut u8) = 0;
        }
    }
}

/// { 0x8bed60 } (thiscall, ret 0x18, transfer_manager.cpp, quadro SEH da oferta local):
/// `TRANSFER_MANAGER::make_loan_offer(jogador, clube, fim, percentual, a5, a6)`: numa TRANSFER_OFFER local (0x8e5680)
/// monta a oferta de empréstimo (0x8e7810) e, num STAFF_CONTRACT local (0x85e410), o contrato pelo atual (0x85e590 com o
/// fim, a porcentagem do salário e as duas cláusulas); acrescenta a oferta (0x8be300 com o contrato) e a oferta de
/// contrato (0x4cc490 com a vaga +0x20 e o valor +0x24 da oferta, 0xb, 0, 1, 0). Destrói a oferta local (0x8e5710) e
/// devolve a vaga (+0x20).
unsafe extern "thiscall" fn make_loan_offer(this: usize, jogador: usize, clube: usize, fim: usize, write_percent_pair: u32, a5: u32, a6: u32) -> u32 {
    let construir: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x8e5680usize);
    let emprestimo: unsafe extern "thiscall" fn(*mut u8, usize, usize) = core::mem::transmute(0x8e7810usize);
    let construir_contrato: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x85e410usize);
    let pelo_atual: unsafe extern "thiscall" fn(*mut u8, usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x85e590usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, *mut u8, *mut u8) -> u32 = core::mem::transmute(0x8be300usize);
    let acrescentar_contrato: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4cc490usize);
    let destruir: unsafe extern "thiscall" fn(*mut u8) = core::mem::transmute(0x8e5710usize);
    let campo = |o: &[u8; 0x44], k: usize| u32::from_le_bytes([o[k], o[k + 1], o[k + 2], o[k + 3]]);
    let mut o = [0u8; 0x44];
    construir(o.as_mut_ptr());
    emprestimo(o.as_mut_ptr(), jogador, clube);
    let mut c = [0u8; 0x50];
    construir_contrato(c.as_mut_ptr());
    crate::comparison::branch("cpq_8 loan offer");
    pelo_atual(c.as_mut_ptr(), jogador, clube, fim, write_percent_pair, a5, a6); // o salário pela porcentagem
    acrescentar(this, o.as_mut_ptr(), c.as_mut_ptr());
    acrescentar_contrato(cpq8_d(0xae1050), c.as_mut_ptr(), campo(&o, 0x20), campo(&o, 0x24), 0xb, 0, 1, 0);
    let vaga = campo(&o, 0x20);
    destruir(o.as_mut_ptr());
    vaga
}

/// { 0x8cbbe0 } (thiscall, sem argumentos, transfer_manager.cpp): `TRANSFER_MANAGER::process_due_future_transfers()`:
/// percorre as transferências futuras (12 bytes: +0 a data, +8 o índice da oferta; [this+0x920], conta +0x928) até a
/// primeira depois de hoje (0x456740). Oferta nula: a caixa "Error" 0x2846. Com as permissões (0x8c7690 com 1): se as
/// restrições (0x8cfc90) seguram pelo comprador ou, havendo vendedor, pelo vendedor (motivo 9 não conta), desfaz a
/// oferta (0x8c44e0 com o clube e o motivo); senão a notícia (0x8dbef0), conclui (0x8c9390), estado 0x14 e agenda
/// (0x8c6220 com 0xa). Depois tira as processadas do começo do vetor (0x8cbf40 uma a uma) e diminui a conta.
unsafe extern "thiscall" fn process_due_future_transfers(this: usize) {
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let permissoes: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8c7690usize);
    let restricoes: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, u32) -> u32 = core::mem::transmute(0x8cfc90usize);
    let desfazer: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x8c44e0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8dbef0usize);
    let concluir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c9390usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    // o motivo das restrições: só a palavra baixa é gravada; os 16 bits altos ficam 0, como na original
    let mut motivo = 0u32;
    let mut feitas = 0i32;
    while feitas < cpq8_d(this + 0x928) as i32 {
        let e = cpq8_d(this + 0x920) + feitas as usize * 0xc;
        if depois(e, cpq8_d(0xae2c90) as u32, cpq8_d(0xae2c94) as u32) != 0 {
            break;
        }
        let oferta = cpq8_d(cpq8_d(this) + cpq8_d(e + 8) * 4);
        if oferta == 0 {
            show_error_at(0xa8d514, 0x2846);
        } else if permissoes(this, oferta, 1) != 0 {
            let comprador = cpq8_clube(cpq8_d(oferta + 8));
            let mut segura = false;
            if restricoes(this, oferta, comprador, &mut motivo, 0) as u8 != 0 && motivo as u16 != 9 {
                desfazer(this, oferta, comprador, motivo, 0);
                segura = true;
            } else if offer_target_seller_club(oferta) != 0 {
                let v = offer_target_seller_club(oferta);
                if restricoes(this, oferta, v, &mut motivo, 0) as u8 != 0 && motivo as u16 != 9 {
                    let v = offer_target_seller_club(oferta);
                    desfazer(this, oferta, v, motivo, 0);
                    segura = true;
                }
            }
            if !segura {
                noticia(this, oferta);
                crate::comparison::branch("cpq_8 future transfer");
                concluir(this, oferta); // a transferência acontece
                *((oferta + 0x2c) as *mut u8) = 0x14;
                agendar(this, oferta, 0xa, 0);
            }
        }
        feitas += 1;
    }
    if feitas < 0 {
        show_error_at(0xa8d514, 0x288a);
    }
    if feitas == 0 {
        return;
    }
    let mut k = feitas;
    if k < cpq8_d(this + 0x928) as i32 {
        let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8cbf40usize);
        while k < cpq8_d(this + 0x928) as i32 {
            let v = cpq8_d(this + 0x920);
            copiar(v + (k - feitas) as usize * 0xc, v + k as usize * 0xc);
            k += 1;
        }
    }
    let n = (this + 0x928) as *mut i32;
    core::ptr::write_unaligned(n, core::ptr::read_unaligned(n).wrapping_sub(feitas));
}

/// Comparador: muda o estado, as listas e as marcas, posta notícia e agenda: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn compare_collapse_offer(this: usize, oferta: usize, clube: usize, motivo: u32, tirar_futura: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I0%%)(this, oferta, clube, motivo, tirar_futura)
}

/// Comparador: acrescenta a oferta e a oferta de contrato (alocam): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn compare_make_loan_offer(this: usize, jogador: usize, clube: usize, fim: usize, write_percent_pair: u32, a5: u32, a6: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> u32>(%%I1%%)(this, jogador, clube, fim, write_percent_pair, a5, a6)
}

/// Comparador: conclui e desfaz transferências (elencos, dinheiro, notícias): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn compare_process_due_future_transfers(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}
