// ---------- %%LEVA%%a leva: a transferência efetivada, com a notícia da transferência concluída e a do acordo entre os clubes (transfer_manager.cpp) ----------

unsafe fn cpx3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpx3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpx3_clube(id: usize) -> usize {
    cpx3_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpx3_pessoa(id: usize) -> usize {
    cpx3_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0 em [0xae1050]), sem guardar.
unsafe fn cpx3_contrato_de(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    f(cpx3_d(0xae1050), pessoa)
}

/// O mesmo, guardado em [0xae1080] como a original faz.
unsafe fn get_contract_store_result(pessoa: usize) -> usize {
    let c = cpx3_contrato_de(pessoa);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato principal que não é de tipo 5 (ou 0), com o contrato guardado em [0xae1080].
unsafe fn contract_club_not_type5(pessoa: usize) -> usize {
    let c = get_contract_store_result(pessoa);
    if c == 0 || cpx3_b(c + 0x35) & 0x3f == 5 {
        0
    } else {
        cpx3_clube(cpx3_d(c + 4))
    }
}

/// Os itens de uma LLIST do começo (o cursor +0xa volta a +2), como os laços da original.
unsafe fn cpx3_itens(l: usize) -> Vec<usize> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpx3_d(l + 2));
    let mut v = Vec::new();
    let mut e = proximo(l);
    while e != 0 {
        v.push(e);
        e = proximo(l);
    }
    v
}

/// O teste da oferta encerrada (estados 2, 3, 0x14, 0x10, 4; 0xf com +0x2e de 3).
unsafe fn cpx3_encerrada(o: usize) -> bool {
    match cpx3_b(o + 0x2c) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => cpx3_b(o + 0x2e) >= 3,
        _ => false,
    }
}

/// O clube que recebe: sem técnico humano (0x600a50) e com números de camisa (SQUAD_MANAGER 0x85add0 em [0xdd7ef0]),
/// os números de quem chegou (0x85a470).
unsafe fn allocate_arrival_squad_numbers(clube: usize) {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 1, 1, 0, 0) != 0 {
        return;
    }
    let usa: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x85add0usize);
    if usa(cpx3_d(0xdd7ef0), clube, 0) != 0 {
        let numerar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x85a470usize);
        numerar(cpx3_d(0xdd7ef0), clube, 0); // cpx3: os números de camisa de quem chegou
    }
}

