// ---------- %%LEVA%%a leva: os contratos da transferência: a permissão de trabalho na mudança de clube, concluir a oferta de contrato aceita e oferecer contrato ao jogador da oferta (transfer_manager.cpp) ----------

unsafe fn cpq7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq7_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpq7_clube(id: usize) -> usize {
    cpq7_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpq7_pessoa(id: usize) -> usize {
    cpq7_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// A permissão de trabalho da `pessoa` (14 bytes, ordenadas pelo id: [this+0x93a], conta +0x93e), como a original
/// procura: pessoa com id fora de [0xae2364] ou sem a marca (bit 0x20 do registro de 8 bytes em [this+0x86e]), 0; senão
/// a busca binária (0x946464 com 0x8e55c0).
unsafe fn cpq7_permissao(this: usize, pessoa: usize) -> usize {
    if pessoa != 0 && cpq7_d(pessoa) as i32 >= cpq7_d(0xae2364) as i32 {
        return 0;
    }
    let id = cpq7_d(pessoa) as u32;
    if cpq7_b(cpq7_d(this + 0x86e) + id as usize * 8) & 0x20 == 0 {
        return 0;
    }
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    bsearch(&id, cpq7_d(this + 0x93a), cpq7_d(this + 0x93e), 0xe, 0x8e55c0)
}

/// { 0x8c7e20 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH só da data local):
/// `TRANSFER_MANAGER::update_work_permit_on_move(pessoa, clube)`: se o jogador precisa de permissão no clube novo
/// (0x8c7560 com 1) e já tem uma: troca a nação da permissão (+0xa) pela do clube (+0x53), zera +4 e +6 e, se mudou de
/// nação e as duas não são da marca 0xe (+0x76), grava o ano de hoje (+8); clube sem nação: a caixa "Error" 0x1dc2. Sem
/// permissão: a nova (0x8c7070 com o ano de hoje, ordenando), depois de hoje + 52 semanas (0x54c770, sem uso). Se não
/// precisa (a 0x8c7560 é chamada de novo) ou sem clube: tira a permissão que tiver (a marca 0x20, a última no lugar
/// dela, conta − 1 e reordena com 0x945333).
unsafe extern "thiscall" fn update_work_permit_on_move(this: usize, pessoa: usize, clube: usize) {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    if clube != 0 {
        if precisa(this, pessoa, clube, 1) != 0 {
            let r = cpq7_permissao(this, pessoa);
            if r != 0 {
                let nacao = cpq7_d(0xae23a8).wrapping_add(cpq7_d(r + 0xa).wrapping_mul(0x122));
                let nc = cpq7_d(clube + 0x53);
                if nc == 0 {
                    aviso_em(0xa8d514, 0x1dc2);
                    return;
                }
                if nacao != nc && !(cpq7_b(nacao + 0x76) == 0xe && cpq7_b(nc + 0x76) == 0xe) {
                    core::ptr::write_unaligned((r + 8) as *mut u16, *((0xae2c92) as *const u16));
                }
                core::ptr::write_unaligned((r + 4) as *mut u16, 0);
                core::ptr::write_unaligned((r + 6) as *mut u16, 0);
                core::ptr::write_unaligned((r + 0xa) as *mut u32, cpq7_d(nc) as u32);
                return;
            }
            let semanas: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(0x54c770usize);
            let dar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x8c7070usize);
            let mut t = [0u32; 2];
            semanas(0xae2c90, &mut t, 0x34);
            crate::comparacao::ramo("cpq_7 permissao nova");
            // ponytail: a original empurra o ano com lixo nos 16 bits altos; a 0x8c7070 só grava a palavra
            let ano = *((0xae2c92) as *const u16) as u32;
            dar(this, pessoa, cpq7_d(clube + 0x53), ano, 0, 1); // a permissão nova
            return;
        }
        if precisa(this, pessoa, clube, 1) != 0 {
            return;
        }
    }
    let r = cpq7_permissao(this, pessoa);
    if r == 0 {
        return;
    }
    let marca = (cpq7_d(this + 0x86e) + cpq7_d(pessoa) * 8) as *mut u8;
    *marca &= 0xdf;
    let ultima = cpq7_d(this + 0x93a) + cpq7_d(this + 0x93e) * 0xe - 0xe;
    core::ptr::copy(ultima as *const u8, r as *mut u8, 0xe);
    let n = cpq7_d(this + 0x93e).wrapping_sub(1);
    core::ptr::write_unaligned((this + 0x93e) as *mut usize, n);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpq7_d(this + 0x93a), n as u32, 0xe, 0x8e55c0);
}

