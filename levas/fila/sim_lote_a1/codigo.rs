// ---------- %%LEVA%%a leva: o fim do empréstimo, a contratação pelas regras da nação e as trocas que oferecem o jogador (transfer_manager.cpp) ----------

/// As regras de transferência da nação: das 28 em [gerente+0x8ac] (ponteiros), a de nação (+0xa) igual;
/// 0 se nenhuma.
unsafe fn regras_de_transferencia_da_nacao(gerente: usize, nacao: usize) -> usize {
    (0..28)
        .map(|i| *((gerente + 0x8ac + 4 * i) as *const usize))
        .find(|&r| core::ptr::read_unaligned((r + 0xa) as *const usize) == nacao)
        .unwrap_or(0)
}

/// { 0x8ced40 } (thiscall, ret 0x18, transfer_manager.cpp): se o `clube` pode contratar o `jogador`,
/// pelas regras da nação do clube (método +0x10 com clube, jogador, a4, a2 e o motivo); sem regras, 1. O
/// motivo (palavra) vai para `saida` (zerada antes). Recusado com `a5`, clube humano (0x600a50) e motivo
/// de estrangeiros (0x11, 0x14, 0x15, 0x1f): pergunta de novo com a nação do jogador (+0x1a) trocada pela
/// do clube (e devolvida depois); aceito assim, 1.
unsafe extern "thiscall" fn pode_contratar_jogador(gerente: usize, clube: usize, a2: u32, jogador: usize, a4: u32, a5: u32, saida: *mut u16) -> u32 {
    let regras = regras_de_transferencia_da_nacao(gerente, core::ptr::read_unaligned((clube + 0x53) as *const usize));
    if !saida.is_null() {
        *saida = 0;
    }
    if regras == 0 {
        return 1;
    }
    let consultar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut u32) -> u32 =
        core::mem::transmute(*((*(regras as *const usize) + 0x10) as *const usize));
    // o motivo é a vaga do argumento `clube` na pilha da original: sem resposta das regras fica o clube
    let mut motivo = clube as u32;
    let r = consultar(regras, clube, jogador, a4, a2, &mut motivo);
    if !saida.is_null() {
        *saida = motivo as u16;
    }
    if r != 0 {
        return r;
    }
    if a5 == 0 {
        return 0; // recusado, sem segunda pergunta
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 0, 1, 0, 0) == 0 || !matches!(motivo as u16, 0x11 | 0x14 | 0x15 | 0x1f) {
        if !saida.is_null() {
            *saida = motivo as u16;
        }
        return 0;
    }
    let nacao = (jogador + 0x1a) as *mut u32;
    let guardada = core::ptr::read_unaligned(nacao);
    core::ptr::write_unaligned(nacao, core::ptr::read_unaligned((clube + 0x53) as *const u32));
    let r = consultar(regras, clube, jogador, a4, a2, &mut motivo);
    core::ptr::write_unaligned(nacao, guardada);
    if r != 0 {
        return 1;
    }
    if !saida.is_null() {
        *saida = motivo as u16;
    }
    0
}

/// { 0x8c5e40 } (thiscall, ret 8, transfer_manager.cpp): quantas ofertas abertas do clube do contrato do
/// `jogador` (0x4cf5a0) o oferecem na troca (lista +4 da oferta, nó com o id dele), anotando o índice
/// (+0x20) em `saida` (0x68d0d0) se houver. Ofertas no estado (+0x2c) 2, 3, 4, 0x10, 0x14 ou 0xf com
/// +0x2e ≥ 3 não contam. Para ao chegar ao número de trocas marcado ([+0x86e] + 8 × id + 2); marcado 0
/// ou id fora da faixa, 0.
unsafe extern "thiscall" fn trocas_oferecendo_o_jogador(gerente: usize, jogador: usize, saida: usize) -> u32 {
    if jogador != 0 && *(jogador as *const i32) >= *(0xae2364 as *const i32) {
        return 0;
    }
    let limite = || *((*((gerente + 0x86e) as *const usize) as isize + *(jogador as *const i32) as isize * 8 + 2) as *const u8) as u32;
    if limite() == 0 {
        return 0; // sem trocas marcadas
    }
    if clube_do_contrato(jogador) == 0 {
        return 0;
    }
    let clube = *(clube_do_contrato(jogador) as *const u32);
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let anexar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let mut conta = 0u32;
    let mut i = 0;
    while i < *((gerente + 4) as *const i32) {
        let vaga = (*(gerente as *const usize) + 4 * i as usize) as *const usize;
        i += 1;
        let oferta = *vaga;
        if oferta == 0 || *((oferta + 8) as *const u32) != clube {
            continue;
        }
        let estado = *((oferta + 0x2c) as *const u8);
        if matches!(estado, 2 | 3 | 4 | 0x10 | 0x14) || (estado == 0xf && *((oferta + 0x2e) as *const u8) >= 3) {
            continue;
        }
        let l = *((oferta + 4) as *const usize);
        if l != 0 {
            core::ptr::write_unaligned((l + 0xa) as *mut u32, core::ptr::read_unaligned((l + 2) as *const u32));
            loop {
                let no = prox(*((*vaga + 4) as *const usize));
                if no == 0 {
                    break;
                }
                if *(no as *const u32) == *(jogador as *const u32) {
                    if saida != 0 {
                        anexar(saida, *vaga + 0x20, 4);
                    }
                    conta += 1; // trocas
                    break;
                }
            }
        }
        if conta == limite() {
            break;
        }
    }
    conta
}


unsafe extern "thiscall" fn comparar_pode_contratar_jogador(gerente: usize, clube: usize, a2: u32, jogador: usize, a4: u32, a5: u32, saida: *mut u16) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, *mut u16) -> u32>(%%I0%%);
    let mut regioes = vec![];
    if !saida.is_null() {
        regioes.push((saida as usize, 2));
    }
    if jogador != 0 {
        regioes.push((jogador + 0x1a, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(gerente, clube, a2, jogador, a4, a5, saida),
        || pode_contratar_jogador(gerente, clube, a2, jogador, a4, a5, saida));
    PLACAR.registrar(igual && r == n, || format!("pode_contratar_jogador(a5 {}) original={} rust={} {:x?} x {:x?}", a5, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_trocas_oferecendo_o_jogador(gerente: usize, jogador: usize, saida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%);
    if saida != 0 {
        // ponytail: com lista de saída a original anexa (aloca); só a original
        return o(gerente, jogador, saida);
    }
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if jogador != 0 && *(jogador as *const i32) < *(0xae2364 as *const i32) {
        let guardado = *(0xae1080 as *const usize);
        let p = clube_do_contrato(jogador);
        *(0xae1080 as *mut usize) = guardado;
        if p != 0 {
            // os cursores das listas de troca das ofertas do clube (as que a rotina pode percorrer)
            let clube = *(p as *const u32);
            for i in 0..(*((gerente + 4) as *const i32)).max(0) as usize {
                let oferta = *((*(gerente as *const usize) + 4 * i) as *const usize);
                if oferta != 0 && *((oferta + 8) as *const u32) == clube && *((oferta + 4) as *const usize) != 0 {
                    regioes.push((*((oferta + 4) as *const usize) + 0xa, 4));
                }
            }
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(gerente, jogador, saida),
        || trocas_oferecendo_o_jogador(gerente, jogador, saida));
    PLACAR.registrar(igual && r == n, || format!("trocas_oferecendo_o_jogador original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}