/// { 0x8c9390 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::finalise_transfer(oferta)`, pelo
/// match_wrap_up da oferta (0x8c34e0) e pela transferência futura. A confiança das diretorias (0x6978f0); as regras da
/// nação do comprador (as 28 de +0x8ac, pela nação +0xa): método 1 (sem +0x1f) ou 2 com a oferta; as cláusulas
/// anteriores (0x8c91e0); com vendedor (contrato sem tipo 5), o pagamento (0x8ca8a0); sem +0x1f = 1, os registros do
/// técnico (0x86cd70); os recordes do clube (0x448760). Com troca, o primeiro da troca sai do elenco do clube dele
/// (+0xd7, 50 vagas). Com valor (+0x1b) e sem +0x69 na pessoa, a parte dos clubes anteriores (0x8e4f20). O jogador vai
/// ao comprador (0x8c9720) e os da troca ao vendedor, com os números de camisa; estado 0x14, a oferta volta em 10 dias
/// (0x8c6220); sem rival em aberto, a marca de oferta (+0x86e) sai; os lances (0x84aeb0) e as ofertas de contrato
/// ligadas (0x8c6db0).
unsafe extern "thiscall" fn finalise_transfer(this: usize, o: usize) {
    let p = cpx3_pessoa(cpx3_d(o + 0xc));
    let comprador = cpx3_clube(cpx3_d(o + 8));
    let vendedor = contract_club_not_type5(p);
    let confianca: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x6978f0usize);
    crate::comparacao::ramo("cpx_3 confianca das diretorias");
    confianca(cpx3_d(0xb67a44), o); // cpx3: a confiança das diretorias
    let nacao = cpx3_d(cpx3_clube(cpx3_d(o + 8)) + 0x53);
    for k in 0..0x1c {
        let regras = cpx3_d(this + 0x8ac + k * 4);
        if cpx3_d(regras + 0xa) == nacao {
            if regras != 0 {
                let metodo = cpx3_d(cpx3_d(regras) + if cpx3_b(o + 0x1f) == 0 { 4 } else { 8 });
                let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(metodo);
                f(regras, o);
            }
            break;
        }
    }
    let com_a_oferta = |f: usize, t: usize| {
        let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
        f(t, o);
    };
    com_a_oferta(0x8c91e0, this);
    if vendedor != 0 {
        crate::comparacao::ramo("cpx_3 paga a transferencia");
        com_a_oferta(0x8ca8a0, this); // cpx3: o pagamento da transferência
    }
    if cpx3_b(o + 0x1f) != 1 {
        com_a_oferta(0x86cd70, cpx3_d(0xdd7f00));
    }
    com_a_oferta(0x448760, cpx3_d(0xad9dac));
    let troca = cpx3_d(o + 4);
    if troca != 0 {
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        core::ptr::write_unaligned((troca + 0xa) as *mut usize, cpx3_d(troca + 2));
        let q = cpx3_pessoa(cpx3_d(proximo(cpx3_d(o + 4))));
        let clube = cpx3_d(q + 0x39);
        for j in 0..0x32usize {
            let vaga = clube + 0xd7 + j * 4;
            if cpx3_d(vaga) == q {
                *(vaga as *mut usize) = 0;
                break;
            }
        }
    }
    if cpx3_d(o + 0x1b) != 0 && cpx3_d(p + 0x69) == 0 {
        crate::comparacao::ramo("cpx_3 parte dos clubes anteriores");
        com_a_oferta(0x8e4f20, this); // cpx3: a parte dos clubes anteriores
    }
    let mudar: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x8c9720usize);
    mudar(this, p, comprador, o);
    allocate_arrival_squad_numbers(comprador);
    if cpx3_d(o + 4) != 0 {
        for e in cpx3_itens(cpx3_d(o + 4)) {
            mudar(this, cpx3_pessoa(cpx3_d(e)), vendedor, o);
        }
        allocate_arrival_squad_numbers(vendedor);
    }
    *((o + 0x2c) as *mut u8) = 0x14;
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    agendar(this, o, 0xa, 0);
    'marca: {
        if !cpx3_encerrada(o) {
            break 'marca;
        }
        let rivais = cpx3_d(o);
        if rivais != 0 {
            let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
            core::ptr::write_unaligned((rivais + 0xa) as *mut usize, cpx3_d(rivais + 2));
            let mut e = proximo(cpx3_d(o));
            while e != 0 {
                if !cpx3_encerrada(cpx3_d(cpx3_d(this).wrapping_add(cpx3_d(e).wrapping_mul(4)))) {
                    break 'marca;
                }
                e = proximo(cpx3_d(o));
            }
        }
        let m = (cpx3_d(this + 0x86e).wrapping_add(cpx3_d(o + 0xc).wrapping_mul(8))) as *mut u8;
        if *m & 1 != 0 {
            *m &= 0xfe;
        }
    }
    let p = cpx3_pessoa(cpx3_d(o + 0xc));
    let c = get_contract_store_result(p);
    let novo = if c == 0 { 0 } else { cpx3_clube(cpx3_d(c + 4)) };
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    desmarcar(cpx3_d(this + 0x856), cpx3_clube(cpx3_d(o + 8)), p);
    if cpx3_d(o + 4) != 0 {
        for e in cpx3_itens(cpx3_d(o + 4)) {
            desmarcar(cpx3_d(this + 0x856), novo, cpx3_pessoa(cpx3_d(e)));
        }
    }
    com_a_oferta(0x8c6db0, this);
}

