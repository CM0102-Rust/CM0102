// ---------- %%LEVA%%a leva: as ofertas em espera no fechamento da janela, a notícia das expectativas da federação e a do protesto da torcida (transfer_manager.cpp, manager_manager.cpp) ----------

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
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contract_of(cpv3_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato (id em +4); com `filtro`, nenhum para o contrato de tipo 5 (+0x35 & 0x3f).
unsafe fn offer_contract_club(c: usize, filtro: bool) -> usize {
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
unsafe fn put_offer_on_hold(o: usize) {
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
unsafe extern "thiscall" fn hold_offers_at_window_close(this: usize, nacao: usize) {
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let completar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c34e0usize);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let noticia_da_espera: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8e1150usize);
    let retirar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8bfcb0usize);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8bf680usize);
    let human_club: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
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
            put_offer_on_hold(o);
            let humano_vende = || {
                let c = cpv3_contrato(cpv3_pessoa(cpv3_d(o + 0xc)));
                if c == 0 || cpv3_b(c + 0x35) & 0x3f == 5 || cpv3_clube(cpv3_d(c + 4)) == 0 {
                    return false;
                }
                let v = offer_contract_club(cpv3_contrato(cpv3_pessoa(cpv3_d(o + 0xc))), true);
                humano(cpv3_d(0xb7431c), v) != 0
            };
            if humano(cpv3_d(0xb7431c), cpv3_clube(cpv3_d(o + 8))) != 0 || humano_vende() {
                noticia_da_espera(this, o, nacao);
            } else {
                crate::comparison::branch("cpv_3 withdrawn in the window");
                // a vaga relida de [this] como na original
                retirar(this, cpv3_d(this).wrapping_add((i - 1) * 4), 0, 0); // cpv3: o comprador retira
            }
            if human_club(cpv3_clube(cpv3_d(o + 8)), 0, 1, 0, 0) == 0 {
                crate::comparison::branch("cpv_3 rescheduled in the window");
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
        let vendedor = offer_contract_club(cpv3_contrato(cpv3_pessoa(cpv3_d(o + 0xc))), false);
        if cpv3_d(vendedor + 0x53) != nacao || cpv3_resolvida(o) || cpv3_b(o + 0x2c) == 0x13 {
            continue;
        }
        if regras != 0 {
            let dono = offer_contract_club(cpv3_contrato(cpv3_pessoa(cpv3_d(o + 0xc))), false);
            if !aceita(o, dono) {
                continue;
            }
        }
        put_offer_on_hold(o);
        let humano_vende = || {
            let c = cpv3_contrato(cpv3_pessoa(cpv3_d(o + 0xc)));
            if c == 0 || cpv3_clube(cpv3_d(c + 4)) == 0 {
                return false;
            }
            let v = offer_contract_club(cpv3_contrato(cpv3_pessoa(cpv3_d(o + 0xc))), false);
            humano(cpv3_d(0xb7431c), v) != 0
        };
        if humano(cpv3_d(0xb7431c), cpv3_clube(cpv3_d(o + 8))) != 0 || humano_vende() {
            noticia_da_espera(this, o, nacao);
        } else {
            crate::comparison::branch("cpv_3 refused in the window");
            recusar(this, cpv3_d(this).wrapping_add((i - 1) * 4), 0, 0); // cpv3: o vendedor recusa
        }
        if human_club(cpv3_clube(cpv3_d(o + 8)), 0, 1, 0, 0) == 0 {
            cpv3_pb(o + 0x2c, 3);
            agendar(this, o, 1, 0);
        }
    }
}

/// { 0x6a44a0 } (thiscall, ret 0xc, manager_manager.cpp, quadro SEH sem destrutor): `MANAGER_MANAGER::
/// post_fa_expectations_news(seleção, expectativa, competição)`: a notícia 0x177e (local): 0 = o id da seleção, 1 = a
/// expectativa (palavra com sinal), 2 = a competição; pelo alcance (0x786090) com a seleção.
unsafe extern "thiscall" fn post_fa_expectations_news(_this: usize, selecao: usize, expectativa: u32, competicao: u32) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let por_alcance: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x786090usize);
    crate::comparison::branch("cpv_3 federation news");
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
unsafe extern "thiscall" fn post_supporter_protest_news(_this: usize, clube: usize) {
    let dia = *(0xdd6ec8 as *const i16);
    let ano = *(0xdd6eca as *const u16);
    let hoje_ano = *(0xae2c92 as *const u16);
    if ano > hoje_ano || (ano == hoje_ano && dia > *(0xae2c90 as *const i16)) {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let por_alcance: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x786090usize);
    crate::comparison::branch("cpv_3 protest news");
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
unsafe extern "thiscall" fn compare_hold_offers_at_window_close(this: usize, nacao: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, nacao)
}

// ponytail: a notícia vai pelo alcance (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_post_fa_expectations_news(this: usize, selecao: usize, expectativa: u32, competicao: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I1%%)(this, selecao, expectativa, competicao)
}

// ponytail: a notícia vai pelo alcance (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_post_supporter_protest_news(this: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, clube)
}