/// { 0x8c38b0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da oferta local):
/// `TRANSFER_MANAGER::complete_accepted_contract_offer(oferta)`: oferta de contrato (+0 a pessoa, +4 o clube, +8 as
/// luvas, +0x61 o cargo). Se o jogador precisa de permissão no clube (0x8c7560 com 1) e ela não sai (0x8c7950): com
/// contrato de tipo 5 (0x4cf5a0, guardado em [0xae1080]), a notícia da oferta que caiu (0x8de790 com uma
/// TRANSFER_OFFER local: 0x8e5680, ids em +8 e +0xc, +0x1f = 0, motivo 0x13; 0x8e5710); senão a notícia da permissão
/// recusada (0x8e4290); devolve 0. Senão assina (0x4cf720) e, assinado, o clube paga as luvas (0x5a0590, 0x5998e0) e
/// devolve 1.
unsafe extern "thiscall" fn complete_accepted_contract_offer(this: usize, oferta: usize) -> u32 {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    let concedida: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c7950usize);
    let pessoa = cpq7_pessoa(cpq7_d(oferta));
    let clube = cpq7_clube(cpq7_d(oferta + 4));
    if precisa(this, pessoa, clube, 1) != 0 && concedida(this, pessoa, cpq7_d(clube + 0x53), oferta, 0, 0) == 0 {
        let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let c = contract_of(cpq7_d(0xae1050), pessoa);
        *(0xae1080 as *mut usize) = c;
        if c != 0 && cpq7_b(c + 0x35) & 0x3f == 5 {
            let construir: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x8e5680usize);
            let caiu: unsafe extern "thiscall" fn(usize, *mut u8, usize, u32) = core::mem::transmute(0x8de790usize);
            let destruir: unsafe extern "thiscall" fn(*mut u8) = core::mem::transmute(0x8e5710usize);
            let mut o = [0u8; 0x44];
            construir(o.as_mut_ptr());
            o[8..12].copy_from_slice(&(cpq7_d(clube) as u32).to_le_bytes());
            o[0xc..0x10].copy_from_slice(&(cpq7_d(pessoa) as u32).to_le_bytes());
            o[0x1f] = 0;
            caiu(this, o.as_mut_ptr(), clube, 0x13);
            destruir(o.as_mut_ptr());
        } else {
            let recusada: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8e4290usize);
            recusada(this, pessoa, oferta);
        }
        return 0;
    }
    let assinar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x4cf720usize);
    // ponytail: a original empurra o cargo num registrador com lixo acima; a 0x4cf720 e a 0x53bd10 só usam o byte
    if assinar(cpq7_d(0xae1050), oferta, cpq7_b(oferta + 0x61) as u32) == 0 {
        return 0;
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let luvas: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x5998e0usize);
    let valor = cpq7_d(oferta + 8) as u32;
    let fin = financas(cpq7_d(0xae2a38), clube);
    crate::comparacao::ramo("cpq_7 luvas");
    luvas(fin, valor); // as luvas
    1
}

/// { 0x8c1e40 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH do contrato local):
/// `TRANSFER_MANAGER::offer_contract_to_offer_player(clube, pessoa, oferta)`: monta o contrato (STAFF_CONTRACT local,
/// 0x85e410) que o clube ofereceria (0x4cda70 com a vaga +0x20 da oferta, fator de salário 1 e modo 2). Montado:
/// acrescenta a oferta de contrato (0x4cc490 com a vaga +0x20, o valor +0x24, 0xb, 0, 1, 0). Sem contrato: sendo o
/// jogador principal (+0xc), atualiza a lista do clube (0x849ce0 e 0x843e10 com 1) e retira a oferta (0x8bfcb0, motivo
/// 0x16); sendo da troca, soma o valor dele (+0x52) ao da oferta (+0x1b) e o tira da troca (0x8c41c0 com 1 e 0x16).
/// Devolve o que a última chamada devolveu.
unsafe extern "thiscall" fn offer_contract_to_offer_player(this: usize, clube: usize, pessoa: usize, oferta: usize) -> u32 {
    let construir: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x85e410usize);
    let oferecer: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, usize, f64, u32) -> u32 = core::mem::transmute(0x4cda70usize);
    let mut c = [0u8; 0x50];
    construir(c.as_mut_ptr());
    crate::comparacao::ramo("cpq_7 contrato da oferta");
    if oferecer(cpq7_d(0xae1050), pessoa, clube, c.as_mut_ptr(), cpq7_d(oferta + 0x20), 1.0f64, 2) != 0 {
        let acrescentar: unsafe extern "thiscall" fn(usize, *mut u8, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4cc490usize);
        return acrescentar(cpq7_d(0xae1050), c.as_mut_ptr(), cpq7_d(oferta + 0x20), cpq7_d(oferta + 0x24), 0xb, 0, 1, 0);
    }
    if cpq7_d(pessoa) == cpq7_d(oferta + 0xc) {
        let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
        let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x843e10usize);
        let retirar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(0x8bfcb0usize);
        let e = entrada(cpq7_d(this + 0x856), clube, pessoa, (cpq7_b(oferta + 0x11) != 0xb) as u32, 0);
        if e != 0 {
            // ponytail: a original empurra o tipo (setne) com o resto do ponteiro da entrada acima; a 0x843e10 só lê o byte
            atualizar(cpq7_d(this + 0x856), pessoa, clube, 1, (cpq7_b(e) != 0xb) as u32);
        }
        let mut o = oferta;
        return retirar(this, &mut o, 0, 0x16);
    }
    let v = (oferta + 0x1b) as *mut i32;
    core::ptr::write_unaligned(v, core::ptr::read_unaligned(v).wrapping_add(cpq7_d(pessoa + 0x52) as i32));
    let tirar_da_troca: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c41c0usize);
    tirar_da_troca(cpq7_d(0xde1f54), pessoa, oferta, 1, 0x16)
}

/// Comparador: grava as permissões (pode crescer o vetor e reordena): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_update_work_permit_on_move(this: usize, pessoa: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(this, pessoa, clube)
}

/// Comparador: assina o contrato (aloca), paga e posta notícias: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_complete_accepted_contract_offer(this: usize, oferta: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, oferta)
}

/// Comparador: acrescenta ofertas de contrato (aloca), mexe nas listas e retira ofertas: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_offer_contract_to_offer_player(this: usize, clube: usize, pessoa: usize, oferta: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(%%I2%%)(this, clube, pessoa, oferta)
}