// ponytail: muda jogadores de clube, paga e agenda (não pode rodar duas vezes): só a original, com o plano do estado
// final (0x14, que nenhuma chamada depois dele muda); a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_finalise_transfer(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, o);
    let e = cpx3_b(o + 0x2c);
    PLACAR.registrar(e == 0x14, || format!("finalise_transfer(oferta {:#x}) estado={:#x} (plano 0x14)", o, e));
}

/// A notícia local com a pessoa (o id e as três chaves) nas vagas `k` a `k` + 3.
unsafe fn put_person_in_news(nt: usize, k: u8, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    gravar(nt, k as i8, cpx3_d(p) as u32);
    gravar(nt, k.wrapping_add(1) as i8, cpx3_d(cpx3_d(p + 4) + 0x33) as u32);
    gravar(nt, k.wrapping_add(2) as i8, cpx3_d(cpx3_d(p + 8) + 0x33) as u32);
    gravar(nt, k.wrapping_add(3) as i8, cpx3_d(cpx3_d(p + 0xc) + 0x33) as u32);
}

/// A vaga 5 das duas notícias: o id do clube do contrato (sem tipo 5) ou −1 (a original relê o contrato e, sem ele, lê
/// o id no endereço 0, o que não acontece porque acabou de achá-lo).
unsafe fn contract_club_id_or_minus1(p: usize) -> u32 {
    if contract_club_not_type5(p) == 0 {
        return u32::MAX;
    }
    cpx3_d(contract_club_not_type5(p)) as u32
}

/// Os da troca (+4) na notícia a partir da vaga `k` (os três primeiros, quatro vagas cada); devolve (quantos, a vaga
/// seguinte).
unsafe fn put_exchange_players_in_news(nt: usize, o: usize, mut k: u8) -> (i8, u8) {
    let mut n = 0i8;
    let troca = cpx3_d(o + 4);
    if troca != 0 {
        for e in cpx3_itens(troca) {
            if n < 3 {
                put_person_in_news(nt, k, cpx3_pessoa(cpx3_d(e)));
                k = k.wrapping_add(4);
            }
            n = n.wrapping_add(1);
        }
    }
    (n, k)
}

/// A entrega: com técnico humano no comprador (0x7875e0), a todos os técnicos (0x786780); senão a cada destinatário.
unsafe fn deliver_transfer_news(nt: usize, o: usize, lista: &[usize], n: i8) {
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    if humano(cpx3_d(0xb7431c), cpx3_clube(cpx3_d(o + 8))) != 0 {
        let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
        todos(cpx3_d(0xb7431c), nt);
        return;
    }
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    for &d in lista.iter().take(n.max(0) as usize) {
        entregar(cpx3_d(0xb7431c), nt, d);
    }
}

/// { 0x8dbef0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_transfer_completed_news(oferta)`, pelo match_wrap_up da oferta (0x8c34e0). Os clubes humanos que recebem (0x8d1130
/// com 0x5f, até 30); sem nenhum, nada. A notícia 0xfa8: a pessoa, o comprador, o clube do contrato, o valor (+0x1b), 0,
/// o valor total (0x8e7a70 com 1, 1, 0, 1), +0x2f, +0x11, as cláusulas (0x8e7ef0); com +0x2f, contrato de clube e +0x30
/// & 0xf = 3, o fim do contrato se ainda vier (vagas 10 e 11). Os da troca (até 3, da vaga 15) e quantos (14); um
/// sorteio(3) na vaga seguinte; com ficha, 1, os pontos fortes (0x546ab0, sorteia) e as posições (0x5b7d20), senão 0.
unsafe extern "thiscall" fn post_transfer_completed_news(this: usize, o: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x8e7a70usize);
    let clausulas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8e7ef0usize);
    let p = cpx3_pessoa(cpx3_d(o + 0xc));
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, 0x5f, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    crate::comparacao::ramo("cpx_3 noticia da transferencia concluida");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfa8, 0);
    put_person_in_news(nt, 0, p);
    gravar(nt, 4, cpx3_d(o + 8) as u32);
    gravar(nt, 5, contract_club_id_or_minus1(p));
    gravar(nt, 6, cpx3_d(o + 0x1b) as u32);
    gravar(nt, 7, 0);
    gravar(nt, 8, total(o, 1, 1, 0, 1));
    gravar(nt, 9, cpx3_b(o + 0x2f) as i8 as u32);
    gravar(nt, 0xc, cpx3_b(o + 0x11) as i8 as u32);
    gravar(nt, 0xd, clausulas(o));
    if cpx3_b(o + 0x2f) != 0 && contract_club_not_type5(p) != 0 && cpx3_b(o + 0x30) & 0xf == 3 {
        let c = cpx3_contrato_de(p);
        let ano = cpx3_w(0xae2c92) as u16;
        let vem = c != 0
            && (ano < cpx3_w(c + 0x2f) as u16 || (ano == cpx3_w(c + 0x2f) as u16 && cpx3_w(0xae2c90) < cpx3_w(c + 0x2d)));
        if vem {
            gravar(nt, 10, cpx3_w(c + 0x2d) as i32 as u32);
            gravar(nt, 11, cpx3_w(c + 0x2f) as u16 as u32);
        } else {
            gravar(nt, 10, 0);
            gravar(nt, 11, 0);
        }
    }
    let (m, k) = put_exchange_players_in_news(nt, o, 0xf);
    gravar(nt, 0xe, m as i32 as u32);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    gravar(nt, k as i8, sorteio(3) as u32);
    let ficha = cpx3_d(p + 0x61);
    if ficha != 0 {
        gravar(nt, k.wrapping_add(1) as i8, 1);
        let fortes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x546ab0usize);
        let posicoes: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(0x5b7d20usize);
        let clube = contract_club_not_type5(p);
        gravar(nt, k.wrapping_add(2) as i8, fortes(p, clube));
        gravar(nt, k.wrapping_add(3) as i8, posicoes(cpx3_d(p + 0x61)) as i32 as u32);
    } else {
        gravar(nt, k.wrapping_add(1) as i8, 0);
    }
    deliver_transfer_news(nt, o, &lista, n);
}

// ponytail: a notícia sorteia e vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_post_transfer_completed_news(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, o)
}

/// { 0x8e05d0 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_transfer_agreed_news(oferta, data)`, pela aprovação dos clubes (0x8c3310). Os clubes humanos que recebem
/// (0x8d1130 com 0xf, até 30); sem nenhum, nada. A notícia 0xfb3: a pessoa, o comprador, o clube do contrato, o valor
/// (+0x1b), +0x2f, o valor total (0x8e7a70 com 0, 1, 0, 1), o nibble baixo de +0x30 com sinal, a data da conclusão (dia
/// com sinal, ano), os da troca (até 3, da vaga 13) e quantos (12).
unsafe extern "thiscall" fn post_transfer_agreed_news(this: usize, o: usize, data: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x8e7a70usize);
    let p = cpx3_pessoa(cpx3_d(o + 0xc));
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, 0xf, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    crate::comparacao::ramo("cpx_3 noticia da transferencia acertada");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfb3, 0);
    put_person_in_news(nt, 0, p);
    gravar(nt, 4, cpx3_d(o + 8) as u32);
    gravar(nt, 5, contract_club_id_or_minus1(p));
    gravar(nt, 6, cpx3_d(o + 0x1b) as u32);
    gravar(nt, 7, cpx3_b(o + 0x2f) as i8 as u32);
    gravar(nt, 8, total(o, 0, 1, 0, 1));
    gravar(nt, 9, (((cpx3_b(o + 0x30) << 4) as i8) >> 4) as i32 as u32);
    gravar(nt, 0xa, cpx3_w(data) as i32 as u32);
    gravar(nt, 0xb, cpx3_w(data + 2) as u16 as u32);
    let (m, _) = put_exchange_players_in_news(nt, o, 0xd);
    gravar(nt, 0xc, m as i32 as u32);
    deliver_transfer_news(nt, o, &lista, n);
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_post_transfer_agreed_news(this: usize, o: usize, data: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%)(this, o, data)
}
